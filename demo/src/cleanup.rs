use std::io::Result;

/// 不清理的设备类 GUID 黑名单 (这些清了会出问题)
/// 只排除软件设备和当前活跃的系统关键设备类
const SKIP_CLASS_GUIDS: &[(&str, &str)] = &[
    // 软件设备 - 系统内部使用, 清了可能导致蓝屏
    ("{62f9c741-b25a-46ce-b54c-9bccce08b6f2}", "SoftwareComponent"),
];

/// 清理所有幽灵设备 (和 DeviceCleanup 一样的逻辑)
/// 只要是 phantom (problem 45 或 ret 13), 就移除
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

    log::info!("调用 SetupDiGetClassDevsW(DIGCF_ALLCLASSES) ...");
    let h_dev_info = unsafe {
        SetupDiGetClassDevsW(None, None, HWND::default(), DIGCF_ALLCLASSES)
    }
    .map_err(|e| Error::new(ErrorKind::Other, format!("SetupDiGetClassDevsW 失败: {}", e)))?;

    let _guard = DevInfoGuard(h_dev_info);
    log::info!("设备枚举句柄获取成功");

    let mut index = 0u32;
    let mut scanned = 0u32;
    let mut phantom_total = 0u32;
    let mut removed = 0u32;
    let mut failed = 0u32;
    let mut skipped = 0u32;

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

        // 检查是否为幽灵设备
        let mut status = CM_DEVNODE_STATUS_FLAGS(0);
        let mut problem = CM_PROB(0);
        let ret = unsafe {
            CM_Get_DevNode_Status(&mut status, &mut problem, dev_info_data.DevInst, 0)
        };

        // CR_NO_SUCH_DEVINST = 13, CM_PROB_PHANTOM = 45
        let is_phantom = (ret.0 == 13) || (problem.0 == 45);
        if !is_phantom {
            continue;
        }

        phantom_total += 1;

        // 获取设备 ID
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

        // 获取设备类 GUID 字符串
        let guid = dev_info_data.ClassGuid;
        let guid_str = format!(
            "{{{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}}}",
            guid.data1, guid.data2, guid.data3,
            guid.data4[0], guid.data4[1], guid.data4[2], guid.data4[3],
            guid.data4[4], guid.data4[5], guid.data4[6], guid.data4[7],
        );

        // 获取设备描述
        let desc = get_device_description(dev_info_data.DevInst);

        // 获取设备类名
        let class_name = get_device_class_name(dev_info_data.DevInst);

        // 检查黑名单
        let in_blacklist = SKIP_CLASS_GUIDS
            .iter()
            .any(|(g, _)| g.eq_ignore_ascii_case(&guid_str));

        if in_blacklist {
            log::debug!(
                "  [黑名单跳过] #{}: 类={} ID={}",
                phantom_total, class_name, device_id
            );
            skipped += 1;
            continue;
        }

        log::info!(
            "  [清理] 幽灵设备 #{}: 类={} 描述=\"{}\"",
            phantom_total, class_name, desc
        );
        log::info!("          ID: {}", device_id);
        log::debug!("          GUID: {}, ret={}, problem={}", guid_str, ret.0, problem.0);

        // 执行移除
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
            let err = unsafe { GetLastError() };
            log::error!("          ✗ SetupDiSetClassInstallParamsW 失败: {:?}", err);
            failed += 1;
            continue;
        }

        let remove_ok = unsafe {
            SetupDiCallClassInstaller(DIF_REMOVE, h_dev_info, Some(&dev_info_data))
        };

        if remove_ok.is_err() {
            let err = unsafe { GetLastError() };
            log::error!("          ✗ DIF_REMOVE 失败: {:?}", err);
            failed += 1;
            continue;
        }

        // 验证移除
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
            log::info!("          ✓ 移除成功");
        } else {
            log::warn!("          △ 移除后仍存在");
        }
    }

    log::info!("==========================================================");
    log::info!("扫描统计:");
    log::info!("  总设备数:       {}", scanned);
    log::info!("  幽灵设备数:     {}", phantom_total);
    log::info!("  黑名单跳过:     {}", skipped);
    log::info!("  尝试移除:       {}", phantom_total - skipped);
    log::info!("  成功移除:       {}", removed);
    log::info!("  移除失败:       {}", failed);
    log::info!("==========================================================");

    Ok((scanned, removed))
}

/// 获取设备描述 (FriendlyName 或 DeviceDesc)
#[cfg(windows)]
fn get_device_description(dev_inst: u32) -> String {
    use windows::Win32::Devices::DeviceAndDriverInstallation::*;

    let mut buf = [0u16; 512];
    let mut buf_type = 0u32;
    let mut buf_size = 0u32;

    // 先尝试 FriendlyName (0x0D)
    let _ = unsafe {
        CM_Get_DevNode_Registry_PropertyW(
            dev_inst, 0x0D,
            Some(&mut buf_type),
            Some(buf.as_mut_ptr() as *mut _),
            &mut buf_size, 0,
        )
    };

    if buf_size > 2 {
        let len = (buf_size as usize / 2).saturating_sub(1);
        return String::from_utf16_lossy(&buf[..len]);
    }

    // 再尝试 DeviceDesc (0x01)
    buf_size = 0;
    let _ = unsafe {
        CM_Get_DevNode_Registry_PropertyW(
            dev_inst, 0x01,
            Some(&mut buf_type),
            Some(buf.as_mut_ptr() as *mut _),
            &mut buf_size, 0,
        )
    };

    if buf_size > 2 {
        let len = (buf_size as usize / 2).saturating_sub(1);
        return String::from_utf16_lossy(&buf[..len]);
    }

    "(未知)".to_string()
}

/// 获取设备类名 (Class property 0x08)
#[cfg(windows)]
fn get_device_class_name(dev_inst: u32) -> String {
    use windows::Win32::Devices::DeviceAndDriverInstallation::*;

    let mut buf = [0u16; 256];
    let mut buf_type = 0u32;
    let mut buf_size = 0u32;

    // CM_DRP_CLASS = 0x08
    let _ = unsafe {
        CM_Get_DevNode_Registry_PropertyW(
            dev_inst, 0x08,
            Some(&mut buf_type),
            Some(buf.as_mut_ptr() as *mut _),
            &mut buf_size, 0,
        )
    };

    if buf_size > 2 {
        let len = (buf_size as usize / 2).saturating_sub(1);
        return String::from_utf16_lossy(&buf[..len]);
    }

    "(未知类)".to_string()
}

#[cfg(not(windows))]
pub fn cleanup_phantom_devices() -> Result<(u32, u32)> {
    log::info!("[非 Windows] 跳过幽灵设备清理");
    Ok((0, 0))
}

#[cfg(not(windows))]
fn get_device_description(_dev_inst: u32) -> String { String::new() }

#[cfg(not(windows))]
fn get_device_class_name(_dev_inst: u32) -> String { String::new() }
