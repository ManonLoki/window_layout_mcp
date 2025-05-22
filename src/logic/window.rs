use std::process;

use windows::Win32::Foundation::{HWND, LPARAM, MAX_PATH, RECT};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClassNameW, GetWindowRect, GetWindowTextW, GetWindowThreadProcessId, HWND_TOP,
    IsWindow, IsWindowVisible, SW_MINIMIZE, SW_RESTORE, SWP_FRAMECHANGED, SWP_SHOWWINDOW,
    SetForegroundWindow, SetWindowPos, ShowWindow,
};
use windows::core::BOOL;

use super::WindowInfo;

// 定义错误类型
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("无效的窗口句柄")]
    InvalidWindowHandle,
    #[error("设置窗口位置失败: {0}")]
    SetWindowPositionFailed(String),
}

// 获取进程可执行文件名
unsafe fn get_process_name(process_id: u32) -> Option<String> {
    let process_handle = unsafe {
        OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        )
    }
    .ok()?;

    let mut path_buf = [0u16; MAX_PATH as usize];
    let path_len = unsafe { GetModuleFileNameExW(Some(process_handle), None, &mut path_buf) };

    if path_len > 0 {
        let path = String::from_utf16_lossy(&path_buf[..path_len as usize]);
        // 获取文件名部分
        path.split('\\').last().map(|s| s.to_lowercase())
    } else {
        None
    }
}

// 扫描系统中所有窗口
pub fn scan_windows() -> Vec<WindowInfo> {
    let windows = Vec::new();
    let current_pid = process::id();

    // 创建一个元组来存储状态
    let mut state = (windows, current_pid);

    unsafe {
        // 使用 Windows API 枚举所有窗口
        let _ = EnumWindows(Some(enum_window), LPARAM(&mut state as *mut _ as isize));
    }

    // 返回收集到的窗口信息
    state.0
}

// 窗口枚举回调函数
unsafe extern "system" fn enum_window(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let state = unsafe { &mut *(lparam.0 as *mut (Vec<WindowInfo>, u32)) };
    let windows = &mut state.0;
    let current_pid = state.1;

    // 只处理可见且非最小化的窗口
    if unsafe { IsWindowVisible(hwnd).as_bool() } {
        let mut rect = RECT::default();
        let _ = unsafe { GetWindowRect(hwnd, &mut rect) };

        // 获取窗口标题
        let mut title = [0u16; 512];
        unsafe { GetWindowTextW(hwnd, &mut title) };
        let title =
            String::from_utf16_lossy(&title[..title.iter().position(|&x| x == 0).unwrap_or(0)]);

        // 获取窗口进程ID
        let mut process_id: u32 = 0;
        unsafe { GetWindowThreadProcessId(hwnd, Some(&mut process_id)) };

        // 获取窗口类名
        let mut class_name = [0u16; 512];
        unsafe { GetClassNameW(hwnd, &mut class_name) };
        let class_name = String::from_utf16_lossy(
            &class_name[..class_name.iter().position(|&x| x == 0).unwrap_or(0)],
        );

        let process_name = unsafe { get_process_name(process_id) };

        if !title.trim().is_empty()
            && process_id > 4
            && process_id != current_pid
            && !class_name.contains("Shell_TrayWnd")
            && !class_name.contains("NotifyIconOverflowWindow")
            && !title.contains("Program Manager")
        {
            windows.push(WindowInfo {
                handle: hwnd.0 as isize,
                title,
                width: rect.right - rect.left,
                height: rect.bottom - rect.top,
                x: rect.left,
                y: rect.top,
                process_name,
            });
        }
    }

    BOOL(1)
}

/// 安全地设置窗口位置
pub fn set_window_position_safe(
    handle: isize,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<(), Error> {
    unsafe {
        // 首先检查窗口是否有效
        if !IsWindow(Some(HWND(handle as _))).as_bool() {
            tracing::warn!("窗口句柄无效: {}", handle);
            return Err(Error::InvalidWindowHandle);
        }

        tracing::info!(
            "正在设置窗口位置: handle={}, x={}, y={}, width={}, height={}",
            handle,
            x,
            y,
            width,
            height
        );

        // 尝试设置窗口位置
        match SetWindowPos(
            HWND(handle as _),
            Some(HWND_TOP),
            x,
            y,
            width,
            height,
            SWP_SHOWWINDOW | SWP_FRAMECHANGED,
        ) {
            Ok(_) => {
                tracing::info!("设置窗口位置成功");
                Ok(())
            }
            Err(e) => {
                tracing::error!("设置窗口位置失败:{}", e);
                Err(Error::SetWindowPositionFailed(e.to_string()))
            }
        }
    }
}

/// 安全地设置窗口位置
pub fn set_window_position(window_info: WindowInfo) -> Result<(), Error> {
    // 使用 set_window_position_safe 函数来设置窗口位置
    set_window_position_safe(
        window_info.handle,
        window_info.x,
        window_info.y,
        window_info.width,
        window_info.height,
    )
}

///   将窗口置前
pub fn set_window_to_top(handle: isize) -> Result<(), Error> {
    unsafe {
        // 首先检查窗口是否有效
        if !IsWindow(Some(HWND(handle as _))).as_bool() {
            tracing::warn!("窗口句柄无效: {}", handle);
            return Err(Error::InvalidWindowHandle);
        }

        // 获取窗口句柄
        let hwnd = HWND(handle as *mut _);
        let _ = ShowWindow(hwnd, SW_MINIMIZE);

        let _ = ShowWindow(hwnd, SW_RESTORE);

        let _ = SetForegroundWindow(hwnd);

        Ok(())
    }
}
