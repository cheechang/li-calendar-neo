//! macOS 菜单栏托盘：日期图标、标题模板与点击切换弹窗。
use crate::app_runtime::macos::tray_bar_icon::MacosTrayBarIcon;
use crate::app_runtime::macos::tray_calendar_icon::{calendar_tray_image, TrayDateIconStyle};
use crate::app_runtime::macos::tray_lunarbar_calendar_icon::apply_native_lunarbar_calendar_symbol_to_status_item;
use crate::app_runtime::shared::tray::{normalize_click_position, toggle_popup_by_click};
use crate::app_runtime::tray_icon_px::TrayIconPx;
use crate::window_manager::shared::popup_manager::PopupManager;
use crate::AppState;
use chrono::{Datelike, Local};
use std::sync::{Arc, Mutex};
use tauri::tray::{
    MouseButton as TrayMouseButton, MouseButtonState as TrayMouseButtonState, TrayIcon,
    TrayIconBuilder, TrayIconEvent,
};
use tauri::{AppHandle, State};

const MACOS_TRAY_ID: &str = "calendar-tray";
const STARTUP_POPUP_DELAY_MS: u64 = 180;
const DATE_TITLE_REFRESH_INTERVAL_SECS: u64 = 30;
pub const DEFAULT_TRAY_TITLE_TEMPLATE: &str = "";

fn normalize_tray_title_template(template: &str) -> String {
    template.trim().to_string()
}

fn current_date_tooltip() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn read_tray_title_template(state: &State<'_, AppState>) -> String {
    state
        .macos_tray_title_template
        .lock()
        .map(|template| template.clone())
        .unwrap_or_else(|_| "".to_string())
}

fn read_tray_date_icon_style(state: &State<'_, AppState>) -> TrayDateIconStyle {
    state.macos_tray_date_icon_style.lock().map(|style| *style).unwrap_or_default()
}

fn read_tray_icon_px(state: &State<'_, AppState>) -> TrayIconPx {
    state.macos_tray_icon_px.lock().map(|px| *px).unwrap_or_default()
}

fn read_tray_bar_icon(state: &State<'_, AppState>) -> MacosTrayBarIcon {
    state.macos_tray_bar_icon.lock().map(|b| *b).unwrap_or_default()
}

pub fn set_tray_bar_icon(state: &State<'_, AppState>, icon: MacosTrayBarIcon) {
    if let Ok(mut g) = state.macos_tray_bar_icon.lock() {
        *g = icon;
    }
}

fn tray_date_image(
    day: u32,
    icon_style: TrayDateIconStyle,
    icon_px: TrayIconPx,
) -> tauri::image::Image<'static> {
    calendar_tray_image(day, icon_style, icon_px)
}

/// 日期数字与 LunarBar「日历」模式均为模板图，由系统按浅色/深色菜单栏着色。
fn tray_icon_as_template(_bar_icon: MacosTrayBarIcon) -> bool {
    true
}

pub fn set_tray_date_icon_style(state: &State<'_, AppState>, style: TrayDateIconStyle) {
    if let Ok(mut s) = state.macos_tray_date_icon_style.lock() {
        *s = style;
    }
}

pub fn set_tray_icon_px(state: &State<'_, AppState>, px: TrayIconPx) {
    if let Ok(mut g) = state.macos_tray_icon_px.lock() {
        *g = px;
    }
}

/// 将模板中的占位符替换为当前日期；模板中可直接写 emoji（如 🐿），原样保留。
fn format_current_date_title(template: &str) -> String {
    let now = Local::now();
    let normalized = normalize_tray_title_template(template);

    normalized
        .replace("{YYYY}", &now.format("%Y").to_string())
        .replace("{MM}", &now.format("%m").to_string())
        .replace("{M}", &now.month().to_string())
        .replace("{DD}", &now.format("%d").to_string())
        .replace("{D}", &now.day().to_string())
        .replace("{dddd}", &now.format("%A").to_string())
        .replace("{ddd}", &now.format("%a").to_string())
}

pub fn set_tray_title_template(state: &State<'_, AppState>, template: &str) {
    if let Ok(mut state_template) = state.macos_tray_title_template.lock() {
        *state_template = normalize_tray_title_template(template);
    }
}

fn try_apply_native_calendar_icon(tray_icon: &TrayIcon) -> Result<bool, String> {
    tray_icon
        .with_inner_tray_icon(|inner| {
            inner
                .ns_status_item()
                .map(|status_item| {
                    apply_native_lunarbar_calendar_symbol_to_status_item(
                        objc2::rc::Retained::as_ptr(&status_item) as _,
                    )
                })
                .unwrap_or(false)
        })
        .map_err(|error| error.to_string())
}

fn apply_tray_bar_icon_and_title(
    tray_icon: &TrayIcon,
    template: &str,
    day: u32,
    icon_style: TrayDateIconStyle,
    icon_px: TrayIconPx,
    bar_icon: MacosTrayBarIcon,
) -> Result<(), String> {
    match bar_icon {
        MacosTrayBarIcon::Calendar => {
            tray_icon.set_icon(None).map_err(|error| error.to_string())?;
            if !try_apply_native_calendar_icon(tray_icon)? {
                let img = tray_date_image(day, icon_style, icon_px);
                tray_icon.set_icon(Some(img)).map_err(|error| error.to_string())?;
                tray_icon
                    .set_icon_as_template(tray_icon_as_template(bar_icon))
                    .map_err(|error| error.to_string())?;
            }
        }
        MacosTrayBarIcon::Date => {
            let img = tray_date_image(day, icon_style, icon_px);
            tray_icon.set_icon(Some(img)).map_err(|error| error.to_string())?;
            tray_icon
                .set_icon_as_template(tray_icon_as_template(bar_icon))
                .map_err(|error| error.to_string())?;
        }
    }
    if template.is_empty() {
        tray_icon.set_title(Some(String::new())).map_err(|error| error.to_string())?;
    } else {
        let title = format_current_date_title(template);
        tray_icon.set_title(Some(title)).map_err(|error| error.to_string())?;
    }
    tray_icon.set_tooltip(Some(current_date_tooltip())).map_err(|error| error.to_string())
}

