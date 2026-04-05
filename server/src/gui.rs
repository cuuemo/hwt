use crate::auth;
use crate::listener::{self, ClientInfo};
use crate::machine;
use eframe::egui;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Commands sent from GUI thread to the background tokio runtime.
pub enum AuthCommand {
    Login {
        username: String,
        password: String,
    },
    Register {
        username: String,
        password: String,
    },
}

/// Results sent from background tokio runtime back to the GUI.
pub enum AuthResult {
    LoginSuccess {
        license_type: Option<String>,
        expire_at: Option<String>,
        message: String,
    },
    LoginFailed {
        error: String,
    },
    RegisterSuccess {
        message: String,
    },
    RegisterFailed {
        error: String,
    },
    ReVerifySuccess {
        license_type: Option<String>,
        expire_at: Option<String>,
    },
    ReVerifyFailed {
        error: String,
    },
    Log(String),
}

#[derive(PartialEq)]
enum Page {
    Login,
    Register,
    Main,
}

pub struct App {
    // Page state
    page: Page,
    username: String,
    password: String,
    login_error: Option<String>,
    logging_in: bool,

    // Register state
    reg_username: String,
    reg_password: String,
    reg_password2: String,
    reg_error: Option<String>,
    reg_message: Option<String>,
    registering: bool,

    // Authorization info
    authorized: Arc<AtomicBool>,
    license_type: Option<String>,
    expire_at: Option<String>,
    machine_code: String,
    last_verify_time: Option<String>,

    // Online clients (shared with listener via tokio::sync::Mutex)
    clients: Arc<Mutex<Vec<ClientInfo>>>,
    /// Snapshot of clients for GUI rendering (updated each frame from the async Mutex)
    clients_snapshot: Vec<ClientInfo>,

    // Log messages
    log_messages: Vec<String>,

    // Channels for background communication
    cmd_tx: mpsc::Sender<AuthCommand>,
    result_rx: mpsc::Receiver<AuthResult>,
}

impl App {
    pub fn new(
        authorized: Arc<AtomicBool>,
        clients: Arc<Mutex<Vec<ClientInfo>>>,
        cmd_tx: mpsc::Sender<AuthCommand>,
        result_rx: mpsc::Receiver<AuthResult>,
    ) -> Self {
        let machine_code = machine::get_machine_code().unwrap_or_else(|e| {
            log::error!("Failed to get machine code: {}", e);
            "error".to_string()
        });

        Self {
            page: Page::Login,
            username: String::new(),
            password: String::new(),
            login_error: None,
            logging_in: false,
            reg_username: String::new(),
            reg_password: String::new(),
            reg_password2: String::new(),
            reg_error: None,
            reg_message: None,
            registering: false,
            authorized,
            license_type: None,
            expire_at: None,
            machine_code,
            last_verify_time: None,
            clients,
            clients_snapshot: Vec::new(),
            log_messages: Vec::new(),
            cmd_tx,
            result_rx,
        }
    }

