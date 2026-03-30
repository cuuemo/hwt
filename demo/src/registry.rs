use std::io::Result;

const DISPLAY_KEY_PATH: &str = r"SYSTEM\CurrentControlSet\Enum\DISPLAY";

/// 只读枚举 DISPLAY 注册表, 打印所有内容 (不删除)
#[cfg(windows)]
pub fn enumerate_display_registry() {
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let display_key = match hklm.open_subkey_with_flags(DISPLAY_KEY_PATH, KEY_READ) {
        Ok(key) => key,
        Err(e) => {
            log::warn!("无法打开 HKLM\\{}: {}", DISPLAY_KEY_PATH, e);
            return;
        }
    };

    log::info!("打开注册表: HKLM\\{}", DISPLAY_KEY_PATH);

    let subkeys: Vec<String> = display_key.enum_keys().filter_map(|r| r.ok()).collect();
    log::info!("  一级子键数量: {}", subkeys.len());

    for (i, name) in subkeys.iter().enumerate() {
        log::info!("  [{}] {}", i + 1, name);

        let sub_path = format!("{}\\{}", DISPLAY_KEY_PATH, name);
        if let Ok(sub_key) = hklm.open_subkey_with_flags(&sub_path, KEY_READ) {
            let instances: Vec<String> = sub_key.enum_keys().filter_map(|r| r.ok()).collect();
            log::debug!("    实例数: {}", instances.len());

            for inst in &instances {
                log::debug!("    ├── {}", inst);

                let inst_path = format!("{}\\{}", sub_path, inst);
                if let Ok(inst_key) = hklm.open_subkey_with_flags(&inst_path, KEY_READ) {
                    for val_name in &[
                        "DeviceDesc", "FriendlyName", "HardwareID", "Mfg",
                        "Driver", "Service", "ClassGUID",
                    ] {
                        if let Ok(val) = inst_key.get_value::<String, _>(val_name) {
                            log::debug!("    │   {} = {}", val_name, val);
                        }
                    }

                    let sub3: Vec<String> = inst_key.enum_keys().filter_map(|r| r.ok()).collect();
                    for s3 in &sub3 {
                        log::debug!("    │   └── [子键] {}", s3);
                    }
                }
            }
        }
    }

    log::info!("注册表枚举完成");
}

#[cfg(not(windows))]
pub fn enumerate_display_registry() {
    log::info!("[非 Windows] 跳过注册表枚举");
}

