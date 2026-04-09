//! 跨平台共用的 Tauri 命令实现（窗口显示、问候等）。
use crate::window_manager::shared::popup_manager::PopupManager;
use crate::window_manager::CalendarWindowManager;
use crate::AppState;
#[cfg(windows)]
use std::sync::atomic::Ordering;
use tauri::{AppHandle, State, WebviewWindow};

/// 内部辅助函数：执行需要操作 `window_manager` 的闭包，并处理锁定和错误转换。
///
/// * `state` - 应用状态
/// * `require_enabled` - 是否要求任务栏组件启用
/// * `action` - 要执行的具体操作
#[cfg(windows)]
fn with_window_manager<F>(
    state: &State<'_, AppState>,
    require_enabled: bool,
    action: F,
) -> Result<(), String>
where
    F: FnOnce(&mut CalendarWindowManager) -> Result<(), String>,
{
    #[cfg(windows)]
    if require_enabled && !state.taskbar_widget_enabled.load(Ordering::SeqCst) {
        return Err("任务栏弹窗功能未启用".to_string());
    }

    // 尝试获取窗口管理器的互斥锁
    if let Ok(mut manager_guard) = state.window_manager.lock() {
        // 如果存在窗口管理器，则执行回调
        if let Some(window_manager) = manager_guard.as_mut() {
            return action(window_manager);
        }
    }

    // 如果无法获取锁或管理器为空，返回空结果或可在此抛出错误
    Ok(())
}

#[cfg(not(windows))]
fn with_window_manager<F>(
    state: &State<'_, AppState>,
    _require_enabled: bool,
    action: F,
) -> Result<(), String>
where
    F: FnOnce(&mut CalendarWindowManager) -> Result<(), String>,
{
    if let Ok(mut manager_guard) = state.window_manager.lock() {
        if let Some(window_manager) = manager_guard.as_mut() {
            return action(window_manager);
        }
    }

    Ok(())
}

/// 欢迎测试命令。
///
/// * `name` - 问候的名字
#[tauri::command]
pub fn greet(name: &str) -> String {
    // 格式化返回的问候语
    format!("你好，{}！此问候来自 Rust 后端。", name)
}

/// 切换日历窗口的显示或隐藏状态。
///
/// * `state` - 注入的应用状态
#[tauri::command]
pub async fn toggle_calendar(state: State<'_, AppState>) -> Result<(), String> {
    with_window_manager(&state, true, |wm| wm.toggle_popup().map_err(|error| error.to_string()))
}

/// 在系统时钟附近显示日历窗口。
///
/// * `state` - 注入的应用状态
#[tauri::command]
pub async fn show_calendar(state: State<'_, AppState>) -> Result<(), String> {
    with_window_manager(&state, true, |wm| {
        wm.show_popup_near_clock().map_err(|error| error.to_string())
    })
}

/// 通知后台日历弹窗已经加载准备就绪。
///
/// * `state` - 注入的应用状态
#[tauri::command]
pub async fn popup_ready(state: State<'_, AppState>) -> Result<(), String> {
    with_window_manager(&state, false, |wm| wm.on_popup_ready().map_err(|error| error.to_string()))
}

/// 隐藏日历窗口。
///
/// * `state` - 注入的应用状态
#[tauri::command]
pub async fn hide_calendar(state: State<'_, AppState>) -> Result<(), String> {
    with_window_manager(&state, true, |wm| wm.hide_popup().map_err(|error| error.to_string()))
}

/// 打开主界面窗口；若不存在则创建。
///
/// * `app_handle` - 应用程序句柄
#[tauri::command]
pub async fn open_main_window(app_handle: AppHandle, window: WebviewWindow) -> Result<(), String> {
    let current_window_label = window.label().to_string();
    let current_window = window.clone();
    let main_thread_app_handle = app_handle.clone();
    let (sender, receiver) = std::sync::mpsc::channel();

    app_handle
        .run_on_main_thread(move || {
            let result: Result<(), String> = (|| {
                if current_window_label == "calendar" {
                    current_window.hide().map_err(|error| error.to_string())?;
                }
                crate::window_manager::show_or_create_main_window(&main_thread_app_handle);
                Ok(())
            })();
            let _ = sender.send(result);
        })
        .map_err(|error| error.to_string())?;

    receiver.recv().map_err(|error| error.to_string())?
}

/// 在指定鼠标点击位置切换日历窗口状态。
///
/// * `state` - 注入的应用状态
/// * `click_x` - 鼠标点击的X坐标
/// * `click_y` - 鼠标点击的Y坐标
#[tauri::command]
pub async fn toggle_calendar_at_position(
    state: State<'_, AppState>,
    click_x: i32,
    click_y: i32,
) -> Result<(), String> {
    with_window_manager(&state, true, |wm| {
        wm.toggle_popup_at_position(click_x, click_y).map_err(|error| error.to_string())
    })
}

/// 切换弹窗固定状态。
///
/// * `state` - 注入的应用状态
/// * `pin` - 是否固定
#[tauri::command]
pub async fn set_calendar_pin(state: State<'_, AppState>, pin: bool) -> Result<(), String> {
    with_window_manager(&state, false, |wm| {
        wm.set_popup_pin(pin).map_err(|error| error.to_string())
    })
}

/// 使用标准库读取当前系统时间：自 Unix 纪元起的毫秒数（Windows / macOS 等均可用）。
#[tauri::command]
pub fn get_system_time_millis_since_epoch() -> Result<u64, String> {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("系统时间无效: {}", e))?;
    u64::try_from(duration.as_millis()).map_err(|_| "时间戳超出可表示范围".to_string())
}
