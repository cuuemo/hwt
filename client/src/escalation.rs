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
        broadcast_log(&state.event_tx, "warn", "Authorization failed — opening purchase page");
        open_browser(PURCHASE_URL);
    } else if count == WARN_SHUTDOWN_THRESHOLD {
        broadcast_log(
            &state.event_tx,
            "error",
            "Authorization still failed — system will shut down soon!",
        );
        open_browser(PURCHASE_URL);
        show_warning(
            "AT Warning",
            "Authorization expired!\nSystem will shut down after next check.\nPlease purchase a license.",
        );
    } else if count >= SHUTDOWN_THRESHOLD {
        broadcast_log(&state.event_tx, "error", "Shutting down system — unauthorized");
        open_browser(PURCHASE_URL);
        show_warning(
            "AT Shutdown",
            "Authorization expired!\nSystem is shutting down in 30 seconds.",
        );
        shutdown_system();
    }
}

/// Called after a successful cleanup cycle. Resets the counter.
pub fn on_cycle_success(state: &Arc<ClientState>) {
    state.unauthorized_count.store(0, Ordering::Relaxed);
}

// ─── System-level actions (Windows) ───────────────────────────────

#[cfg(windows)]
fn open_browser(url: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::core::PCWSTR;

    let operation: Vec<u16> = OsStr::new("open").encode_wide().chain(Some(0)).collect();
    let url_w: Vec<u16> = OsStr::new(url).encode_wide().chain(Some(0)).collect();

    unsafe {
        ShellExecuteW(
            None,
            PCWSTR(operation.as_ptr()),
            PCWSTR(url_w.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL,
        );
    }
    log::info!("Opened browser: {}", url);
}

#[cfg(not(windows))]
fn open_browser(url: &str) {
    log::info!("[non-windows] Would open browser: {}", url);
}

#[cfg(windows)]
fn show_warning(title: &str, message: &str) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, MB_ICONWARNING, MB_OK, MB_SETFOREGROUND, MB_TOPMOST,
    };
    use windows::core::PCWSTR;

    let title_w: Vec<u16> = OsStr::new(title).encode_wide().chain(Some(0)).collect();
    let msg_w: Vec<u16> = OsStr::new(message).encode_wide().chain(Some(0)).collect();

    // Spawn in a thread so it doesn't block the async runtime
    let title_w = title_w.clone();
    let msg_w = msg_w.clone();
    std::thread::spawn(move || unsafe {
        MessageBoxW(
            None,
            PCWSTR(msg_w.as_ptr()),
            PCWSTR(title_w.as_ptr()),
            MB_OK | MB_ICONWARNING | MB_TOPMOST | MB_SETFOREGROUND,
        );
    });
    log::info!("Showed system warning: {}", title);
}

#[cfg(not(windows))]
fn show_warning(title: &str, message: &str) {
    log::info!("[non-windows] Warning dialog: {} — {}", title, message);
}

#[cfg(windows)]
fn shutdown_system() {
    log::warn!("Initiating system shutdown in 30 seconds");
    let _ = std::process::Command::new("shutdown")
        .args(["/s", "/t", "30", "/c", "AT: Authorization expired. Shutting down."])
        .spawn();
}

#[cfg(not(windows))]
fn shutdown_system() {
    log::warn!("[non-windows] Would shut down system");
}
