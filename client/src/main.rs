mod cleanup;
mod escalation;
mod hwid;
mod protocol;
mod registry;
mod scanner;
mod service;
mod web;

use at_protocol::encrypted_log::EncryptedLogWriter;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use web::{ClientEvent, ClientState};

const CLOUD_PUBLIC_KEY_PEM: &str =
    include_str!(concat!(env!("OUT_DIR"), "/cloud_public_key.pem"));

fn log_file_path() -> std::path::PathBuf {
    let ts = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let base = if cfg!(windows) {
        std::path::PathBuf::from(r"C:\ProgramData\AT\logs")
    } else {
        std::env::temp_dir().join("at-logs")
    };
    base.join(format!("at-client-{}.log.enc", ts))
}

fn init_file_logger_if_possible() {
    if CLOUD_PUBLIC_KEY_PEM.trim().is_empty() {
        log::warn!(
            "Encrypted file log disabled: CLOUD_PUBLIC_KEY_PEM is empty in this build"
        );
        return;
    }
    let path = log_file_path();
    match EncryptedLogWriter::create(&path, CLOUD_PUBLIC_KEY_PEM) {
        Ok(w) => {
            web::init_file_logger(w);
            log::info!("Encrypted log file: {}", path.display());
        }
        Err(e) => {
            log::error!("Failed to open encrypted log {}: {}", path.display(), e);
        }
    }
}

#[cfg(windows)]
fn disable_quick_edit() {
    use windows::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, SetConsoleMode, CONSOLE_MODE, ENABLE_EXTENDED_FLAGS,
        ENABLE_QUICK_EDIT_MODE, STD_INPUT_HANDLE,
    };
    unsafe {
        if let Ok(h) = GetStdHandle(STD_INPUT_HANDLE) {
            let mut mode = CONSOLE_MODE(0);
            if GetConsoleMode(h, &mut mode).is_ok() {
                let new = CONSOLE_MODE(
                    (mode.0 & !ENABLE_QUICK_EDIT_MODE.0) | ENABLE_EXTENDED_FLAGS.0,
                );
                let _ = SetConsoleMode(h, new);
            }
        }
    }
}

fn main() {
    #[cfg(windows)]
    disable_quick_edit();
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("install") => {
            init_env_logger();
            service::install();
        }
        Some("uninstall") => {
            init_env_logger();
            service::uninstall();
        }
        Some("status") => {
            init_env_logger();
            service::status();
        }
        Some("run") => run_foreground(),
        // `boot` is what the scheduled task invokes (as SYSTEM) on every logon:
        // one-shot privileged cleanup, hand off the heartbeat, then exit. The
        // bare no-arg invocation does the same thing.
        Some("boot") => run_boot(),
        _ => run_boot(),
    }
}

/// One-shot boot run, invoked by the scheduled task as SYSTEM on every logon.
/// Performs the privileged cleanup once, hands the licensing heartbeat off to a
/// normal-user process, then exits. Never registers a service and never stays
/// resident — so nothing AT-related is present while a game later runs.
fn run_boot() {
    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    let server_ip = rt.block_on(async {
        let (event_tx, _) = broadcast::channel::<ClientEvent>(256);
        web::init_logger(event_tx.clone());
        init_file_logger_if_possible();
        let state = Arc::new(ClientState::new(event_tx));

        log::info!("at-client boot run starting");
        match protocol::run_cleanup_cycle(state.clone()).await {
            Ok(server_ip) => {
                log::info!("Boot cleanup completed; server at {}", server_ip);
                escalation::on_cycle_success(&state);
                Some(server_ip)
            }
            Err(e) => {
                log::error!("Boot cleanup failed: {}", e);
                escalation::on_cycle_failure(&state).await;
                None
            }
        }
    });

    if let Some(ip) = server_ip {
        launch_heartbeat_user_session(ip);
    }
    log::info!("at-client boot run finished");
}

/// Launch the heartbeat into the active console (user) session. Because `boot`
/// runs as SYSTEM in session 0, a plain spawn would land in the wrong session;
/// we must create the process against the active user token. The console
/// session may not be ready for a moment around logon, so retry briefly.
#[cfg(windows)]
fn launch_heartbeat_user_session(server_ip: std::net::IpAddr) {
    let path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join("at-heartbeat.exe")));
    let Some(path) = path else {
        log::error!("Cannot resolve exe directory; heartbeat not launched");
        return;
    };
    if !path.exists() {
        log::error!(
            "at-heartbeat.exe not found at {} — heartbeat not launched",
            path.display()
        );
        return;
    }
    let cmdline = format!("\"{}\" {}", path.display(), server_ip);
    for _ in 0..12 {
        if escalation::spawn_in_active_session(&cmdline) {
            log::info!("Launched heartbeat in user session: {}", path.display());
            return;
        }
        std::thread::sleep(Duration::from_secs(5));
    }
    log::error!("Failed to launch heartbeat in user session after retries");
}

#[cfg(not(windows))]
fn launch_heartbeat_user_session(server_ip: std::net::IpAddr) {
    spawn_heartbeat_sibling(server_ip);
}

fn init_env_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();
}

/// Run the cleanup cycle in foreground mode (for debugging).
fn run_foreground() {
    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    rt.block_on(async {
        let (event_tx, _) = broadcast::channel::<ClientEvent>(256);
        web::init_logger(event_tx.clone());
        init_file_logger_if_possible();

        let state = Arc::new(ClientState::new(event_tx));

        // Spawn web UI server
        tokio::spawn(web::start_web_server(state.clone()));

        log::info!("Starting at-client in foreground mode");

        loop {
            match protocol::run_cleanup_cycle(state.clone()).await {
                Ok(server_ip) => {
                    log::info!("Boot cleanup completed; server at {}", server_ip);
                    escalation::on_cycle_success(&state);
                    // Mirror the service: hand off to the sibling heartbeat
                    // binary and stop. In foreground we are already in the user
                    // session, so a plain spawn is enough.
                    spawn_heartbeat_sibling(server_ip);
                    break;
                }
                Err(e) => {
                    log::error!("Boot cleanup failed: {}", e);
                    escalation::on_cycle_failure(&state).await;
                }
            }
            // Reset status for next attempt
            *state.connection.write().await = "idle".to_string();
            *state.auth.write().await = "pending".to_string();
            *state.heartbeat.write().await = "--".to_string();
            web::broadcast_status(&state).await;

            log::info!("Waiting 60 seconds before retrying...");
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}

/// Spawn the sibling `at-heartbeat` binary (next to this exe), passing the
/// server IP. Used by foreground mode; the Windows service uses its own
/// session-aware launcher instead.
fn spawn_heartbeat_sibling(server_ip: std::net::IpAddr) {
    let bin = if cfg!(windows) {
        "at-heartbeat.exe"
    } else {
        "at-heartbeat"
    };
    let path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join(bin)));
    match path {
        Some(p) if p.exists() => match std::process::Command::new(&p)
            .arg(server_ip.to_string())
            .spawn()
        {
            Ok(_) => log::info!("Spawned heartbeat: {}", p.display()),
            Err(e) => log::error!("Failed to spawn heartbeat {}: {}", p.display(), e),
        },
        Some(p) => log::warn!("Heartbeat binary not found at {} — skipping", p.display()),
        None => log::warn!("Cannot resolve exe directory — skipping heartbeat"),
    }
}
