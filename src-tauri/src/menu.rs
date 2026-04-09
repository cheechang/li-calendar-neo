//! 应用菜单构建与菜单项 ID 常量（与前端/托盘事件约定一致，ID 字符串需保持稳定）。
use tauri::{AppHandle, Manager, State};

use crate::window_manager::shared::popup_manager::PopupManager;
use crate::AppState;

#[cfg(target_os = "macos")]
#[path = "menu/macos_main_menu.rs"]
mod macos_main_menu;
#[cfg(not(target_os = "macos"))]
#[path = "menu/main_menu.rs"]
mod main_menu;
#[path = "menu/taskbar_context_menu.rs"]
mod taskbar_context_menu;
#[path = "menu/tray_menu.rs"]
mod tray_menu;

// 菜单事件：显示或切换日历弹窗。
pub const MENU_SHOW: &str = "show";
// 菜单事件：打开主窗口。
pub const MENU_OPEN_MAIN: &str = "open_main";
// 菜单事件：打开设置窗口。
pub const MENU_SETTINGS: &str = "settings";
// 菜单事件：从菜单触发显示/隐藏日历。
pub const MENU_TOGGLE_CALENDAR: &str = "toggle_calendar";
// 菜单事件：退出应用。
pub const MENU_EXIT: &str = "exit";

/// 构建主窗口顶部菜单栏（文件/视图），与托盘和任务栏菜单分离。
///
/// * `app_handle` - 应用程序句柄
pub fn build_main_menu(
    app_handle: &AppHandle,
) -> Result<tauri::menu::Menu<tauri::Wry>, tauri::Error> {
    #[cfg(target_os = "macos")]
    {
        macos_main_menu::build_main_menu(app_handle)
    }
    #[cfg(not(target_os = "macos"))]
    {
        main_menu::build_main_menu(app_handle)
    }
}

/// macOS：启动时为 Accessory 策略，系统不展示应用菜单栏；主窗口显示并切为 Regular 后须再次应用菜单。
#[cfg(target_os = "macos")]
pub fn sync_macos_app_menu(app_handle: &AppHandle) {
    if let Ok(menu) = build_main_menu(app_handle) {
        let _ = app_handle.set_menu(menu);
    }
}

/// 构建托盘菜单，仅用于系统托盘右键菜单。
///
/// * `app_handle` - 应用程序句柄
pub fn build_tray_menu(
    app_handle: &AppHandle,
) -> Result<tauri::menu::Menu<tauri::Wry>, tauri::Error> {
    tray_menu::build_tray_menu(app_handle)
}

/// 构建 Windows 任务栏时钟区域右键菜单。
///
/// * `app_handle` - 应用程序句柄
#[cfg(windows)]
pub fn build_context_menu(
    app_handle: &AppHandle,
) -> Result<tauri::menu::Menu<tauri::Wry>, tauri::Error> {
    taskbar_context_menu::build_context_menu(app_handle)
}

/// 统一处理菜单事件。
///
/// * `app_handle` - 应用程序句柄
/// * `menu_id` - 菜单项标识
pub fn handle_menu_event(app_handle: &AppHandle, menu_id: &str) {
    match menu_id {
        MENU_SHOW | MENU_TOGGLE_CALENDAR => {
            // 获取共享状态中的窗口管理器。
            let app_state: State<AppState> = app_handle.state();
            if let Ok(mut window_manager_guard) = app_state.window_manager.lock() {
                if let Some(window_manager) = window_manager_guard.as_mut() {
                    let _ = window_manager.toggle_popup_at_position(9999, 9999);
                }
            };
        }
        MENU_OPEN_MAIN | MENU_SETTINGS => {
            crate::window_manager::show_or_create_main_window(app_handle);
        }
        MENU_EXIT => {
            #[cfg(windows)]
            {
                // 退出程序后，恢复系统默认时钟
                // let _ = crate::windows_hook::disable_custom_clock();
            }
            crate::request_app_exit(app_handle);
        }
        _ => {}
    }
}
