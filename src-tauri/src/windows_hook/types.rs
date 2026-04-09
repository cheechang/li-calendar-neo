//! 鼠标钩子向异步运行时投递的点击事件类型。

/// 鼠标按键枚举（序列化供前端或日志使用）。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum MouseButton {
    /// 鼠标左键。
    Left,
    /// 鼠标右键。
    Right,
}

/// 低级鼠标钩子解析后投递到异步运行时的一条点击记录。
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClickEvent {
    /// 点击点的屏幕横坐标。
    pub x: i32,
    /// 点击点的屏幕纵坐标。
    pub y: i32,
    /// 是否落在缓存的时钟矩形内。
    pub in_clock_area: bool,
    /// 触发事件的鼠标按键。
    pub button: MouseButton,
}