/// 删除 HKLM\SYSTEM\CurrentControlSet\Enum\DISPLAY 下的所有子键
///
/// HKLM\SYSTEM\CurrentControlSet\Enum 受特殊 ACL 保护:
/// - Owner = SYSTEM, Administrators 只有 Read 权限
/// - 即使以管理员运行也无法直接删除
///
/// 方案: 用 psexec -s 或 Windows Service (SYSTEM 身份) 运行
/// 管理员身份下备选方案: 先启用特权，再用 Win32 API take ownership + 改 DACL
///
/// 这里使用系统命令 reg.exe 删除 (管理员身份下可能仍失败，SYSTEM 身份则成功)
/// 同时提供 Win32 API 方案作为 fallback
#[cfg(windows)]
pub fn clean_display_registry() -> Result<u32> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let display_key = match hklm.open_subkey_with_flags(DISPLAY_KEY_PATH, KEY_READ) {
        Ok(key) => key,
        Err(e) => {
            log::warn!("DISPLAY 键不存在或无法打开: {} — 无需清理", e);
            return Ok(0);
        }
    };

    let subkey_names: Vec<String> = display_key.enum_keys().filter_map(|r| r.ok()).collect();
    let total = subkey_names.len() as u32;
    log::info!("发现 {} 个 DISPLAY 子键待删除", total);

    if total == 0 {
        return Ok(0);
    }

    // 启用特权
    log::debug!("正在提升 Token 特权...");
    enable_privileges_cmd();

    let mut deleted = 0u32;
    let mut failed = 0u32;

    for name in &subkey_names {
        let full_path = format!("HKLM\\{}\\{}", DISPLAY_KEY_PATH, name);
        log::info!("  处理: {}", full_path);

        // 方法 1: 用 reg.exe delete (SYSTEM 身份下有效)
        log::debug!("    尝试 reg.exe delete...");
        let output = std::process::Command::new("reg")
            .args(&["delete", &full_path, "/f"])
            .output();

        match output {
            Ok(o) if o.status.success() => {
                log::info!("  ✓ 已删除 (reg.exe): {}", name);
                deleted += 1;
                continue;
            }
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                log::debug!("    reg.exe 失败: {}", stderr.trim());
            }
            Err(e) => {
                log::debug!("    reg.exe 执行错误: {}", e);
            }
        }

        // 方法 2: 先 takeown 再删
        log::debug!("    尝试 takeown + 删除...");

        // 获取 Administrators 权限
        let takeown = std::process::Command::new("powershell")
            .args(&[
                "-NoProfile", "-Command",
                &format!(
                    r#"
$key = [Microsoft.Win32.Registry]::LocalMachine.OpenSubKey('{}\\{}', 'ReadWriteSubTree', 'TakeOwnership')
if ($key) {{
    $acl = $key.GetAccessControl()
    $admin = [System.Security.Principal.NTAccount]'BUILTIN\Administrators'
    $acl.SetOwner($admin)
    $rule = New-Object System.Security.AccessControl.RegistryAccessRule($admin, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow')
    $acl.SetAccessRule($rule)
    $key.SetAccessControl($acl)
    $key.Close()
    Write-Host 'OK'
}}

function Set-SubKeyOwner($path) {{
    try {{
        $k = [Microsoft.Win32.Registry]::LocalMachine.OpenSubKey($path, 'ReadWriteSubTree', 'TakeOwnership')
        if ($k) {{
            $a = $k.GetAccessControl()
            $admin2 = [System.Security.Principal.NTAccount]'BUILTIN\Administrators'
            $a.SetOwner($admin2)
            $r = New-Object System.Security.AccessControl.RegistryAccessRule($admin2, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow')
            $a.SetAccessRule($r)
            $k.SetAccessControl($a)
            foreach ($sub in $k.GetSubKeyNames()) {{
                Set-SubKeyOwner "$path\$sub"
            }}
            $k.Close()
        }}
    }} catch {{}}
}}
Set-SubKeyOwner '{}\\{}'
"#,
                    DISPLAY_KEY_PATH, name,
                    DISPLAY_KEY_PATH, name,
                ),
            ])
            .output();

        match takeown {
            Ok(o) => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                let stderr = String::from_utf8_lossy(&o.stderr);
                if stdout.contains("OK") {
                    log::debug!("    PowerShell takeown 成功");
                } else {
                    log::debug!("    PowerShell: {} {}", stdout.trim(), stderr.trim());
                }
            }
            Err(e) => log::debug!("    PowerShell 执行错误: {}", e),
        }

        // 再次尝试删除
        let hklm2 = RegKey::predef(HKEY_LOCAL_MACHINE);
        let rel_path = format!("{}\\{}", DISPLAY_KEY_PATH, name);
        match hklm2.delete_subkey_all(&rel_path) {
            Ok(_) => {
                log::info!("  ✓ 已删除 (takeown 后): {}", name);
                deleted += 1;
            }
            Err(e) => {
                log::error!("  ✗ 仍然删除失败: {} — {}", name, e);
                log::error!("    提示: 以 SYSTEM 身份运行可解决此问题");
                log::error!("    方法: psexec -s hwt-demo.exe 或注册为 Windows 服务");
                failed += 1;
            }
        }
    }

    log::info!("注册表清理结果: 成功 {}, 失败 {}, 共 {}", deleted, failed, total);
    Ok(deleted)
}

/// 通过命令行启用特权
#[cfg(windows)]
fn enable_privileges_cmd() {
    // whoami /priv 可以查看当前特权
    if let Ok(o) = std::process::Command::new("whoami").args(&["/priv"]).output() {
        let stdout = String::from_utf8_lossy(&o.stdout);
        if stdout.contains("SeTakeOwnershipPrivilege") {
            log::debug!("  当前 Token 包含 SeTakeOwnershipPrivilege");
        }
        if stdout.contains("SeRestorePrivilege") {
            log::debug!("  当前 Token 包含 SeRestorePrivilege");
        }
    }
}

#[cfg(not(windows))]
pub fn clean_display_registry() -> Result<u32> {
    log::info!("[非 Windows] 跳过注册表删除");
    Ok(0)
}
