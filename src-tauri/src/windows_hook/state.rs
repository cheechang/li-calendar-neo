//! 钩子、自定义任务栏文案等跨模块共享的全局状态。

use once_cell::sync::Lazy;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::mpsc;
use windows::Win32::Foundation::RECT;

use super::types::ClickEvent;

/// 全局低级钩子句柄。
pub static HOOK_HANDLE: Lazy<Arc<Mutex<Option<isize>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

/// 用于防止右键菜单重复弹出的原子锁。
pub static IS_MENU_OPEN: AtomicBool = AtomicBool::new(false);

/// 钩子线程向异步运行时发送点击事件的发送端（可选）。
pub static EVENT_SENDER: Lazy<Arc<Mutex<Option<mpsc::UnboundedSender<ClickEvent>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

/// 是否启用「自定义任务栏时钟文案」功能（为真时才写注册表）。
pub static CUSTOM_CLOCK_ENABLED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
/// 用户编辑的多行时钟显示文本（时间与日期分行）。
pub static CUSTOM_CLOCK_TEXT: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
/// 最近一次解析出的「时间 + 换行 + 日期」缓存，减少重复读注册表。
pub static CLOCK_TEXT_CACHE: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
/// 是否启用任务栏日历替换（为假时钩子直接放行所有消息）。
pub static TASKBAR_WIDGET_ENABLED: AtomicBool = AtomicBool::new(false);

/// 任务栏时钟区域矩形缓存（钩子初始化或应用/恢复自定义时钟后 UIA 写入；[`super::clock_window::is_mouse_in_clock_area`] 只读）。
///
/// 绝不能在 `WH_MOUSE_LL` 回调里调用 UIA，否则会造成全系统输入卡顿。
pub static CLOCK_AREA_RECT_CACHE: Lazy<RwLock<Option<RECT>>> =
    Lazy::new(|| RwLock::new(None));
