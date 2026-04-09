//! 任务栏旁日历弹窗：定位、`PopupManager` 实现与失焦隐藏策略。
use super::{get_window_hwnd, refresh_shared_frameless_window, CalendarWindowManager};
use crate::window_manager::shared::popup_manager::PopupManager;
use crate::windows_hook::get_taskbar_info;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};
use tauri::{Manager, WebviewWindow, WindowEvent};
use windows::Win32::{
    Foundation::RECT,
    UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO},
    UI::WindowsAndMessaging::{
        GetAncestor, GetForegroundWindow, GetSystemMetrics, GA_ROOT, SM_CXSCREEN, SM_CYSCREEN,
    },
};

/// 表示“在系统时钟附近展示”的挂起坐标哨兵值。
const PENDING_SHOW_NEAR_CLOCK: (i32, i32) = (99_999, 99_999);
const STARTUP_FOCUS_GRACE_PERIOD: Duration = Duration::from_millis(1500);
impl PopupManager for CalendarWindowManager {
    /// 隐藏任务栏弹窗但保留窗口实例以便快速再次显示。
    fn hide_popup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.suppress_popup_auto_hide.store(false, Ordering::SeqCst);
        if let Some(popup_window) = &self.taskbar_popup_window {
            popup_window.hide()?;
        }
        Ok(())
    }

    /// 检查任务栏弹窗当前是否处于可见状态。
    fn is_popup_visible(&self) -> bool {
        self.taskbar_popup_window.as_ref().map(|w| w.is_visible().unwrap_or(false)).unwrap_or(false)
    }

    /// 在系统时钟附近显示任务栏弹窗，首次创建时先缓存展示意图等待前端就绪。
    fn show_popup_near_clock(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 检查后端是否已存活名为 "calendar" 的窗口
        let has_live_popup_window = self.app_handle.get_webview_window("calendar").is_some();
        if let Some(popup_window) = self.ensure_taskbar_popup_window() {
            // 如果窗口刚被创建且前端尚未就绪，则缓存系统时钟附近展示请求
            if !has_live_popup_window {
                self.pending_popup_click_position = Some(PENDING_SHOW_NEAR_CLOCK);
                println!("日历窗口已创建，等待前端就绪信号");
                return Ok(());
            }

            let popup_window = popup_window.clone();
            // 获取任务栏信息计算位置
            if let Some((taskbar_rect, is_horizontal)) = get_taskbar_info() {
                let (popup_x, popup_y) =
                    self.calculate_window_position(&taskbar_rect, is_horizontal);
                self.present_popup_window(
                    &popup_window,
                    Some(tauri::PhysicalPosition { x: popup_x, y: popup_y }),
                )?;
                println!("日历窗口已显示在位置 ({}, {})", popup_x, popup_y);
            } else {
                self.present_popup_window(&popup_window, None)?;
            }
        }
        Ok(())
    }

    /// 按给定点击坐标显示任务栏弹窗，必要时等待窗口首次初始化完成。
    ///
    /// * `click_x` - 点击的横坐标
    /// * `click_y` - 点击的纵坐标
    fn show_popup_at_position(
        &mut self,
        click_x: i32,
        click_y: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let has_live_popup_window = self.app_handle.get_webview_window("calendar").is_some();
        if let Some(popup_window) = self.ensure_taskbar_popup_window() {
            if !has_live_popup_window {
                // 记录点击坐标，待前端准备好后再展示
                self.pending_popup_click_position = Some((click_x, click_y));
                println!("日历窗口已创建，等待前端就绪后再按坐标显示");
                return Ok(());
            }

            let popup_window = popup_window.clone();
            // 计算安全的窗口显示位置，防止超出屏幕
            let (popup_x, popup_y) = self.calculate_safe_position(click_x, click_y);
            println!("🎯 计算出的窗口位置: ({}, {})", popup_x, popup_y);
            self.present_popup_window(
                &popup_window,
                Some(tauri::PhysicalPosition { x: popup_x, y: popup_y }),
            )?;
            println!(
                "✅ 日历窗口已使用Tauri原生API定位: 点击位置({}, {}) -> 窗口位置: ({}, {})",
                click_x, click_y, popup_x, popup_y
            );
        }
        Ok(())
    }

    /// 在前端弹窗就绪后处理此前缓存的展示请求。
    fn on_popup_ready(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 如果存在挂起的坐标则进行处理
        if let Some((click_x, click_y)) = self.pending_popup_click_position.take() {
            println!("日历前端已就绪，执行挂起的显示请求 ({}, {})", click_x, click_y);
            if (click_x, click_y) == PENDING_SHOW_NEAR_CLOCK {
                self.show_popup_near_clock()?;
            } else {
                self.show_popup_at_position(click_x, click_y)?;
            }
        }
        Ok(())
    }

    /// 设置弹窗是否固定。
    fn set_popup_pin(&mut self, pin: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.is_pinned.store(pin, Ordering::SeqCst);
        Ok(())
    }
}

