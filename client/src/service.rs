// Windows Service management for AtCleanupService.
// All Windows-specific code is behind #[cfg(windows)].

// ============================================================
// Windows implementation
// ============================================================
#[cfg(windows)]
mod win {
    const SERVICE_NAME: &str = "AtCleanupService";
    const SERVICE_DISPLAY_NAME: &str = "AT Device Cleanup Service";
    const SERVICE_DESCRIPTION: &str =
        "AT 网维系统客户端：负责扫描局域网网维服务器、完成授权握手并执行设备清理。";

    use std::ffi::OsString;
    use std::sync::Arc;
    use std::time::Duration;
    use windows_service::service::{
        ServiceAccess, ServiceAction, ServiceActionType, ServiceControl, ServiceControlAccept,
        ServiceErrorControl, ServiceExitCode, ServiceFailureActions, ServiceFailureResetPeriod,
        ServiceInfo, ServiceStartType, ServiceState, ServiceStatus, ServiceType,
    };
    use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
    use windows_service::service_dispatcher;
    use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
    use windows_service::Error as SvcError;

    fn open_manager(access: ServiceManagerAccess) -> Option<ServiceManager> {
        match ServiceManager::local_computer(None::<&str>, access) {
            Ok(m) => Some(m),
            Err(e) => {
                eprintln!(
                    "无法连接服务控制管理器（请使用管理员权限运行）：{}",
                    e
                );
                None
            }
        }
    }

