use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::web::{broadcast_log, ClientState};

const PURCHASE_URL: &str = "https://m.tb.cn/h.imD9Vqp?tk=SSp054qFuub";
const OPEN_BROWSER_THRESHOLD: u32 = 5;
const WARN_SHUTDOWN_THRESHOLD: u32 = 8;
const SHUTDOWN_THRESHOLD: u32 = 9;

/// Called after each failed cleanup cycle. Increments counter and triggers
/// escalating system-level actions.
pub async fn on_cycle_failure(state: &Arc<ClientState>) {
    let count = state.unauthorized_count.fetch_add(1, Ordering::Relaxed) + 1;
    log::warn!("Consecutive unauthorized cycles: {}", count);

    if count == OPEN_BROWSER_THRESHOLD {
        broadcast_log(&state.event_tx, "warn", "授权失败 — 正在打开购买页面");
        open_browser(PURCHASE_URL);
    } else if count == WARN_SHUTDOWN_THRESHOLD {
        broadcast_log(
            &state.event_tx,
            "error",
            "授权仍然失败 — 系统即将关机！",
        );
        open_browser(PURCHASE_URL);
        show_warning(
            "AT 警告",
            "授权已过期！\n系统将在下次检查后关机。\n请及时购买授权。",
        );
    } else if count >= SHUTDOWN_THRESHOLD {
        broadcast_log(&state.event_tx, "error", "系统即将关机 — 未授权");
        open_browser(PURCHASE_URL);
        show_warning(
            "AT 关机",
            "授权已过期！\n系统将在 30 秒后关机。",
        );
        shutdown_system();
    }
}

/// Called after a successful cleanup cycle. Resets the counter.
pub fn on_cycle_success(state: &Arc<ClientState>) {
    state.unauthorized_count.store(0, Ordering::Relaxed);
}

// ─── Session-0-aware Windows helpers ──────────────────────────────
//
// A Windows service runs in Session 0, isolated from the user desktop
// (Session 1+). Plain MessageBoxW / ShellExecuteW from here will appear
// on an invisible window station. We route GUI actions through the
// Terminal Services APIs so they reach the active console session.

#[cfg(windows)]
fn active_session_user_token() -> Option<windows::Win32::Foundation::HANDLE> {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::System::RemoteDesktop::{WTSGetActiveConsoleSessionId, WTSQueryUserToken};

    unsafe {
        let session_id = WTSGetActiveConsoleSessionId();
        if session_id == 0xFFFF_FFFF {
            log::warn!("No active console session");
            return None;
        }
        let mut token = HANDLE::default();
        match WTSQueryUserToken(session_id, &mut token) {
            Ok(()) => Some(token),
            Err(e) => {
                log::warn!("WTSQueryUserToken failed: {}", e);
                None
            }
        }
    }
}

#[cfg(windows)]
fn open_browser(url: &str) {
    use windows::core::{PCWSTR, PWSTR};
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Environment::{CreateEnvironmentBlock, DestroyEnvironmentBlock};
    use windows::Win32::System::Threading::{
        CreateProcessAsUserW, CREATE_NO_WINDOW, CREATE_UNICODE_ENVIRONMENT, PROCESS_INFORMATION,
        STARTUPINFOW,
    };

    let Some(token) = active_session_user_token() else {
        log::warn!("Cannot open browser: no user session");
        return;
    };

    let cmdline = format!("cmd.exe /c start \"\" \"{}\"", url);
    let mut cmd_w: Vec<u16> = cmdline.encode_utf16().chain(Some(0)).collect();

    unsafe {
        let mut env_block: *mut core::ffi::c_void = std::ptr::null_mut();
        let have_env = CreateEnvironmentBlock(&mut env_block, token, false).is_ok();

        let mut si: STARTUPINFOW = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();

        let mut flags = CREATE_NO_WINDOW;
        if have_env {
            flags |= CREATE_UNICODE_ENVIRONMENT;
        }

        let result = CreateProcessAsUserW(
            token,
            PCWSTR::null(),
            PWSTR(cmd_w.as_mut_ptr()),
            None,
            None,
            false,
            flags,
            if have_env { Some(env_block) } else { None },
            PCWSTR::null(),
            &si,
            &mut pi,
        );

        match result {
            Ok(()) => {
                log::info!("Opened browser in user session: {}", url);
                let _ = CloseHandle(pi.hProcess);
                let _ = CloseHandle(pi.hThread);
            }
            Err(e) => log::error!("CreateProcessAsUserW failed: {}", e),
        }

        if have_env {
            let _ = DestroyEnvironmentBlock(env_block);
        }
        let _ = CloseHandle(token);
    }
}