impl CalendarWindowManager {
    pub fn set_startup_popup_persistence(&self, enabled: bool) {
        self.suppress_popup_auto_hide.store(enabled, Ordering::SeqCst);
    }

    /// 挂起"贴近时钟显示"请求，等前端 `popup_ready` 信号后再执行。
    pub fn queue_show_near_clock_on_ready(&mut self) {
        self.pending_popup_click_position = Some(PENDING_SHOW_NEAR_CLOCK);
    }

    /// 挂起"在指定坐标显示"请求，等前端 `popup_ready` 信号后再执行。
    pub fn queue_show_at_position_on_ready(&mut self, x: i32, y: i32) {
        self.pending_popup_click_position = Some((x, y));
    }

    fn current_last_input_tick() -> Option<u32> {
        let mut last_input_info =
            LASTINPUTINFO { cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };
        unsafe {
            if GetLastInputInfo(&mut last_input_info).as_bool() {
                Some(last_input_info.dwTime)
            } else {
                None
            }
        }
    }

    fn present_popup_window(
        &self,
        popup_window: &WebviewWindow,
        position: Option<tauri::PhysicalPosition<i32>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(position) = position {
            popup_window.set_position(tauri::Position::Physical(position))?;
        } else {
            popup_window.center()?;
        }
        if let Some(last_input_tick) = Self::current_last_input_tick() {
            self.popup_last_show_input_tick.store(last_input_tick, Ordering::SeqCst);
        }
        popup_window.show()?;
        popup_window.set_focus()?;
        Ok(())
    }

    fn reapply_saved_popup_vibrancy(&self, window: &WebviewWindow) {
        if self.popup_vibrancy_enabled {
            let _ = Self::apply_window_vibrancy(window, self.popup_vibrancy_effect.as_deref());
        } else {
            Self::clear_window_vibrancy(window);
        }
    }

    pub fn refresh_popup_chrome(window: &WebviewWindow) {
        refresh_shared_frameless_window(window, true);
    }

