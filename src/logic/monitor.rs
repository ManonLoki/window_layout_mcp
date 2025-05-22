use super::MonitorInfo;
use windows::Win32::Foundation::{LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFOEXW};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
use windows::core::BOOL;

/// 获取监视器信息
pub fn get_monitor_info() -> Vec<MonitorInfo> {
    let mut monitors = Vec::new();
    let state = &mut monitors as *mut Vec<MonitorInfo>;

    unsafe {
        let _ = EnumDisplayMonitors(None, None, Some(enum_monitor), LPARAM(state as isize));
    }

    monitors
}

unsafe extern "system" fn enum_monitor(
    hmonitor: windows::Win32::Graphics::Gdi::HMONITOR,
    _: windows::Win32::Graphics::Gdi::HDC,
    _: *mut RECT,
    lparam: LPARAM,
) -> BOOL {
    let monitors = unsafe { &mut *(lparam.0 as *mut Vec<MonitorInfo>) };

    let mut monitor_info = MONITORINFOEXW::default();
    monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

    unsafe {
        if GetMonitorInfoW(hmonitor, &mut monitor_info as *mut _ as *mut _).as_bool() {
            let name = String::from_utf16_lossy(
                &monitor_info.szDevice[..monitor_info
                    .szDevice
                    .iter()
                    .position(|&x| x == 0)
                    .unwrap_or(0)],
            );

            // 获取DPI信息
            let mut dpi_x = 0u32;
            let mut dpi_y = 0u32;
            let _ = GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y);
            let dpi_scale = dpi_x as f64 / 96.0; // 96 DPI是Windows的基准DPI

            monitors.push(MonitorInfo {
                name,
                width: monitor_info.monitorInfo.rcMonitor.right
                    - monitor_info.monitorInfo.rcMonitor.left,
                height: monitor_info.monitorInfo.rcMonitor.bottom
                    - monitor_info.monitorInfo.rcMonitor.top,
                x: monitor_info.monitorInfo.rcMonitor.left,
                y: monitor_info.monitorInfo.rcMonitor.top,
                is_primary: (monitor_info.monitorInfo.dwFlags & 1) != 0,
                work_width: monitor_info.monitorInfo.rcWork.right
                    - monitor_info.monitorInfo.rcWork.left,
                work_height: monitor_info.monitorInfo.rcWork.bottom
                    - monitor_info.monitorInfo.rcWork.top,
                work_x: monitor_info.monitorInfo.rcWork.left,
                work_y: monitor_info.monitorInfo.rcWork.top,
                dpi_scale,
            });
        }
    }

    BOOL(1)
}
