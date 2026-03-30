use std::io::Result;

/// Clean up ALL phantom (ghost) devices using the Windows Setup API.
/// Same logic as DeviceCleanup: any device with problem code 45 or
/// CM_Get_DevNode_Status returning CR_NO_SUCH_DEVINST (13) is removed.
///
/// Returns (scanned_count, removed_count).
#[cfg(windows)]
pub fn cleanup_phantom_devices() -> Result<(u32, u32)> {
    use std::io::{Error, ErrorKind};
    use windows::Win32::Devices::DeviceAndDriverInstallation::*;
    use windows::Win32::Foundation::*;

    struct DevInfoGuard(HDEVINFO);
    impl Drop for DevInfoGuard {
        fn drop(&mut self) {
            unsafe {
                let _ = SetupDiDestroyDeviceInfoList(self.0);
            }
        }
    }

    #[repr(C)]
    struct SP_REMOVEDEVICE_PARAMS {
        class_install_header: SP_CLASSINSTALL_HEADER,
        scope: u32,
        hw_profile: u32,
    }

    // Step 1: Enumerate all devices
    let h_dev_info = unsafe {
        SetupDiGetClassDevsW(None, None, HWND::default(), DIGCF_ALLCLASSES)
    }
    .map_err(|e| Error::new(ErrorKind::Other, format!("SetupDiGetClassDevsW failed: {}", e)))?;

    let _guard = DevInfoGuard(h_dev_info);

    let mut index = 0u32;
    let mut scanned = 0u32;
    let mut phantom_total = 0u32;
    let mut removed = 0u32;
    let mut failed = 0u32;

    loop {
        let mut dev_info_data = SP_DEVINFO_DATA {
            cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
            ..Default::default()
        };

        if unsafe { SetupDiEnumDeviceInfo(h_dev_info, index, &mut dev_info_data) }.is_err() {
            break;
        }

        index += 1;
        scanned += 1;

        // Step 2: Check if phantom
        let mut status = CM_DEVNODE_STATUS_FLAGS(0);
        let mut problem = CM_PROB(0);
        let ret = unsafe {
            CM_Get_DevNode_Status(&mut status, &mut problem, dev_info_data.DevInst, 0)
        };

        let is_phantom = (ret.0 == 13) || (problem.0 == 45);
        if !is_phantom {
            continue;
        }

        phantom_total += 1;

        // Step 3: Get device ID for logging
        let mut device_id_buf = [0u16; 260];
        let _ = unsafe {
            CM_Get_Device_IDW(dev_info_data.DevInst, &mut device_id_buf, 0)
        };
        let device_id = String::from_utf16_lossy(
            &device_id_buf[..device_id_buf
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(device_id_buf.len())],
        );

        log::debug!("Phantom device #{}: {}", phantom_total, device_id);

        // Step 4: Remove via DIF_REMOVE with SP_REMOVEDEVICE_PARAMS (16 bytes)
        let params = SP_REMOVEDEVICE_PARAMS {
            class_install_header: SP_CLASSINSTALL_HEADER {
                cbSize: std::mem::size_of::<SP_CLASSINSTALL_HEADER>() as u32,
                InstallFunction: DIF_REMOVE,
            },
            scope: 1, // DI_REMOVEDEVICE_GLOBAL
            hw_profile: 0,
        };

        let set_ok = unsafe {
            SetupDiSetClassInstallParamsW(
                h_dev_info,
                Some(&dev_info_data),
                Some(&params.class_install_header),
                std::mem::size_of::<SP_REMOVEDEVICE_PARAMS>() as u32,
            )
        };

        if set_ok.is_err() {
            log::error!("SetupDiSetClassInstallParamsW failed for: {}", device_id);
            failed += 1;
            continue;
        }

        let remove_ok = unsafe {
            SetupDiCallClassInstaller(DIF_REMOVE, h_dev_info, Some(&dev_info_data))
        };

        if remove_ok.is_err() {
            log::error!("DIF_REMOVE failed for: {}", device_id);
            failed += 1;
            continue;
        }

        // Step 5: Verify removal
        let mut verify_inst: u32 = 0;
        let verify = unsafe {
            CM_Locate_DevNodeW(
                &mut verify_inst,
                windows::core::PCWSTR(device_id_buf.as_ptr()),
                CM_LOCATE_DEVNODE_FLAGS(0),
            )
        };

        if verify.0 != 0 {
            removed += 1;
            log::debug!("Removed: {}", device_id);
        } else {
            log::warn!("Still exists after removal: {}", device_id);
        }
    }

    log::info!(
        "Phantom device cleanup: scanned={}, phantom={}, removed={}, failed={}",
        scanned, phantom_total, removed, failed
    );
    Ok((scanned, removed))
}

/// Stub implementation for non-Windows platforms.
#[cfg(not(windows))]
pub fn cleanup_phantom_devices() -> Result<(u32, u32)> {
    log::info!("Phantom device cleanup is not supported on this platform");
    Ok((0, 0))
}
