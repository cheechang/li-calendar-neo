use crate::window_manager::shared::popup_manager::PopupManager;
use crate::window_manager::CalendarWindowManager;
use std::sync::{Arc, Mutex};
use tauri::Position;

/// 将托盘事件中的坐标统一转换为整型屏幕坐标。
pub fn normalize_click_position(position: Position) -> (i32, i32) {
    // 将物理坐标与逻辑坐标统一折叠为 i32 物理坐标。
    match position {
        Position::Physical(physical_position) => (physical_position.x, physical_position.y),
        Position::Logical(logical_position) => {
            (logical_position.x as i32, logical_position.y as i32)
        }
    }
}

/// 尝试使用共享窗口管理器在点击位置切换弹窗。
pub fn toggle_popup_by_click(
    window_manager: &Arc<Mutex<Option<CalendarWindowManager>>>,
    click_x: i32,
    click_y: i32,
) -> bool {
    // 从共享状态中提取窗口管理器并执行切换。
    if let Ok(mut window_manager_guard) = window_manager.lock() {
        if let Some(calendar_window_manager) = window_manager_guard.as_mut() {
            if let Err(error) = calendar_window_manager.toggle_popup_at_position(click_x, click_y) {
                eprintln!("在点击位置切换弹窗失败: {}", error);
            }
            return true;
        }
    }
    false
}