    /// 在锁外构建任务栏弹窗并绑定失焦自动隐藏事件，不需要持有 `window_manager` 锁。
    /// 构建完成后调用 `attach_popup_window` 将其存入 manager。
    pub fn build_popup_window(
        app_handle: &tauri::AppHandle,
        is_pinned: Arc<AtomicBool>,
        suppress_popup_auto_hide: Arc<AtomicBool>,
        popup_last_show_input_tick: Arc<std::sync::atomic::AtomicU32>,
    ) -> Option<WebviewWindow> {
        let popup_window = tauri::WebviewWindowBuilder::new(
            app_handle,
            "calendar",
            tauri::WebviewUrl::App("index.html?window=popup".into()),
        )
        .title("松鼠日历")
        .inner_size(360.0, 520.0)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .visible(false)
        .focused(true)
        .skip_taskbar(true)
        .build()
        .ok()?;

        let popup_window_handle = popup_window.clone();
        let popup_root_hwnd = get_window_hwnd(&popup_window).map(|hwnd| hwnd.0 as isize);
        let popup_has_gained_focus = Arc::new(AtomicBool::new(false));
        let popup_has_gained_focus_clone = popup_has_gained_focus.clone();
        let ignore_focus_loss_until = Instant::now() + STARTUP_FOCUS_GRACE_PERIOD;
        let app_handle_for_event = app_handle.clone();

        popup_window.on_window_event(move |event| match event {
            WindowEvent::Focused(true) => {
                popup_has_gained_focus_clone.store(true, Ordering::SeqCst);
            }
            WindowEvent::Focused(false) => {
                if !popup_window_handle.is_visible().unwrap_or(false)
                    || Instant::now() < ignore_focus_loss_until
                    || !popup_has_gained_focus_clone.load(Ordering::SeqCst)
                    || is_pinned.load(Ordering::SeqCst)
                {
                    return;
                }

                let last_show_input_tick = popup_last_show_input_tick.load(Ordering::SeqCst);
                let has_new_user_input = CalendarWindowManager::current_last_input_tick()
                    .is_some_and(|current_tick| current_tick != last_show_input_tick);

                let desktop_widget_hwnd = app_handle_for_event
                    .get_webview_window("desktop_calendar")
                    .and_then(|w| get_window_hwnd(&w))
                    .map(|hwnd| hwnd.0 as isize);

                let should_hide = has_new_user_input
                    && popup_root_hwnd.is_some_and(|popup_hwnd| unsafe {
                        let foreground_hwnd = GetForegroundWindow();
                        if foreground_hwnd.0.is_null() {
                            return false;
                        }
                        let foreground_root_hwnd = GetAncestor(foreground_hwnd, GA_ROOT);
                        let foreground_isize = foreground_root_hwnd.0 as isize;
                        if foreground_isize == popup_hwnd {
                            return false;
                        }
                        if desktop_widget_hwnd.is_some_and(|dw| foreground_isize == dw) {
                            return false;
                        }
                        true
                    });

                if should_hide {
                    let _ = suppress_popup_auto_hide.swap(false, Ordering::SeqCst);
                    let _ = popup_window_handle.hide();
                }
            }
            _ => {}
        });

        Some(popup_window)
    }

    /// 将已构建好的弹窗存入 manager 并应用毛玻璃效果（需持锁调用）。
    pub fn attach_popup_window(&mut self, popup_window: WebviewWindow) {
        self.reapply_saved_popup_vibrancy(&popup_window);
        self.taskbar_popup_window = Some(popup_window);
    }

    /// 确保任务栏弹窗存在；若句柄失效则重建，并绑定失焦自动隐藏行为（持锁版）。
    fn ensure_taskbar_popup_window(&mut self) -> Option<&WebviewWindow> {
        // 如果实例存在但 WebView 实际窗口已经销毁，清理实例
        if self.taskbar_popup_window.is_some()
            && self.app_handle.get_webview_window("calendar").is_none()
        {
            self.taskbar_popup_window = None;
        }

        if self.taskbar_popup_window.is_none() {
            let popup_window = Self::build_popup_window(
                &self.app_handle.clone(),
                self.is_pinned.clone(),
                self.suppress_popup_auto_hide.clone(),
                self.popup_last_show_input_tick.clone(),
            )?;
            self.attach_popup_window(popup_window);
        }
        self.taskbar_popup_window.as_ref()
    }

    /// 关闭并释放任务栏弹窗窗口实例。
    pub fn close_calendar_window(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(popup_window) = self.taskbar_popup_window.take() {
            popup_window.close()?;
        }
        Ok(())
    }

    /// 在前端弹窗就绪后处理此前缓存的展示请求。
    pub fn on_calendar_ready(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.on_popup_ready()
    }

