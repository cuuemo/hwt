mod auth;
mod listener;
mod machine;
mod web;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};
use web::{AppState, AuthCommand, ServerEvent};

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

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    disable_quick_edit();
    let (event_tx, _) = broadcast::channel::<ServerEvent>(256);
    web::init_logger(event_tx.clone());

    let (cmd_tx, cmd_rx) = mpsc::channel::<AuthCommand>(32);

    let state = Arc::new(AppState {
        authorized: Arc::new(AtomicBool::new(false)),
        logged_in: Arc::new(AtomicBool::new(false)),
        license_type: Default::default(),
        expire_at: Default::default(),
        machine_code: Default::default(),
        last_verify_time: Default::default(),
        clients: Arc::new(Mutex::new(Vec::new())),
        event_tx: event_tx.clone(),
        cmd_tx,
        session_token: Default::default(),
    });

    // Spawn background auth loop
    let bg_state = state.clone();
    tokio::spawn(async move {
        web::background_loop(bg_state, cmd_rx).await;
    });

    // Start web server on port 19880
    let app = web::build_router(state);
    match tokio::net::TcpListener::bind("0.0.0.0:19880").await {
        Ok(listener) => {
            log::info!("Web UI listening on http://0.0.0.0:19880");
            if let Err(e) = axum::serve(listener, app).await {
                log::error!("Web server error: {}", e);
            }
        }
        Err(e) => {
            log::error!("Failed to bind port 19880: {}", e);
        }
    }
}
