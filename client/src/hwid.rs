use std::io::Result;

/// 随机化 Windows 机器标识 (仅修改经微软文档验证确认的注册表值)
///
/// 修改以下 5 个位置:
/// 1. HKLM\SOFTWARE\Microsoft\Cryptography → MachineGuid
///    格式: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx (小写无大括号)
///    用途: 最广泛使用的机器指纹, 大量软件读取此值
///
/// 2. HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion → ProductId
///    格式: XXXXX-XXX-XXXXXXX-XXXXX (5-3-7-5位数字, 4段)
///    用途: Windows 产品ID, 软件许可/遥测常读取
///
/// 3. HKLM\SOFTWARE\Microsoft\SQMClient → MachineId
///    格式: {XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX} (大写有大括号)
///    用途: 软件质量度量客户端ID
///
/// 4. HKLM\SYSTEM\CurrentControlSet\Control\IDConfigDB\Hardware Profiles\0001 → HwProfileGuid
///    格式: {xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx} (小写有大括号)
///    用途: 硬件配置文件GUID, GetCurrentHwProfile() API 返回此值
///
/// 5. HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate → SusClientId
///    格式: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx (小写无大括号)
///    用途: Windows Update 客户端标识
///    附带删除 SusClientIdValidation (二进制校验值, 不删会校验失败)
#[cfg(windows)]
pub fn randomize_machine_ids() -> Result<(String, String)> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // === 1. MachineGuid (最重要的机器指纹) ===
    let machine_guid = generate_guid_lower();
    write_reg(&hklm,
        r"SOFTWARE\Microsoft\Cryptography",
        "MachineGuid", &machine_guid, "MachineGuid");

    // === 2. ProductId (格式: XXXXX-XXX-XXXXXXX-XXXXX) ===
    let product_id = generate_product_id();
    write_reg(&hklm,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "ProductId", &product_id, "ProductId");

    // === 3. SQMClient MachineId ===
    let sqm_id = format!("{{{}}}", generate_guid_upper());
    write_reg(&hklm,
        r"SOFTWARE\Microsoft\SQMClient",
        "MachineId", &sqm_id, "SQM MachineId");

    // === 4. HwProfileGuid ===
    let hw_guid = format!("{{{}}}", generate_guid_lower());
    write_reg(&hklm,
        r"SYSTEM\CurrentControlSet\Control\IDConfigDB\Hardware Profiles\0001",
        "HwProfileGuid", &hw_guid, "HwProfileGuid");

    // === 5. SusClientId (Windows Update) ===
    let sus_id = generate_guid_lower();
    write_reg(&hklm,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate",
        "SusClientId", &sus_id, "SusClientId");

    // 删除 SusClientIdValidation (二进制校验值, 不匹配会导致 WU 重新生成)
    if let Ok(key) = hklm.open_subkey_with_flags(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate",
        KEY_READ | KEY_WRITE,
    ) {
        match key.delete_value("SusClientIdValidation") {
            Ok(_) => log::info!("  ✓ 已删除 SusClientIdValidation (WU将自动重新生成)"),
            Err(_) => log::debug!("  SusClientIdValidation 不存在或已删除"),
        }
    }

    Ok((machine_guid, product_id))
}

#[cfg(windows)]
fn write_reg(hklm: &winreg::RegKey, path: &str, name: &str, value: &str, label: &str) {
    use winreg::enums::*;

    match hklm.open_subkey_with_flags(path, KEY_READ | KEY_WRITE) {
        Ok(key) => {
            if let Ok(old) = key.get_value::<String, _>(name) {
                log::info!("  旧 {}: {}", label, old);
            }
            match key.set_value(name, &value) {
                Ok(_) => log::info!("  ✓ 新 {}: {}", label, value),
                Err(e) => log::error!("  ✗ 写入 {} 失败: {}", label, e),
            }
        }
        Err(_) => {
            // 键不存在, 尝试创建
            match hklm.create_subkey(path) {
                Ok((key, _)) => match key.set_value(name, &value) {
                    Ok(_) => log::info!("  ✓ 新 {} (创建键): {}", label, value),
                    Err(e) => log::error!("  ✗ 写入 {} 失败: {}", label, e),
                },
                Err(e) => log::error!("  ✗ 创建键 {} 失败: {}", path, e),
            }
        }
    }
}

/// 生成 ProductId: XXXXX-XXXXX-XXXXX-XXXXX
/// 格式: 4段各5位, 与 Windows 设置→关于 中显示的格式一致
/// 实例: 00330-80000-00000-AA813
/// 第1段: 产品代码 (00xxx 系列, 如 00330, 00325, 00331)
/// 第2段: OEM/渠道 (如 80000, 10000)
/// 第3段: 序列号 (纯数字)
/// 第4段: 附加标识 (2字母+3数字, 如 AA813)
fn generate_product_id() -> String {
    let b = random_bytes(8);

    // 第1段: 003xx (Windows 10 常见范围)
    let seg1 = format!("00{}", 300 + (b[0] as u32 % 100));

    // 第2段: x0000 (常见: 80000, 10000, 50000, 20000)
    let prefixes = [8, 1, 5, 2, 3, 7];
    let prefix = prefixes[(b[1] as usize) % prefixes.len()];
    let seg2 = format!("{}0000", prefix);

    // 第3段: 5位数字
    let seg3 = format!("{:05}", {
        let v = (b[2] as u32) << 8 | b[3] as u32;
        v % 100000
    });

    // 第4段: 2字母 + 3数字 (如 AA813, BK025)
    let letters = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let c1 = letters[(b[4] as usize) % 26] as char;
    let c2 = letters[(b[5] as usize) % 26] as char;
    let num = ((b[6] as u32) << 8 | b[7] as u32) % 1000;
    let seg4 = format!("{}{}{:03}", c1, c2, num);

    format!("{}-{}-{}-{}", seg1, seg2, seg3, seg4)
}

/// GUID 小写无大括号: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
fn generate_guid_lower() -> String {
    guid_impl(false)
}

/// GUID 大写无大括号: XXXXXXXX-XXXX-4XXX-YXXX-XXXXXXXXXXXX
fn generate_guid_upper() -> String {
    guid_impl(true)
}

fn guid_impl(upper: bool) -> String {
    let mut b = random_bytes(16);
    b[6] = (b[6] & 0x0f) | 0x40; // version 4
    b[8] = (b[8] & 0x3f) | 0x80; // variant 1

    let s = format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
        b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15],
    );
    if upper { s.to_uppercase() } else { s }
}

/// 生成 N 字节伪随机数据 (基于时间纳秒 + 原子计数器)
fn random_bytes(n: usize) -> Vec<u8> {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static CTR: AtomicU64 = AtomicU64::new(0);

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let c = CTR.fetch_add(1, Ordering::Relaxed);
    let mut state = ts ^ c.wrapping_mul(2654435761);

    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        out.push((state >> 33) as u8);
    }
    out
}

#[cfg(not(windows))]
pub fn randomize_machine_ids() -> Result<(String, String)> {
    let guid = generate_guid_lower();
    let pid = generate_product_id();
    log::info!("[非 Windows] 模拟随机化:");
    log::info!("  MachineGuid:   {}", guid);
    log::info!("  ProductId:     {}", pid);
    log::info!("  SQM MachineId: {{{}}}", generate_guid_upper());
    log::info!("  HwProfileGuid: {{{}}}", generate_guid_lower());
    log::info!("  SusClientId:   {}", generate_guid_lower());
    Ok((guid, pid))
}
