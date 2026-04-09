//! macOS 日历弹窗与叠加文本窗口管理。
use crate::window_manager::shared::popup_manager::PopupManager;
use tauri::{AppHandle, Manager, WebviewWindow, WindowEvent};

pub struct CalendarWindowManager {
    /// 应用句柄。
    app_handle: AppHandle,
    /// 日历弹窗 Webview。
    popup_window: Option<WebviewWindow>,
    /// 可选叠加说明 Webview。
    overlay_text_window: Option<WebviewWindow>,
    /// 弹窗是否固定。
    is_pinned: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl PopupManager for CalendarWindowManager {
    fn hide_popup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(popup_window) = &self.popup_window {
            popup_window.hide()?;
        }
        Ok(())
    }

    fn is_popup_visible(&self) -> bool {
        self.popup_window.as_ref().and_then(|w| w.is_visible().ok()).unwrap_or(false)
    }

    /// 在默认位置显示弹窗窗口。
    fn show_popup_near_clock(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let popup_window = self.ensure_popup_window()?.clone();
        popup_window.center()?;
        popup_window.show()?;
        popup_window.set_focus()?;
        Ok(())
    }

    /// 在指定坐标显示弹窗窗口。
    fn show_popup_at_position(
        &mut self,
        click_x: i32,
        click_y: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let popup_window = self.ensure_popup_window()?.clone();
        let (x, y) = self.calculate_safe_position(click_x, click_y);
        popup_window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }))?;
        popup_window.show()?;
        popup_window.set_focus()?;
        Ok(())
    }

    /// 处理前端弹窗就绪回调，当前平台无额外处理逻辑。
    fn on_popup_ready(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// 设置弹窗是否固定。
    fn set_popup_pin(&mut self, pin: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.is_pinned.store(pin, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
}

impl CalendarWindowManager {
    /// 创建 macOS 平台弹窗窗口管理器并初始化状态。
    pub fn new(app_handle: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            app_handle: app_handle.clone(),
            popup_window: None,
            overlay_text_window: None,
            is_pinned: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    /// 在启动后预创建隐藏弹窗，使 WKWebView 与前端在首次点击前完成加载，缩短第一次展示延迟。
    pub fn preload_popup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.ensure_popup_window()?;
        Ok(())
    }

    /// 确保弹窗窗口存在；若句柄失效则重建。
    fn ensure_popup_window(&mut self) -> Result<&WebviewWindow, Box<dyn std::error::Error>> {
        if self.popup_window.is_some() && self.app_handle.get_webview_window("calendar").is_none() {
            self.popup_window = None;
        }

        if self.popup_window.is_none() {
            let popup_window = tauri::WebviewWindowBuilder::new(
                &self.app_handle,
                "calendar",
                tauri::WebviewUrl::App("index.html?window=macos-popup".into()),
            )
            .title("松鼠日历")
            .inner_size(340.0, 480.0)
            .resizable(true)
            .decorations(false)
            .always_on_top(true)
            .visible(false)
            .skip_taskbar(true)
            .transparent(true)
            .build()?;

            let _ = popup_window.set_shadow(true);
            let popup_window_handle = popup_window.clone();
            let is_pinned_clone = self.is_pinned.clone();
            popup_window.on_window_event(move |event| {
                if matches!(event, WindowEvent::Focused(false))
                    && !is_pinned_clone.load(std::sync::atomic::Ordering::SeqCst)
                {
                    let _ = popup_window_handle.hide();
                }
            });

            self.popup_window = Some(popup_window);
        }

        match self.popup_window.as_ref() {
            Some(window) => Ok(window),
            None => Err("初始化日历窗口失败".into()),
        }
    }

    /// 注册叠加文本窗口句柄。
    pub fn set_overlay_window(&mut self, overlay_window: WebviewWindow) {
        self.overlay_text_window = Some(overlay_window);
    }

    /// 向叠加文本窗口发送文本变更事件。
    pub fn update_overlay_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        crate::window_manager::shared::overlay::update_overlay_text(&self.overlay_text_window, text)
    }

    /// 处理前端弹窗就绪回调，当前平台无额外处理逻辑。
    pub fn on_calendar_ready(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    /// 计算不越界的安全弹窗位置。
    fn calculate_safe_position(&self, click_x: i32, click_y: i32) -> (i32, i32) {
        let fallback_size = (340, 480);
        let (window_width, window_height) = if let Some(popup_window) = &self.popup_window {
            match popup_window.inner_size() {
                Ok(size) => (size.width as i32, size.height as i32),
                Err(_) => fallback_size,
            }
        } else {
            fallback_size
        };

        let margin = 8;
        let edge_margin = 12;

        if let Some(popup_window) = &self.popup_window {
            if let Ok(Some(monitor)) = popup_window.current_monitor() {
                let monitor_size = monitor.size();
                let monitor_position = monitor.position();
                let min_x = monitor_position.x + edge_margin;
                let min_y = monitor_position.y + edge_margin;
                let max_x =
                    monitor_position.x + monitor_size.width as i32 - window_width - edge_margin;
                let max_y =
                    monitor_position.y + monitor_size.height as i32 - window_height - edge_margin;
                let x = (click_x - window_width / 2).clamp(min_x, max_x);
                let y = (click_y + margin).clamp(min_y, max_y);
                return (x, y);
            }
        }

        let x = click_x - window_width / 2;
        let y = click_y + margin;
        (x, y)
    }
}

/// 转发到共享主窗口恢复逻辑。
pub fn show_or_create_main_window(app: &AppHandle) {
    crate::window_manager::shared::main_window::show_or_create_main_window(app);
}
