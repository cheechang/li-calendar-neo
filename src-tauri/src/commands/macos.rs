//! 非 Windows（主要为 macOS）实现的 Tauri 命令；与 `commands/windows` 互为占位。
#[cfg(target_os = "macos")]
use crate::app_runtime::macos::tray::{
    refresh_tray_title, set_tray_bar_icon, set_tray_date_icon_style, set_tray_icon_px,
    set_tray_title_template,
};
#[cfg(target_os = "macos")]
use crate::app_runtime::macos::tray_bar_icon::MacosTrayBarIcon;
#[cfg(target_os = "macos")]
use crate::app_runtime::macos::tray_calendar_icon::TrayDateIconStyle;
use crate::app_runtime::tray_icon_px::TrayIconPx;
use crate::AppState;
use tauri::State;
#[cfg(target_os = "macos")]
use tauri::{
    window::{Effect, EffectState, EffectsBuilder},
    AppHandle, Manager, WebviewWindow,
};

/// 统一返回当前命令仅支持 Windows 的错误。
fn windows_only_error<T>() -> Result<T, String> {
    Err("仅支持 Windows".to_string())
}

#[cfg(target_os = "macos")]
const MACOS_SUPPORTED_WINDOW_EFFECTS: &[&str] =
    &["popover", "sidebar", "hud-window", "header-view", "under-window-background"];

#[tauri::command]
pub async fn get_macos_tray_title_template(state: State<'_, AppState>) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        state
            .macos_tray_title_template
            .lock()
            .map(|template| template.clone())
            .map_err(|error| error.to_string())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = state;
        windows_only_error()
    }
}

#[tauri::command]
pub async fn set_macos_tray_title_template(
    #[cfg(target_os = "macos")] app_handle: AppHandle,
    state: State<'_, AppState>,
    template: String,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        set_tray_title_template(&state, &template);
        refresh_tray_title(&app_handle, &state)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = state;
        let _ = template;
        windows_only_error()
    }
}

#[tauri::command]
pub async fn get_macos_tray_date_icon_style(state: State<'_, AppState>) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        state
            .macos_tray_date_icon_style
            .lock()
            .map(|s| s.as_config_str().to_string())
            .map_err(|e| e.to_string())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = state;
        windows_only_error()
    }
}

#[tauri::command]
pub async fn set_macos_tray_date_icon_style(
    #[cfg(target_os = "macos")] app_handle: AppHandle,
    state: State<'_, AppState>,
    style: String,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        set_tray_date_icon_style(&state, TrayDateIconStyle::from_config(&style));
        refresh_tray_title(&app_handle, &state)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = state;
        let _ = style;
        windows_only_error()
    }
}

#[tauri::command]
pub async fn get_macos_tray_icon_px(state: State<'_, AppState>) -> Result<TrayIconPx, String> {
    #[cfg(target_os = "macos")]
    {
        state.macos_tray_icon_px.lock().map(|p| *p).map_err(|e| e.to_string())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = state;
        windows_only_error()
    }
}

#[tauri::command]
pub async fn get_macos_tray_bar_icon(state: State<'_, AppState>) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        state
            .macos_tray_bar_icon
            .lock()
            .map(|b| b.as_config_str().to_string())
            .map_err(|e| e.to_string())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = state;
        windows_only_error()
    }
}

#[tauri::command]
pub async fn set_macos_tray_bar_icon(
    #[cfg(target_os = "macos")] app_handle: AppHandle,
    state: State<'_, AppState>,
    icon: String,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        set_tray_bar_icon(&state, MacosTrayBarIcon::from_config(&icon));
        refresh_tray_title(&app_handle, &state)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = state;
        let _ = icon;
        windows_only_error()
    }
}

#[tauri::command]
pub async fn set_macos_tray_icon_px(
    #[cfg(target_os = "macos")] app_handle: AppHandle,
    state: State<'_, AppState>,
    width: u32,
    height: u32,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        set_tray_icon_px(&state, TrayIconPx::sanitize(width, height));
        refresh_tray_title(&app_handle, &state)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = state;
        let _ = width;
        let _ = height;
        windows_only_error()
    }
}

