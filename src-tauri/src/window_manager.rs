//! 日历相关窗口的创建、定位与生命周期：按平台选择 `windows` 或 `macos` 实现。
#[cfg(target_os = "macos")]
#[path = "window_manager/macos/popup.rs"]
mod macos_popup;
#[path = "window_manager/shared.rs"]
pub(crate) mod shared;
#[cfg(windows)]
#[path = "window_manager/windows.rs"]
mod windows;

#[cfg(windows)]
pub use windows::main_window::show_or_create_main_window;
#[cfg(windows)]
pub use windows::CalendarWindowManager;

#[cfg(target_os = "macos")]
pub use macos_popup::{show_or_create_main_window, CalendarWindowManager};

#[cfg(not(any(windows, target_os = "macos")))]
pub use shared::main_window::show_or_create_main_window;

#[cfg(not(any(windows, target_os = "macos")))]
pub struct CalendarWindowManager {
    app_handle: tauri::AppHandle,
}

#[cfg(not(any(windows, target_os = "macos")))]
impl shared::popup_manager::PopupManager for CalendarWindowManager {
    fn hide_popup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn is_popup_visible(&self) -> bool {
        false
    }

    fn show_popup_near_clock(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn show_popup_at_position(
        &mut self,
        _click_x: i32,
        _click_y: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn on_popup_ready(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn set_popup_pin(&mut self, _pin: bool) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[cfg(not(any(windows, target_os = "macos")))]
impl CalendarWindowManager {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            app_handle: app_handle.clone(),
        })
    }

    pub fn preload_popup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
