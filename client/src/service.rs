// Windows Service management for HwtCleanupService.
// All Windows-specific code is behind #[cfg(windows)].

// ============================================================
// Windows implementation
// ============================================================
#[cfg(windows)]
mod win {
    const SERVICE_NAME: &str = "HwtCleanupService";
    const SERVICE_DISPLAY_NAME: &str = "HWT Device Cleanup Service";
    use std::ffi::OsString;
    use std::sync::mpsc;
    use std::time::Duration;
    use windows_service::define_windows_service;
    use windows_service::service::{
        ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl, ServiceExitCode,
        ServiceInfo, ServiceStartType, ServiceState, ServiceStatus, ServiceType,
    };
    use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
    use windows_service::service_dispatcher;
    use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

    /// Register and start the Windows service.
    pub fn install() {
        let manager =
            ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CREATE_SERVICE)
                .expect("Failed to open Service Control Manager");

        let exe_path = std::env::current_exe().expect("Failed to get current exe path");

        let service_info = ServiceInfo {
            name: OsString::from(SERVICE_NAME),
            display_name: OsString::from(SERVICE_DISPLAY_NAME),
            service_type: ServiceType::OWN_PROCESS,
            start_type: ServiceStartType::AutoStart,
            error_control: ServiceErrorControl::Normal,
            executable_path: exe_path,
            launch_arguments: vec![],
            dependencies: vec![],
            account_name: None, // LocalSystem
            account_password: None,
        };

        let service = manager
            .create_service(
                &service_info,
                ServiceAccess::START | ServiceAccess::QUERY_STATUS,
            )
            .expect("Failed to create service");

        log::info!("Service '{}' created successfully", SERVICE_NAME);

        if let Err(e) = service.start::<String>(&[]) {
            log::warn!("Failed to start service: {}", e);
        } else {
            log::info!("Service '{}' started", SERVICE_NAME);
        }
    }

    /// Stop and delete the Windows service.
    pub fn uninstall() {
        let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
            .expect("Failed to open Service Control Manager");

        let service = manager
            .open_service(
                SERVICE_NAME,
                ServiceAccess::STOP | ServiceAccess::DELETE | ServiceAccess::QUERY_STATUS,
            )
            .expect("Failed to open service");

        // Try to stop the service first
        let stop_status = ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::StopPending,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(5),
            process_id: None,
        };
        let _ = service.stop();
        log::info!("Sent stop signal to service '{}'", SERVICE_NAME);
        drop(stop_status); // just used for documentation

        // Wait a moment for the service to stop
        std::thread::sleep(Duration::from_secs(2));

        service.delete().expect("Failed to delete service");
        log::info!("Service '{}' deleted successfully", SERVICE_NAME);
    }

    /// Query and print the service status.
    pub fn status() {
        let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
            .expect("Failed to open Service Control Manager");

        match manager.open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS) {
            Ok(service) => {
                let status = service
                    .query_status()
                    .expect("Failed to query service status");
                let state_str = match status.current_state {
                    ServiceState::Stopped => "Stopped",
                    ServiceState::StartPending => "Start Pending",
                    ServiceState::StopPending => "Stop Pending",
                    ServiceState::Running => "Running",
                    ServiceState::ContinuePending => "Continue Pending",
                    ServiceState::PausePending => "Pause Pending",
                    ServiceState::Paused => "Paused",
                };
                println!("Service '{}': {}", SERVICE_NAME, state_str);
            }
            Err(e) => {
                println!("Service '{}' not installed ({})", SERVICE_NAME, e);
            }
        }
    }

    define_windows_service!(ffi_service_main, service_main);

    /// Dispatch to the Windows Service Control Manager.
    pub fn dispatch() -> windows_service::Result<()> {
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }

    fn service_main(_args: Vec<OsString>) {
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

        let status_handle =
            service_control_handler::register(SERVICE_NAME, move |control| match control {
                ServiceControl::Stop | ServiceControl::Shutdown => {
                    let _ = shutdown_tx.send(());
                    ServiceControlHandlerResult::NoError
                }
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
                _ => ServiceControlHandlerResult::NotImplemented,
            })
            .expect("Failed to register service control handler");

        // Report Running
        let _ = status_handle.set_service_status(ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        });

        log::info!("Service '{}' is now running", SERVICE_NAME);

        // Build tokio runtime and run main loop
        let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
        rt.block_on(async {
            use std::sync::Arc as StdArc;
            use std::sync::Mutex as StdMutex;
            let shutdown_rx = StdArc::new(StdMutex::new(Some(shutdown_rx)));

            let (event_tx, _) =
                tokio::sync::broadcast::channel::<crate::web::ClientEvent>(256);
            crate::web::init_logger(event_tx.clone());
            let state = std::sync::Arc::new(crate::web::ClientState::new(event_tx));

            // Spawn web UI server
            tokio::spawn(crate::web::start_web_server(state.clone()));

            loop {
                match crate::protocol::run_cleanup_cycle(state.clone()).await {
                    Ok(_) => log::info!("Cleanup cycle completed successfully"),
                    Err(e) => log::error!("Cleanup cycle failed: {}", e),
                }

                // Reset status for next cycle
                *state.connection.write().await = "idle".to_string();
                *state.auth.write().await = "pending".to_string();
                *state.heartbeat.write().await = "--".to_string();
                crate::web::broadcast_status(&state).await;

                // Wait 60 seconds or until stop signal
                let rx_clone = shutdown_rx.clone();
                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(60)) => {
                        continue;
                    }
                    _ = tokio::task::spawn_blocking(move || {
                        if let Some(rx) = rx_clone.lock().unwrap().take() {
                            let _ = rx.recv();
                        }
                    }) => {
                        log::info!("Received stop signal");
                        break;
                    }
                }
            }
        });

        // Report Stopped
        let _ = status_handle.set_service_status(ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        });

        log::info!("Service '{}' stopped", SERVICE_NAME);
    }
}

// ============================================================
// Public interface — delegates to platform-specific code
// ============================================================

#[cfg(windows)]
pub fn install() {
    win::install();
}

#[cfg(not(windows))]
pub fn install() {
    eprintln!("Windows service installation is not supported on this platform");
}

#[cfg(windows)]
pub fn uninstall() {
    win::uninstall();
}

#[cfg(not(windows))]
pub fn uninstall() {
    eprintln!("Windows service uninstallation is not supported on this platform");
}

#[cfg(windows)]
pub fn status() {
    win::status();
}

#[cfg(not(windows))]
pub fn status() {
    eprintln!("Windows service status query is not supported on this platform");
}

#[cfg(windows)]
pub fn dispatch() -> windows_service::Result<()> {
    win::dispatch()
}
