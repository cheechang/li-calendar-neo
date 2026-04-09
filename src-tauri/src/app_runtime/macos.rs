//! macOS 桌面端：菜单栏托盘、日期图标与 LunarBar 对齐逻辑。

use tauri::AppHandle;

/// 主窗口显示时显示 Dock，关闭时隐藏。
pub fn set_activation_policy_for_main_window_visible(
    app: &AppHandle,
    visible: bool,
) -> tauri::Result<()> {
    app.set_activation_policy(if visible {
        tauri::ActivationPolicy::Regular
    } else {
        tauri::ActivationPolicy::Accessory
    })
}

pub mod tray;
pub mod tray_bar_icon;
pub mod tray_calendar_icon;
pub mod tray_lunarbar_calendar_icon;
