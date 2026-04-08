use crate::auth;
use crate::listener::{self, ClientInfo};
use crate::machine;
use axum::extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::http::header;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};

// ─── Embedded assets ───────────────────────────────────────────────
const LOGIN_HTML: &str = include_str!("assets/login.html");
const REGISTER_HTML: &str = include_str!("assets/register.html");
const DASHBOARD_HTML: &str = include_str!("assets/dashboard.html");
const STYLE_CSS: &str = include_str!("assets/style.css");
const COMMON_JS: &str = include_str!("assets/common.js");

// ─── Commands & Events ────────────────────────────────────────────

pub enum AuthCommand {
    Login { username: String, password: String },
    Register { username: String, password: String },
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ServerEvent {
    AuthStatusChanged {
        authorized: bool,
        license_type: Option<String>,
        expire_at: Option<String>,
        machine_code: String,
        last_verify_time: Option<String>,
    },
    ClientListChanged {
        clients: Vec<ClientInfoDto>,
    },
    Log {
        timestamp: String,
        level: String,
        message: String,
    },
    LoginResult {
        success: bool,
        message: String,
    },
    RegisterResult {
        success: bool,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfoDto {
    pub ip: String,
    pub client_id: String,
    pub connected_at: String,
}

impl From<&ClientInfo> for ClientInfoDto {
    fn from(c: &ClientInfo) -> Self {
        Self {
            ip: c.ip.to_string(),
            client_id: c.client_id.clone(),
            connected_at: c.connected_at.format("%H:%M:%S").to_string(),
        }
    }
}

// ─── Shared State ─────────────────────────────────────────────────

pub struct AppState {
    pub authorized: Arc<AtomicBool>,
    pub logged_in: Arc<AtomicBool>,
    pub license_type: Arc<RwLock<Option<String>>>,
    pub expire_at: Arc<RwLock<Option<String>>>,
    pub machine_code: Arc<RwLock<String>>,
    pub last_verify_time: Arc<RwLock<Option<String>>>,
    pub clients: Arc<Mutex<Vec<ClientInfo>>>,
    pub event_tx: broadcast::Sender<ServerEvent>,
    pub cmd_tx: mpsc::Sender<AuthCommand>,
}

impl AppState {
    async fn snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            authorized: self.authorized.load(Ordering::Relaxed),
            logged_in: self.logged_in.load(Ordering::Relaxed),
            license_type: self.license_type.read().await.clone(),
            expire_at: self.expire_at.read().await.clone(),
            machine_code: self.machine_code.read().await.clone(),
            last_verify_time: self.last_verify_time.read().await.clone(),
            clients: self
                .clients
                .lock()
                .await
                .iter()
                .map(ClientInfoDto::from)
                .collect(),
        }
    }
}

#[derive(Serialize)]
struct StateSnapshot {
    authorized: bool,
    logged_in: bool,
    license_type: Option<String>,
    expire_at: Option<String>,
    machine_code: String,
    last_verify_time: Option<String>,
    clients: Vec<ClientInfoDto>,
}

// ─── Router ───────────────────────────────────────────────────────

pub fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(page_login))
        .route("/register", get(page_register))
        .route("/dashboard", get(page_dashboard))
        .route("/api/login", post(api_login))
        .route("/api/register", post(api_register))
        .route("/api/state", get(api_state))
        .route("/ws", get(ws_upgrade))
        .route("/assets/style.css", get(asset_css))
        .route("/assets/common.js", get(asset_js))
        .with_state(state)
}

// ─── Page Handlers ────────────────────────────────────────────────

async fn page_login(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if state.logged_in.load(Ordering::Relaxed) {
        return Redirect::to("/dashboard").into_response();
    }
    Html(LOGIN_HTML).into_response()
}

async fn page_register() -> Html<&'static str> {
    Html(REGISTER_HTML)
}

