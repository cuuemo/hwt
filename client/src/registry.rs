use std::io::Result;

#[allow(dead_code)]
const DISPLAY_KEY_PATH: &str = r"SYSTEM\CurrentControlSet\Enum\DISPLAY";

/// Delete all subkeys under HKLM\SYSTEM\CurrentControlSet\Enum\DISPLAY.
/// Returns the number of subkeys deleted.
///
/// Requires SYSTEM privileges (satisfied when running as a Windows Service).
#[cfg(windows)]
pub fn clean_display_registry() -> Result<u32> {
    use std::io::{Error, ErrorKind};
    use winreg::enums::*;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // Open the DISPLAY key; if it doesn't exist, nothing to clean
    let display_key = match hklm.open_subkey_with_flags(DISPLAY_KEY_PATH, KEY_READ | KEY_WRITE) {
        Ok(key) => key,
        Err(e) => {
            log::warn!(
                "Cannot open registry key '{}': {} (may not exist)",
                DISPLAY_KEY_PATH,
                e
            );
            return Ok(0);
        }
    };

    // Enumerate all first-level subkeys (monitor model names)
    let subkey_names: Vec<String> = display_key
        .enum_keys()
        .filter_map(|r| r.ok())
        .collect();

    let total = subkey_names.len() as u32;
    let mut deleted = 0u32;

    for name in &subkey_names {
        let full_path = format!("{}\\{}", DISPLAY_KEY_PATH, name);
        match hklm.delete_subkey_all(&full_path) {
            Ok(_) => {
                log::info!("Deleted registry subkey: {}", full_path);
                deleted += 1;
            }
            Err(e) => {
                log::error!("Failed to delete registry subkey '{}': {}", full_path, e);
                // Continue deleting other subkeys
            }
        }
    }

    log::info!(
        "Display registry cleanup: deleted {}/{} subkeys",
        deleted,
        total
    );
    Ok(deleted)
}

/// Stub implementation for non-Windows platforms.
#[cfg(not(windows))]
pub fn clean_display_registry() -> Result<u32> {
    log::info!("Display registry cleanup is not supported on this platform");
    Ok(0)
}
