//! Windows 平台实现的 Tauri 命令（任务栏钩子、桌面组件等）。
//! 以 `get_macos_*` / `set_macos_*` 命名的命令在本模块中为占位实现，返回「仅支持 macOS」。
use crate::app_runtime;
use crate::app_runtime::tray_icon_px::TrayIconPx;
use crate::window_manager::CalendarWindowManager;
use crate::windows_hook::{
    disable_custom_clock, get_current_system_time_format, refresh_clock_area_cache,
    set_custom_clock_text, set_taskbar_widget_enabled as apply_taskbar_widget_enabled,
};
use crate::AppState;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Manager, State};

/// 前端可选的 Windows 窗口视觉效果标识符列表（与 Tauri `Effect` 映射一致，需保持英文键名）。
const WINDOWS_SUPPORTED_WINDOW_EFFECTS: &[&str] = &[
    "blur",
    "acrylic",
    "mica",
    "mica-dark",
    "mica-light",
    "tabbed",
    "tabbed-dark",
    "tabbed-light",
];

/// 确保窗口管理器已被初始化，如果未初始化则创建它。
///
/// * `app_handle` - 应用程序句柄
/// * `state` - 应用状态
fn ensure_window_manager_initialized(
    app_handle: &AppHandle,
    state: &State<'_, AppState>,
) -> Result<(), String> {
    // 检查是否需要初始化管理器
    let needs_manager = if let Ok(manager_guard) = state.window_manager.lock() {
        manager_guard.is_none()
    } else {
        false
    };

    // 如果需要，则初始化并赋值
    if needs_manager {
        let window_manager =
            CalendarWindowManager::new(app_handle).map_err(|error| error.to_string())?;
        if let Ok(mut manager_guard) = state.window_manager.lock() {
            *manager_guard = Some(window_manager);
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn get_macos_tray_title_template(_state: State<'_, AppState>) -> Result<String, String> {
    Err("仅支持 macOS".to_string())
}

#[tauri::command]
pub async fn set_macos_tray_title_template(
    _state: State<'_, AppState>,
    _template: String,
) -> Result<(), String> {
    Err("仅支持 macOS".to_string())
}

#[tauri::command]
pub async fn get_macos_tray_date_icon_style(_state: State<'_, AppState>) -> Result<String, String> {
    Err("仅支持 macOS".to_string())
}

#[tauri::command]
pub async fn set_macos_tray_date_icon_style(
    _state: State<'_, AppState>,
    _style: String,
) -> Result<(), String> {
    Err("仅支持 macOS".to_string())
}

#[tauri::command]
pub async fn get_macos_tray_icon_px(_state: State<'_, AppState>) -> Result<TrayIconPx, String> {
    Err("仅支持 macOS".to_string())
}

#[tauri::command]
pub async fn set_macos_tray_icon_px(
    _state: State<'_, AppState>,
    _width: u32,
    _height: u32,
) -> Result<(), String> {
    Err("仅支持 macOS".to_string())
}

#[tauri::command]
pub async fn get_macos_tray_bar_icon(_state: State<'_, AppState>) -> Result<String, String> {
    Err("仅支持 macOS".to_string())
}

#[tauri::command]
pub async fn set_macos_tray_bar_icon(
    _state: State<'_, AppState>,
    _icon: String,
) -> Result<(), String> {
    Err("仅支持 macOS".to_string())
}

/// 恢复系统默认时钟显示。
///
/// * `_state` - 注入的应用状态（未使用）
#[tauri::command]
pub async fn restore_default_clock(_state: State<'_, AppState>) -> Result<(), String> {
    // 调用底层方法恢复默认时钟并转换错误类型
    disable_custom_clock().map_err(|error| error.to_string())?;
    // 任务栏时钟控件可能重绘/尺寸变化，重新探测点击区域
    refresh_clock_area_cache();
    Ok(())
}

/// 应用自定义时钟文本。
///
/// * `_state` - 注入的应用状态（未使用）
/// * `text` - 要设置的自定义时钟文本
#[tauri::command]
pub async fn apply_custom_clock_text(
    _state: State<'_, AppState>,
    text: String,
) -> Result<(), String> {
    // 调用底层方法设置自定义时钟文本
    set_custom_clock_text(&text).map_err(|error| error.to_string())?;
    // 自定义文案会改变时钟区域布局，重新探测点击区域
    refresh_clock_area_cache();
    Ok(())
}

/// 获取当前系统时间格式文本。
///
/// * `_state` - 注入的应用状态（未使用）
#[tauri::command]
pub async fn get_clock_text(_state: State<'_, AppState>) -> Result<String, String> {
    // 调用底层方法获取系统时间格式
    get_current_system_time_format().map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_supported_window_effects() -> Result<Vec<String>, String> {
    Ok(WINDOWS_SUPPORTED_WINDOW_EFFECTS.iter().map(|effect| effect.to_string()).collect())
}

#[tauri::command]
pub async fn set_macos_vibrancy(
    app_handle: AppHandle,
    enabled: bool,
    effect: Option<String>,
    window_label: Option<String>,
) -> Result<(), String> {
    let target_window_label = window_label.unwrap_or_else(|| "calendar".to_string());
    let Some(window) = app_handle.get_webview_window(&target_window_label) else {
        return Ok(());
    };
    let effect_for_reopen = effect.clone();

    CalendarWindowManager::clear_window_vibrancy(&window);
    if enabled {
        if target_window_label == "desktop_calendar" {
            CalendarWindowManager::apply_desktop_window_vibrancy(&window, effect.as_deref())?;
        } else {
            CalendarWindowManager::apply_window_vibrancy(&window, effect.as_deref())?;
        }
    }

    if target_window_label == "calendar" || target_window_label == "desktop_calendar" {
        let state = app_handle.state::<AppState>();
        let mut manager_guard = match state.window_manager.lock() {
            Ok(guard) => guard,
            Err(_) => return Ok(()),
        };
        if let Some(window_manager) = manager_guard.as_mut() {
            if target_window_label == "calendar" {
                window_manager.set_popup_vibrancy_config(enabled, effect_for_reopen.clone());
            } else if target_window_label == "desktop_calendar" {
                window_manager.set_desktop_vibrancy_config(enabled, effect_for_reopen.clone());
            }
        }
    }

    Ok(())
}

/// 设置桌面组件的启用状态。
///
/// * `app_handle` - 应用程序句柄
/// * `state` - 注入的应用状态
/// * `enabled` - 是否启用桌面组件
#[tauri::command]
pub async fn set_desktop_widget_enabled(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    // 存储启用状态
    state.desktop_widget_enabled.store(enabled, Ordering::SeqCst);

    if enabled {
        // 确保窗口管理器初始化
        ensure_window_manager_initialized(&app_handle, &state)?;

        // 从持久化配置读取上次保存的窗口位置
        let initial_pos = crate::app_runtime::config::load_persisted_feature_config(&app_handle)
            .desktop_window_position
            .map(|p| (p.x, p.y));

        // 标记桌面组件初始化已开始
        state.desktop_init_started.store(true, Ordering::SeqCst);
        if let Ok(mut manager_guard) = state.window_manager.lock() {
            if let Some(window_manager) = manager_guard.as_mut() {
                // 确保桌面窗口存在，并使用持久化位置避免闪烁
                window_manager
                    .ensure_desktop_window(initial_pos)
                    .map_err(|error| error.to_string())?;
            }
        }
    } else {
        if let Ok(mut manager_guard) = state.window_manager.lock() {
            if let Some(window_manager) = manager_guard.as_mut() {
                // 关闭桌面窗口
                let _ = window_manager.close_desktop_window();
            }
        }
        // 标记桌面组件初始化已结束
        state.desktop_init_started.store(false, Ordering::SeqCst);
    }

    Ok(())
}

/// 设置任务栏组件的启用状态。
///
/// * `app_handle` - 应用程序句柄
/// * `state` - 注入的应用状态
/// * `enabled` - 是否启用任务栏组件
#[tauri::command]
pub async fn set_taskbar_widget_enabled_command(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    enabled: bool,
) -> Result<(), String> {
    // 存储启用状态并应用到底层
    state.taskbar_widget_enabled.store(enabled, Ordering::SeqCst);
    apply_taskbar_widget_enabled(enabled);

    if enabled {
        // 确保窗口管理器初始化
        ensure_window_manager_initialized(&app_handle, &state)?;

        // 启动任务栏运行环境
        app_runtime::windows::hook_runtime::start_taskbar_runtime(app_handle, &state);
    } else {
        if let Ok(mut manager_guard) = state.window_manager.lock() {
            if let Some(window_manager) = manager_guard.as_mut() {
                // 关闭日历窗口
                let _ = window_manager.close_calendar_window();
            }
        }
        // 标记任务栏初始化已结束
        state.taskbar_init_started.store(false, Ordering::SeqCst);
    }

    Ok(())
}

/// 测试时钟检测功能，返回检测结果。
///
/// * `state` - 注入的应用状态
#[tauri::command]
pub async fn test_clock_detection(state: State<'_, AppState>) -> Result<String, String> {
    // 检查任务栏组件是否启用
    if !state.taskbar_widget_enabled.load(Ordering::SeqCst) {
        return Err("任务栏弹窗功能未启用".to_string());
    }
    // 获取钩子管理器的互斥锁
    if let Ok(hook_guard) = state.hook_manager.lock() {
        if let Some(hook_manager) = hook_guard.as_ref() {
            // 查找时钟窗口句柄
            if let Some(clock_hwnd) = hook_manager.find_clock_window() {
                Ok(format!("成功找到时钟窗口句柄: {:?}", clock_hwnd))
            } else {
                Ok("未能找到时钟窗口句柄".to_string())
            }
        } else {
            Err("Hook管理器未初始化".to_string())
        }
    } else {
        Err("无法访问Hook管理器".to_string())
    }
}
