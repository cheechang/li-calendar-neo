//! Windows 启动阶段：初始化 `CalendarWindowManager` 与首次弹窗展示策略。
use super::hook_runtime::start_taskbar_runtime;
use crate::window_manager::CalendarWindowManager;
use crate::AppState;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, State};

/// 根据功能开关初始化 Windows 端窗口管理器实例。
pub fn initialize_window_manager_for_windows(
    app_handle: &AppHandle,
    state: &State<'_, AppState>,
) -> Result<(), String> {
    let calendar_window_manager = CalendarWindowManager::new(app_handle)
        .map_err(|error: Box<dyn std::error::Error>| error.to_string())?;
    if let Ok(mut window_manager_guard) = state.window_manager.lock() {
        *window_manager_guard = Some(calendar_window_manager);
    }
    Ok(())
}

/// 启动桌面组件与任务栏/托盘弹窗。
///
/// 仅当「桌面 + 启动弹窗」同时需要时，在同一后台线程内**顺序**创建两个 Webview（先桌面后弹窗），
/// 避免 WebView2 并发初始化与 `popup_ready` 过早 `show`+`set_focus` 导致桌面组件无法显示。
/// 其余情况仍采用「锁外 build、锁内 attach」；仅桌面或仅弹窗时与原先一致。
pub fn spawn_windows_concurrently(
    app_handle: AppHandle,
    state: &State<'_, AppState>,
    should_show_startup_popup: bool,
    desktop_widget_enabled: bool,
    taskbar_widget_enabled: bool,
    desktop_initial_pos: Option<(i32, i32)>,
) {
    // ── 第一步：锁内取出 build 弹窗所需的 Arc 字段（极短暂持锁）─────────
    // 无论是否开启"替换任务栏日历"，只要 should_show_startup_popup 为真就需要弹窗
    let popup_arcs = if should_show_startup_popup {
        state
            .window_manager
            .lock()
            .ok()
            .and_then(|guard| guard.as_ref().map(|wm| wm.popup_build_arcs()))
    } else {
        None
    };

    // ── 同时需要桌面 + 启动弹窗：单线程顺序 build，避免与桌面并发冲突 ──
    if desktop_widget_enabled && should_show_startup_popup {
        if let Some((is_pinned, suppress_auto_hide, last_tick)) = popup_arcs {
            state.desktop_init_started.store(true, Ordering::SeqCst);
            let app_handle_seq = app_handle.clone();
            let wm_arc = state.window_manager.clone();
            let tray_pos = if !taskbar_widget_enabled {
                app_handle
                    .tray_by_id(super::tray::WINDOWS_TRAY_ID)
                    .and_then(|t| t.rect().ok().flatten())
                    .map(|rect| {
                        crate::app_runtime::shared::tray::normalize_click_position(rect.position)
                    })
            } else {
                None
            };

            if taskbar_widget_enabled {
                start_taskbar_runtime(app_handle.clone(), state);
            }

            std::thread::Builder::new()
                .name("desktop-then-popup-init".into())
                .spawn(move || {
                    match CalendarWindowManager::build_desktop_window(
                        &app_handle_seq,
                        desktop_initial_pos,
                    ) {
                        Ok(window) => {
                            if let Ok(mut guard) = wm_arc.lock() {
                                if let Some(wm) = guard.as_mut() {
                                    wm.attach_desktop_window(window);
                                }
                            }
                        }
                        Err(e) => eprintln!("创建桌面日历窗口失败: {e}"),
                    }

                    let Some(popup_window) = CalendarWindowManager::build_popup_window(
                        &app_handle_seq,
                        is_pinned,
                        suppress_auto_hide,
                        last_tick,
                    ) else {
                        return;
                    };
                    if let Ok(mut guard) = wm_arc.lock() {
                        if let Some(wm) = guard.as_mut() {
                            wm.attach_popup_window(popup_window);
                            wm.set_startup_popup_persistence(true);
                            if let Some((x, y)) = tray_pos {
                                wm.queue_show_at_position_on_ready(x, y);
                            } else {
                                wm.queue_show_near_clock_on_ready();
                            }
                        }
                    }
                })
                .ok();
            return;
        }
    }

    // ── 仅桌面组件 ─────────────────────────────────────────────────────
    if desktop_widget_enabled {
        state.desktop_init_started.store(true, Ordering::SeqCst);
        let app_handle_d = app_handle.clone();
        let wm_arc_d = state.window_manager.clone();
        std::thread::Builder::new()
            .name("desktop-widget-init".into())
            .spawn(move || {
                match CalendarWindowManager::build_desktop_window(
                    &app_handle_d,
                    desktop_initial_pos,
                ) {
                    Ok(window) => {
                        if let Ok(mut guard) = wm_arc_d.lock() {
                            if let Some(wm) = guard.as_mut() {
                                wm.attach_desktop_window(window);
                            }
                        }
                    }
                    Err(e) => eprintln!("创建桌面日历窗口失败: {e}"),
                }
            })
            .ok();
    }

    // ── 仅任务栏弹窗（或桌面未启用时的弹窗）────────────────────────────
    if taskbar_widget_enabled {
        // Hook 运行时在当前线程启动
        start_taskbar_runtime(app_handle.clone(), state);

        if !should_show_startup_popup {
            return;
        }

        if let Some((is_pinned, suppress_auto_hide, last_tick)) = popup_arcs {
            let app_handle_p = app_handle.clone();
            let wm_arc_p = state.window_manager.clone();
            std::thread::Builder::new()
                .name("taskbar-popup-init".into())
                .spawn(move || {
                    let Some(popup_window) = CalendarWindowManager::build_popup_window(
                        &app_handle_p,
                        is_pinned,
                        suppress_auto_hide,
                        last_tick,
                    ) else {
                        return;
                    };
                    if let Ok(mut guard) = wm_arc_p.lock() {
                        if let Some(wm) = guard.as_mut() {
                            wm.attach_popup_window(popup_window);
                            wm.set_startup_popup_persistence(true);
                            wm.queue_show_near_clock_on_ready();
                        }
                    }
                })
                .ok();
        }
    } else if should_show_startup_popup {
        // taskbar_widget_enabled=false：不替换任务栏日历，但仍需在托盘图标位置显示弹窗。
        if let Some((is_pinned, suppress_auto_hide, last_tick)) = popup_arcs {
            let app_handle_p = app_handle.clone();
            let wm_arc_p = state.window_manager.clone();
            let tray_pos = app_handle
                .tray_by_id(super::tray::WINDOWS_TRAY_ID)
                .and_then(|t| t.rect().ok().flatten())
                .map(|rect| {
                    crate::app_runtime::shared::tray::normalize_click_position(rect.position)
                });
            std::thread::Builder::new()
                .name("tray-popup-init".into())
                .spawn(move || {
                    let Some(popup_window) = CalendarWindowManager::build_popup_window(
                        &app_handle_p,
                        is_pinned,
                        suppress_auto_hide,
                        last_tick,
                    ) else {
                        return;
                    };
                    if let Ok(mut guard) = wm_arc_p.lock() {
                        if let Some(wm) = guard.as_mut() {
                            wm.attach_popup_window(popup_window);
                            wm.set_startup_popup_persistence(true);
                            if let Some((x, y)) = tray_pos {
                                wm.queue_show_at_position_on_ready(x, y);
                            } else {
                                wm.queue_show_near_clock_on_ready();
                            }
                        }
                    }
                })
                .ok();
        }
    }
}
