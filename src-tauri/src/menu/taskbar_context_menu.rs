//! 任务栏时钟区域右键菜单（Windows）。
#[cfg(windows)]
use tauri::menu::{Menu, MenuItem};
#[cfg(windows)]
use tauri::{AppHandle, Wry};

#[cfg(windows)]
use crate::menu::{MENU_EXIT, MENU_SETTINGS};

/// 构建 Windows 任务栏时钟区域右键菜单。
///
/// * `app_handle` - 应用程序句柄
#[cfg(windows)]
pub fn build_context_menu(app_handle: &AppHandle) -> Result<Menu<Wry>, tauri::Error> {
    // 任务栏菜单中的“设置”菜单项。
    let settings_item = MenuItem::with_id(app_handle, MENU_SETTINGS, "设置", true, None::<&str>)?;
    // 任务栏菜单中的“退出”菜单项。
    let exit_item = MenuItem::with_id(app_handle, MENU_EXIT, "退出", true, None::<&str>)?;

    // 组装任务栏右键菜单。
    Menu::with_items(app_handle, &[&settings_item, &exit_item])
}
