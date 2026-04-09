//! 桌面日历窗口：挂接到 `Progman`、被动显示与刷新。
use super::{get_window_hwnd, CalendarWindowManager};
use tauri::WebviewWindow;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetForegroundWindow, SetForegroundWindow, SetWindowLongPtrW, SetWindowPos,
    ShowWindow, GWLP_HWNDPARENT, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOSIZE,
    SWP_SHOWWINDOW, SW_SHOWNOACTIVATE,
};

/// 将桌面组件窗口挂载到 Progman 下并置底，保持当前位置和尺寸不变。
fn pin_window_to_desktop(desktop_widget_window: &WebviewWindow) {
    let window_hwnd = match get_window_hwnd(desktop_widget_window) {
        Some(hwnd) => hwnd,
        None => return,
    };
    unsafe {
        if let Ok(progman_hwnd) = FindWindowW(windows::core::w!("Progman"), None) {
            if !progman_hwnd.0.is_null() {
                SetWindowLongPtrW(window_hwnd, GWLP_HWNDPARENT, progman_hwnd.0 as isize);
                let _ = SetWindowPos(
                    window_hwnd,
                    None,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOOWNERZORDER | SWP_SHOWWINDOW,
                );
                println!("✅ 桌面日历窗口已挂接到 Progman（GWLP_HWNDPARENT）");
            }
        }
    }
}

/// 以不抢焦点的方式在指定物理坐标处显示桌面组件并固定到底层。
/// `position` 为 `Some((x, y))` 时移动到目标位置，为 `None` 时保持当前位置。
fn show_desktop_window_at(desktop_widget_window: &WebviewWindow, position: Option<(i32, i32)>) {
    let window_hwnd = match get_window_hwnd(desktop_widget_window) {
        Some(hwnd) => hwnd,
        None => return,
    };
    unsafe {
        let _ = ShowWindow(window_hwnd, SW_SHOWNOACTIVATE);
        let (x, y, flags) = match position {
            Some((x, y)) => {
                (x, y, SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOOWNERZORDER | SWP_SHOWWINDOW)
            }
            None => (
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOOWNERZORDER | SWP_SHOWWINDOW,
            ),
        };
        let _ = SetWindowPos(window_hwnd, None, x, y, 0, 0, flags);
    }
}

impl CalendarWindowManager {
    fn reapply_saved_desktop_vibrancy(&self, window: &WebviewWindow) {
        if self.desktop_vibrancy_enabled {
            let _ = Self::apply_desktop_window_vibrancy(
                window,
                self.desktop_vibrancy_effect.as_deref(),
            );
        } else {
            Self::clear_window_vibrancy(window);
        }
    }

    /// 初始化桌面组件的窗口层级与显示状态。
    /// `initial_position` 为物理像素坐标，在 show() 前先定好位，避免位置跳变。
    pub fn refresh_desktop_widget_chrome(
        window: &WebviewWindow,
        initial_position: Option<(i32, i32)>,
    ) {
        let hwnd = get_window_hwnd(window);

        // 先把窗口挂到 Progman 下（此时还隐藏）
        pin_window_to_desktop(window);

        // 若有持久化位置，在 show() 前用 Win32 直接移动到目标物理坐标，
        // 这样 Tauri show() 触发时窗口已在正确位置，彻底消除闪烁。
        if let (Some(hwnd), Some((x, y))) = (hwnd, initial_position) {
            unsafe {
                let _ = SetWindowPos(
                    hwnd,
                    None,
                    x,
                    y,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOACTIVATE | SWP_NOOWNERZORDER | SWP_SHOWWINDOW,
                );
            }
        }

        // 用 Tauri show() 激活 WebView2 合成管线，立即用 SW_SHOWNOACTIVATE 压制激活，
        // 再归还焦点给之前的前台窗口，避免任务栏弹窗因失焦而自动隐藏。
        let prev_foreground = unsafe { GetForegroundWindow() };
        let _ = window.show();
        unsafe {
            if let Some(hwnd) = get_window_hwnd(window) {
                let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
            }
            if !prev_foreground.0.is_null() {
                let _ = SetForegroundWindow(prev_foreground);
            }
        }

        // show() 后再做一次置底，防止 Tauri 内部重新调整层级
        show_desktop_window_at(window, None);
        pin_window_to_desktop(window);
    }

    /// 构建桌面组件 WebviewWindow，**不需要持有 `window_manager` 锁**，可在锁外并发调用。
    /// 构建完成后调用 `attach_desktop_window` 将其存入 manager。
    pub fn build_desktop_window(
        app_handle: &tauri::AppHandle,
        initial_position: Option<(i32, i32)>,
    ) -> Result<tauri::WebviewWindow, Box<dyn std::error::Error>> {
        let window = tauri::WebviewWindowBuilder::new(
            app_handle,
            "desktop_calendar",
            tauri::WebviewUrl::App("index.html?window=desktop".into()),
        )
        .title("桌面日历")
        .inner_size(360.0, 520.0)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .always_on_top(false)
        .visible(false)
        .focused(false)
        .skip_taskbar(true)
        .build()?;
        Self::refresh_desktop_widget_chrome(&window, initial_position);
        Ok(window)
    }

    /// 将已构建好的桌面组件窗口存入 manager 并应用毛玻璃效果。
    pub fn attach_desktop_window(&mut self, window: tauri::WebviewWindow) {
        self.reapply_saved_desktop_vibrancy(&window);
        self.desktop_widget_window = Some(window);
    }

    /// 确保桌面组件窗口存在，不存在时创建并完成桌面层绑定（持锁版，供单线程路径使用）。
    pub fn ensure_desktop_window(
        &mut self,
        initial_position: Option<(i32, i32)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.desktop_widget_window.is_some() {
            return Ok(());
        }
        let window = Self::build_desktop_window(&self.app_handle.clone(), initial_position)?;
        self.attach_desktop_window(window);
        Ok(())
    }

    /// 关闭并释放桌面组件窗口句柄。
    pub fn close_desktop_window(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(desktop_widget_window) = self.desktop_widget_window.take() {
            desktop_widget_window.close()?;
        }
        Ok(())
    }

    /// 重新显示并置顶桌面组件窗口（用于 WIN+D 或显示桌面后的恢复）。
    /// 调用 `show()` 后立即用 SW_SHOWNOACTIVATE 压制激活，防止失焦导致窗口自动隐藏。
    pub fn refresh_desktop_window_visibility(&self) {
        if let Some(ref window) = self.desktop_widget_window {
            let prev_foreground = unsafe { GetForegroundWindow() };
            let _ = window.show();
            unsafe {
                if let Some(hwnd) = get_window_hwnd(window) {
                    let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
                    pin_window_to_desktop(window);
                    let _ = SetWindowPos(
                        hwnd,
                        None,
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE
                            | SWP_NOSIZE
                            | SWP_NOACTIVATE
                            | SWP_NOOWNERZORDER
                            | SWP_SHOWWINDOW,
                    );
                }
                if !prev_foreground.0.is_null() {
                    let _ = SetForegroundWindow(prev_foreground);
                }
            }
        }
    }
}
