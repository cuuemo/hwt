use axum::extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::http::header;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Json;
use axum::Router;
use serde::Serialize;
use std::sync::{Arc, OnceLock};
use tokio::sync::{broadcast, RwLock};

// ─── Log bridge: captures Rust log output → broadcast channel ─────

static LOG_TX: OnceLock<broadcast::Sender<ClientEvent>> = OnceLock::new();

struct WebLogger;

impl log::Log for WebLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let level = match record.level() {
            log::Level::Error => "error",
            log::Level::Warn => "warn",
            log::Level::Info | log::Level::Debug | log::Level::Trace => "info",
        };
        eprintln!("[{}] {}: {}", chrono::Local::now().format("%H:%M:%S"), record.level(), record.args());
        if let Some(tx) = LOG_TX.get() {
            let _ = tx.send(ClientEvent::Log {
                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                level: level.to_string(),
                message: format!("{}", record.args()),
            });
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(event_tx: broadcast::Sender<ClientEvent>) {
    let _ = LOG_TX.set(event_tx);
    log::set_logger(&WebLogger).unwrap_or(());
    log::set_max_level(log::LevelFilter::Info);
}

const CLIENT_HTML: &str = include_str!("assets/client.html");
const STYLE_CSS: &str = include_str!("assets/style.css");
const COMMON_JS: &str = include_str!("assets/common.js");

// ─── Events ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ClientEvent {
    StatusChanged {
        connection: String,
        auth: String,
        heartbeat: String,
    },
    CleanupResult {
        scanned: u32,
        removed: u32,
        registry_cleaned: u32,
        ids_randomized: bool,
    },
    Log {
        timestamp: String,
        level: String,
        message: String,
    },
}

// ─── State ────────────────────────────────────────────────────────

pub struct ClientState {
    pub connection: Arc<RwLock<String>>,
    pub auth: Arc<RwLock<String>>,
    pub heartbeat: Arc<RwLock<String>>,
    pub cleanup_info: Arc<RwLock<Option<CleanupInfo>>>,
    pub event_tx: broadcast::Sender<ClientEvent>,
}

#[derive(Clone, Serialize)]
pub struct CleanupInfo {
    pub scanned: u32,
    pub removed: u32,
    pub registry_cleaned: u32,
    pub ids_randomized: bool,
}

impl ClientState {
    pub fn new(event_tx: broadcast::Sender<ClientEvent>) -> Self {
        Self {
            connection: Arc::new(RwLock::new("idle".to_string())),
            auth: Arc::new(RwLock::new("pending".to_string())),
            heartbeat: Arc::new(RwLock::new("--".to_string())),
            cleanup_info: Arc::new(RwLock::new(None)),
            event_tx,
        }
    }
}

// ─── Helper ───────────────────────────────────────────────────────

pub fn broadcast_log(event_tx: &broadcast::Sender<ClientEvent>, level: &str, message: &str) {
    let _ = event_tx.send(ClientEvent::Log {
        timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
        level: level.to_string(),
        message: message.to_string(),
    });
}

pub async fn broadcast_status(state: &ClientState) {
    let _ = state.event_tx.send(ClientEvent::StatusChanged {
        connection: state.connection.read().await.clone(),
        auth: state.auth.read().await.clone(),
        heartbeat: state.heartbeat.read().await.clone(),
    });
}

// ─── Router ───────────────────────────────────────────────────────

pub fn build_router(state: Arc<ClientState>) -> Router {
    Router::new()
        .route("/", get(page_client))
        .route("/api/state", get(api_state))
        .route("/ws", get(ws_upgrade))
        .route("/assets/style.css", get(asset_css))
        .route("/assets/common.js", get(asset_js))
        .with_state(state)
}

async fn page_client() -> Html<&'static str> {
    Html(CLIENT_HTML)
}

async fn asset_css() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], STYLE_CSS)
}

async fn asset_js() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
        COMMON_JS,
    )
}

#[derive(Serialize)]
struct StateSnapshot {
    #[serde(rename = "type")]
    event_type: &'static str,
    connection: String,
    auth: String,
    heartbeat: String,
    cleanup: Option<CleanupInfo>,
}

async fn api_state(State(state): State<Arc<ClientState>>) -> Json<StateSnapshot> {
    Json(StateSnapshot {
        event_type: "InitialState",
        connection: state.connection.read().await.clone(),
        auth: state.auth.read().await.clone(),
        heartbeat: state.heartbeat.read().await.clone(),
        cleanup: state.cleanup_info.read().await.clone(),
    })
}

async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ClientState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: Arc<ClientState>) {
    // Send initial state
    let snap = StateSnapshot {
        event_type: "InitialState",
        connection: state.connection.read().await.clone(),
        auth: state.auth.read().await.clone(),
        heartbeat: state.heartbeat.read().await.clone(),
        cleanup: state.cleanup_info.read().await.clone(),
    };
    let _ = socket
        .send(WsMessage::Text(serde_json::to_string(&snap).unwrap().into()))
        .await;

    let mut rx = state.event_tx.subscribe();
    loop {
        match rx.recv().await {
            Ok(event) => {
                let json = serde_json::to_string(&event).unwrap();
                if socket.send(WsMessage::Text(json.into())).await.is_err() {
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(broadcast::error::RecvError::Closed) => break,
        }
    }
}

// ─── Start web server ─────────────────────────────────────────────

pub async fn start_web_server(state: Arc<ClientState>) {
    let app = build_router(state);
    match tokio::net::TcpListener::bind("0.0.0.0:19881").await {
        Ok(listener) => {
            log::info!("Client Web UI listening on http://0.0.0.0:19881");
            let _ = axum::serve(listener, app).await;
        }
        Err(e) => {
            log::error!("Failed to bind client web UI on port 19881: {}", e);
        }
    }
}
