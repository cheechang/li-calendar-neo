//! 松鼠日历 Tauri 后端库：窗口管理、平台命令与托盘/钩子集成。
#[cfg(desktop)]
use tauri::RunEvent;
#[cfg(all(desktop, target_os = "macos"))]
use tauri::WindowEvent;
#[cfg(desktop)]
mod app_runtime;
#[cfg(desktop)]
mod commands;
#[cfg(desktop)]
mod menu;
#[cfg(desktop)]
mod window_manager;
#[cfg(all(desktop, windows))]
mod windows_hook;
#[cfg(desktop)]
use commands::{
    apply_custom_clock_text, get_clock_text, get_macos_tray_bar_icon,
    get_macos_tray_date_icon_style, get_macos_tray_icon_px, get_macos_tray_title_template,
    get_supported_window_effects, get_system_time_millis_since_epoch, greet, hide_calendar,
    open_main_window, popup_ready,
    restore_default_clock, set_calendar_pin, set_desktop_widget_enabled,
    set_macos_tray_bar_icon, set_macos_tray_date_icon_style, set_macos_tray_icon_px,
    set_macos_tray_title_template, set_macos_vibrancy, set_taskbar_widget_enabled_command,
    show_calendar, test_clock_detection, toggle_calendar, toggle_calendar_at_position,
};
#[cfg(desktop)]
use menu::handle_menu_event;
#[cfg(desktop)]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(desktop)]
use std::sync::{Arc, Mutex};
#[cfg(desktop)]
use window_manager::CalendarWindowManager;
#[cfg(all(desktop, windows))]
use windows_hook::WindowsHookManager;

#[cfg(desktop)]
/// 应用全局状态，包含各类窗口管理器和功能开关。
pub struct AppState {
    /// 跨平台日历窗口管理器
    pub window_manager: Arc<Mutex<Option<CalendarWindowManager>>>,
    #[cfg(target_os = "macos")]
    /// macOS 菜单栏文案模板
    pub macos_tray_title_template: Arc<Mutex<String>>,
    #[cfg(target_os = "macos")]
    /// macOS 菜单栏日期图标样式（实心 / 描边，对齐 LunarBar）
    pub macos_tray_date_icon_style:
        Arc<Mutex<crate::app_runtime::macos::tray_calendar_icon::TrayDateIconStyle>>,
    #[cfg(target_os = "macos")]
    /// macOS 菜单栏日期图标像素尺寸（宽×高）
    pub macos_tray_icon_px: Arc<Mutex<crate::app_runtime::tray_icon_px::TrayIconPx>>,
    #[cfg(target_os = "macos")]
    /// 菜单栏主图标：日期数字（DateIconView）或 LunarBar 同款 SF Symbol `calendar`（模板）
    pub macos_tray_bar_icon: Arc<Mutex<crate::app_runtime::macos::tray_bar_icon::MacosTrayBarIcon>>,
    #[cfg(windows)]
    /// Windows 专属的系统钩子管理器，用于任务栏等底层事件捕获
    pub hook_manager: Arc<Mutex<Option<WindowsHookManager>>>,
    #[cfg(windows)]
    /// 桌面组件初始化状态
    pub desktop_init_started: Arc<AtomicBool>,
    #[cfg(windows)]
    /// 任务栏组件初始化状态
    pub taskbar_init_started: Arc<AtomicBool>,
    #[cfg(windows)]
    /// 桌面组件功能是否开启
    pub desktop_widget_enabled: Arc<AtomicBool>,
    #[cfg(windows)]
    /// 任务栏组件功能是否开启
    pub taskbar_widget_enabled: Arc<AtomicBool>,
}

#[cfg(desktop)]
/// 允许退出标志位，用于拦截默认关闭事件，实现后台运行
static ALLOW_EXIT: AtomicBool = AtomicBool::new(false);

#[cfg(desktop)]
/// 触发应用安全退出。
///
/// * `app` - 应用程序句柄
pub(crate) fn request_app_exit(app: &tauri::AppHandle) {
    // 设置允许退出标志为真
    ALLOW_EXIT.store(true, Ordering::SeqCst);
    // 退出程序并返回状态码 0
    app.exit(0);
}

#[cfg(desktop)]
/// 桌面端主运行入口点。
pub fn run() {
    // 创建共享的应用状态
    let shared_state = app_runtime::desktop::create_shared_state();

    // 构建 Tauri 应用
    let build_result = tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_single_instance::init(|app, _, _cwd| {
            // 单实例回调：如果尝试打开新实例，则聚焦现有窗口
            app_runtime::main_window::focus_first_webview_window(app);
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .on_menu_event(|app, event| {
            // 处理系统菜单事件
            handle_menu_event(app, event.id().as_ref());
        })
        .manage(shared_state) // 注入应用状态
        .invoke_handler(tauri::generate_handler![
            // 注册前端可调用的命令
            get_system_time_millis_since_epoch,
            greet,
            popup_ready,
            toggle_calendar,
            show_calendar,
            hide_calendar,
            toggle_calendar_at_position,
            test_clock_detection,
            restore_default_clock,
            apply_custom_clock_text,
            get_clock_text,
            get_supported_window_effects,
            get_macos_tray_title_template,
            get_macos_tray_date_icon_style,
            get_macos_tray_icon_px,
            get_macos_tray_bar_icon,
            set_desktop_widget_enabled,
            set_macos_tray_title_template,
            set_macos_tray_date_icon_style,
            set_macos_tray_icon_px,
            set_macos_tray_bar_icon,
            set_taskbar_widget_enabled_command,
            set_macos_vibrancy,
            open_main_window,
            set_calendar_pin
        ])
        .setup(|app| app_runtime::desktop::setup_desktop_app(app)) // 设置生命周期钩子
        .on_page_load(|window, _payload| app_runtime::desktop::on_page_load(window))
        .build(tauri::generate_context!());

    // 运行构建好的应用
    match build_result {
        Ok(app) => {
            app.run(|_app_handle, event| {
                #[cfg(target_os = "macos")]
                {
                    match &event {
                        RunEvent::WindowEvent { label, event: win_event, .. } => {
                            if label == "main" && matches!(win_event, WindowEvent::Destroyed) {
                                let _ = crate::app_runtime::macos::set_activation_policy_for_main_window_visible(
                                    _app_handle,
                                    false,
                                );
                            }
                        }
                        RunEvent::Reopen { .. } => {
                            crate::window_manager::show_or_create_main_window(_app_handle);
                        }
                        _ => {}
                    }
                }
                // 监听退出请求事件
                if let RunEvent::ExitRequested { api, .. } = event {
                    // 如果不允许退出，则阻止退出（实现最小化到托盘）
                    if !ALLOW_EXIT.load(Ordering::SeqCst) {
                        api.prevent_exit();
                    }
                }
            });
        }
        Err(e) => {
            eprintln!("运行 Tauri 应用时出错: {}", e);
        }
    }
}

#[cfg(mobile)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// 移动端主运行入口点。
pub fn run() {
    // 构建并运行 Tauri 移动端应用
    let result = tauri::Builder::default()
        // 为前端平台识别提供移动端可用的系统信息插件。
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .run(tauri::generate_context!());

    if let Err(e) = result {
        eprintln!("运行 Tauri 应用时出错: {}", e);
    }
}
