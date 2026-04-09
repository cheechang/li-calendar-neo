//! macOS 主菜单栏：顶层仅含 [`Submenu`]（Tauri / muda 要求），并遵循常见 HIG 顺序。
//! 使用 [`WINDOW_SUBMENU_ID`] / [`HELP_SUBMENU_ID`]，以便 Tauri 在 `init_for_nsapp` 中绑定
//! 标准「窗口」「帮助」菜单角色。
use tauri::menu::{
    AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu, HELP_SUBMENU_ID, WINDOW_SUBMENU_ID,
};
use tauri::{AppHandle, Wry};

use crate::menu::{MENU_EXIT, MENU_SETTINGS};

/// 构建主窗口菜单栏（macOS 全局菜单，由 [`tauri::AppHandle::set_menu`] 应用）。
pub fn build_main_menu(app_handle: &AppHandle) -> Result<Menu<Wry>, tauri::Error> {
    // —— 应用菜单（须为第一项）——
    let about = PredefinedMenuItem::about(
        app_handle,
        None,
        Some(AboutMetadata {
            name: Some("松鼠日历".to_string()),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
            ..Default::default()
        }),
    )?;
    let settings = MenuItem::with_id(app_handle, MENU_SETTINGS, "设置…", true, Some("cmd+,"))?;
    let sep_app_1 = PredefinedMenuItem::separator(app_handle)?;
    let services = PredefinedMenuItem::services(app_handle, None)?;
    let sep_app_2 = PredefinedMenuItem::separator(app_handle)?;
    let hide = PredefinedMenuItem::hide(app_handle, None)?;
    let hide_others = PredefinedMenuItem::hide_others(app_handle, None)?;
    let show_all = PredefinedMenuItem::show_all(app_handle, None)?;
    let sep_app_3 = PredefinedMenuItem::separator(app_handle)?;
    let quit = MenuItem::with_id(app_handle, MENU_EXIT, "退出", true, Some("cmd+q"))?;

    let app_menu = Submenu::with_items(
        app_handle,
        "松鼠日历",
        true,
        &[
            &about,
            &settings,
            &sep_app_1,
            &services,
            &sep_app_2,
            &hide,
            &hide_others,
            &show_all,
            &sep_app_3,
            &quit,
        ],
    )?;

    // —— 文件 ——
    let close_window = PredefinedMenuItem::close_window(app_handle, None)?;
    let file_menu = Submenu::with_items(app_handle, "文件", true, &[&close_window])?;

    // —— 编辑（WebView 内文本框依赖系统编辑命令）——
    let undo = PredefinedMenuItem::undo(app_handle, None)?;
    let redo = PredefinedMenuItem::redo(app_handle, None)?;
    let sep_edit_1 = PredefinedMenuItem::separator(app_handle)?;
    let cut = PredefinedMenuItem::cut(app_handle, None)?;
    let copy = PredefinedMenuItem::copy(app_handle, None)?;
    let paste = PredefinedMenuItem::paste(app_handle, None)?;
    let sep_edit_2 = PredefinedMenuItem::separator(app_handle)?;
    let select_all = PredefinedMenuItem::select_all(app_handle, None)?;
    let edit_menu = Submenu::with_items(
        app_handle,
        "编辑",
        true,
        &[&undo, &redo, &sep_edit_1, &cut, &copy, &paste, &sep_edit_2, &select_all],
    )?;

    // —— 显示 ——
    let fullscreen = PredefinedMenuItem::fullscreen(app_handle, None)?;
    let view_menu = Submenu::with_items(app_handle, "显示", true, &[&fullscreen])?;

    // —— 窗口（固定 id，供 Tauri 设为 NSWindow 菜单）——
    let minimize = PredefinedMenuItem::minimize(app_handle, None)?;
    let zoom = PredefinedMenuItem::maximize(app_handle, None)?;
    let window_menu = Submenu::with_id_and_items(
        app_handle,
        WINDOW_SUBMENU_ID,
        "窗口",
        true,
        &[&minimize, &zoom],
    )?;

    // —— 帮助（固定 id，供 Tauri 设为 NSHelp 菜单）——
    let help_menu = Submenu::with_id_and_items(app_handle, HELP_SUBMENU_ID, "帮助", true, &[])?;

    Menu::with_items(
        app_handle,
        &[&app_menu, &file_menu, &edit_menu, &view_menu, &window_menu, &help_menu],
    )
}
