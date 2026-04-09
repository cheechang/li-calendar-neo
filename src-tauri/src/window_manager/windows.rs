//! Windows 上 `CalendarWindowManager`：任务栏弹窗、桌面组件与窗口特效。
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::{
    window::{Color, Effect, EffectsBuilder},
    AppHandle, WebviewWindow,
};
use windows::Win32::Foundation::HWND;

#[path = "windows/desktop_widget.rs"]
mod desktop_widget;
#[path = "windows/main_window.rs"]
pub mod main_window;
#[path = "windows/taskbar_popup.rs"]
mod taskbar_popup;

/// 管理任务栏日历弹窗、桌面日历与可选叠加说明窗口。
pub struct CalendarWindowManager {
    /// 应用句柄，用于创建/查找子窗口。
    pub(super) app_handle: AppHandle,
    /// 任务栏旁弹出的日历 Webview 窗口。
    pub(super) taskbar_popup_window: Option<WebviewWindow>,
    /// 嵌入桌面的日历 Webview 窗口。
    pub(super) desktop_widget_window: Option<WebviewWindow>,
    /// 可选的叠加文本 Webview 窗口。
    pub(super) overlay_text_window: Option<WebviewWindow>,
    /// 前端就绪前要展示的点击坐标或“贴近时钟”哨兵。
    pub(super) pending_popup_click_position: Option<(i32, i32)>,
    /// 弹窗是否固定（失焦不自动隐藏）。
    pub(super) is_pinned: std::sync::Arc<std::sync::atomic::AtomicBool>,
    /// 启动阶段是否抑制失焦自动隐藏。
    pub(super) suppress_popup_auto_hide: std::sync::Arc<std::sync::atomic::AtomicBool>,
    /// 上次展示弹窗时的 `GetLastInputInfo` 时间戳，用于判断是否应因新输入而隐藏。
    pub(super) popup_last_show_input_tick: std::sync::Arc<std::sync::atomic::AtomicU32>,
    /// 任务栏弹窗是否启用毛玻璃类效果。
    pub(super) popup_vibrancy_enabled: bool,
    /// 任务栏弹窗使用的视觉效果键名。
    pub(super) popup_vibrancy_effect: Option<String>,
    /// 桌面日历是否启用毛玻璃类效果。
    pub(super) desktop_vibrancy_enabled: bool,
    /// 桌面日历使用的视觉效果键名。
    pub(super) desktop_vibrancy_effect: Option<String>,
}

/// 任务栏弹窗合成时的默认浅色 tint（RGBA）。
const POPUP_WINDOWS_VIBRANCY_TINT: (u8, u8, u8, u8) = (245, 245, 245, 32);
/// 桌面日历窗口合成时的默认浅色 tint（RGBA）。
const DESKTOP_WINDOWS_VIBRANCY_TINT: (u8, u8, u8, u8) = (245, 245, 245, 12);

pub(super) fn get_window_hwnd(window: &WebviewWindow) -> Option<HWND> {
    match window.window_handle() {
        Ok(handle) => match handle.as_raw() {
            RawWindowHandle::Win32(window_handle) => {
                Some(HWND(window_handle.hwnd.get() as *mut std::ffi::c_void))
            }
            _ => None,
        },
        Err(_) => None,
    }
}

pub(super) fn refresh_shared_frameless_window(window: &WebviewWindow, always_on_top: bool) {
    let _ = window.set_decorations(false);
    let _ = window.set_resizable(false);
    let _ = window.set_skip_taskbar(true);
    let _ = window.set_always_on_top(always_on_top);
    let _ = window.set_shadow(false);
}

impl CalendarWindowManager {
    fn build_windows_effects_config(
        effect: Option<&str>,
        tint: (u8, u8, u8, u8),
    ) -> tauri::utils::config::WindowEffectsConfig {
        let effect = match effect.unwrap_or("acrylic") {
            "blur" => Effect::Blur,
            "mica" => Effect::Mica,
            "mica-dark" => Effect::MicaDark,
            "mica-light" => Effect::MicaLight,
            "tabbed" => Effect::Tabbed,
            "tabbed-dark" => Effect::TabbedDark,
            "tabbed-light" => Effect::TabbedLight,
            "acrylic" | "vibrancy" | "liquid-glass" => Effect::Acrylic,
            _ => Effect::Acrylic,
        };

        EffectsBuilder::new().effect(effect).color(Color(tint.0, tint.1, tint.2, tint.3)).build()
    }

    fn apply_native_window_vibrancy(
        window: &WebviewWindow,
        effect: Option<&str>,
        tint: (u8, u8, u8, u8),
    ) -> Result<(), String> {
        let effects = Self::build_windows_effects_config(effect, tint);
        window.set_effects(Some(effects)).map_err(|error| error.to_string())
    }

    /// 创建 Windows 平台窗口管理器并初始化各类窗口引用状态。
    pub fn new(app_handle: &AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            app_handle: app_handle.clone(),
            taskbar_popup_window: None,
            desktop_widget_window: None,
            overlay_text_window: None,
            pending_popup_click_position: None,
            is_pinned: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            suppress_popup_auto_hide: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(
                false,
            )),
            popup_last_show_input_tick: std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0)),
            popup_vibrancy_enabled: false,
            popup_vibrancy_effect: None,
            desktop_vibrancy_enabled: false,
            desktop_vibrancy_effect: None,
        })
    }

    /// 注册叠加层窗口实例，供后续文本更新等操作使用。
    pub fn set_overlay_window(&mut self, overlay_window: WebviewWindow) {
        self.overlay_text_window = Some(overlay_window);
    }

    /// 向叠加层窗口发送最新文本内容。
    pub fn update_overlay_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        crate::window_manager::shared::overlay::update_overlay_text(&self.overlay_text_window, text)
    }

    /// 返回并发 build 弹窗所需的三个 Arc（`is_pinned`、`suppress_auto_hide`、`last_tick`）。
    /// 供 startup 在锁外调用 `build_popup_window` 时使用。
    pub fn popup_build_arcs(
        &self,
    ) -> (
        std::sync::Arc<std::sync::atomic::AtomicBool>,
        std::sync::Arc<std::sync::atomic::AtomicBool>,
        std::sync::Arc<std::sync::atomic::AtomicU32>,
    ) {
        (
            self.is_pinned.clone(),
            self.suppress_popup_auto_hide.clone(),
            self.popup_last_show_input_tick.clone(),
        )
    }

    pub fn set_popup_vibrancy_config(&mut self, enabled: bool, effect: Option<String>) {
        self.popup_vibrancy_enabled = enabled;
        self.popup_vibrancy_effect = effect;
    }

    pub fn set_desktop_vibrancy_config(&mut self, enabled: bool, effect: Option<String>) {
        self.desktop_vibrancy_enabled = enabled;
        self.desktop_vibrancy_effect = effect;
    }

    pub fn clear_window_vibrancy(window: &WebviewWindow) {
        let _ = window.set_effects(Option::<tauri::utils::config::WindowEffectsConfig>::None);
    }

    pub fn apply_window_vibrancy(
        window: &WebviewWindow,
        effect: Option<&str>,
    ) -> Result<(), String> {
        Self::apply_native_window_vibrancy(window, effect, POPUP_WINDOWS_VIBRANCY_TINT)?;
        Ok(())
    }

    pub fn apply_desktop_window_vibrancy(
        window: &WebviewWindow,
        effect: Option<&str>,
    ) -> Result<(), String> {
        Self::apply_native_window_vibrancy(window, effect, DESKTOP_WINDOWS_VIBRANCY_TINT)?;
        Ok(())
    }
}
