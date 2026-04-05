use hwt_protocol::crypto::{generate_aes_key, public_key_from_pem, rsa_encrypt};
use hwt_protocol::frame::{read_encrypted, read_frame, write_encrypted, write_frame};
use hwt_protocol::Message;
use std::io::{Error, ErrorKind, Result};
use std::net::IpAddr;
use std::time::Duration;
use tokio::net::TcpStream;

use crate::{cleanup, registry, scanner};

const SERVER_PORT: u16 = 19800;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
const MAX_RETRIES: u32 = 3;
const RETRY_BASE_DELAY: Duration = Duration::from_secs(10);

/// Run a full cleanup cycle: scan -> connect -> handshake -> auth -> cleanup -> heartbeat.
pub async fn run_cleanup_cycle() -> Result<()> {
    // Retry finding and connecting to server
    let mut last_err = Error::new(ErrorKind::NotFound, "No server found");
    for attempt in 0..MAX_RETRIES {
        if attempt > 0 {
            let delay = RETRY_BASE_DELAY * attempt;
            log::info!("Retry {}/{} in {:?}...", attempt, MAX_RETRIES - 1, delay);
            tokio::time::sleep(delay).await;
        }
        match try_cleanup_cycle().await {
            Ok(()) => return Ok(()),
            Err(e) => {
                log::warn!("Attempt {} failed: {}", attempt + 1, e);
                last_err = e;
            }
        }
    }
    Err(last_err)
}

async fn try_cleanup_cycle() -> Result<()> {
    // Step 1: Scan the local network for the HWT server
    let server_ip = scanner::find_server().await?;
    log::info!("Found HWT server at: {}", server_ip);

    // Step 2: TCP connect
    let addr = match server_ip {
        IpAddr::V4(v4) => std::net::SocketAddr::new(IpAddr::V4(v4), SERVER_PORT),
        IpAddr::V6(v6) => std::net::SocketAddr::new(IpAddr::V6(v6), SERVER_PORT),
    };
    let mut stream = TcpStream::connect(addr).await?;
    log::info!("Connected to server at {}", addr);

    // Step 3: RSA handshake
    let session_key = perform_handshake(&mut stream).await?;
    log::info!("RSA handshake completed, AES session established");

    // Step 4: Request authorization
    let authorized = request_auth(&mut stream, &session_key).await?;
    if !authorized {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "Server denied authorization",
        ));
    }
    log::info!("Authorization granted by server");

    // Step 5: Execute cleanup tasks
    // Setup API DIF_REMOVE handles both device removal AND its registry cleanup,
    // so we run phantom device cleanup first (which covers DISPLAY registry entries).
    // Registry direct deletion is kept as fallback for SYSTEM-level service.
    let (scanned, removed) = cleanup::cleanup_phantom_devices()?;
    log::info!(
        "Phantom device cleanup: scanned {}, removed {}",
        scanned,
        removed
    );

    // Fallback: try direct registry deletion (works when running as SYSTEM service)
    match registry::clean_display_registry() {
        Ok(n) if n > 0 => log::info!("Registry cleanup: deleted {} additional items", n),
        Ok(_) => log::debug!("Registry cleanup: no additional items to delete"),
        Err(e) => log::debug!("Registry cleanup skipped (expected if not SYSTEM): {}", e),
    }

    // Step 5b: Randomize machine identifiers (Device ID + Product ID)
    match crate::hwid::randomize_machine_ids() {
        Ok((guid, pid)) => log::info!("Machine IDs randomized: guid={}, pid={}", guid, pid),
        Err(e) => log::warn!("Machine ID randomization failed: {}", e),
    }

    // Step 6: Heartbeat loop
    log::info!("Entering heartbeat loop (interval: {:?})", HEARTBEAT_INTERVAL);
    heartbeat_loop(&mut stream, &session_key).await?;

    Ok(())
}

