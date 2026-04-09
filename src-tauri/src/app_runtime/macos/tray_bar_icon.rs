//! 与 LunarBar [`MenuBarIcon`](https://github.com/LunarBar-app/LunarBar/blob/main/LunarBarMac/Sources/Shared/AppPreferences.swift) 对应：「日期数字」或「日历」SF Symbol（`createCalendarIcon`）。

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MacosTrayBarIcon {
    /// 日期数字图标（实心/描边样式由 `TrayDateIconStyle` 控制，对应 LunarBar `DateIconView`）。
    #[default]
    Date,
    /// SF Symbol `calendar`（LunarBar 同款约 16pt；此处为模板 PNG 由系统着色）。
    Calendar,
}

impl MacosTrayBarIcon {
    pub fn from_config(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "calendar" => Self::Calendar,
            _ => Self::Date,
        }
    }

    pub fn as_config_str(self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Calendar => "calendar",
        }
    }
}
