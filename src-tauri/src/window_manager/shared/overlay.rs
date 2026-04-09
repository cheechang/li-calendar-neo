use tauri::{Emitter, WebviewWindow};

/// 向叠加文本 Webview 发送 `overlay_text` 事件。
///
/// * `overlay_text_window` - 叠加层窗口；若为 `None` 则不做任何事。
/// * `text` - 要推送的文本内容。
pub fn update_overlay_text(
    overlay_text_window: &Option<WebviewWindow>,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(overlay_window) = overlay_text_window {
        let _ = overlay_window.emit("overlay_text", text.to_string());
    }
    Ok(())
}
