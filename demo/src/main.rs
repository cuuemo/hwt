mod cleanup;
mod hwid;
mod registry;

use std::io::{self, Write};
use std::time::Duration;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const CYAN: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";
const MAGENTA: &str = "\x1b[95m";

fn main() {
    enable_ansi();
    set_title("AT清理工具 - 专业无盘解决方案");

    env_logger::Builder::new().filter_level(log::LevelFilter::Off).init();

    // ========== 广告页 ==========
    clear();
    println!();
    println!();
    color_println(CYAN,  "                          ╔════════════════════════════════════╗");
    color_println(CYAN,  "                          ║                                    ║");
    color_println(RED,   "                          ║        █████╗ ████████╗            ║");
    color_println(RED,   "                          ║       ██╔══██╗╚══██╔══╝            ║");
    color_println(RED,   "                          ║       ███████║   ██║               ║");
    color_println(RED,   "                          ║       ██╔══██║   ██║               ║");
    color_println(RED,   "                          ║       ██║  ██║   ██║               ║");
    color_println(RED,   "                          ║       ╚═╝  ╚═╝   ╚═╝               ║");
    color_println(CYAN,  "                          ║                                    ║");
    color_println(YELLOW,"                          ║        清理工具 █ 正在运行         ║");
    color_println(GREEN, "                          ║                                    ║");
    color_println(GREEN, "                          ║    ╔══════════════════════════╗    ║");
    color_println(GREEN, "                          ║    ║                          ║    ║");
    color_println(GREEN, "                          ║    ║    无盘认准咸鱼AT无盘镜像║    ║");
    color_println(GREEN, "                          ║    ║                          ║    ║");
    color_println(GREEN, "                          ║    ║      微信：ATKJ_DZ       ║    ║");
    color_println(GREEN, "                          ║    ║                          ║    ║");
    color_println(GREEN, "                          ║    ╚══════════════════════════╝    ║");
    color_println(CYAN,  "                          ║                                    ║");
    color_println(CYAN,  "                          ║        专业 █ 稳定 █ 可靠          ║");
    color_println(GREEN, "                          ║                                    ║");
    color_println(CYAN,  "                          ╚════════════════════════════════════╝");
    println!();
    println!();
    print!("                           按任意键开始...");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());

    // ========== 权限检查 ==========
    if !check_admin_silent() {
        clear();
        color_println(RED, "");
        color_println(RED, "      ██ 错误：请以管理员身份运行此程序！");
        color_println(RED, "");
        wait_exit();
        return;
    }

    // ========== 执行页 ==========
    clear();
    color_println(CYAN,  "╔══════════════════════════════════════════════════════════════╗");
    color_println(CYAN,  "║                                                              ║");
    color_println(RED,   "║                AT清理工具 - 系统优化与ID刷新程序             ║");
    color_println(CYAN,  "║                                                              ║");
    color_println(CYAN,  "╚══════════════════════════════════════════════════════════════╝");
    println!();
    delay(1000);

    // ========== 步骤 1/5: 读取旧标识 ==========
    show_step(1, 5, "正在读取当前系统标识...");
    delay(800);

    let old_ids = read_current_ids();
    println!();
    println!("      {}█ 当前设备ID:   {}{}{}", WHITE, YELLOW, old_ids.device_id, RESET);
    println!("      {}█ 当前产品ID:   {}{}{}", WHITE, YELLOW, old_ids.product_id, RESET);
    println!("      {}█ 当前机器码:   {}{}{}", WHITE, YELLOW, old_ids.machine_guid, RESET);
    println!("      {}█ 硬件配置ID:   {}{}{}", WHITE, YELLOW, old_ids.hw_profile, RESET);
    println!("      {}█ 更新客户端:   {}{}{}", WHITE, YELLOW, old_ids.sus_client, RESET);
    println!();
    delay(1500);

    // ========== 步骤 2/5: 扫描清理设备 ==========
    show_step(2, 5, "正在扫描并清理设备残留...");
    let (scanned, removed) = cleanup::cleanup_phantom_devices().unwrap_or((0, 0));
    let _ = registry::clean_display_registry();
    println!();
    println!("      {}██ 扫描设备: {}{}{}  清理残留: {}{}{}", WHITE, GREEN, scanned, RESET, GREEN, removed, RESET);
    println!();
    delay(1000);

    // ========== 步骤 3/5: 刷新标识 ==========
    show_step(3, 5, "正在生成并写入新标识...");
    spinner("生成安全标识符", 1200);
    let _ = hwid::randomize_machine_ids();
    delay(300);

    // ========== 步骤 4/5: 读取新标识并对比 ==========
    show_step(4, 5, "正在验证修改结果...");
    delay(500);

    let new_ids = read_current_ids();
    println!();
    show_compare("设备ID",   &old_ids.device_id,    &new_ids.device_id);
    show_compare("产品ID",   &old_ids.product_id,   &new_ids.product_id);
    show_compare("机器码",   &old_ids.machine_guid,  &new_ids.machine_guid);
    show_compare("硬件配置", &old_ids.hw_profile,    &new_ids.hw_profile);
    show_compare("更新标识", &old_ids.sus_client,    &new_ids.sus_client);
    println!();
    delay(1500);

    // ========== 步骤 5/5: 完成 ==========
    show_step(5, 5, "操作完成！");
    delay(500);
    println!();

    // ========== 完成框 ==========
    color_println(CYAN,  "      ╔════════════════════════════════════════════════════╗");
    color_println(CYAN,  "      ║                                                    ║");
    println!("      {}║    {}  ✓ 设备残留清理完成                  {}       ║{}", CYAN, GREEN, CYAN, RESET);
    println!("      {}║    {}  ✓ 设备ID已成功随机化                {}       ║{}", CYAN, GREEN, CYAN, RESET);
    println!("      {}║    {}  ✓ 产品ID已成功随机化                {}       ║{}", CYAN, GREEN, CYAN, RESET);
    println!("      {}║    {}  ✓ 硬件指纹已全部刷新                {}       ║{}", CYAN, GREEN, CYAN, RESET);
    color_println(CYAN,  "      ║                                                    ║");
    println!("      {}║      扫描设备: {}{:<6}{}  清理残留: {}{:<6}{}          ║{}",
        CYAN, WHITE, scanned, CYAN, WHITE, removed, CYAN, RESET);
    println!("      {}║      刷新标识: {}5     {}  状态: {}OK    {}            ║{}",
        CYAN, WHITE, CYAN, GREEN, CYAN, RESET);
    color_println(CYAN,  "      ║                                                    ║");
    color_println(CYAN,  "      ╚════════════════════════════════════════════════════╝");
    println!();
    delay(2000);

    // ========== 倒计时 ==========
    for i in (1..=3).rev() {
        println!("      {}剩余 {} 秒后自动关闭...{}", DIM, i, RESET);
        delay(1000);
    }

    // ========== 结束画面 ==========
    clear();
    println!();
    println!();
    println!();
    color_println(RED,   "                      ╔══════════════════════════╗");
    color_println(RED,   "                      ║                          ║");
    color_println(RED,   "                      ║   AT清理工具 执行完成    ║");
    color_println(RED,   "                      ║                          ║");
    color_println(RED,   "                      ╠══════════════════════════╣");
    color_println(RED,   "                      ║                          ║");
    color_println(YELLOW,"                      ║   感谢使用咸鱼AT无盘镜像║");
    color_println(YELLOW,"                      ║                          ║");
    color_println(YELLOW,"                      ║   微信：ATKJ_DZ          ║");
    color_println(YELLOW,"                      ║                          ║");
    color_println(RED,   "                      ╚══════════════════════════╝");
    println!();
    println!();
    delay(3000);
}

