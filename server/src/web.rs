use crate::auth;
use crate::listener::{self, ClientInfo};
use crate::machine;
use axum::extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::http::{header, HeaderMap};
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::Json;
use axum::Router;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};

// ─── Log bridge: captures Rust log output → broadcast channel ─────

static LOG_TX: OnceLock<broadcast::Sender<ServerEvent>> = OnceLock::new();

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
        eprintln!(
            "[{}] {}: {}",
            chrono::Local::now().format("%H:%M:%S"),
            record.level(),
            record.args()
        );
        if let Some(tx) = LOG_TX.get() {
            let _ = tx.send(ServerEvent::Log {
                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                level: level.to_string(),
                message: format!("{}", record.args()),
            });
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(event_tx: broadcast::Sender<ServerEvent>) {
    let _ = LOG_TX.set(event_tx);
    log::set_logger(&WebLogger).unwrap_or(());
    log::set_max_level(log::LevelFilter::Info);
}

// ─── Embedded assets ───────────────────────────────────────────────
const LOGIN_HTML: &str = include_str!("assets/login.html");
const REGISTER_HTML: &str = include_str!("assets/register.html");
const DASHBOARD_HTML: &str = include_str!("assets/dashboard.html");
const STYLE_CSS: &str = include_str!("assets/style.css");
const COMMON_JS: &str = include_str!("assets/common.js");

// ─── Commands & Events ────────────────────────────────────────────

pub enum AuthCommand {
    Login {
        username: String,
        password: String,
        reply: tokio::sync::oneshot::Sender<AuthReply>,
    },
    Register {
        username: String,
        password: String,
        reply: tokio::sync::oneshot::Sender<AuthReply>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthReply {
    pub success: bool,
    pub message: String,
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
    /// Session token set after successful login; checked on protected routes.
    pub session_token: Arc<RwLock<Option<String>>>,
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

// ─── Session helpers ──────────────────────────────────────────────

fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .find_map(|pair| {
            let pair = pair.trim();
            if let Some(val) = pair.strip_prefix("at_session=") {
                Some(val.to_string())
            } else {
                None
            }
        })
}

async fn check_session(state: &AppState, headers: &HeaderMap) -> bool {
    let token = state.session_token.read().await;
    match (&*token, extract_session_cookie(headers)) {
        (Some(expected), Some(got)) => expected == &got,
        _ => false,
    }
}

fn set_session_cookie(token: &str) -> String {
    format!("at_session={}; Path=/; HttpOnly; SameSite=Strict", token)
}

fn generate_session_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| format!("{:02x}", rng.gen::<u8>()))
        .collect()
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

async fn page_login(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if check_session(&state, &headers).await {
        return Redirect::to("/dashboard").into_response();
    }
    Html(LOGIN_HTML).into_response()
}

async fn page_register() -> Html<&'static str> {
    Html(REGISTER_HTML)
}

async fn page_dashboard(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !check_session(&state, &headers).await {
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
) -> impl IntoResponse {
    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
    let _ = state
        .cmd_tx
        .send(AuthCommand::Login {
            username: req.username,
            password: req.password,
            reply: reply_tx,
        })
        .await;

    match tokio::time::timeout(std::time::Duration::from_secs(60), reply_rx).await {
        Ok(Ok(reply)) => {
            if reply.success {
                // Generate session token and set cookie
                let token = generate_session_token();
                *state.session_token.write().await = Some(token.clone());
                (
                    [(header::SET_COOKIE, set_session_cookie(&token))],
                    Json(serde_json::json!({"ok": true, "message": reply.message})),
                )
                    .into_response()
            } else {
                Json(serde_json::json!({"ok": false, "message": reply.message})).into_response()
            }
        }
        _ => {
            Json(serde_json::json!({"ok": false, "message": "Login timeout"})).into_response()
        }
    }
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

async fn api_register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
    let _ = state
        .cmd_tx
        .send(AuthCommand::Register {
            username: req.username,
            password: req.password,
            reply: reply_tx,
        })
        .await;

    match tokio::time::timeout(std::time::Duration::from_secs(60), reply_rx).await {
        Ok(Ok(reply)) => {
            Json(serde_json::json!({"ok": reply.success, "message": reply.message}))
        }
        _ => Json(serde_json::json!({"ok": false, "message": "Register timeout"})),
    }
}

async fn api_state(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Machine code is public (shown on login page), full state requires session
    if check_session(&state, &headers).await {
        Json(serde_json::to_value(state.snapshot().await).unwrap())
    } else {
        // Only return machine code for login page
        Json(serde_json::json!({
            "machine_code": *state.machine_code.read().await,
            "logged_in": false,
        }))
    }
}

// ─── WebSocket ────────────────────────────────────────────────────

async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let authed = check_session(&state, &headers).await;
    ws.on_upgrade(move |socket| handle_ws(socket, state, authed))
}

async fn handle_ws(mut socket: WebSocket, state: Arc<AppState>, authed: bool) {
    if authed {
        // Send initial state snapshot
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
    }

    let mut rx = state.event_tx.subscribe();
    loop {
        match rx.recv().await {
            Ok(event) => {
                // Unauthenticated WS only gets Log events (for login page feedback)
                if !authed {
                    if !matches!(event, ServerEvent::Log { .. }) {
                        continue;
                    }
                }
                let json = serde_json::to_string(&event).unwrap();
                if socket.send(WsMessage::Text(json.into())).await.is_err() {
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(_)) => {
                if authed {
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
                        .send(WsMessage::Text(
                            serde_json::to_string(&refresh).unwrap().into(),
                        ))
                        .await
                        .is_err()
                    {
                        break;
                    }
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

pub async fn background_loop(state: Arc<AppState>, mut cmd_rx: mpsc::Receiver<AuthCommand>) {
    let http_client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
    // Empty string makes auth.rs use DEFAULT_CLOUD_BASE_URL (compile-time CLOUD_BASE_URL env)
    let cloud_base_url = String::new();

    // Load machine code
    let saved_machine_code = match machine::get_machine_code() {
        Ok(mc) => {
            *state.machine_code.write().await = mc.clone();
            broadcast_log(&state.event_tx, "info", &format!("Machine code: {}", mc));
            Some(mc)
        }
        Err(e) => {
            broadcast_log(
                &state.event_tx,
                "error",
                &format!("Machine code read failed: {}", e),
            );
            None
        }
    };

    #[allow(unused_assignments)]
    let mut saved_account: Option<String> = None;
    #[allow(unused_assignments)]
    let mut saved_password: Option<String> = None;
    let mut saved_mc = saved_machine_code;

    // Command loop: wait for login/register commands from web UI
    loop {
        let Some(cmd) = cmd_rx.recv().await else {
            return;
        };

        match cmd {
            AuthCommand::Register {
                username,
                password,
                reply,
            } => {
                let mut result = Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "not attempted",
                ));
                for attempt in 0..3u32 {
                    if attempt > 0 {
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        broadcast_log(
                            &state.event_tx,
                            "info",
                            &format!("Retry register ({}/3)...", attempt + 1),
                        );
                    }
                    result = auth::cloud_register(
                        &http_client,
                        &cloud_base_url,
                        &username,
                        &password,
                    )
                    .await;
                    if result.is_ok() {
                        break;
                    }
                }
                let _ = reply.send(match result {
                    Ok(msg) => AuthReply {
                        success: true,
                        message: msg,
                    },
                    Err(e) => AuthReply {
                        success: false,
                        message: e.to_string(),
                    },
                });
            }
            AuthCommand::Login {
                username,
                password,
                reply,
            } => {
                let mut reply = Some(reply);
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
                            if let Some(r) = reply.take() {
                                let _ = r.send(AuthReply {
                                    success: false,
                                    message: format!("Machine code read failed: {}", e),
                                });
                            }
                            continue;
                        }
                    }
                };

                broadcast_log(&state.event_tx, "info", "Connecting to cloud...");

                let mut login_ok = false;
                let mut last_err = String::new();
                for attempt in 0..3u32 {
                    if attempt > 0 {
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        broadcast_log(
                            &state.event_tx,
                            "info",
                            &format!("Retry login ({}/3)...", attempt + 1),
                        );
                    }
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
                                        *state.license_type.write().await =
                                            resp.license_type.clone();
                                        *state.expire_at.write().await = resp.expire_at.clone();
                                        *state.last_verify_time.write().await = Some(
                                            chrono::Local::now()
                                                .format("%Y-%m-%d %H:%M")
                                                .to_string(),
                                        );

                                        saved_account = Some(username.clone());
                                        saved_password = Some(password.clone());

                                        if let Some(r) = reply.take() {
                                            let _ = r.send(AuthReply {
                                                success: true,
                                                message: resp.message.clone(),
                                            });
                                        }
                                        broadcast_log(
                                            &state.event_tx,
                                            "success",
                                            &resp.message,
                                        );

                                        let _ = state.event_tx.send(
                                            ServerEvent::AuthStatusChanged {
                                                authorized: true,
                                                license_type: resp.license_type,
                                                expire_at: resp.expire_at,
                                                machine_code: machine_code.clone(),
                                                last_verify_time: state
                                                    .last_verify_time
                                                    .read()
                                                    .await
                                                    .clone(),
                                            },
                                        );

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

                                        login_ok = true;
                                        break;
                                    } else {
                                        // Auth denied — not a network error, don't retry
                                        if let Some(r) = reply.take() {
                                            let _ = r.send(AuthReply {
                                                success: false,
                                                message: resp.message,
                                            });
                                        }
                                        break;
                                    }
                                }
                                Err(e) => {
                                    last_err = format!("Verify failed: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            last_err = format!("Handshake failed: {}", e);
                        }
                    }
                }
                if !login_ok && !last_err.is_empty() {
                    if let Some(r) = reply.take() {
                        let _ = r.send(AuthReply {
                            success: false,
                            message: last_err,
                        });
                    }
                }
                if login_ok {
                    break; // Enter re-verify loop
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
            broadcast_log(&state.event_tx, "info", "Re-verifying...");
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
                                broadcast_log(&state.event_tx, "success", "Re-verification OK");
                                let _ = state.event_tx.send(ServerEvent::AuthStatusChanged {
                                    authorized: true,
                                    license_type: resp.license_type,
                                    expire_at: resp.expire_at,
                                    machine_code: mc.clone(),
                                    last_verify_time: state
                                        .last_verify_time
                                        .read()
                                        .await
                                        .clone(),
                                });
                            } else {
                                state.authorized.store(false, Ordering::Relaxed);
                                broadcast_log(
                                    &state.event_tx,
                                    "error",
                                    &format!("Re-verify denied: {}", resp.message),
                                );
                            }
                        }
                        Err(e) => {
                            state.authorized.store(false, Ordering::Relaxed);
                            broadcast_log(
                                &state.event_tx,
                                "error",
                                &format!("Re-verify error: {}", e),
                            );
                        }
                    }
                }
                Err(e) => {
                    state.authorized.store(false, Ordering::Relaxed);
                    broadcast_log(
                        &state.event_tx,
                        "error",
                        &format!("Re-handshake failed: {}", e),
                    );
                }
            }
        }
    }
}