/// 非 Windows 平台占位命令：恢复系统默认时钟显示。
#[tauri::command]
pub async fn restore_default_clock(_state: State<'_, AppState>) -> Result<(), String> {
    windows_only_error()
}

/// 非 Windows 平台占位命令：设置自定义任务栏时钟文本。
#[tauri::command]
pub async fn apply_custom_clock_text(
    _state: State<'_, AppState>,
    _text: String,
) -> Result<(), String> {
    windows_only_error()
}

/// 非 Windows 平台占位命令：读取任务栏时钟文本。
#[tauri::command]
pub async fn get_clock_text(_state: State<'_, AppState>) -> Result<String, String> {
    windows_only_error()
}

/// 非 Windows 平台占位命令：切换桌面组件开关。
#[tauri::command]
pub async fn set_desktop_widget_enabled(
    _state: State<'_, AppState>,
    _enabled: bool,
) -> Result<(), String> {
    windows_only_error()
}

/// 非 Windows 平台占位命令：切换任务栏组件开关。
#[tauri::command]
pub async fn set_taskbar_widget_enabled_command(
    _state: State<'_, AppState>,
    _enabled: bool,
) -> Result<(), String> {
    windows_only_error()
}

/// 非 Windows 平台占位命令：测试时钟检测能力。
#[tauri::command]
pub async fn test_clock_detection(_state: State<'_, AppState>) -> Result<String, String> {
    windows_only_error()
}

/// macOS 原生毛玻璃效果切换命令。
#[cfg(target_os = "macos")]
fn build_macos_effects_config(effect: Option<&str>) -> tauri::utils::config::WindowEffectsConfig {
    let effect = match effect.unwrap_or("popover") {
        "blur" | "vibrancy" | "popover" => Effect::Popover,
        "acrylic" | "sidebar" => Effect::Sidebar,
        "mica" | "mica-dark" | "mica-light" | "hud-window" => Effect::HudWindow,
        "tabbed" | "tabbed-dark" | "tabbed-light" | "header-view" => Effect::HeaderView,
        "liquid-glass" | "under-window-background" => Effect::UnderWindowBackground,
        _ => Effect::Popover,
    };

    EffectsBuilder::new().effect(effect).state(EffectState::Active).build()
}

#[cfg(target_os = "macos")]
fn clear_macos_window_effects(window: &WebviewWindow) -> Result<(), String> {
    window
        .set_effects(Option::<tauri::utils::config::WindowEffectsConfig>::None)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_supported_window_effects() -> Result<Vec<String>, String> {
    #[cfg(target_os = "macos")]
    {
        Ok(MACOS_SUPPORTED_WINDOW_EFFECTS.iter().map(|effect| effect.to_string()).collect())
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(Vec::new())
    }
}

#[tauri::command]
pub async fn set_macos_vibrancy(
    #[cfg(target_os = "macos")] app_handle: AppHandle,
    #[cfg(target_os = "macos")] window: WebviewWindow,
    enabled: bool,
    effect: Option<String>,
    window_label: Option<String>,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let (sender, receiver) = std::sync::mpsc::channel();
        let effect = effect.clone();
        let target_window = window_label
            .as_deref()
            .and_then(|label| app_handle.get_webview_window(label))
            .unwrap_or(window);
        app_handle
            .run_on_main_thread(move || {
                let result: Result<(), String> = (|| {
                    clear_macos_window_effects(&target_window)?;
                    if enabled {
                        let effects = build_macos_effects_config(effect.as_deref());
                        target_window
                            .set_effects(Some(effects))
                            .map_err(|error| error.to_string())?;
                    } else {
                        target_window
                            .set_effects(Option::<tauri::utils::config::WindowEffectsConfig>::None)
                            .map_err(|error| error.to_string())?;
                    }
                    Ok(())
                })();
                let _ = sender.send(result);
            })
            .map_err(|error| error.to_string())?;
        let result: Result<(), String> = receiver.recv().map_err(|error| error.to_string())?;
        result
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = enabled;
        let _ = effect;
        let _ = window_label;
        windows_only_error()
    }
}
