//! at-heartbeat — lightweight licensing heartbeat, runs as a normal user.
//!
//! The main `at-client` runs as a SYSTEM service and, at boot, performs the
//! privileged one-shot cleanup (phantom-device removal, display-registry
//! cleanup, machine-ID randomization). Those behaviors — plus running as
//! SYSTEM and sweeping the LAN — are exactly what anti-cheat engines such as
//! Tencent ACE (used by 三角洲行动 / Delta Force) flag as a cheat loader, which
//! crashed the game whenever the service was resident.
//!
//! So once boot cleanup is done, the service hands the discovered server IP to
//! THIS process and exits. This process:
//!   * runs as the logged-in user (never SYSTEM),
//!   * connects DIRECTLY to the known server IP (never scans the LAN),
//!   * links none of the SetupAPI / winreg / scanner code,
//!   * goes dormant whenever a game/anti-cheat process is running.
//! That keeps the licensing heartbeat alive without tripping the anti-cheat.

use std::io::{Error, ErrorKind, Result};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use at_protocol::crypto::{generate_aes_key, public_key_from_pem, rsa_encrypt};
use at_protocol::frame::{read_encrypted, read_frame, write_encrypted, write_frame};
use at_protocol::Message;
use tokio::net::TcpStream;

const SERVER_PORT: u16 = 19800;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(300);
const RECONNECT_DELAY: Duration = Duration::from_secs(30);
const GAME_POLL_DELAY: Duration = Duration::from_secs(15);
const ACK_TIMEOUT: Duration = Duration::from_secs(30);

// Escalation thresholds — mirror client/src/escalation.rs so licensing
// enforcement behaves identically once the heartbeat moved out of the service.
const PURCHASE_URL: &str = "https://m.tb.cn/h.imD9Vqp?tk=SSp054qFuub";
const OPEN_BROWSER_THRESHOLD: u32 = 5;
const WARN_SHUTDOWN_THRESHOLD: u32 = 8;
const SHUTDOWN_THRESHOLD: u32 = 9;

/// Process names (lower-cased) that mean "an anti-cheat-protected game is
/// running, stay out of its way". ACE's own helper processes are the reliable
/// signal — they are present for ANY ACE-protected title — plus a few likely
/// Delta Force client names. Adjust this list to match real process names
/// observed on the target machines.
const GAME_ACE_PROCESSES: &[&str] = &[
    "ace-tray.exe",
    "sguard64.exe",
    "sguardsvc64.exe",
    "ace-base.exe",
    "anticheatexpert.exe",
    "deltaforceclient-win64-shipping.exe",
    "deltaforce.exe",
    "df.exe",
];

fn main() {
    let server_ip: IpAddr = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));

    log_line(&format!("at-heartbeat started, server={}", server_ip));

    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            log_line(&format!("fatal: failed to create runtime: {}", e));
            return;
        }
    };
    rt.block_on(run(server_ip));
}

async fn run(server_ip: IpAddr) {
    // Counts consecutive failures to reach an AUTHORIZED session (server
    // unreachable or authorization denied). A mid-session heartbeat drop does
    // NOT count — that just triggers a silent reconnect.
    let mut fail_count: u32 = 0;

    loop {
        if game_running() {
            log_line("game/anti-cheat detected — heartbeat dormant");
            tokio::time::sleep(GAME_POLL_DELAY).await;
            continue;
        }

        match connect_and_auth(server_ip).await {
            Ok((mut stream, key, true)) => {
                fail_count = 0;
                log_line("authorized — entering heartbeat loop");
                heartbeat_until_drop_or_game(&mut stream, &key).await;
            }
            Ok((_, _, false)) => {
                fail_count += 1;
                log_line(&format!("authorization denied ({})", fail_count));
                escalate(fail_count);
            }
            Err(e) => {
                fail_count += 1;
                log_line(&format!("connect/auth failed ({}): {}", fail_count, e));
                escalate(fail_count);
            }
        }

        tokio::time::sleep(RECONNECT_DELAY).await;
    }
}

