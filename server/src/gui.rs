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
}

/// Results sent from background tokio runtime back to the GUI.
pub enum AuthResult {
    LoginSuccess {
        license_type: String,
        expire_at: Option<String>,
        message: String,
    },
    LoginFailed {
        error: String,
    },
    ReVerifySuccess {
        license_type: String,
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
    Main,
}

pub struct App {
    // Page state
    page: Page,
    username: String,
    password: String,
    login_error: Option<String>,
    logging_in: bool,

    // Authorization info
    authorized: Arc<AtomicBool>,
    license_type: String,
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
            authorized,
            license_type: String::new(),
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
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading("Net Admin Server V1.0");
            ui.add_space(20.0);

            egui::Grid::new("login_grid")
                .num_columns(2)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Account:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.username)
                            .desired_width(200.0),
                    );
                    ui.end_row();

                    ui.label("Password:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.password)
                            .password(true)
                            .desired_width(200.0),
                    );
                    ui.end_row();
                });

            ui.add_space(10.0);

            // Machine code display (truncated)
            let mc_display = if self.machine_code.len() > 20 {
                format!("{}...", &self.machine_code[..20])
            } else {
                self.machine_code.clone()
            };
            ui.label(format!("Machine Code: {}", mc_display));

            ui.add_space(15.0);

            let login_enabled = !self.logging_in
                && !self.username.is_empty()
                && !self.password.is_empty();

            let btn_text = if self.logging_in {
                "Logging in..."
            } else {
                "Login"
            };

            if ui
                .add_enabled(login_enabled, egui::Button::new(btn_text).min_size(egui::vec2(120.0, 30.0)))
                .clicked()
            {
                self.login_error = None;
                self.logging_in = true;
                let _ = self.cmd_tx.send(AuthCommand::Login {
                    username: self.username.clone(),
                    password: self.password.clone(),
                });
            }

            ui.add_space(10.0);

            if let Some(ref err) = self.login_error {
                ui.colored_label(egui::Color32::RED, format!("Login failed: {}", err));
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
                ui.label(&self.license_type);
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
    let cloud_base_url = obfstr::obfstr!("https://hwt-cloud.example.com").to_string();

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
