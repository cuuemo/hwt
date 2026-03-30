use std::io::Result;

#[cfg(target_os = "windows")]
use std::io::{Error, ErrorKind};

/// Collect hardware information and generate a machine code (SHA256 hex string, 64 chars).
///
/// On Windows, uses WMI to query CPU ID, baseboard serial, and disk serial.
/// On Linux (dev mode), returns a placeholder string.
pub fn get_machine_code() -> Result<String> {
    #[cfg(target_os = "windows")]
    {
        get_machine_code_windows()
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok("linux-dev-machine-code".to_string())
    }
}

#[cfg(target_os = "windows")]
fn get_machine_code_windows() -> Result<String> {
    use serde::Deserialize;
    use sha2::{Digest, Sha256};
    use wmi::{COMLibrary, WMIConnection};

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct Win32Processor {
        ProcessorId: String,
    }

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct Win32BaseBoard {
        SerialNumber: String,
    }

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct Win32DiskDrive {
        SerialNumber: String,
    }

    // Initialize COM
    let com = COMLibrary::new()
        .map_err(|e| Error::new(ErrorKind::Other, format!("COM init error: {e}")))?;
    let wmi = WMIConnection::new(com)
        .map_err(|e| Error::new(ErrorKind::Other, format!("WMI connect error: {e}")))?;

    // Query CPU ID
    let cpus: Vec<Win32Processor> = wmi
        .raw_query("SELECT ProcessorId FROM Win32_Processor")
        .map_err(|e| Error::new(ErrorKind::Other, format!("WMI CPU query error: {e}")))?;
    let cpu_id = cpus
        .first()
        .map(|c| c.ProcessorId.clone())
        .unwrap_or_default();

    // Query baseboard serial
    let boards: Vec<Win32BaseBoard> = wmi
        .raw_query("SELECT SerialNumber FROM Win32_BaseBoard")
        .map_err(|e| Error::new(ErrorKind::Other, format!("WMI board query error: {e}")))?;
    let board_sn = boards
        .first()
        .map(|b| b.SerialNumber.clone())
        .unwrap_or_default();

    // Query disk serial (first disk)
    let disks: Vec<Win32DiskDrive> = wmi
        .raw_query("SELECT SerialNumber FROM Win32_DiskDrive WHERE Index=0")
        .map_err(|e| Error::new(ErrorKind::Other, format!("WMI disk query error: {e}")))?;
    let disk_sn = disks
        .first()
        .map(|d| d.SerialNumber.clone())
        .unwrap_or_default();

    // Concatenate and hash
    let raw = format!("{}-{}-{}", cpu_id, board_sn, disk_sn);
    let hash = Sha256::digest(raw.as_bytes());
    let machine_code = hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    Ok(machine_code)
}