/// Connect directly to the known server IP, do the RSA handshake, and request
/// authorization. Returns (stream, session_key, authorized).
async fn connect_and_auth(server_ip: IpAddr) -> Result<(TcpStream, [u8; 32], bool)> {
    let addr = SocketAddr::new(server_ip, SERVER_PORT);
    let mut stream = tokio::time::timeout(Duration::from_secs(10), TcpStream::connect(addr))
        .await
        .map_err(|_| Error::new(ErrorKind::TimedOut, "connect timed out"))??;

    let session_key = perform_handshake(&mut stream).await?;
    let authorized = request_auth(&mut stream, &session_key).await?;
    Ok((stream, session_key, authorized))
}

/// Heartbeat until the connection drops or a game launches. Connection drops
/// are normal (network blips) and just end the loop so `run` reconnects — they
/// are NOT treated as licensing failures.
async fn heartbeat_until_drop_or_game(stream: &mut TcpStream, key: &[u8; 32]) {
    loop {
        if game_running() {
            log_line("game/anti-cheat launched — dropping heartbeat connection");
            return;
        }
        tokio::time::sleep(HEARTBEAT_INTERVAL).await;
        if game_running() {
            log_line("game/anti-cheat launched — dropping heartbeat connection");
            return;
        }

        if let Err(e) = write_encrypted(stream, key, &Message::Heartbeat).await {
            log_line(&format!("heartbeat send failed: {}", e));
            return;
        }

        match tokio::time::timeout(ACK_TIMEOUT, read_encrypted(stream, key)).await {
            Ok(Ok(Message::HeartbeatAck)) => log_line("heartbeat ack"),
            Ok(Ok(other)) => log_line(&format!("unexpected message: {:?}", other)),
            Ok(Err(e)) => {
                log_line(&format!("heartbeat read failed: {}", e));
                return;
            }
            Err(_) => {
                log_line("heartbeat ack timed out");
                return;
            }
        }
    }
}

async fn perform_handshake(stream: &mut TcpStream) -> Result<[u8; 32]> {
    let json = serde_json::to_vec(&Message::Handshake)
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
                format!("expected HandshakeResponse, got: {:?}", other),
            ))
        }
    };

    let server_pubkey = public_key_from_pem(&public_key_pem)?;
    let session_key = generate_aes_key();
    let encrypted_key = rsa_encrypt(&server_pubkey, &session_key)?;
    let encoded_key =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &encrypted_key);

    let ke_json = serde_json::to_vec(&Message::KeyExchange {
        encrypted_key: encoded_key,
    })
    .map_err(|e| Error::new(ErrorKind::Other, format!("serialize error: {}", e)))?;
    write_frame(stream, &ke_json).await?;

    match read_encrypted(stream, &session_key).await? {
        Message::KeyExchangeOk => Ok(session_key),
        other => Err(Error::new(
            ErrorKind::InvalidData,
            format!("expected KeyExchangeOk, got: {:?}", other),
        )),
    }
}

async fn request_auth(stream: &mut TcpStream, session_key: &[u8; 32]) -> Result<bool> {
    let hostname = gethostname::gethostname()
        .into_string()
        .unwrap_or_else(|_| "unknown".to_string());

    let auth_msg = Message::AuthRequest {
        client_id: hostname,
        client_mac: get_mac_address(),
    };
    write_encrypted(stream, session_key, &auth_msg).await?;

    match read_encrypted(stream, session_key).await? {
        Message::AuthResponse { authorized, .. } => Ok(authorized),
        other => Err(Error::new(
            ErrorKind::InvalidData,
            format!("expected AuthResponse, got: {:?}", other),
        )),
    }
}

fn get_mac_address() -> Option<String> {
    if let Ok(ifaces) = if_addrs::get_if_addrs() {
        for iface in &ifaces {
            if iface.is_loopback() {
                continue;
            }
            if let IpAddr::V4(ipv4) = iface.ip() {
                if ipv4.octets()[0] == 169 && ipv4.octets()[1] == 254 {
                    continue;
                }
                return Some(iface.name.clone());
            }
        }
    }
    None
}

