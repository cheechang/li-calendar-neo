//! 主设置窗口的配置与单实例唤起时的窗口聚焦。
#[cfg(desktop)]
use tauri::{AppHandle, Manager};

#[cfg(all(desktop, target_os = "macos"))]
pub fn configure_main_window(app_handle: &AppHandle) {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        // 菜单栏在 setup_desktop_app 中已通过 AppHandle::set_menu 配置；窗口级 set_menu 在 macOS 上无效。
        let _ = main_window.close();
    }
}

#[cfg(all(desktop, not(target_os = "macos")))]
pub fn configure_main_window(app_handle: &AppHandle) {
    if let Some(main_window) = app_handle.get_webview_window("main") {
        if let Ok(main_menu) = crate::menu::build_main_menu(app_handle) {
            let _ = main_window.set_menu(main_menu);
        }

        let _ = main_window.close();
    }
}

#[cfg(desktop)]
/// 聚焦并显示第一个可用的 Webview 窗口，用于单实例唤起场景。
///
/// * `app_handle` - 应用程序句柄
pub fn focus_first_webview_window(app_handle: &AppHandle) {
    // 获取所有的 Webview 窗口字典
    let webview_windows = app_handle.webview_windows();
    // 提取第一个可用的窗口，如果不存在则 panic
    let first_window = webview_windows.values().next().expect("未找到任何 Webview 窗口");
    // 还原最小化、显示并尝试聚焦
    first_window.unminimize().expect("无法将窗口从最小化还原");
    first_window.show().expect("无法显示窗口");
    first_window.set_focus().expect("无法将窗口置于前台聚焦");
    #[cfg(target_os = "macos")]
    {
        let _ = crate::app_runtime::macos::set_activation_policy_for_main_window_visible(
            app_handle, true,
        );
        crate::menu::sync_macos_app_menu(app_handle);
    }
}
