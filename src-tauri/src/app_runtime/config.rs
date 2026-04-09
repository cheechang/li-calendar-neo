use serde::Deserialize;
use std::fs;
use tauri::{AppHandle, Manager};

/// 桌面组件窗口的持久化位置（物理像素坐标）。
#[cfg(windows)]
#[derive(Default, Deserialize, Clone)]
pub struct PersistedPosition {
    pub x: i32,
    pub y: i32,
}

/// 自 `liConfig.json` 反序列化的功能开关快照。
#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedFeatureConfig {
    /// 是否启用桌面日历小组件（仅 Windows）。
    #[cfg(windows)]
    pub desktop_widget_enabled: Option<bool>,
    /// 是否启用任务栏日历替换/拦截（仅 Windows）。
    #[cfg(windows)]
    pub taskbar_widget_enabled: Option<bool>,
    /// 桌面日历小组件上次保存的物理像素位置（仅 Windows）。
    #[cfg(windows)]
    pub desktop_window_position: Option<PersistedPosition>,
    /// 菜单栏标题模板字符串（仅 macOS）。
    #[cfg(target_os = "macos")]
    pub macos_tray_title_template: Option<String>,
    /// 菜单栏日期图标样式配置键（仅 macOS）。
    #[cfg(target_os = "macos")]
    pub macos_tray_date_icon_style: Option<String>,
    /// 托盘图标宽度像素（仅 macOS）。
    #[cfg(target_os = "macos")]
    pub macos_tray_icon_width: Option<u32>,
    /// 托盘图标高度像素（仅 macOS）。
    #[cfg(target_os = "macos")]
    pub macos_tray_icon_height: Option<u32>,
    /// 菜单栏主图标类型：`date` / `calendar`（仅 macOS）。
    #[cfg(target_os = "macos")]
    pub macos_tray_bar_icon: Option<String>,
}

/// 从应用配置目录加载持久化功能开关，读取失败时返回默认配置。
pub fn load_persisted_feature_config(app_handle: &AppHandle) -> PersistedFeatureConfig {
    let Some(config_dir) = app_handle.path().app_config_dir().ok() else {
        return PersistedFeatureConfig::default();
    };
    let config_path = config_dir.join("liConfig.json");
    let Ok(content) = fs::read_to_string(config_path) else {
        return PersistedFeatureConfig::default();
    };
    serde_json::from_str::<PersistedFeatureConfig>(&content).unwrap_or_default()
}
