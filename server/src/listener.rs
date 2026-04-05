use chrono::Local;
use hwt_protocol::crypto::{generate_rsa_keypair, public_key_to_pem, rsa_decrypt};
use hwt_protocol::frame::{read_encrypted, read_frame, write_encrypted, write_frame};
use hwt_protocol::Message;
use std::io::{Error, ErrorKind, Result};
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

const LISTEN_PORT: u16 = 19800;

/// Information about a connected client, displayed in the GUI.
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub ip: IpAddr,
    pub client_id: String,
    pub connected_at: chrono::DateTime<chrono::Local>,
}

/// Start TCP listener on 0.0.0.0:19800 and handle incoming client connections.
///
/// `authorized` is a shared flag indicating whether this server is authorized.
/// `clients` is the shared list of online clients shown in the GUI.
/// `log_tx` sends log messages to the GUI.
pub async fn start_listener(
    authorized: Arc<AtomicBool>,
    clients: Arc<Mutex<Vec<ClientInfo>>>,
    log_tx: std::sync::mpsc::Sender<String>,
) -> Result<()> {
    // Generate RSA keypair for this session (regenerated on each server start)
    let (rsa_private, rsa_public) = generate_rsa_keypair();
    let public_key_pem = public_key_to_pem(&rsa_public);

    let listener = TcpListener::bind(("0.0.0.0", LISTEN_PORT)).await?;
    let _ = log_tx.send(format!(
        "[{}] TCP listener started on 0.0.0.0:{}",
        Local::now().format("%H:%M:%S"),
        LISTEN_PORT,
    ));

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        let auth = authorized.clone();
        let key = rsa_private.clone();
        let pem = public_key_pem.clone();
        let clients_list = clients.clone();
        let ltx = log_tx.clone();

        tokio::spawn(async move {
            if let Err(e) =
                handle_client(stream, peer_addr, auth, key, pem, clients_list, ltx.clone()).await
            {
                log::error!("Client {} error: {}", peer_addr, e);
                let _ = ltx.send(format!(
                    "[{}] Client {} disconnected: {}",
                    Local::now().format("%H:%M:%S"),
                    peer_addr,
                    e,
                ));
            }
        });
    }
}

async fn handle_client(
    mut stream: TcpStream,
    peer_addr: SocketAddr,
    authorized: Arc<AtomicBool>,
    rsa_private: rsa::RsaPrivateKey,
    public_key_pem: String,
    clients: Arc<Mutex<Vec<ClientInfo>>>,
    log_tx: std::sync::mpsc::Sender<String>,
) -> Result<()> {
    let _ = log_tx.send(format!(
        "[{}] New connection from {}",
        Local::now().format("%H:%M:%S"),
        peer_addr,
    ));

    // === RSA Handshake ===

    // 1. Read client handshake (plaintext frame)
    let frame = read_frame(&mut stream).await?;
    let msg: Message = serde_json::from_slice(&frame)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("invalid handshake: {e}")))?;

    match msg {
        Message::Handshake => {}
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "expected Handshake message",
            ));
        }
    }

    // 2. Send RSA public key (plaintext frame)
    let resp = Message::HandshakeResponse {
        public_key: public_key_pem,
    };
    let resp_json = serde_json::to_vec(&resp)
        .map_err(|e| Error::new(ErrorKind::Other, format!("serialize error: {e}")))?;
    write_frame(&mut stream, &resp_json).await?;

    // 3. Read key_exchange (plaintext frame)
    let frame = read_frame(&mut stream).await?;
    let msg: Message = serde_json::from_slice(&frame)
        .map_err(|e| Error::new(ErrorKind::InvalidData, format!("invalid key_exchange: {e}")))?;

    let session_key: [u8; 32] = match msg {
        Message::KeyExchange { encrypted_key } => {
            use base64::engine::general_purpose::STANDARD as BASE64;
            use base64::Engine;
            let encrypted_bytes = BASE64
                .decode(&encrypted_key)
                .map_err(|e| Error::new(ErrorKind::InvalidData, format!("base64 decode: {e}")))?;
            let decrypted = rsa_decrypt(&rsa_private, &encrypted_bytes)?;
            if decrypted.len() != 32 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("expected 32-byte key, got {}", decrypted.len()),
                ));
            }
            let mut key = [0u8; 32];
            key.copy_from_slice(&decrypted);
            key
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "expected KeyExchange message",
            ));
        }
    };

    // 4. Send key_exchange_ok (encrypted frame using session key)
    write_encrypted(&mut stream, &session_key, &Message::KeyExchangeOk).await?;

    let _ = log_tx.send(format!(
        "[{}] Handshake completed with {}",
        Local::now().format("%H:%M:%S"),
        peer_addr,
    ));

    // === Business Loop ===
    loop {
        let msg = read_encrypted(&mut stream, &session_key).await?;
        match msg {
            Message::AuthRequest {
                client_id,
                client_mac: _,
            } => {
                let is_auth = authorized.load(Ordering::Relaxed);
                let resp = Message::AuthResponse {
                    authorized: is_auth,
                    message: if is_auth {
                        "Authorized".to_string()
                    } else {
                        "Server not authorized".to_string()
                    },
                    server_time: now_unix_timestamp(),
                };
                write_encrypted(&mut stream, &session_key, &resp).await?;

                if is_auth {
                    let info = ClientInfo {
                        ip: peer_addr.ip(),
                        client_id: client_id.clone(),
                        connected_at: Local::now(),
                    };
                    clients.lock().await.push(info);
                    let _ = log_tx.send(format!(
                        "[{}] Client {} ({}) authorized",
                        Local::now().format("%H:%M:%S"),
                        client_id,
                        peer_addr.ip(),
                    ));
                }
            }
            Message::Heartbeat => {
                write_encrypted(&mut stream, &session_key, &Message::HeartbeatAck).await?;
            }
            _ => {
                // Unknown message type, disconnect
                break;
            }
        }
    }

    // Remove from online list
    let peer_ip = peer_addr.ip();
    clients.lock().await.retain(|c| c.ip != peer_ip);
    let _ = log_tx.send(format!(
        "[{}] Client {} disconnected",
        Local::now().format("%H:%M:%S"),
        peer_addr,
    ));

    Ok(())
}

fn now_unix_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
