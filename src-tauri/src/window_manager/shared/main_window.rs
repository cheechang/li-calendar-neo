use tauri::{AppHandle, Manager};

/// 显示主窗口；若主窗口不存在则按统一参数创建并恢复菜单。
///
/// * `app_handle` - 应用程序句柄
pub fn show_or_create_main_window(app_handle: &AppHandle) {
    // 尝试获取已经存在的主窗口
    if let Some(main_window) = app_handle.get_webview_window("main") {
        // 若已最小化，需先还原再 show，否则无法前台显示（与单实例唤起 `focus_first_webview_window` 一致）
        let _ = main_window.unminimize();
        let _ = main_window.set_always_on_top(true);
        let _ = main_window.show();
        let _ = main_window.set_focus();
        #[cfg(target_os = "macos")]
        {
            let _ = crate::app_runtime::macos::set_activation_policy_for_main_window_visible(
                app_handle, true,
            );
            crate::menu::sync_macos_app_menu(app_handle);
        }
        return;
    }

    // 如果主窗口不存在，则构建一个新的 Webview 窗口
    if let Ok(main_window) = tauri::WebviewWindowBuilder::new(
        app_handle,
        "main",
        // 设置加载的入口 URL
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("松鼠日历") // 设置窗口标题
    .inner_size(800.0, 600.0) // 设置窗口内部大小
    .center() // 窗口居中
    .resizable(true) // 允许调整大小
    .always_on_top(true) // 主设置窗口默认置顶
    .visible(true) // 初始化为可见
    .decorations(true) // 显示系统边框装饰
    .build()
    {
        // 构建并绑定主窗口菜单栏（macOS 为应用级菜单，在 setup 中设置）。
        #[cfg(not(target_os = "macos"))]
        if let Ok(main_menu) = crate::menu::build_main_menu(app_handle) {
            let _ = main_window.set_menu(main_menu);
        }
        let _ = main_window.unminimize();
        let _ = main_window.set_always_on_top(true);
        let _ = main_window.show();
        let _ = main_window.set_focus();
        #[cfg(target_os = "macos")]
        {
            let _ = crate::app_runtime::macos::set_activation_policy_for_main_window_visible(
                app_handle, true,
            );
            crate::menu::sync_macos_app_menu(app_handle);
        }
    }
}
