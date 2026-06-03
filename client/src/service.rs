// Boot-task management for the AT client.
//
// Older builds installed a long-running Windows SERVICE (AtCleanupService).
// Anti-cheat engines (Tencent ACE, used by 三角洲行动) flagged that service even
// when it was only *stopped*: a registered SYSTEM service whose image imports
// device-removal / session APIs looks like a cheat loader, so the bare presence
// of the service registration crashed the game. Operator confirmed that only
// *uninstalling* the service — not stopping it — avoids the crash.
//
// So we no longer run as a service at all. Instead we register a SCHEDULED TASK
// that runs `at-client.exe boot` once at every logon, as SYSTEM. That task:
//   * does the one-shot privileged cleanup, hands the licensing heartbeat off
//     to a normal-user process, and exits within seconds — before any game;
//   * leaves NO service registered and nothing resident while games run (the
//     exact "uninstalled" state the operator verified as working);
//   * is not self-deleting, so it survives in the captured master image and
//     re-runs on every boot of the deployed restore-on-reboot machines without
//     any reinstall.

#[cfg(windows)]
mod win {
    const TASK_NAME: &str = "ATBootCleanup";
    const LEGACY_SERVICE_NAME: &str = "AtCleanupService";
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    use std::os::windows::process::CommandExt;
    use std::process::Command;

    fn current_exe_string() -> Option<String> {
        std::env::current_exe()
            .ok()
            .map(|p| p.display().to_string())
    }

    /// Stop and delete any legacy AtCleanupService left by older builds — that
    /// service registration is exactly what anti-cheat flagged.
    fn remove_legacy_service() {
        for args in [["stop", LEGACY_SERVICE_NAME], ["delete", LEGACY_SERVICE_NAME]] {
            let _ = Command::new("sc")
                .args(args)
                .creation_flags(CREATE_NO_WINDOW)
                .status();
        }
    }

    /// Register the boot task (ONLOGON, runs as SYSTEM) and trigger it once.
    pub fn install() {
        let Some(exe) = current_exe_string() else {
            eprintln!("无法获取当前程序路径。");
            return;
        };

        remove_legacy_service();

        // Inner quotes keep the exe path intact even if it contains spaces.
        let run = format!("\"{}\" boot", exe);
        let create = Command::new("schtasks")
            .args([
                "/Create", "/TN", TASK_NAME, "/TR", &run, "/SC", "ONLOGON", "/RU", "SYSTEM", "/F",
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .status();

        match create {
            Ok(s) if s.success() => {
                eprintln!(
                    "计划任务 '{}' 创建成功（每次登录时以 SYSTEM 运行一次）。",
                    TASK_NAME
                );
                // Run once now so the current session is set up without a reboot.
                let _ = Command::new("schtasks")
                    .args(["/Run", "/TN", TASK_NAME])
                    .creation_flags(CREATE_NO_WINDOW)
                    .status();
                eprintln!("已立即触发一次，可以直接打包镜像。");
            }
            Ok(s) => eprintln!(
                "创建计划任务失败（退出码 {:?}）——请用管理员权限运行。",
                s.code()
            ),
            Err(e) => eprintln!("创建计划任务失败：{}（需要管理员权限）", e),
        }
    }

    /// Delete the boot task (and any legacy service).
    pub fn uninstall() {
        remove_legacy_service();
        let del = Command::new("schtasks")
            .args(["/Delete", "/TN", TASK_NAME, "/F"])
            .creation_flags(CREATE_NO_WINDOW)
            .status();
        match del {
            Ok(s) if s.success() => eprintln!("计划任务 '{}' 已删除。", TASK_NAME),
            Ok(_) => eprintln!("计划任务 '{}' 不存在或删除失败。", TASK_NAME),
            Err(e) => eprintln!("删除计划任务失败：{}", e),
        }
    }

    /// Print the boot task status.
    pub fn status() {
        let out = Command::new("schtasks")
            .args(["/Query", "/TN", TASK_NAME, "/V", "/FO", "LIST"])
            .status();
        if let Ok(s) = out {
            if !s.success() {
                eprintln!("计划任务 '{}' 未安装。", TASK_NAME);
            }
        }
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
    eprintln!("计划任务安装仅支持 Windows。");
}

#[cfg(windows)]
pub fn uninstall() {
    win::uninstall();
}

#[cfg(not(windows))]
pub fn uninstall() {
    eprintln!("计划任务卸载仅支持 Windows。");
}

#[cfg(windows)]
pub fn status() {
    win::status();
}

#[cfg(not(windows))]
pub fn status() {
    eprintln!("状态查询仅支持 Windows。");
}