    fn process_results(&mut self) {
        while let Ok(result) = self.result_rx.try_recv() {
            match result {
                AuthResult::LoginSuccess {
                    license_type,
                    expire_at,
                    message,
                } => {
                    self.logging_in = false;
                    self.license_type = license_type;
                    self.expire_at = expire_at;
                    self.last_verify_time =
                        Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string());
                    self.log_messages.push(format!(
                        "[{}] {}",
                        chrono::Local::now().format("%H:%M"),
                        message
                    ));
                    self.page = Page::Main;
                }
                AuthResult::LoginFailed { error } => {
                    self.logging_in = false;
                    self.login_error = Some(error);
                }
                AuthResult::RegisterSuccess { message } => {
                    self.registering = false;
                    self.reg_message = Some(message);
                    self.reg_error = None;
                }
                AuthResult::RegisterFailed { error } => {
                    self.registering = false;
                    self.reg_error = Some(error);
                }
                AuthResult::ReVerifySuccess {
                    license_type,
                    expire_at,
                } => {
                    self.license_type = license_type;
                    self.expire_at = expire_at;
                    self.last_verify_time =
                        Some(chrono::Local::now().format("%Y-%m-%d %H:%M").to_string());
                    self.log_messages.push(format!(
                        "[{}] Re-verification successful",
                        chrono::Local::now().format("%H:%M"),
                    ));
                }
                AuthResult::ReVerifyFailed { error } => {
                    self.authorized.store(false, Ordering::Relaxed);
                    self.log_messages.push(format!(
                        "[{}] Re-verification failed: {}",
                        chrono::Local::now().format("%H:%M"),
                        error,
                    ));
                }
                AuthResult::Log(msg) => {
                    self.log_messages.push(msg);
                }
            }
        }
    }

    fn update_clients_snapshot(&mut self) {
        // Try to get the lock without blocking; if it fails, keep the old snapshot
        if let Ok(guard) = self.clients.try_lock() {
            self.clients_snapshot = guard.clone();
        }
    }

    fn render_login_page(&mut self, ui: &mut egui::Ui) {
        let style = ui.style_mut();
        style.spacing.item_spacing = egui::vec2(8.0, 10.0);

        ui.vertical_centered(|ui| {
            ui.add_space(30.0);
            ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(220, 230, 255));
            ui.heading(egui::RichText::new("⚡ Net Admin Server").size(22.0).strong());
            ui.add_space(4.0);
            ui.label(egui::RichText::new("V1.0").size(12.0).color(egui::Color32::from_rgb(120, 140, 180)));
            ui.add_space(20.0);

            let mc_display = if self.machine_code.len() > 24 {
                format!("{}...", &self.machine_code[..24])
            } else {
                self.machine_code.clone()
            };
            ui.label(egui::RichText::new(format!("🖥 {}", mc_display)).size(11.0).color(egui::Color32::from_rgb(100, 120, 160)));
            ui.add_space(16.0);

            egui::Frame::none()
                .fill(egui::Color32::from_rgba_premultiplied(30, 40, 70, 200))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.set_width(280.0);
                    egui::Grid::new("login_grid").num_columns(2).spacing([8.0, 10.0]).show(ui, |ui| {
                        ui.label(egui::RichText::new("账号").color(egui::Color32::from_rgb(160, 180, 220)));
                        ui.add(egui::TextEdit::singleline(&mut self.username).desired_width(180.0).hint_text("请输入账号"));
                        ui.end_row();
                        ui.label(egui::RichText::new("密码").color(egui::Color32::from_rgb(160, 180, 220)));
                        ui.add(egui::TextEdit::singleline(&mut self.password).password(true).desired_width(180.0).hint_text("请输入密码"));
                        ui.end_row();
                    });
                });

            ui.add_space(12.0);

            let login_enabled = !self.logging_in && !self.username.is_empty() && !self.password.is_empty();
            let btn_text = if self.logging_in { "登录中..." } else { "登 录" };
            if ui.add_enabled(login_enabled, egui::Button::new(egui::RichText::new(btn_text).size(14.0)).min_size(egui::vec2(140.0, 32.0))).clicked() {
                self.login_error = None;
                self.logging_in = true;
                let _ = self.cmd_tx.send(AuthCommand::Login {
                    username: self.username.clone(),
                    password: self.password.clone(),
                });
            }

            if let Some(ref err) = self.login_error {
                ui.add_space(6.0);
                ui.colored_label(egui::Color32::from_rgb(255, 100, 100), format!("✗ {}", err));
            }

            ui.add_space(10.0);
            if ui.small_button(egui::RichText::new("没有账号？立即注册").color(egui::Color32::from_rgb(100, 160, 255))).clicked() {
                self.login_error = None;
                self.page = Page::Register;
            }
        });
    }

    fn render_register_page(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(30.0);
            ui.heading(egui::RichText::new("📝 注册账号").size(22.0).strong());
            ui.add_space(20.0);

            egui::Frame::none()
                .fill(egui::Color32::from_rgba_premultiplied(30, 40, 70, 200))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.set_width(280.0);
                    egui::Grid::new("reg_grid").num_columns(2).spacing([8.0, 10.0]).show(ui, |ui| {
                        ui.label(egui::RichText::new("账号").color(egui::Color32::from_rgb(160, 180, 220)));
                        ui.add(egui::TextEdit::singleline(&mut self.reg_username).desired_width(180.0).hint_text("2-64个字符"));
                        ui.end_row();
                        ui.label(egui::RichText::new("密码").color(egui::Color32::from_rgb(160, 180, 220)));
                        ui.add(egui::TextEdit::singleline(&mut self.reg_password).password(true).desired_width(180.0).hint_text("至少6个字符"));
                        ui.end_row();
                        ui.label(egui::RichText::new("确认密码").color(egui::Color32::from_rgb(160, 180, 220)));
                        ui.add(egui::TextEdit::singleline(&mut self.reg_password2).password(true).desired_width(180.0).hint_text("再次输入密码"));
                        ui.end_row();
                    });
                });

            ui.add_space(12.0);

            if let Some(ref msg) = self.reg_message.clone() {
                ui.colored_label(egui::Color32::from_rgb(100, 220, 100), format!("✓ {}", msg));
                ui.add_space(6.0);
                if ui.button("返回登录").clicked() {
                    self.reg_message = None;
                    self.reg_username.clear();
                    self.reg_password.clear();
                    self.reg_password2.clear();
                    self.page = Page::Login;
                }
                return;
            }

            let reg_enabled = !self.registering
                && self.reg_username.len() >= 2
                && self.reg_password.len() >= 6
                && !self.reg_password2.is_empty();
            let btn_text = if self.registering { "注册中..." } else { "注 册" };
            if ui.add_enabled(reg_enabled, egui::Button::new(egui::RichText::new(btn_text).size(14.0)).min_size(egui::vec2(140.0, 32.0))).clicked() {
                if self.reg_password != self.reg_password2 {
                    self.reg_error = Some("两次密码不一致".to_string());
                } else {
                    self.reg_error = None;
                    self.registering = true;
                    let _ = self.cmd_tx.send(AuthCommand::Register {
                        username: self.reg_username.clone(),
                        password: self.reg_password.clone(),
                    });
                }
            }

            if let Some(ref err) = self.reg_error {
                ui.add_space(6.0);
                ui.colored_label(egui::Color32::from_rgb(255, 100, 100), format!("✗ {}", err));
            }

            ui.add_space(10.0);
            if ui.small_button(egui::RichText::new("已有账号？返回登录").color(egui::Color32::from_rgb(100, 160, 255))).clicked() {
                self.reg_error = None;
                self.page = Page::Login;
            }
        });
    }

    fn render_main_page(&mut self, ui: &mut egui::Ui) {
        let is_auth = self.authorized.load(Ordering::Relaxed);

        ui.add_space(10.0);

        // Authorization status section
        egui::Grid::new("status_grid")
            .num_columns(2)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                ui.label("Auth Status:");
                if is_auth {
                    ui.colored_label(egui::Color32::GREEN, "Authorized");
                } else {
                    ui.colored_label(egui::Color32::RED, "Not Authorized");
                }
                ui.end_row();

                ui.label("License Type:");
                ui.label(self.license_type.as_deref().unwrap_or("未授权"));
                ui.end_row();

                ui.label("Expires:");
                ui.label(
                    self.expire_at
                        .as_deref()
                        .unwrap_or("Permanent"),
                );
                ui.end_row();

                ui.label("Machine Code:");
                let mc_display = if self.machine_code.len() > 32 {
                    format!("{}...", &self.machine_code[..32])
                } else {
                    self.machine_code.clone()
                };
                ui.label(mc_display);
                ui.end_row();

                ui.label("Last Verified:");
                ui.label(
                    self.last_verify_time
                        .as_deref()
                        .unwrap_or("N/A"),
                );
                ui.end_row();
            });

        ui.separator();

        // Online clients table
        let client_count = self.clients_snapshot.len();
        ui.heading(format!("Online Clients ({})", client_count));
        ui.add_space(5.0);

        egui::ScrollArea::vertical()
            .max_height(200.0)
            .id_source("clients_scroll")
            .show(ui, |ui| {
                egui::Grid::new("clients_table")
                    .num_columns(3)
                    .spacing([20.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        // Header
                        ui.strong("IP");
                        ui.strong("Client ID");
                        ui.strong("Connected At");
                        ui.end_row();

                        for client in &self.clients_snapshot {
                            ui.label(client.ip.to_string());
                            ui.label(&client.client_id);
                            ui.label(client.connected_at.format("%H:%M:%S").to_string());
                            ui.end_row();
                        }
                    });
            });

        ui.separator();

        // Log area
        ui.heading("Log");
        egui::ScrollArea::vertical()
            .max_height(150.0)
            .id_source("log_scroll")
            .stick_to_bottom(true)
            .show(ui, |ui: &mut egui::Ui| {
                for msg in &self.log_messages {
                    ui.label(msg);
                }
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process any pending results from the background thread
        self.process_results();
        // Update the client list snapshot
        self.update_clients_snapshot();

        // Request a repaint periodically to pick up async updates
        ctx.request_repaint_after(std::time::Duration::from_secs(1));

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.page {
                Page::Login => self.render_login_page(ui),
                Page::Register => self.render_register_page(ui),
                Page::Main => self.render_main_page(ui),
            }
        });
    }
}

