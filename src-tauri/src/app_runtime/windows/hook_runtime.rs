//! 任务栏低级鼠标钩子与点击事件分发。
use crate::menu::build_context_menu;
use crate::window_manager::shared::popup_manager::PopupManager;
use crate::window_manager::CalendarWindowManager;
use crate::windows_hook::{
    is_desktop_in_foreground, start_hook_message_thread, ClickEvent, MouseButton,
    WindowsHookManager, IS_MENU_OPEN,
};
use crate::AppState;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use tokio::sync::mpsc;

/// 启动任务栏 Hook 运行时并接管点击事件流。
pub fn start_taskbar_runtime(app_handle: AppHandle, state: &AppState) {
    // 检查任务栏功能开关，未开启时跳过初始化。
    if !state.taskbar_widget_enabled.load(Ordering::SeqCst) {
        return;
    }
    // 防止重复启动 Hook 运行时。
    if state.taskbar_init_started.swap(true, Ordering::SeqCst) {
        return;
    }
    // 克隆共享窗口管理器句柄供异步任务使用。
    let shared_window_manager = Arc::clone(&state.window_manager);
    // 克隆共享 Hook 管理器句柄用于回填状态。
    let shared_hook_manager = Arc::clone(&state.hook_manager);
    tauri::async_runtime::spawn(async move {
        // 创建当前运行时专用的 Hook 管理器。
        let mut runtime_hook_manager = WindowsHookManager::new();
        // 启动消息循环线程（安装钩子后同线程会做一次 UIA 初始化时钟区域缓存）。
        start_hook_message_thread();
        // 获取事件接收器并开始监听点击事件。
        if let Some(event_receiver) = runtime_hook_manager.take_event_receiver() {
            tauri::async_runtime::spawn(start_hook_listener(
                app_handle.clone(),
                event_receiver,
                Arc::clone(&shared_window_manager),
            ));
        }
        // 将 Hook 管理器写回全局状态。
        if let Ok(mut hook_manager_guard) = shared_hook_manager.lock() {
            *hook_manager_guard = Some(runtime_hook_manager);
        }
        println!("Windows 钩子系统已初始化");
    });
}

/// 监听并处理来自系统时钟区域的点击事件。
pub async fn start_hook_listener(
    app_handle: AppHandle,
    mut event_receiver: mpsc::UnboundedReceiver<ClickEvent>,
    window_manager: Arc<Mutex<Option<CalendarWindowManager>>>,
) {
    while let Some(click_event) = event_receiver.recv().await {
        if is_desktop_in_foreground() {
            if let Ok(window_manager_guard) = window_manager.lock() {
                if let Some(calendar_window_manager) = window_manager_guard.as_ref() {
                    calendar_window_manager.refresh_desktop_window_visibility();
                }
            }
        }

        if !click_event.in_clock_area {
            continue;
        }
        match click_event.button {
            MouseButton::Left => handle_left_click(&window_manager, click_event.x, click_event.y),
            MouseButton::Right => handle_right_click(&app_handle),
        }
    }
}

/// 处理左键点击事件并切换日历弹窗。
fn handle_left_click(
    window_manager: &Arc<Mutex<Option<CalendarWindowManager>>>,
    click_x: i32,
    click_y: i32,
) {
    if let Ok(mut window_manager_guard) = window_manager.lock() {
        if let Some(calendar_window_manager) = window_manager_guard.as_mut() {
            if let Err(error) = calendar_window_manager.toggle_popup_at_position(click_x, click_y) {
                eprintln!("❌ 切换日历窗口失败: {}", error);
                if let Err(fallback_error) = calendar_window_manager.toggle_popup() {
                    eprintln!("❌ 回退到默认切换也失败: {}", fallback_error);
                }
            }
        }
    }
}

/// 处理右键点击事件并弹出上下文菜单。
fn handle_right_click(app_handle: &AppHandle) {
    if IS_MENU_OPEN.load(Ordering::SeqCst) {
        return;
    }
    if let Ok(context_menu) = build_context_menu(app_handle) {
        let target_window = app_handle
            .get_webview_window("main")
            .or_else(|| app_handle.get_webview_window("calendar"))
            .or_else(|| app_handle.get_webview_window("desktop_calendar"));
        if let Some(window) = target_window {
            IS_MENU_OPEN.store(true, Ordering::SeqCst);
            let _ = window.set_focus();
            let _ = window.popup_menu(&context_menu);
            IS_MENU_OPEN.store(false, Ordering::SeqCst);
        }
    }
}
