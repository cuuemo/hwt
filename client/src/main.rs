mod cleanup;
mod hwid;
mod protocol;
mod registry;
mod scanner;
mod service;
mod web;

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use web::{ClientEvent, ClientState};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("install") => service::install(),
        Some("uninstall") => service::uninstall(),
        Some("status") => service::status(),
        Some("run") => run_foreground(),
        _ => {
            #[cfg(windows)]
            {
                match service::dispatch() {
                    Ok(()) => {}
                    Err(windows_service::Error::Winapi(err))
                        if err.raw_os_error() == Some(1063) =>
                    {
                        log::warn!(
                            "Service Control Manager unavailable, falling back to foreground mode"
                        );
                        run_foreground();
                    }
                    Err(e) => {
                        log::error!("Failed to start service mode: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(windows))]
            {
                run_foreground();
            }
        }
    }
}

/// Run the cleanup cycle in foreground mode (for debugging).
fn run_foreground() {
    log::info!("Starting hwt-client in foreground mode");

    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    rt.block_on(async {
        let (event_tx, _) = broadcast::channel::<ClientEvent>(256);
        let state = Arc::new(ClientState::new(event_tx));

        // Spawn web UI server
        tokio::spawn(web::start_web_server(state.clone()));

        loop {
            match protocol::run_cleanup_cycle(state.clone()).await {
                Ok(_) => log::info!("Cleanup cycle completed successfully"),
                Err(e) => log::error!("Cleanup cycle failed: {}", e),
            }
            // Reset status for next cycle
            *state.connection.write().await = "idle".to_string();
            *state.auth.write().await = "pending".to_string();
            *state.heartbeat.write().await = "--".to_string();
            web::broadcast_status(&state).await;

            log::info!("Waiting 60 seconds before next cycle...");
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}
