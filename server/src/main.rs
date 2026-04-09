mod auth;
mod listener;
mod machine;
mod web;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};
use web::{AppState, AuthCommand, ServerEvent};

#[tokio::main]
async fn main() {
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
    });

    // Spawn background auth loop
    let bg_state = state.clone();
    tokio::spawn(async move {
        web::background_loop(bg_state, cmd_rx).await;
    });

    // Start web server on port 19880
    let app = web::build_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:19880")
        .await
        .expect("Failed to bind port 19880");
    log::info!("Web UI listening on http://0.0.0.0:19880");
    axum::serve(listener, app).await.unwrap();
}
