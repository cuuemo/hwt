mod cleanup;
mod hwid;
mod protocol;
mod registry;
mod scanner;
mod service;

use std::time::Duration;

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
                service::dispatch();
            }
            #[cfg(not(windows))]
            {
                eprintln!("Usage: hwt-client <install|uninstall|status|run>");
                eprintln!("  install    - Register as Windows service and start");
                eprintln!("  uninstall  - Stop and remove Windows service");
                eprintln!("  status     - Query service status");
                eprintln!("  run        - Run in foreground (debug mode)");
            }
        }
    }
}

/// Run the cleanup cycle in foreground mode (for debugging).
fn run_foreground() {
    log::info!("Starting hwt-client in foreground mode");

    let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    rt.block_on(async {
        loop {
            match protocol::run_cleanup_cycle().await {
                Ok(_) => log::info!("Cleanup cycle completed successfully"),
                Err(e) => log::error!("Cleanup cycle failed: {}", e),
            }
            log::info!("Waiting 60 seconds before next cycle...");
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}
