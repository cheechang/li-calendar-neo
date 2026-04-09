//! Windows 下转发到共享 `main_window` 实现。
use tauri::AppHandle;

/// 显示主窗口；若主窗口已被销毁则按统一配置重新创建。
pub fn show_or_create_main_window(app: &AppHandle) {
    crate::window_manager::shared::main_window::show_or_create_main_window(app);
}
