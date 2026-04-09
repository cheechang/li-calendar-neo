//! 低级鼠标钩子安装、消息泵线程与点击事件投递。

use std::ffi::c_void;
use std::sync::atomic::Ordering;
use std::thread;
use tokio::sync::mpsc;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use super::clock_window::{find_clock_window, is_mouse_in_clock_area, update_clock_area_cache};
use super::registry_clock::disable_custom_clock;
use super::state::{EVENT_SENDER, HOOK_HANDLE, IS_MENU_OPEN, TASKBAR_WIDGET_ENABLED};
use super::types::{ClickEvent, MouseButton};
use super::window_utils::is_foreground_fullscreen;

/// 封装低级鼠标钩子的安装与从 Tokio 侧消费点击事件的通道。
pub struct WindowsHookManager {
    /// 从钩子线程接收点击事件的异步接收端（仅 [`Self::take_event_receiver`] 可取走一次）。
    event_receiver: Option<mpsc::UnboundedReceiver<ClickEvent>>,
}

/// 设置是否启用任务栏日历组件；关闭时会清理自定义时钟。
///
/// * `enabled` - 为真时安装钩子并允许拦截；为假时清空事件通道并恢复系统时钟。
pub fn set_taskbar_widget_enabled(enabled: bool) {
    TASKBAR_WIDGET_ENABLED.store(enabled, Ordering::SeqCst);
    if !enabled {
        if let Ok(mut global_sender) = EVENT_SENDER.lock() {
            *global_sender = None;
        }
        let _ = disable_custom_clock();
    }
}

impl WindowsHookManager {
    /// 创建管理器并注册全局点击事件发送端。
    pub fn new() -> Self {
        // `sender` 交给全局 `EVENT_SENDER`，供 `mouse_hook_proc` 投递；`receiver` 由本结构体持有
        let (sender, receiver) = mpsc::unbounded_channel();

        if let Ok(mut global_sender) = EVENT_SENDER.lock() {
            *global_sender = Some(sender);
        }

        Self { event_receiver: Some(receiver) }
    }

    /// 在当前模块句柄上安装 `WH_MOUSE_LL` 低级鼠标钩子。
    pub fn install_mouse_hook(&self) -> Result<()> {
        unsafe {
            // 当前模块实例，用于 `SetWindowsHookExW` 的 `hmod`（与 `mouse_hook_proc` 同模块）
            let module_handle = GetModuleHandleW(None)?;

            let hook = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(mouse_hook_proc),
                Some(HINSTANCE(module_handle.0)),
                0,
            )?;

            if let Ok(mut handle) = HOOK_HANDLE.lock() {
                *handle = Some(hook.0 as isize);
            }

            println!("Windows 鼠标钩子安装成功");
            Ok(())
        }
    }

    /// 卸载全局鼠标钩子并释放句柄。
    pub fn uninstall_hook(&self) -> Result<()> {
        unsafe {
            if let Ok(mut handle) = HOOK_HANDLE.lock() {
                if let Some(hook) = handle.take() {
                    UnhookWindowsHookEx(HHOOK(hook as *mut c_void))?;
                    println!("Windows 鼠标钩子已卸载");
                }
            }
            Ok(())
        }
    }

    /// 取走点击事件接收端（仅可调用一次，供异步任务消费）。
    pub fn take_event_receiver(&mut self) -> Option<mpsc::UnboundedReceiver<ClickEvent>> {
        self.event_receiver.take()
    }

    /// 查找任务栏时钟子窗口句柄。
    pub fn find_clock_window(&self) -> Option<HWND> {
        find_clock_window()
    }
}

impl Drop for WindowsHookManager {
    fn drop(&mut self) {
        // 释放时卸载钩子并尝试恢复系统默认时钟，避免残留注册表覆盖
        let _ = self.uninstall_hook();
        let _ = disable_custom_clock();
    }
}

