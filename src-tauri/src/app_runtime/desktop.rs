//! 桌面端应用初始化：共享状态、按平台加载配置、创建窗口管理器与托盘。
#[cfg(not(windows))]
use crate::window_manager::CalendarWindowManager;
#[cfg(windows)]
use crate::windows_hook::set_taskbar_widget_enabled as apply_taskbar_widget_enabled;
use crate::AppState;
#[cfg(windows)]
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

/// 构建桌面端共享状态，按平台初始化对应字段。
pub fn create_shared_state() -> AppState {
    #[cfg(windows)]
    {
        AppState {
            window_manager: Arc::new(Mutex::new(None)),
            hook_manager: Arc::new(Mutex::new(None)),
            desktop_init_started: Arc::new(AtomicBool::new(false)),
            taskbar_init_started: Arc::new(AtomicBool::new(false)),
            desktop_widget_enabled: Arc::new(AtomicBool::new(false)),
            taskbar_widget_enabled: Arc::new(AtomicBool::new(false)),
        }
    }
    #[cfg(not(windows))]
    {
        #[cfg(target_os = "macos")]
        {
            AppState {
                window_manager: Arc::new(Mutex::new(None)),
                macos_tray_title_template: Arc::new(Mutex::new(
                    super::macos::tray::DEFAULT_TRAY_TITLE_TEMPLATE.to_string(),
                )),
                macos_tray_date_icon_style: Arc::new(Mutex::new(
                    super::macos::tray_calendar_icon::TrayDateIconStyle::default(),
                )),
                macos_tray_icon_px: Arc::new(
                    Mutex::new(super::tray_icon_px::TrayIconPx::default()),
                ),
                macos_tray_bar_icon: Arc::new(Mutex::new(
                    super::macos::tray_bar_icon::MacosTrayBarIcon::default(),
                )),
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            AppState { window_manager: Arc::new(Mutex::new(None)) }
        }
    }
}

/// 组装桌面端启动流程：加载配置、初始化窗口管理器并注册托盘行为。
pub fn setup_desktop_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();
    let app_state: State<AppState> = app.state();
    #[cfg(windows)]
    let show_startup_popup = !std::env::args().any(|arg| arg == "--autostart");

    #[cfg(target_os = "macos")]
    {
        crate::app_runtime::macos::set_activation_policy_for_main_window_visible(
            app_handle, false,
        )?;
        let persisted_config = super::config::load_persisted_feature_config(app_handle);
        if let Some(tray_title_template) = persisted_config.macos_tray_title_template.as_deref() {
            super::macos::tray::set_tray_title_template(&app_state, tray_title_template);
        }
        if let Some(style) = persisted_config.macos_tray_date_icon_style.as_deref() {
            super::macos::tray::set_tray_date_icon_style(
                &app_state,
                super::macos::tray_calendar_icon::TrayDateIconStyle::from_config(style),
            );
        }
        super::macos::tray::set_tray_icon_px(
            &app_state,
            super::tray_icon_px::TrayIconPx::from_optional(
                persisted_config.macos_tray_icon_width,
                persisted_config.macos_tray_icon_height,
            ),
        );
        if let Some(kind) = persisted_config.macos_tray_bar_icon.as_deref() {
            super::macos::tray::set_tray_bar_icon(
                &app_state,
                super::macos::tray_bar_icon::MacosTrayBarIcon::from_config(kind),
            );
        }
    }

    super::main_window::configure_main_window(app_handle);

    #[cfg(target_os = "macos")]
    {
        // macOS 菜单栏为应用级，须通过 AppHandle::set_menu 以触发 init_for_nsapp 与 Window/Help 角色菜单。
        if let Ok(menu) = crate::menu::build_main_menu(app_handle) {
            let _ = app_handle.set_menu(menu);
        }
    }

    #[cfg(windows)]
    {
        let persisted_config = super::config::load_persisted_feature_config(app_handle);
        let desktop_widget_enabled = persisted_config.desktop_widget_enabled.unwrap_or(true);
        let taskbar_widget_enabled = persisted_config.taskbar_widget_enabled.unwrap_or(true);
        app_state.desktop_widget_enabled.store(desktop_widget_enabled, Ordering::SeqCst);
        app_state.taskbar_widget_enabled.store(taskbar_widget_enabled, Ordering::SeqCst);
        apply_taskbar_widget_enabled(taskbar_widget_enabled);

        if let Err(error) =
            super::windows::startup::initialize_window_manager_for_windows(app_handle, &app_state)
        {
            eprintln!("创建日历窗口管理器失败: {}", error);
            return Ok(());
        }

        super::windows::tray::setup_windows_tray(app_handle, &app_state);

        // 若同时启用桌面与启动弹窗，则在 startup 中顺序创建（先桌面后弹窗）；否则仍可能并发创建单一路径。
        let initial_pos = persisted_config.desktop_window_position.as_ref().map(|p| (p.x, p.y));
        super::windows::startup::spawn_windows_concurrently(
            app_handle.clone(),
            &app_state,
            show_startup_popup,
            desktop_widget_enabled,
            taskbar_widget_enabled,
            initial_pos,
        );
    }

    #[cfg(not(windows))]
    {
        let window_manager = match CalendarWindowManager::new(app_handle) {
            Ok(manager) => manager,
            Err(error) => {
                eprintln!("创建日历窗口管理器失败: {}", error);
                return Ok(());
            }
        };

        if let Ok(mut manager_guard) = app_state.window_manager.lock() {
            *manager_guard = Some(window_manager);
        }

        // macOS 平台：设置托盘并调度启动弹窗
        #[cfg(target_os = "macos")]
        {
            super::macos::tray::setup_macos_tray(app_handle, &app_state);
            super::macos::tray::schedule_startup_popup(app_handle.clone(), &app_state);

            let window_manager = app_state.window_manager.clone();
            let warmup = app_handle.clone();
            let _ = warmup.run_on_main_thread(move || {
                if let Ok(mut guard) = window_manager.lock() {
                    if let Some(manager) = guard.as_mut() {
                        if let Err(error) = manager.preload_popup() {
                            eprintln!("预加载日历弹窗失败: {}", error);
                        }
                    }
                }
            });
        }
    }

    Ok(())
}

#[cfg(windows)]
pub fn on_page_load(window: &tauri::Webview) {
    let _ = window;
}

#[cfg(not(windows))]
pub fn on_page_load(_window: &tauri::Webview) {}