#[cfg(not(windows))]
fn open_browser(url: &str) {
    log::info!("[non-windows] Would open browser: {}", url);
}

#[cfg(windows)]
fn show_warning(title: &str, message: &str) {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::System::RemoteDesktop::{WTSGetActiveConsoleSessionId, WTSSendMessageW};
    use windows::Win32::UI::WindowsAndMessaging::{
        MB_ICONWARNING, MB_OK, MB_SETFOREGROUND, MB_TOPMOST, MESSAGEBOX_RESULT,
    };

    unsafe {
        let session_id = WTSGetActiveConsoleSessionId();
        if session_id == 0xFFFF_FFFF {
            log::warn!("No active console session for warning dialog");
            return;
        }

        let title_w: Vec<u16> = title.encode_utf16().collect();
        let msg_w: Vec<u16> = message.encode_utf16().collect();
        let mut response = MESSAGEBOX_RESULT(0);

        let result = WTSSendMessageW(
            HANDLE::default(), // WTS_CURRENT_SERVER_HANDLE
            session_id,
            PCWSTR(title_w.as_ptr()),
            (title_w.len() * 2) as u32,
            PCWSTR(msg_w.as_ptr()),
            (msg_w.len() * 2) as u32,
            MB_OK | MB_ICONWARNING | MB_TOPMOST | MB_SETFOREGROUND,
            0,     // timeout = no timeout
            &mut response,
            false, // don't wait for user response
        );
        match result {
            Ok(()) => log::info!("Showed system warning: {}", title),
            Err(e) => log::error!("WTSSendMessageW failed: {}", e),
        }
    }
}

#[cfg(not(windows))]
fn show_warning(title: &str, message: &str) {
    log::info!("[non-windows] Warning dialog: {} — {}", title, message);
}

#[cfg(windows)]
fn enable_shutdown_privilege() -> windows::core::Result<()> {
    use windows::core::w;
    use windows::Win32::Foundation::{CloseHandle, HANDLE, LUID};
    use windows::Win32::Security::{
        AdjustTokenPrivileges, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED,
        TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = HANDLE::default();
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        )?;

        let mut luid = LUID::default();
        LookupPrivilegeValueW(None, w!("SeShutdownPrivilege"), &mut luid)?;

        let tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };
        let result = AdjustTokenPrivileges(token, false, Some(&tp), 0, None, None);
        let _ = CloseHandle(token);
        result
    }
}

#[cfg(windows)]
fn shutdown_system() {
    log::warn!("Initiating system shutdown in 30 seconds");
    if let Err(e) = enable_shutdown_privilege() {
        log::warn!("Failed to enable shutdown privilege: {}", e);
    }
    match std::process::Command::new("shutdown")
        .args(["/s", "/t", "30", "/c", "AT：授权已过期，系统即将关机。"])
        .spawn()
    {
        Ok(_) => log::info!("shutdown.exe spawned"),
        Err(e) => log::error!("Failed to spawn shutdown.exe: {}", e),
    }
}

#[cfg(not(windows))]
fn shutdown_system() {
    log::warn!("[non-windows] Would shut down system");
}