pub fn refresh_tray_title(
    app_handle: &AppHandle,
    state: &State<'_, AppState>,
) -> Result<(), String> {
    let Some(tray_icon) = app_handle.tray_by_id(MACOS_TRAY_ID) else {
        return Ok(());
    };

    let template = read_tray_title_template(state);
    let day = Local::now().day();

    let icon_style = read_tray_date_icon_style(state);
    let icon_px = read_tray_icon_px(state);
    let bar_icon = read_tray_bar_icon(state);
    apply_tray_bar_icon_and_title(&tray_icon, &template, day, icon_style, icon_px, bar_icon)
}

fn start_tray_calendar_icon_updater(
    tray_icon: TrayIcon,
    shared_template: Arc<Mutex<String>>,
    shared_icon_style: Arc<Mutex<TrayDateIconStyle>>,
    shared_icon_px: Arc<Mutex<TrayIconPx>>,
    shared_bar_icon: Arc<Mutex<MacosTrayBarIcon>>,
) {
    std::thread::spawn(move || {
        let mut last_day = Local::now().day();

        loop {
            std::thread::sleep(std::time::Duration::from_secs(DATE_TITLE_REFRESH_INTERVAL_SECS));

            let current_day = Local::now().day();
            if current_day == last_day {
                continue;
            }

            let template = shared_template
                .lock()
                .map(|template| template.clone())
                .unwrap_or_else(|_| "".to_string());

            let icon_style = shared_icon_style.lock().map(|s| *s).unwrap_or_default();

            let icon_px = shared_icon_px.lock().map(|p| *p).unwrap_or_default();
            let bar_icon = shared_bar_icon.lock().map(|b| *b).unwrap_or_default();

            let _ = apply_tray_bar_icon_and_title(
                &tray_icon,
                &template,
                current_day,
                icon_style,
                icon_px,
                bar_icon,
            );
            last_day = current_day;
        }
    });
}

/// 创建 macOS 托盘图标并绑定左键弹窗切换行为。
pub fn setup_macos_tray(app_handle: &tauri::AppHandle, state: &State<'_, AppState>) {
    let shared_window_manager = state.window_manager.clone();
    let shared_template = state.macos_tray_title_template.clone();
    let shared_icon_style = state.macos_tray_date_icon_style.clone();
    let shared_icon_px = state.macos_tray_icon_px.clone();
    let shared_bar_icon = state.macos_tray_bar_icon.clone();
    let initial_day = Local::now().day();
    let template = read_tray_title_template(state);
    let initial_style = read_tray_date_icon_style(state);
    let initial_px = read_tray_icon_px(state);
    let initial_bar_icon = read_tray_bar_icon(state);

    if let Ok(tray_menu) = crate::menu::build_tray_menu(app_handle) {
        let mut builder = TrayIconBuilder::with_id(MACOS_TRAY_ID)
            .tooltip(current_date_tooltip())
            .menu(&tray_menu)
            .show_menu_on_left_click(false)
            .on_tray_icon_event(move |_tray: &TrayIcon, event| {
                if let TrayIconEvent::Click { button, button_state, rect, .. } = event {
                    if button == TrayMouseButton::Left && button_state == TrayMouseButtonState::Up {
                        let (click_x, click_y) = normalize_click_position(rect.position);
                        let _ = toggle_popup_by_click(&shared_window_manager, click_x, click_y);
                    }
                }
            });

        builder = builder
            .icon(tray_date_image(initial_day, initial_style, initial_px))
            .icon_as_template(tray_icon_as_template(initial_bar_icon));
        if template.is_empty() {
            builder = builder.title("");
        } else {
            builder = builder.title(format_current_date_title(&template));
        }

        if let Ok(tray_icon) = builder.build(app_handle) {
            let _ = apply_tray_bar_icon_and_title(
                &tray_icon,
                &template,
                initial_day,
                initial_style,
                initial_px,
                initial_bar_icon,
            );
            start_tray_calendar_icon_updater(
                tray_icon,
                shared_template,
                shared_icon_style,
                shared_icon_px,
                shared_bar_icon,
            );
        }
    }
}

pub fn schedule_startup_popup(app_handle: AppHandle, state: &State<'_, AppState>) {
    const SHOW_STARTUP_POPUP: bool = false;
    if !SHOW_STARTUP_POPUP {
        return;
    }
    let shared_window_manager = state.window_manager.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(STARTUP_POPUP_DELAY_MS));
        let main_thread_app_handle = app_handle.clone();
        let _ = app_handle.run_on_main_thread(move || {
            let did_show_popup = main_thread_app_handle
                .tray_by_id(MACOS_TRAY_ID)
                .and_then(|tray_icon| tray_icon.rect().ok().flatten())
                .map(|rect| {
                    let (click_x, click_y) = normalize_click_position(rect.position);
                    toggle_popup_by_click(&shared_window_manager, click_x, click_y)
                })
                .unwrap_or(false);

            if !did_show_popup {
                if let Ok(mut window_manager_guard) = shared_window_manager.lock() {
                    if let Some(calendar_window_manager) = window_manager_guard.as_mut() {
                        if let Err(error) = calendar_window_manager.show_popup_near_clock() {
                            eprintln!("macOS 启动时展示日历弹窗失败: {}", error);
                        }
                    }
                }
            }
        });
    });
}
