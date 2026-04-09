//! 桌面端运行时：窗口、托盘、平台专属启动逻辑。
#[cfg(desktop)]
pub mod desktop;
pub mod main_window;
#[cfg(desktop)]
pub mod tray_icon_px;

#[cfg(desktop)]
pub mod config;

#[cfg(desktop)]
pub mod shared;

#[cfg(all(desktop, windows))]
pub mod windows;

#[cfg(all(desktop, target_os = "macos"))]
pub mod macos;
