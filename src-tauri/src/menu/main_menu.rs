//! 主窗口菜单栏：仅含「文件 → 退出」。
use tauri::menu::{Menu, MenuItem, Submenu};
use tauri::{AppHandle, Wry};

use crate::menu::MENU_EXIT;

pub fn build_main_menu(app_handle: &AppHandle) -> Result<Menu<Wry>, tauri::Error> {
    // “文件”菜单中的“退出”菜单项。
    let exit_item = MenuItem::with_id(app_handle, MENU_EXIT, "退出", true, None::<&str>)?;

    // 构建“文件”子菜单。
    let file_submenu = Submenu::with_items(app_handle, "文件", true, &[&exit_item])?;

    // 组装主菜单栏。
    Menu::with_items(app_handle, &[&file_submenu])
}
