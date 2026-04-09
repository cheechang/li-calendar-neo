//! 窗口类名与桌面前台检测。

use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromWindow, MONITOR_DEFAULTTONEAREST, MONITORINFO};
use windows::Win32::UI::WindowsAndMessaging::*;

/// 读取窗口类名（UTF-16 缓冲）。
///
/// * `hwnd` - 目标窗口句柄。
pub fn get_window_class_name(hwnd: HWND) -> String {
    unsafe {
        // Win32 类名缓冲（最多 256 个 wchar，含终止符余量）
        let mut class_name = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut class_name);
        if len > 0 {
            String::from_utf16_lossy(&class_name[..len as usize])
        } else {
            String::new()
        }
    }
}

/// 检测桌面是否处于前台（WIN+D 后桌面会在前台显示 WorkerW）。
/// 返回 true 表示桌面正在前台，需要重新显示桌面组件。
pub fn is_desktop_in_foreground() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return false;
        }
        let class_name = get_window_class_name(hwnd);
        if class_name == "WorkerW" || class_name == "Progman" {
            return true;
        }
        false
    }
}

/// 前台窗口是否近似铺满其所在显示器的物理区域（全屏游戏、全屏视频、无边框全屏等）。
///
/// 用于在命中任务栏时钟缓存矩形时进一步判断：全屏前台应用时不拦截鼠标，避免干扰游戏。
pub fn is_foreground_fullscreen() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return false;
        }
        let mut win_rect = RECT::default();
        if GetWindowRect(hwnd, &mut win_rect).is_err() {
            return false;
        }
        let hmon = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        if hmon.is_invalid() {
            return false;
        }
        let mut mi = MONITORINFO::default();
        mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if !GetMonitorInfoW(hmon, &mut mi).as_bool() {
            return false;
        }
        let m = mi.rcMonitor;
        const TOL: i32 = 8;
        win_rect.left <= m.left + TOL
            && win_rect.top <= m.top + TOL
            && win_rect.right >= m.right - TOL
            && win_rect.bottom >= m.bottom - TOL
    }
}