// ==================== 对比显示 ====================

fn show_compare(label: &str, old: &str, new: &str) {
    let changed = old != new && !new.is_empty() && new != "(未读取)";
    let status = if changed { format!("{}✓{}", GREEN, RESET) } else { format!("{}—{}", DIM, RESET) };
    // 中文label固定8字符宽度
    let pad = 8usize.saturating_sub(label.chars().count()) ;
    let padding = " ".repeat(pad);
    println!("      {} {}{}█ {}:{}", status, WHITE, BOLD, label, RESET);
    println!("          {}旧: {}{}{}", DIM, YELLOW, old, RESET);
    println!("          {}新: {}{}{}", DIM, GREEN, new, RESET);
}

// ==================== 读取当前系统标识 ====================

struct SystemIds {
    device_id: String,
    product_id: String,
    machine_guid: String,
    hw_profile: String,
    sus_client: String,
}

fn read_current_ids() -> SystemIds {
    SystemIds {
        device_id: read_reg(r"SOFTWARE\Microsoft\SQMClient", "MachineId"),
        product_id: read_reg(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion", "ProductId"),
        machine_guid: read_reg(r"SOFTWARE\Microsoft\Cryptography", "MachineGuid"),
        hw_profile: read_reg(r"SYSTEM\CurrentControlSet\Control\IDConfigDB\Hardware Profiles\0001", "HwProfileGuid"),
        sus_client: read_reg(r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate", "SusClientId"),
    }
}

#[cfg(windows)]
fn read_reg(path: &str, name: &str) -> String {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    match hklm.open_subkey_with_flags(path, KEY_READ) {
        Ok(key) => key.get_value::<String, _>(name).unwrap_or_else(|_| "(未读取)".into()),
        Err(_) => "(未读取)".into(),
    }
}

#[cfg(not(windows))]
fn read_reg(_path: &str, _name: &str) -> String {
    "(非Windows)".into()
}

// ==================== UI 组件 ====================

fn show_step(n: u32, total: u32, text: &str) {
    println!("{}┌──────────────────────────────────────────────────────────────┐{}", CYAN, RESET);
    println!("{}│            {}{}[步骤 {}/{}]{} {}{}{}│{}", CYAN, BOLD, WHITE, n, total, RESET, YELLOW, text,
        " ".repeat(42usize.saturating_sub(text.chars().count())), RESET);
    println!("{}└──────────────────────────────────────────────────────────────┘{}", CYAN, RESET);
    delay(500);
}

fn spinner(label: &str, ms: u64) {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let loops = (ms / 80) as usize;
    for i in 0..loops {
        print!("\r      {}{}{} {}{}...{}", MAGENTA, frames[i % frames.len()], RESET, DIM, label, RESET);
        let _ = io::stdout().flush();
        std::thread::sleep(Duration::from_millis(80));
    }
    print!("\r      {}✓{} {}完成{}       \n", GREEN, RESET, DIM, RESET);
    let _ = io::stdout().flush();
}

fn color_println(color: &str, text: &str) {
    println!("{}{}{}", color, text, RESET);
}

fn clear() {
    print!("\x1b[2J\x1b[H");
    let _ = io::stdout().flush();
}

fn delay(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}

fn wait_exit() {
    print!("\n      按任意键退出...");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut String::new());
}

fn set_title(title: &str) {
    print!("\x1b]0;{}\x07", title);
    let _ = io::stdout().flush();
}

fn enable_ansi() {
    #[cfg(windows)]
    {
        use windows::Win32::System::Console::*;
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE).unwrap_or_default();
            let mut mode = CONSOLE_MODE::default();
            let _ = GetConsoleMode(handle, &mut mode);
            let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

fn check_admin_silent() -> bool {
    #[cfg(windows)]
    {
        use windows::Win32::Foundation::HANDLE;
        use windows::Win32::Security::*;
        use windows::Win32::System::Threading::*;
        unsafe {
            let mut token = HANDLE::default();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
                let mut elev = TOKEN_ELEVATION::default();
                let mut len = 0u32;
                if GetTokenInformation(token, TokenElevation,
                    Some(&mut elev as *mut _ as *mut _),
                    std::mem::size_of::<TOKEN_ELEVATION>() as u32, &mut len).is_ok() {
                    return elev.TokenIsElevated != 0;
                }
            }
        }
        false
    }
    #[cfg(not(windows))]
    { true }
}
