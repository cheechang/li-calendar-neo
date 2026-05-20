//! Windows 系统托盘：左键切换弹窗、失败时回退显示主窗口。
use crate::app_runtime::shared::tray::{normalize_click_position, toggle_popup_by_click};
use crate::AppState;
use tauri::tray::{
    MouseButton as TrayMouseButton, MouseButtonState as TrayMouseButtonState, TrayIconBuilder,
    TrayIconEvent,
};
use tauri::{AppHandle, Manager, State};

/// 尝试加载托盘图标，优先使用应用默认图标，回退到文件系统。
fn load_tray_icon<'a>(app_handle: &'a AppHandle) -> Option<tauri::image::Image<'a>> {
    if let Some(default_icon) = app_handle.default_window_icon() {
        return Some(tauri::image::Image::new(
            default_icon.rgba(),
            default_icon.width(),
            default_icon.height(),
        ));
    }
    eprintln!("[tray] default_window_icon() returned None, trying to load from file");

    // 尝试从多个可能路径加载图标
    let possible_paths = [
        app_handle.path().resource_dir().ok().map(|p| p.join("icons/icon.ico")),
        std::env::current_exe()
            .ok()
            .map(|p| p.parent().map(|exe_dir| exe_dir.join("icons/icon.ico")))
            .flatten(),
        Some(std::path::PathBuf::from("icons/icon.ico")),
        Some(std::path::PathBuf::from("src-tauri/icons/icon.ico")),
    ];

    for path in possible_paths.iter().flatten() {
        if path.exists() {
            match std::fs::read(path) {
                Ok(bytes) => match tauri::image::Image::from_bytes(&bytes) {
                    Ok(img) => {
                        println!("[tray] Loaded tray icon from: {:?}", path);
                        return Some(img);
                    }
                    Err(e) => eprintln!("[tray] Failed to decode icon from {:?}: {}", path, e),
                },
                Err(e) => eprintln!("[tray] Failed to read icon file {:?}: {}", path, e),
            }
        }
    }

    eprintln!("[tray] Could not load tray icon from any path");
    None
}

/// 与托盘构建时使用的 `TrayIconBuilder::with_id` 一致，供按 ID 查找。
pub const WINDOWS_TRAY_ID: &str = "calendar-tray";

/// 创建 Windows 托盘图标并绑定左键点击切换弹窗行为。
pub fn setup_windows_tray(app_handle: &AppHandle, state: &State<'_, AppState>) {
    // 共享窗口管理器句柄，用于托盘点击时切换弹窗。
    let shared_window_manager = state.window_manager.clone();
    // 回退逻辑使用的应用句柄副本。
    let fallback_app_handle = app_handle.clone();
    // 菜单事件处理使用的应用句柄副本。
    let menu_app_handle = app_handle.clone();
    // 读取默认图标创建托盘实例。
    if let Some(icon) = load_tray_icon(app_handle) {
        // 构建托盘菜单。
        match crate::menu::build_tray_menu(app_handle) {
            Ok(tray_menu) => {
                let tray_icon_result = TrayIconBuilder::with_id(WINDOWS_TRAY_ID)
                    .icon(icon)
                    .menu(&tray_menu)
                    .show_menu_on_left_click(false)
                    .tooltip("松鼠日历")
                    .on_menu_event(move |_tray, event| {
                        let menu_id = event.id().as_ref();
                        crate::menu::handle_menu_event(&menu_app_handle, menu_id);
                    })
                    .on_tray_icon_event(move |_tray, event| {
                        if let TrayIconEvent::Click { button, button_state, rect, .. } = event {
                            if button == TrayMouseButton::Left
                                && button_state == TrayMouseButtonState::Up
                            {
                                // 提取点击坐标并执行弹窗切换。
                                let (click_x, click_y) = normalize_click_position(rect.position);
                                if toggle_popup_by_click(&shared_window_manager, click_x, click_y) {
                                    return;
                                }
                                // 找不到窗口管理器时回退显示主窗口。
                                if let Some(main_window) =
                                    fallback_app_handle.get_webview_window("main")
                                {
                                    let _ = main_window.show();
                                    let _ = main_window.set_focus();
                                }
                            }
                        }
                    })
                    .build(app_handle);
                match tray_icon_result {
                    Ok(_) => println!("[tray] Windows tray icon created successfully"),
                    Err(e) => eprintln!("[tray] Failed to create tray icon: {}", e),
                }
            }
            Err(e) => eprintln!("[tray] Failed to build tray menu: {}", e),
        }
    } else {
        eprintln!("[tray] Failed to load tray icon, skipping tray setup");
    }
}