/// 在独立线程中安装钩子并运行 `GetMessageW` 消息泵（避免阻塞主线程）。
pub fn start_hook_message_thread() {
    thread::spawn(|| {
        unsafe {
            // UIA 取时钟矩形需要 COM，与钩子同线程（低级钩子在安装线程上回调）
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            // 独立线程中挂载钩子并运行 `GetMessageW` 循环，满足低级钩子的消息泵要求
            if let Ok(module_handle) = GetModuleHandleW(None) {
                match SetWindowsHookExW(
                    WH_MOUSE_LL,
                    Some(mouse_hook_proc),
                    Some(HINSTANCE(module_handle.0)),
                    0,
                ) {
                    Ok(hook) => {
                        if let Ok(mut handle) = HOOK_HANDLE.lock() {
                            *handle = Some(hook.0 as isize);
                        }
                        println!("✅ 已在专用消息泵线程上安装鼠标钩子");
                        // 初始化时做一次 UIA 取矩形；钩子回调内只读缓存，不得在此线程之外重复轮询刷新。
                        update_clock_area_cache();
                    }
                    Err(e) => {
                        eprintln!("❌ 安装鼠标钩子失败: {:?}", e);
                        return;
                    }
                }
            } else {
                eprintln!("❌ 钩子线程无法获取模块句柄");
                return;
            }

            let mut msg = MSG::default();
            // 阻塞式消息泵；仅当线程收到 `WM_QUIT` 时结束（本流程通常长期运行）
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    });
}

/// 低级鼠标钩子过程：在任务栏时钟区域内吞掉按键并投递 [`ClickEvent`]。
///
/// * `code` - 钩子代码；`<0` 时必须转发。
/// * `wparam` - 鼠标消息 ID（如 `WM_LBUTTONDOWN`）。
/// * `lparam` - 指向 [`MSLLHOOKSTRUCT`] 的指针。
unsafe extern "system" fn mouse_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if !TASKBAR_WIDGET_ENABLED.load(Ordering::SeqCst) {
        return unsafe { CallNextHookEx(None, code, wparam, lparam) };
    }

    // 当前鼠标消息类型（来自 `wparam`）
    let msg = wparam.0 as u32;

    if code < 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    let is_down = msg == WM_LBUTTONDOWN || msg == WM_RBUTTONDOWN;
    let is_up = msg == WM_LBUTTONUP || msg == WM_RBUTTONUP;

    if !is_down && !is_up {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    let mouse_struct = *(lparam.0 as *const MSLLHOOKSTRUCT);
    let x = mouse_struct.pt.x;
    let y = mouse_struct.pt.y;

    if is_mouse_in_clock_area(x, y) {
        // 全屏前台（游戏/全屏视频等）时不拦截，避免吞掉本应交给前台的鼠标消息
        if is_foreground_fullscreen() {
            return CallNextHookEx(None, code, wparam, lparam);
        }
        if is_down {
            let mouse_button =
                if msg == WM_LBUTTONDOWN { MouseButton::Left } else { MouseButton::Right };

            // 与任务栏右键菜单配合：菜单已打开时不再重复投递右键按下
            if mouse_button == MouseButton::Right && IS_MENU_OPEN.load(Ordering::SeqCst) {
                return LRESULT(1);
            }

            if mouse_button == MouseButton::Right {
                unsafe {
                    // 取消模式并伪造时钟区 `WM_MOUSELEAVE`（数值 675），收起系统 Tooltip
                    if let Ok(shell_tray) = FindWindowW(w!("Shell_TrayWnd"), None) {
                        let _ = PostMessageW(Some(shell_tray), WM_CANCELMODE, WPARAM(0), LPARAM(0));
                    }
                    if let Some(clock_hwnd) = find_clock_window() {
                        let _ = PostMessageW(Some(clock_hwnd), 675, WPARAM(0), LPARAM(0));
                    }
                }
            }

            if let Ok(sender_guard) = EVENT_SENDER.lock() {
                if let Some(sender) = sender_guard.as_ref() {
                    let event = ClickEvent { x, y, in_clock_area: true, button: mouse_button };
                    let _ = sender.send(event);
                }
            }
        }

        // 在时钟区域内吞掉消息，阻止系统弹出原生任务栏右键菜单
        return LRESULT(1);
    }

    CallNextHookEx(None, code, wparam, lparam)
}
