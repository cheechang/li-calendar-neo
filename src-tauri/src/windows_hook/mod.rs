//! Windows 任务栏时钟区域：低级鼠标钩子、注册表时间格式、高亮覆盖层等。
//!
//! - `types`：钩子投递的事件类型
//! - `state`：跨模块共享的全局静态
//! - `window_utils`：窗口类名与桌面前台检测
//! - `clock_window`：时钟 HWND 查找与 UIA 矩形
//! - `registry_clock`：国际化注册表与自定义时钟文案
//! - `mouse_hook`：低级鼠标钩子与 `WindowsHookManager`

mod clock_window;
mod mouse_hook;
mod registry_clock;
mod state;
mod types;
mod window_utils;

// ---- 对外 API（与拆分前 `windows_hook` 根模块保持一致）----

/// 主任务栏几何信息与是否横向；供弹窗定位使用。
pub use clock_window::get_taskbar_info;
pub(crate) use clock_window::refresh_clock_area_cache;
/// 任务栏组件开关、钩子消息泵线程、钩子管理器。
pub use mouse_hook::{set_taskbar_widget_enabled, start_hook_message_thread, WindowsHookManager};
/// 注册表中的时间/日期格式与自定义任务栏文案。
pub use registry_clock::{
    disable_custom_clock, get_current_system_time_format, set_custom_clock_text,
};
/// 任务栏右键菜单是否正在显示（防重复弹出）。
pub use state::IS_MENU_OPEN;
/// 钩子向 Tokio 侧发送的点击事件与按键枚举。
pub use types::{ClickEvent, MouseButton};
/// 桌面前台检测（用于检测 WIN+D 后桌面显示状态）。
pub use window_utils::is_desktop_in_foreground;
