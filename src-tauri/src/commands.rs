//! 前端 `invoke` 可调用的 Tauri 命令：按平台拆分到 `common` 与 `windows` / `macos`。
pub mod common;

#[cfg(not(windows))]
pub mod macos;
#[cfg(windows)]
pub mod windows;

pub use common::{
    get_system_time_millis_since_epoch, greet, hide_calendar, open_main_window, popup_ready,
    set_calendar_pin, show_calendar, toggle_calendar, toggle_calendar_at_position,
};

#[cfg(not(windows))]
pub use macos::{
    apply_custom_clock_text, get_clock_text, get_macos_tray_bar_icon,
    get_macos_tray_date_icon_style, get_macos_tray_icon_px, get_macos_tray_title_template,
    get_supported_window_effects, restore_default_clock, set_desktop_widget_enabled,
    set_macos_tray_bar_icon, set_macos_tray_date_icon_style, set_macos_tray_icon_px,
    set_macos_tray_title_template, set_macos_vibrancy, set_taskbar_widget_enabled_command,
    test_clock_detection,
};
#[cfg(windows)]
pub use windows::{
    apply_custom_clock_text, get_clock_text, get_macos_tray_bar_icon,
    get_macos_tray_date_icon_style, get_macos_tray_icon_px, get_macos_tray_title_template,
    get_supported_window_effects, restore_default_clock, set_desktop_widget_enabled,
    set_macos_tray_bar_icon, set_macos_tray_date_icon_style, set_macos_tray_icon_px,
    set_macos_tray_title_template, set_macos_vibrancy, set_taskbar_widget_enabled_command,
    test_clock_detection,
};