async fn page_dashboard(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if !state.logged_in.load(Ordering::Relaxed) {
        return Redirect::to("/").into_response();
    }
    Html(DASHBOARD_HTML).into_response()
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

// ─── API Handlers ─────────────────────────────────────────────────

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn api_login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    let _ = state
        .cmd_tx
        .send(AuthCommand::Login {
            username: req.username,
            password: req.password,
        })
        .await;
    Json(serde_json::json!({"ok": true, "message": "processing"}))
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

async fn api_register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Json<serde_json::Value> {
    let _ = state
        .cmd_tx
        .send(AuthCommand::Register {
            username: req.username,
            password: req.password,
        })
        .await;
    Json(serde_json::json!({"ok": true, "message": "processing"}))
}

async fn api_state(State(state): State<Arc<AppState>>) -> Json<StateSnapshot> {
    Json(state.snapshot().await)
}

// ─── WebSocket ────────────────────────────────────────────────────

async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: Arc<AppState>) {
    // Send initial state snapshot as a special event
    let snap = state.snapshot().await;
    let init = serde_json::json!({
        "type": "InitialState",
        "authorized": snap.authorized,
        "logged_in": snap.logged_in,
        "license_type": snap.license_type,
        "expire_at": snap.expire_at,
        "machine_code": snap.machine_code,
        "last_verify_time": snap.last_verify_time,
        "clients": snap.clients,
    });
    let _ = socket
        .send(WsMessage::Text(serde_json::to_string(&init).unwrap().into()))
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
            Err(broadcast::error::RecvError::Lagged(_)) => {
                // Missed events, send fresh snapshot
                let snap = state.snapshot().await;
                let refresh = serde_json::json!({
                    "type": "InitialState",
                    "authorized": snap.authorized,
                    "logged_in": snap.logged_in,
                    "license_type": snap.license_type,
                    "expire_at": snap.expire_at,
                    "machine_code": snap.machine_code,
                    "last_verify_time": snap.last_verify_time,
                    "clients": snap.clients,
                });
                if socket
                    .send(WsMessage::Text(serde_json::to_string(&refresh).unwrap().into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Err(broadcast::error::RecvError::Closed) => break,
        }
    }
}

// ─── Helper: broadcast a log event ────────────────────────────────

pub fn broadcast_log(event_tx: &broadcast::Sender<ServerEvent>, level: &str, message: &str) {
    let _ = event_tx.send(ServerEvent::Log {
        timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
        level: level.to_string(),
        message: message.to_string(),
    });
}

// ─── Background Loop ──────────────────────────────────────────────

pub async fn background_loop(
    state: Arc<AppState>,
    mut cmd_rx: mpsc::Receiver<AuthCommand>,
) {
    let http_client = reqwest::Client::new();
    let cloud_base_url = String::new();

    // Load machine code
    let saved_machine_code = match machine::get_machine_code() {
        Ok(mc) => {
            *state.machine_code.write().await = mc.clone();
            broadcast_log(&state.event_tx, "info", &format!("机器码: {}", mc));
            Some(mc)
        }
        Err(e) => {
            broadcast_log(
                &state.event_tx,
                "error",
                &format!("机器码读取失败: {}", e),
            );
            None
        }
    };

    let mut _session_id: Option<String> = None;
    let mut _session_key: Option<[u8; 32]> = None;
    #[allow(unused_assignments)]
    let mut saved_account: Option<String> = None;
    #[allow(unused_assignments)]
    let mut saved_password: Option<String> = None;
    let mut saved_mc = saved_machine_code;

    // Command loop: wait for login/register commands from web UI
    loop {
        let Some(cmd) = cmd_rx.recv().await else {
            return; // channel closed
        };

        match cmd {
            AuthCommand::Register { username, password } => {
                match auth::cloud_register(&http_client, &cloud_base_url, &username, &password)
                    .await
                {
                    Ok(msg) => {
                        let _ = state.event_tx.send(ServerEvent::RegisterResult {
                            success: true,
                            message: msg,
                        });
                    }
                    Err(e) => {
                        let _ = state.event_tx.send(ServerEvent::RegisterResult {
                            success: false,
                            message: e.to_string(),
                        });
                    }
                }
            }
            AuthCommand::Login { username, password } => {
                let machine_code = if let Some(mc) = saved_mc.clone() {
                    mc
                } else {
                    match machine::get_machine_code() {
                        Ok(mc) => {
                            *state.machine_code.write().await = mc.clone();
                            saved_mc = Some(mc.clone());
                            mc
                        }
                        Err(e) => {
                            let _ = state.event_tx.send(ServerEvent::LoginResult {
                                success: false,
                                message: format!("机器码读取失败: {}", e),
                            });
                            continue;
                        }
                    }
                };

                broadcast_log(&state.event_tx, "info", "正在连接云端...");

                match auth::cloud_handshake(&http_client, &cloud_base_url).await {
                    Ok((sid, skey)) => {
                        match auth::cloud_verify(
                            &http_client,
                            &cloud_base_url,
                            &sid,
                            &skey,
                            &username,
                            &password,
                            &machine_code,
                        )
                        .await
                        {
                            Ok(resp) => {
                                if resp.authorized {
                                    state.authorized.store(true, Ordering::Relaxed);
                                    state.logged_in.store(true, Ordering::Relaxed);
                                    *state.license_type.write().await = resp.license_type.clone();
                                    *state.expire_at.write().await = resp.expire_at.clone();
                                    *state.last_verify_time.write().await = Some(
                                        chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
                                    );

                                    _session_id = Some(sid);
                                    _session_key = Some(skey);
                                    saved_account = Some(username);
                                    saved_password = Some(password);

                                    let _ = state.event_tx.send(ServerEvent::LoginResult {
                                        success: true,
                                        message: resp.message.clone(),
                                    });
                                    broadcast_log(&state.event_tx, "success", &resp.message);

                                    // Broadcast auth status
                                    let _ =
                                        state.event_tx.send(ServerEvent::AuthStatusChanged {
                                            authorized: true,
                                            license_type: resp.license_type,
                                            expire_at: resp.expire_at,
                                            machine_code: machine_code.clone(),
                                            last_verify_time: state
                                                .last_verify_time
                                                .read()
                                                .await
                                                .clone(),
                                        });

                                    // Start TCP listener
                                    let auth_clone = state.authorized.clone();
                                    let clients_clone = state.clients.clone();
                                    let etx_clone = state.event_tx.clone();
                                    tokio::spawn(async move {
                                        if let Err(e) = listener::start_listener(
                                            auth_clone,
                                            clients_clone,
                                            etx_clone,
                                        )
                                        .await
                                        {
                                            log::error!("Listener error: {}", e);
                                        }
                                    });

                                    break; // Enter re-verify loop
                                } else {
                                    let _ = state.event_tx.send(ServerEvent::LoginResult {
                                        success: false,
                                        message: resp.message,
                                    });
                                }
                            }
                            Err(e) => {
                                let _ = state.event_tx.send(ServerEvent::LoginResult {
                                    success: false,
                                    message: format!("验证失败: {}", e),
                                });
                            }
                        }
                    }
                    Err(e) => {
                        let _ = state.event_tx.send(ServerEvent::LoginResult {
                            success: false,
                            message: format!("握手失败: {}", e),
                        });
                    }
                }
            }
        }
    }

    // Re-verification loop (every 60 minutes)
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 60));
    interval.tick().await; // skip first immediate tick

    loop {
        interval.tick().await;

        if let (Some(ref acct), Some(ref pwd), Some(ref mc)) =
            (&saved_account, &saved_password, &saved_mc)
        {
            broadcast_log(&state.event_tx, "info", "正在重新验证...");
            match auth::cloud_handshake(&http_client, &cloud_base_url).await {
                Ok((new_sid, new_skey)) => {
                    match auth::cloud_verify(
                        &http_client,
                        &cloud_base_url,
                        &new_sid,
                        &new_skey,
                        acct,
                        pwd,
                        mc,
                    )
                    .await
                    {
                        Ok(resp) => {
                            if resp.authorized {
                                state.authorized.store(true, Ordering::Relaxed);
                                *state.license_type.write().await = resp.license_type.clone();
                                *state.expire_at.write().await = resp.expire_at.clone();
                                *state.last_verify_time.write().await = Some(
                                    chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
                                );
                                _session_id = Some(new_sid);
                                _session_key = Some(new_skey);
                                broadcast_log(&state.event_tx, "success", "重新验证成功");
                                let _ = state.event_tx.send(ServerEvent::AuthStatusChanged {
                                    authorized: true,
                                    license_type: resp.license_type,
                                    expire_at: resp.expire_at,
                                    machine_code: mc.clone(),
                                    last_verify_time: state.last_verify_time.read().await.clone(),
                                });
                            } else {
                                state.authorized.store(false, Ordering::Relaxed);
                                broadcast_log(
                                    &state.event_tx,
                                    "error",
                                    &format!("重新验证失败: {}", resp.message),
                                );
                            }
                        }
                        Err(e) => {
                            state.authorized.store(false, Ordering::Relaxed);
                            broadcast_log(
                                &state.event_tx,
                                "error",
                                &format!("重新验证错误: {}", e),
                            );
                        }
                    }
                }
                Err(e) => {
                    state.authorized.store(false, Ordering::Relaxed);
                    broadcast_log(
                        &state.event_tx,
                        "error",
                        &format!("重新握手失败: {}", e),
                    );
                }
            }
        }
    }
}