/// Perform the RSA handshake and establish an AES-256 session key.
async fn perform_handshake(stream: &mut TcpStream) -> Result<[u8; 32]> {
    // 3a: Send Handshake (plaintext frame)
    let handshake_msg = Message::Handshake;
    let json = serde_json::to_vec(&handshake_msg)
        .map_err(|e| Error::new(ErrorKind::Other, format!("serialize error: {}", e)))?;
    write_frame(stream, &json).await?;

    // 3b: Receive HandshakeResponse with server's RSA public key
    let response_data = read_frame(stream).await?;
    let response: Message = serde_json::from_slice(&response_data)
        .map_err(|e| Error::new(ErrorKind::Other, format!("deserialize error: {}", e)))?;

    let public_key_pem = match response {
        Message::HandshakeResponse { public_key } => public_key,
        other => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Expected HandshakeResponse, got: {:?}", other),
            ));
        }
    };

    // 3c: Parse the server's RSA public key
    let server_pubkey = public_key_from_pem(&public_key_pem)?;

    // 3d: Generate random AES-256 session key
    let session_key = generate_aes_key();

    // 3e: RSA-encrypt the session key and send KeyExchange
    let encrypted_key = rsa_encrypt(&server_pubkey, &session_key)?;
    let encoded_key = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &encrypted_key,
    );

    let key_exchange_msg = Message::KeyExchange {
        encrypted_key: encoded_key,
    };
    let ke_json = serde_json::to_vec(&key_exchange_msg)
        .map_err(|e| Error::new(ErrorKind::Other, format!("serialize error: {}", e)))?;
    write_frame(stream, &ke_json).await?;

    // 3f: Receive KeyExchangeOk (encrypted with session key, to verify)
    let ok_msg = read_encrypted(stream, &session_key).await?;
    match ok_msg {
        Message::KeyExchangeOk => {
            log::debug!("Key exchange confirmed by server");
        }
        other => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Expected KeyExchangeOk, got: {:?}", other),
            ));
        }
    }

    Ok(session_key)
}

/// Send an auth request and check the response.
async fn request_auth(stream: &mut TcpStream, session_key: &[u8; 32]) -> Result<bool> {
    let hostname = gethostname::gethostname()
        .into_string()
        .unwrap_or_else(|_| "unknown".to_string());

    // Try to get a MAC address for identification
    let client_mac = get_mac_address();

    let auth_msg = Message::AuthRequest {
        client_id: hostname.clone(),
        client_mac,
    };
    write_encrypted(stream, session_key, &auth_msg).await?;

    let response = read_encrypted(stream, session_key).await?;
    match response {
        Message::AuthResponse {
            authorized,
            message,
            server_time: _,
        } => {
            if authorized {
                log::info!("Auth response: authorized ({})", message);
            } else {
                log::warn!("Auth response: denied ({})", message);
            }
            Ok(authorized)
        }
        other => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Expected AuthResponse, got: {:?}", other),
        )),
    }
}

/// Heartbeat loop: send heartbeats every HEARTBEAT_INTERVAL.
/// Returns when the connection is lost.
async fn heartbeat_loop(stream: &mut TcpStream, session_key: &[u8; 32]) -> Result<()> {
    loop {
        tokio::time::sleep(HEARTBEAT_INTERVAL).await;

        let hb = Message::Heartbeat;
        if let Err(e) = write_encrypted(stream, session_key, &hb).await {
            log::warn!("Heartbeat send failed: {}", e);
            return Err(e);
        }

        match tokio::time::timeout(Duration::from_secs(30), read_encrypted(stream, session_key))
            .await
        {
            Ok(Ok(Message::HeartbeatAck)) => {
                log::debug!("Heartbeat acknowledged");
            }
            Ok(Ok(other)) => {
                log::warn!("Unexpected message during heartbeat: {:?}", other);
            }
            Ok(Err(e)) => {
                log::warn!("Heartbeat read error: {}", e);
                return Err(e);
            }
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::TimedOut,
                    "Heartbeat acknowledgment timed out",
                ));
            }
        }
    }
}

/// Try to get a MAC address from the system.
fn get_mac_address() -> Option<String> {
    if let Ok(ifaces) = if_addrs::get_if_addrs() {
        for iface in &ifaces {
            if iface.is_loopback() {
                continue;
            }
            if let std::net::IpAddr::V4(ipv4) = iface.ip() {
                if ipv4.octets()[0] == 169 && ipv4.octets()[1] == 254 {
                    continue;
                }
                // Return the interface name as a pseudo-identifier on non-Windows
                // On Windows, a proper MAC could be obtained via GetAdaptersAddresses
                return Some(iface.name.clone());
            }
        }
    }
    None
}
