use crate::web::{broadcast_log, broadcast_status, CleanupInfo, ClientState};
use at_protocol::crypto::{generate_aes_key, public_key_from_pem, rsa_encrypt};
use at_protocol::frame::{read_encrypted, read_frame, write_encrypted, write_frame};
use at_protocol::Message;
use std::io::{Error, ErrorKind, Result};
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;

use crate::{cleanup, registry, scanner};

const SERVER_PORT: u16 = 19800;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(300);
const MAX_RETRIES: u32 = 3;
const RETRY_BASE_DELAY: Duration = Duration::from_secs(10);

/// Run a full cleanup cycle with web UI event broadcasting.
pub async fn run_cleanup_cycle(state: Arc<ClientState>) -> Result<()> {
    let mut last_err = Error::new(ErrorKind::NotFound, "No server found");
    for attempt in 0..MAX_RETRIES {
        if attempt > 0 {
            let delay = RETRY_BASE_DELAY * attempt;
            broadcast_log(
                &state.event_tx,
                "info",
                &format!("Retry {}/{}...", attempt, MAX_RETRIES - 1),
            );
            tokio::time::sleep(delay).await;
        }
        match try_cleanup_cycle(state.clone()).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                log::warn!("Attempt {} failed: {}", attempt + 1, e);
                broadcast_log(
                    &state.event_tx,
                    "warn",
                    &format!("Attempt {} failed: {}", attempt + 1, e),
                );
                last_err = e;
            }
        }
    }
    Err(last_err)
}

async fn try_cleanup_cycle(state: Arc<ClientState>) -> Result<()> {
    // Step 1: Scan
    *state.connection.write().await = "searching".to_string();
    *state.auth.write().await = "pending".to_string();
    broadcast_status(&state).await;
    broadcast_log(&state.event_tx, "info", "Scanning LAN for server...");

    let server_ip = scanner::find_server().await?;
    log::info!("Found AT server at: {}", server_ip);
    *state.connection.write().await = format!("connected to {}", server_ip);
    broadcast_status(&state).await;
    broadcast_log(
        &state.event_tx,
        "success",
        &format!("Found server: {}", server_ip),
    );

    // Step 2: TCP connect
    let addr = match server_ip {
        IpAddr::V4(v4) => std::net::SocketAddr::new(IpAddr::V4(v4), SERVER_PORT),
        IpAddr::V6(v6) => std::net::SocketAddr::new(IpAddr::V6(v6), SERVER_PORT),
    };
    let mut stream = TcpStream::connect(addr).await?;
    log::info!("Connected to server at {}", addr);
    broadcast_log(&state.event_tx, "info", &format!("TCP connected to {}", addr));

    // Step 3: RSA handshake
    let session_key = perform_handshake(&mut stream).await?;
    log::info!("RSA handshake completed, AES session established");
    broadcast_log(&state.event_tx, "success", "Handshake completed");

    // Step 4: Authorization
    let authorized = request_auth(&mut stream, &session_key).await?;
    if !authorized {
        *state.auth.write().await = "denied".to_string();
        broadcast_status(&state).await;
        broadcast_log(&state.event_tx, "error", "Server denied authorization");
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "Server denied authorization",
        ));
    }
    *state.auth.write().await = "authorized".to_string();
    broadcast_status(&state).await;
    broadcast_log(&state.event_tx, "success", "Authorization granted");

    // Step 5: Cleanup
    let (scanned, removed) = cleanup::cleanup_phantom_devices()?;
    log::info!(
        "Phantom device cleanup: scanned {}, removed {}",
        scanned,
        removed
    );
    broadcast_log(
        &state.event_tx,
        "info",
        &format!("Device cleanup: scanned {}, removed {}", scanned, removed),
    );

    let registry_cleaned = match registry::clean_display_registry() {
        Ok(n) if n > 0 => {
            log::info!("Registry cleanup: deleted {} additional items", n);
            broadcast_log(
                &state.event_tx,
                "info",
                &format!("Registry cleanup: {} items", n),
            );
            n as u32
        }
        Ok(_) => 0,
        Err(e) => {
            log::debug!("Registry cleanup skipped: {}", e);
            0
        }
    };

    let ids_randomized = match crate::hwid::randomize_machine_ids() {
        Ok((guid, pid)) => {
            log::info!("Machine IDs randomized: guid={}, pid={}", guid, pid);
            broadcast_log(&state.event_tx, "success", "Machine IDs randomized");
            true
        }
        Err(e) => {
            log::warn!("Machine ID randomization failed: {}", e);
            broadcast_log(
                &state.event_tx,
                "warn",
                &format!("Machine ID randomization failed: {}", e),
            );
            false
        }
    };

    // Update cleanup info in state
    *state.cleanup_info.write().await = Some(CleanupInfo {
        scanned: scanned as u32,
        removed: removed as u32,
        registry_cleaned,
        ids_randomized,
    });
    let _ = state.event_tx.send(crate::web::ClientEvent::CleanupResult {
        scanned: scanned as u32,
        removed: removed as u32,
        registry_cleaned,
        ids_randomized,
    });

    // Step 6: Heartbeat loop
    broadcast_log(&state.event_tx, "info", "Entering heartbeat loop");
    *state.heartbeat.write().await = "active".to_string();
    broadcast_status(&state).await;
    heartbeat_loop(&mut stream, &session_key, state.clone()).await?;

    Ok(())
}