// ─── Escalation (open browser → warn → shutdown) ──────────────────────────

fn escalate(count: u32) {
    if count == OPEN_BROWSER_THRESHOLD {
        open_browser(PURCHASE_URL);
    } else if count == WARN_SHUTDOWN_THRESHOLD {
        open_browser(PURCHASE_URL);
        show_warning(
            "AT 警告",
            "授权已过期！\n系统将在下次检查后关机。\n请及时购买授权。",
        );
    } else if count >= SHUTDOWN_THRESHOLD {
        show_warning("AT 关机", "授权已过期！\n系统将在 30 秒后关机。");
        shutdown_system();
    }
}

#[cfg(windows)]
fn open_browser(url: &str) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    match std::process::Command::new("cmd")
        .args(["/c", "start", "", url])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
    {
        Ok(_) => log_line(&format!("opened browser: {}", url)),
        Err(e) => log_line(&format!("open_browser failed: {}", e)),
    }
}

#[cfg(not(windows))]
fn open_browser(url: &str) {
    log_line(&format!("[non-windows] would open browser: {}", url));
}

#[cfg(windows)]
fn show_warning(title: &str, message: &str) {
    // Runs in the user session, so a plain MessageBox shows on the desktop.
    // Spawn a thread because MessageBoxW blocks until dismissed.
    let title = title.to_string();
    let message = message.to_string();
    std::thread::spawn(move || {
        use windows::core::PCWSTR;
        use windows::Win32::UI::WindowsAndMessaging::{
            MessageBoxW, MB_ICONWARNING, MB_OK, MB_SETFOREGROUND, MB_TOPMOST,
        };
        let title_w: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();
        let msg_w: Vec<u16> = message.encode_utf16().chain(Some(0)).collect();
        unsafe {
            MessageBoxW(
                None,
                PCWSTR(msg_w.as_ptr()),
                PCWSTR(title_w.as_ptr()),
                MB_OK | MB_ICONWARNING | MB_TOPMOST | MB_SETFOREGROUND,
            );
        }
    });
}

#[cfg(not(windows))]
fn show_warning(title: &str, message: &str) {
    log_line(&format!("[non-windows] warning: {} — {}", title, message));
}

#[cfg(windows)]
fn shutdown_system() {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    match std::process::Command::new("shutdown")
        .args(["/s", "/t", "30", "/c", "AT：授权已过期，系统即将关机。"])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
    {
        Ok(_) => log_line("shutdown scheduled (30s)"),
        Err(e) => log_line(&format!("shutdown failed: {}", e)),
    }
}

#[cfg(not(windows))]
fn shutdown_system() {
    log_line("[non-windows] would shut down system");
}

// ─── Game / anti-cheat detection ──────────────────────────────────────────

#[cfg(windows)]
fn game_running() -> bool {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
        TH32CS_SNAPPROCESS,
    };

    unsafe {
        let snap = match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
            Ok(h) => h,
            Err(_) => return false,
        };

        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        let mut found = false;
        if Process32FirstW(snap, &mut entry).is_ok() {
            loop {
                let len = entry
                    .szExeFile
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(entry.szExeFile.len());
                let name = String::from_utf16_lossy(&entry.szExeFile[..len]).to_lowercase();
                if GAME_ACE_PROCESSES.iter().any(|p| name == *p) {
                    found = true;
                    break;
                }
                if Process32NextW(snap, &mut entry).is_err() {
                    break;
                }
            }
        }

        let _ = CloseHandle(snap);
        found
    }
}

#[cfg(not(windows))]
fn game_running() -> bool {
    false
}

// ─── Logging ──────────────────────────────────────────────────────────────

/// Append a timestamped line to a plaintext log in the temp dir. Kept dead
/// simple and dependency-light; the temp dir is always writable by the user.
fn log_line(msg: &str) {
    use std::io::Write;
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let path = std::env::temp_dir().join("at-heartbeat.log");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "[{}] {}", ts, msg);
    }
}
