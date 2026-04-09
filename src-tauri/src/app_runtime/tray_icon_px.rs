//! 菜单栏日期图标像素尺寸（macOS 托盘模板图），供前端配置与跨平台命令签名共用。

use serde::{Deserialize, Serialize};

/// 默认：与 LunarBar 的 21×15 点图一致，并匹配 tray 层将整图压到约 **18pt 高** 的槽位。
/// 画布为 **21×18** 点 × @2×（42×36 像素）：其中 21×15 区域绘日期图，上下留白，使屏幕上图形高度与 LunarBar 同为 **15pt**。
pub const DEFAULT_TRAY_ICON_WIDTH_PX: u32 = 42;
pub const DEFAULT_TRAY_ICON_HEIGHT_PX: u32 = 36;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrayIconPx {
    /// 托盘位图宽度（像素）。
    pub width: u32,
    /// 托盘位图高度（像素）。
    pub height: u32,
}

impl Default for TrayIconPx {
    fn default() -> Self {
        Self { width: DEFAULT_TRAY_ICON_WIDTH_PX, height: DEFAULT_TRAY_ICON_HEIGHT_PX }
    }
}

#[cfg(target_os = "macos")]
impl TrayIconPx {
    const MIN: u32 = 16;
    const MAX: u32 = 128;

    pub fn sanitize(width: u32, height: u32) -> Self {
        Self {
            width: width.clamp(Self::MIN, Self::MAX),
            height: height.clamp(Self::MIN, Self::MAX),
        }
    }

    pub fn from_optional(width: Option<u32>, height: Option<u32>) -> Self {
        match (width, height) {
            (Some(w), Some(h)) => Self::sanitize(w, h),
            _ => Self::default(),
        }
    }
}
