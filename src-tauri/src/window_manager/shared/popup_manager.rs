//! 日历弹窗行为抽象，由 Windows / macOS 的 `CalendarWindowManager` 实现。
/// 定义跨平台日历弹窗所需的基础操作。
pub trait PopupManager {
    /// 隐藏任务栏/桌面弹窗但保留窗口实例以便快速再次显示。
    fn hide_popup(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// 检查任务栏/桌面弹窗当前是否处于可见状态。
    fn is_popup_visible(&self) -> bool;

    /// 在系统时钟附近显示任务栏/桌面弹窗。
    fn show_popup_near_clock(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// 按给定点击坐标显示任务栏/桌面弹窗。
    fn show_popup_at_position(
        &mut self,
        click_x: i32,
        click_y: i32,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// 在前端弹窗就绪后处理此前缓存的展示请求。
    fn on_popup_ready(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// 在任务栏/桌面弹窗可见与隐藏之间切换状态。
    fn toggle_popup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_popup_visible() {
            self.hide_popup()?;
        } else {
            self.show_popup_near_clock()?;
        }
        Ok(())
    }

    /// 根据点击位置执行任务栏/桌面弹窗显示/隐藏切换。
    fn toggle_popup_at_position(
        &mut self,
        click_x: i32,
        click_y: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_popup_visible() {
            self.hide_popup()?;
        } else {
            self.show_popup_at_position(click_x, click_y)?;
        }
        Ok(())
    }

    /// 设置弹窗是否固定。
    fn set_popup_pin(&mut self, pin: bool) -> Result<(), Box<dyn std::error::Error>>;
}