async fn perform_handshake(stream: &mut TcpStream) -> Result<[u8; 32]> {
    let handshake_msg = Message::Handshake;
    let json = serde_json::to_vec(&handshake_msg)
        .map_err(|e| Error::new(ErrorKind::Other, format!("serialize error: {}", e)))?;
    write_frame(stream, &json).await?;

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

    let server_pubkey = public_key_from_pem(&public_key_pem)?;
    let session_key = generate_aes_key();

    let encrypted_key = rsa_encrypt(&server_pubkey, &session_key)?;
    let encoded_key =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &encrypted_key);

    let key_exchange_msg = Message::KeyExchange {
        encrypted_key: encoded_key,
    };
    let ke_json = serde_json::to_vec(&key_exchange_msg)
        .map_err(|e| Error::new(ErrorKind::Other, format!("serialize error: {}", e)))?;
    write_frame(stream, &ke_json).await?;

    let ok_msg = read_encrypted(stream, &session_key).await?;
    match ok_msg {
        Message::KeyExchangeOk => {}
        other => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Expected KeyExchangeOk, got: {:?}", other),
            ));
        }
    }

    Ok(session_key)
}

async fn request_auth(stream: &mut TcpStream, session_key: &[u8; 32]) -> Result<bool> {
    let hostname = gethostname::gethostname()
        .into_string()
        .unwrap_or_else(|_| "unknown".to_string());

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

async fn heartbeat_loop(
    stream: &mut TcpStream,
    session_key: &[u8; 32],
    state: Arc<ClientState>,
) -> Result<()> {
    loop {
        tokio::time::sleep(HEARTBEAT_INTERVAL).await;

        let hb = Message::Heartbeat;
        if let Err(e) = write_encrypted(stream, session_key, &hb).await {
            log::warn!("Heartbeat send failed: {}", e);
            *state.heartbeat.write().await = format!("failed: {}", e);
            broadcast_status(&state).await;
            broadcast_log(
                &state.event_tx,
                "error",
                &format!("Heartbeat failed: {}", e),
            );
            return Err(e);
        }

        match tokio::time::timeout(Duration::from_secs(30), read_encrypted(stream, session_key))
            .await
        {
            Ok(Ok(Message::HeartbeatAck)) => {
                log::debug!("Heartbeat acknowledged");
                let now = chrono::Local::now().format("%H:%M:%S").to_string();
                *state.heartbeat.write().await = format!("active ({})", now);
                broadcast_status(&state).await;
            }
            Ok(Ok(other)) => {
                log::warn!("Unexpected message during heartbeat: {:?}", other);
            }
            Ok(Err(e)) => {
                log::warn!("Heartbeat read error: {}", e);
                *state.heartbeat.write().await = format!("failed: {}", e);
                broadcast_status(&state).await;
                return Err(e);
            }
            Err(_) => {
                *state.heartbeat.write().await = "timeout".to_string();
                broadcast_status(&state).await;
                return Err(Error::new(
                    ErrorKind::TimedOut,
                    "Heartbeat acknowledgment timed out",
                ));
            }
        }
    }
}

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
                return Some(iface.name.clone());
            }
        }
    }
    None
}
