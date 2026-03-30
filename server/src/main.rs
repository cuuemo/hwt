mod auth;
mod gui;
mod listener;
mod machine;

use gui::{App, AuthCommand, AuthResult};
use listener::ClientInfo;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    // Shared state between GUI and background threads
    let authorized = Arc::new(AtomicBool::new(false));
    let clients: Arc<Mutex<Vec<ClientInfo>>> = Arc::new(Mutex::new(Vec::new()));

    // Channels for GUI <-> background communication
    let (cmd_tx, cmd_rx) = mpsc::channel::<AuthCommand>();
    let (result_tx, result_rx) = mpsc::channel::<AuthResult>();

    // Spawn background tokio runtime in a separate thread
    gui::spawn_background_runtime(
        authorized.clone(),
        clients.clone(),
        cmd_rx,
        result_tx,
    );

    // Run eframe/egui on the main thread
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 640.0])
            .with_title("Net Admin Server"),
        ..Default::default()
    };

    eframe::run_native(
        "hwt-server",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(App::new(authorized, clients, cmd_tx, result_rx)))
        }),
    )
}