/// Spawn the background tokio runtime that handles auth commands, TCP listener, etc.
pub fn spawn_background_runtime(
    authorized: Arc<AtomicBool>,
    clients: Arc<Mutex<Vec<ClientInfo>>>,
    cmd_rx: mpsc::Receiver<AuthCommand>,
    result_tx: mpsc::Sender<AuthResult>,
) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        rt.block_on(async move {
            background_loop(authorized, clients, cmd_rx, result_tx).await;
        });
    });
}

#[allow(unused_assignments)]
async fn background_loop(
    authorized: Arc<AtomicBool>,
    clients: Arc<Mutex<Vec<ClientInfo>>>,
    cmd_rx: mpsc::Receiver<AuthCommand>,
    result_tx: mpsc::Sender<AuthResult>,
) {
    let http_client = reqwest::Client::new();
    let cloud_base_url = String::new(); // use DEFAULT_CLOUD_BASE_URL from auth.rs

    // Channel for log messages from the TCP listener
    let (log_tx, log_rx) = mpsc::channel::<String>();

    // Spawn a task to forward log messages from listener to GUI
    let result_tx_log = result_tx.clone();
    let _log_forward_handle = tokio::spawn(async move {
        loop {
            match log_rx.try_recv() {
                Ok(msg) => {
                    let _ = result_tx_log.send(AuthResult::Log(msg));
                }
                Err(mpsc::TryRecvError::Empty) => {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }
                Err(mpsc::TryRecvError::Disconnected) => break,
            }
        }
    });

    // Saved session info for re-verification
    let mut session_id: Option<String> = None;
    let mut session_key: Option<[u8; 32]> = None;
    let mut saved_account: Option<String> = None;
    let mut saved_password: Option<String> = None;
    let mut saved_machine_code: Option<String> = None;

    // Main command loop
    loop {
        // Check for commands from GUI (non-blocking)
        match cmd_rx.try_recv() {
            Ok(AuthCommand::Register { username, password }) => {
                match auth::cloud_register(&http_client, &cloud_base_url, &username, &password).await {
                    Ok(msg) => { let _ = result_tx.send(AuthResult::RegisterSuccess { message: msg }); }
                    Err(e) => { let _ = result_tx.send(AuthResult::RegisterFailed { error: e.to_string() }); }
                }
            }
            Ok(AuthCommand::Login { username, password }) => {
                let machine_code = machine::get_machine_code().unwrap_or_else(|e| {
                    log::error!("Machine code error: {}", e);
                    "error".to_string()
                });

                // Perform cloud handshake + verify
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
                                    authorized.store(true, Ordering::Relaxed);
                                    session_id = Some(sid);
                                    session_key = Some(skey);
                                    saved_account = Some(username);
                                    saved_password = Some(password);
                                    saved_machine_code = Some(machine_code);

                                    let _ = result_tx.send(AuthResult::LoginSuccess {
                                        license_type: resp.license_type,
                                        expire_at: resp.expire_at,
                                        message: resp.message,
                                    });

                                    // Start TCP listener
                                    let auth_clone = authorized.clone();
                                    let clients_clone = clients.clone();
                                    let log_tx_clone = log_tx.clone();
                                    tokio::spawn(async move {
                                        if let Err(e) = listener::start_listener(
                                            auth_clone,
                                            clients_clone,
                                            log_tx_clone,
                                        )
                                        .await
                                        {
                                            log::error!("Listener error: {}", e);
                                        }
                                    });

                                    // Start periodic re-verification (every 60 minutes)
                                    break; // Exit the command loop, enter re-verify loop below
                                } else {
                                    let _ = result_tx.send(AuthResult::LoginFailed {
                                        error: resp.message,
                                    });
                                }
                            }
                            Err(e) => {
                                let _ = result_tx.send(AuthResult::LoginFailed {
                                    error: format!("Verify error: {}", e),
                                });
                            }
                        }
                    }
                    Err(e) => {
                        let _ = result_tx.send(AuthResult::LoginFailed {
                            error: format!("Handshake error: {}", e),
                        });
                    }
                }
            }
            Err(mpsc::TryRecvError::Empty) => {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                return;
            }
        }
    }

    // Periodic re-verification loop (every 60 minutes)
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 60));
    interval.tick().await; // First tick is immediate, skip it

    loop {
        interval.tick().await;

        if let (Some(ref _sid), Some(ref _skey), Some(ref acct), Some(ref pwd), Some(ref mc)) = (
            &session_id,
            &session_key,
            &saved_account,
            &saved_password,
            &saved_machine_code,
        ) {
            // Re-handshake to get fresh session
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
                                authorized.store(true, Ordering::Relaxed);
                                session_id = Some(new_sid);
                                session_key = Some(new_skey);
                                let _ = result_tx.send(AuthResult::ReVerifySuccess {
                                    license_type: resp.license_type,
                                    expire_at: resp.expire_at,
                                });
                            } else {
                                authorized.store(false, Ordering::Relaxed);
                                let _ = result_tx.send(AuthResult::ReVerifyFailed {
                                    error: resp.message,
                                });
                            }
                        }
                        Err(e) => {
                            authorized.store(false, Ordering::Relaxed);
                            let _ = result_tx.send(AuthResult::ReVerifyFailed {
                                error: format!("Re-verify error: {}", e),
                            });
                        }
                    }
                }
                Err(e) => {
                    authorized.store(false, Ordering::Relaxed);
                    let _ = result_tx.send(AuthResult::ReVerifyFailed {
                        error: format!("Re-handshake error: {}", e),
                    });
                }
            }
        }
    }
}