    fn service_exists(manager: &ServiceManager) -> bool {
        manager
            .open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS)
            .is_ok()
    }

    fn apply_extra_config(service: &windows_service::service::Service) {
        if let Err(e) = service.set_description(SERVICE_DESCRIPTION) {
            eprintln!("设置服务描述失败：{}", e);
        }
        if let Err(e) = service.set_delayed_auto_start(true) {
            eprintln!("设置延迟启动失败：{}", e);
        }
        let actions = vec![
            ServiceAction {
                action_type: ServiceActionType::Restart,
                delay: Duration::from_secs(5),
            },
            ServiceAction {
                action_type: ServiceActionType::Restart,
                delay: Duration::from_secs(30),
            },
            ServiceAction {
                action_type: ServiceActionType::Restart,
                delay: Duration::from_secs(60),
            },
        ];
        let failure = ServiceFailureActions {
            reset_period: ServiceFailureResetPeriod::After(Duration::from_secs(86400)),
            reboot_msg: None,
            command: None,
            actions: Some(actions),
        };
        if let Err(e) = service.update_failure_actions(failure) {
            eprintln!("设置失败恢复策略失败：{}", e);
        }
        if let Err(e) = service.set_failure_actions_on_non_crash_failures(true) {
            eprintln!("启用非崩溃失败恢复失败：{}", e);
        }
    }

    /// Register and start the Windows service.
    pub fn install() {
        let Some(manager) = open_manager(
            ServiceManagerAccess::CREATE_SERVICE | ServiceManagerAccess::CONNECT,
        ) else {
            return;
        };

        if service_exists(&manager) {
            eprintln!(
                "服务 '{}' 已安装，如需重装请先运行 uninstall。",
                SERVICE_NAME
            );
            return;
        }

        let exe_path = match std::env::current_exe() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("获取当前程序路径失败：{}", e);
                return;
            }
        };

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

        let service = match manager.create_service(
            &service_info,
            ServiceAccess::START
                | ServiceAccess::QUERY_STATUS
                | ServiceAccess::CHANGE_CONFIG,
        ) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("创建服务失败：{}", e);
                return;
            }
        };

        eprintln!("服务 '{}' 创建成功。", SERVICE_NAME);
        apply_extra_config(&service);

        match service.start::<String>(&[]) {
            Ok(_) => eprintln!("服务 '{}' 已启动。", SERVICE_NAME),
            Err(e) => eprintln!("启动服务失败：{}", e),
        }
    }

    /// Stop and delete the Windows service.
    pub fn uninstall() {
        let Some(manager) = open_manager(ServiceManagerAccess::CONNECT) else {
            return;
        };

        let service = match manager.open_service(
            SERVICE_NAME,
            ServiceAccess::STOP | ServiceAccess::DELETE | ServiceAccess::QUERY_STATUS,
        ) {
            Ok(s) => s,
            Err(SvcError::Winapi(e)) if e.raw_os_error() == Some(1060) => {
                eprintln!("服务 '{}' 未安装，跳过卸载。", SERVICE_NAME);
                return;
            }
            Err(e) => {
                eprintln!("打开服务失败：{}", e);
                return;
            }
        };

        let _ = service.stop();
        eprintln!("已向服务 '{}' 发送停止信号。", SERVICE_NAME);

        // Wait up to 10s for the service to actually stop before deleting.
        for _ in 0..20 {
            match service.query_status() {
                Ok(s) if s.current_state == ServiceState::Stopped => break,
                _ => std::thread::sleep(Duration::from_millis(500)),
            }
        }

        match service.delete() {
            Ok(_) => eprintln!("服务 '{}' 已删除。", SERVICE_NAME),
            Err(e) => eprintln!("删除服务失败：{}", e),
        }
    }

    /// Start an installed service.
    pub fn start_service() {
        let Some(manager) = open_manager(ServiceManagerAccess::CONNECT) else {
            return;
        };
        let service = match manager.open_service(SERVICE_NAME, ServiceAccess::START) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("打开服务失败：{}", e);
                return;
            }
        };
        match service.start::<String>(&[]) {
            Ok(_) => eprintln!("服务 '{}' 启动命令已发送。", SERVICE_NAME),
            Err(e) => eprintln!("启动服务失败：{}", e),
        }
    }

    /// Stop a running service.
    pub fn stop_service() {
        let Some(manager) = open_manager(ServiceManagerAccess::CONNECT) else {
            return;
        };
        let service = match manager
            .open_service(SERVICE_NAME, ServiceAccess::STOP | ServiceAccess::QUERY_STATUS)
        {
            Ok(s) => s,
            Err(e) => {
                eprintln!("打开服务失败：{}", e);
                return;
            }
        };
        match service.stop() {
            Ok(_) => eprintln!("服务 '{}' 停止命令已发送。", SERVICE_NAME),
            Err(e) => eprintln!("停止服务失败：{}", e),
        }
    }

    /// Stop-then-start the service.
    pub fn restart_service() {
        stop_service();
        std::thread::sleep(Duration::from_secs(1));
        start_service();
    }

    /// Query and print the service status.
    pub fn status() {
        let Some(manager) = open_manager(ServiceManagerAccess::CONNECT) else {
            return;
        };
        match manager.open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS) {
            Ok(service) => match service.query_status() {
                Ok(status) => {
                    let state_str = match status.current_state {
                        ServiceState::Stopped => "已停止",
                        ServiceState::StartPending => "启动中",
                        ServiceState::StopPending => "停止中",
                        ServiceState::Running => "运行中",
                        ServiceState::ContinuePending => "恢复中",
                        ServiceState::PausePending => "暂停中",
                        ServiceState::Paused => "已暂停",
                    };
                    println!("服务 '{}': {}", SERVICE_NAME, state_str);
                }
                Err(e) => println!("查询服务状态失败：{}", e),
            },
            Err(SvcError::Winapi(e)) if e.raw_os_error() == Some(1060) => {
                println!("服务 '{}' 未安装。", SERVICE_NAME);
            }
            Err(e) => println!("打开服务失败：{}", e),
        }
    }

    windows_service::define_windows_service!(ffi_service_main, service_main);

    /// Dispatch to the Windows Service Control Manager.
    pub fn dispatch() -> windows_service::Result<()> {
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }

    fn service_main(_args: Vec<OsString>) {
        let shutdown = Arc::new(tokio::sync::Notify::new());
        let shutdown_clone = shutdown.clone();

        let status_handle =
            service_control_handler::register(SERVICE_NAME, move |control| match control {
                ServiceControl::Stop | ServiceControl::Shutdown => {
                    shutdown_clone.notify_waiters();
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

        let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
        rt.block_on(async move {
            let (event_tx, _) =
                tokio::sync::broadcast::channel::<crate::web::ClientEvent>(256);
            crate::web::init_logger(event_tx.clone());
            crate::init_file_logger_if_possible();
            let state = std::sync::Arc::new(crate::web::ClientState::new(event_tx));

            tokio::spawn(crate::web::start_web_server(state.clone()));

            loop {
                match crate::protocol::run_cleanup_cycle(state.clone()).await {
                    Ok(_) => {
                        log::info!("Cleanup cycle completed successfully");
                        crate::escalation::on_cycle_success(&state);
                    }
                    Err(e) => {
                        log::error!("Cleanup cycle failed: {}", e);
                        crate::escalation::on_cycle_failure(&state).await;
                    }
                }

                *state.connection.write().await = "idle".to_string();
                *state.auth.write().await = "pending".to_string();
                *state.heartbeat.write().await = "--".to_string();
                crate::web::broadcast_status(&state).await;

                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(60)) => continue,
                    _ = shutdown.notified() => {
                        log::info!("Received stop signal");
                        break;
                    }
                }
            }
        });

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
pub fn start_service() {
    win::start_service();
}

#[cfg(not(windows))]
pub fn start_service() {
    eprintln!("Windows service start is not supported on this platform");
}

#[cfg(windows)]
pub fn stop_service() {
    win::stop_service();
}

#[cfg(not(windows))]
pub fn stop_service() {
    eprintln!("Windows service stop is not supported on this platform");
}

#[cfg(windows)]
pub fn restart_service() {
    win::restart_service();
}

#[cfg(not(windows))]
pub fn restart_service() {
    eprintln!("Windows service restart is not supported on this platform");
}

#[cfg(windows)]
pub fn dispatch() -> windows_service::Result<()> {
    win::dispatch()
}
