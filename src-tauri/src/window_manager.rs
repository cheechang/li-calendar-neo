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