    /// 计算弹窗当前应使用的物理尺寸，优先读取窗口真实尺寸。
    fn calendar_physical_size(&self) -> (i32, i32) {
        if let Some(popup_window) = &self.taskbar_popup_window {
            // 尝试获取外部尺寸
            if let Ok(size) = popup_window.outer_size() {
                if size.width > 0 && size.height > 100 {
                    return (size.width as i32, size.height as i32);
                }
            }
            // 退而求其次尝试获取内部尺寸
            if let Ok(size) = popup_window.inner_size() {
                if size.width > 0 && size.height > 100 {
                    return (size.width as i32, size.height as i32);
                }
            }
            // 获取不到则使用逻辑尺寸和缩放因子计算
            let scale = popup_window.scale_factor().unwrap_or(1.0);
            ((360.0 * scale) as i32, (520.0 * scale) as i32)
        } else {
            (360, 520)
        }
    }

    /// 按点击点与屏幕边界规则计算安全弹窗位置。
    fn calculate_safe_position(&self, click_x: i32, click_y: i32) -> (i32, i32) {
        let (window_width, window_height) = self.calendar_physical_size();
        println!("📐 窗口物理尺寸: {}x{}", window_width, window_height);

        let margin = 8;
        let edge_margin = 32;

        unsafe {
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);

            println!("🖥️ 屏幕尺寸: {}x{}", screen_width, screen_height);
            println!("🖱️ 点击位置: ({}, {})", click_x, click_y);

            let (taskbar_info, is_horizontal) =
                if let Some((taskbar_rect, is_horizontal)) = get_taskbar_info() {
                    println!(
                        "🔍 检测到任务栏: 位置({}, {}, {}, {}), 水平: {}",
                        taskbar_rect.left,
                        taskbar_rect.top,
                        taskbar_rect.right,
                        taskbar_rect.bottom,
                        is_horizontal
                    );
                    (Some(taskbar_rect), is_horizontal)
                } else {
                    println!("❌ 无法检测到任务栏");
                    (None, true)
                };

            let mut x = click_x - window_width / 2;
            let mut y = if click_y > screen_height / 2 {
                if let Some(rect) = taskbar_info {
                    if is_horizontal && rect.top > screen_height / 2 {
                        rect.top - window_height - margin
                    } else {
                        click_y - window_height - margin
                    }
                } else {
                    click_y - window_height - margin
                }
            } else if let Some(rect) = taskbar_info {
                if is_horizontal && rect.bottom < screen_height / 2 {
                    rect.bottom + margin
                } else {
                    click_y + margin
                }
            } else {
                click_y + margin
            };

            if x < edge_margin {
                x = edge_margin;
            } else if x + window_width > screen_width - edge_margin {
                x = screen_width - window_width - edge_margin;
            }

            if y < margin {
                y = margin;
            } else if y + window_height > screen_height - margin {
                y = screen_height - window_height - margin;
            }

            println!("🎯 计算窗口位置: ({}, {})", x, y);
            (x, y)
        }
    }

    /// 按任务栏位置计算默认弹出位置，用于贴近系统时钟展示。
    fn calculate_window_position(&self, taskbar_rect: &RECT, is_horizontal: bool) -> (i32, i32) {
        let (window_width, window_height) = self.calendar_physical_size();
        let margin = 12;
        let edge_margin = 24;

        unsafe {
            let screen_width = GetSystemMetrics(SM_CXSCREEN);
            let screen_height = GetSystemMetrics(SM_CYSCREEN);

            if is_horizontal {
                if taskbar_rect.top < screen_height / 2 {
                    let x = screen_width - window_width - edge_margin;
                    let y = taskbar_rect.bottom + margin;
                    (x.clamp(edge_margin, screen_width - window_width - edge_margin), y)
                } else {
                    let x = screen_width - window_width - edge_margin;
                    let y = taskbar_rect.top - window_height - margin;
                    (x.clamp(edge_margin, screen_width - window_width - edge_margin), y.max(0))
                }
            } else if taskbar_rect.left < screen_width / 2 {
                let x = taskbar_rect.right + margin;
                let y = taskbar_rect.bottom - window_height - margin;
                (x, y.max(0))
            } else {
                let x = screen_width
                    - (taskbar_rect.right - taskbar_rect.left)
                    - window_width
                    - edge_margin;
                let y = taskbar_rect.bottom - window_height - margin;
                (x.max(0), y.max(0))
            }
        }
    }
}
