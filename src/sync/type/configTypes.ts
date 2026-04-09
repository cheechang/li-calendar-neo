import type { TrayClockDateFormat, TrayClockTimeFormat } from '../../enums/trayClockEnum.ts';

export type MacosVibrancyEffect =
  | 'blur'
  | 'acrylic'
  | 'popover'
  | 'sidebar'
  | 'mica'
  | 'mica-dark'
  | 'mica-light'
  | 'hud-window'
  | 'tabbed'
  | 'tabbed-dark'
  | 'tabbed-light'
  | 'header-view'
  | 'vibrancy'
  | 'liquid-glass'
  | 'under-window-background';

export type FrontendWindowEffect = 'transparent';
export type CalendarTheme = 'light' | 'dark';

export type MacosTrayDateIconStyle = 'filled' | 'outlined';

/**
 * 菜单栏主图标类型：`date` = 日期数字（DateIconView），`calendar` = SF Symbol「日历」（与 LunarBar createCalendarIcon 一致）。
 */
export type MacosTrayBarIconKind = 'date' | 'calendar';

export interface ConfigItem
  extends SystemConfig,
    CalendarFooterVisible,
    ConfigWindows,
    ConfigMacos {}

export interface SystemConfig {
  // 开机自启动
  autostart: boolean;
  theme: CalendarTheme;
  /** 日历小窗是否固定在最前（与顶栏图钉一致，持久化） */
  calendarPinned: boolean;
}

export interface CalendarFooterVisible {
  /** 显示底部信息区域 */
  calendarFooterVisible: boolean;
  /** 显示节日信息 */
  footerFestivalVisible: boolean;
  /** 显示宜忌信息 */
  footerYiJiVisible: boolean;
  /** 显示倒计时信息 */
  footerCountdownVisible: boolean;
}

export interface ConfigWindows extends WindowsDesktop, WindowsTaskbar {
  /** 启用桌面组件 */
  desktopWidgetEnabled: boolean;
  /** 启用任务栏弹窗组件 */
  taskbarWidgetEnabled: boolean;
}

export interface WindowsDesktop {
  /** 桌面窗口位置 */
  desktopWindowPosition: { x: number; y: number } | null;
}
export interface WindowsTaskbar {
  /** 自定义托盘时钟 */
  customTrayClockEnabled: boolean;
  /** 时间格式 */
  timeFormat: TrayClockTimeFormat;
  /** 日期格式 */
  dateFormat: TrayClockDateFormat;
}

export interface ConfigMacos {
  /** macOS 半透明 */
  isWindowsEffect: boolean;
  /** macOS 半透明效果 */
  macosEffect: MacosVibrancyEffect;
  frontendWindowEffectEnabled: boolean;
  frontendWindowEffect: FrontendWindowEffect;
  /** 纯前端效果下窗口背景的透明度 0–100，数值越大越透明 */
  frontendWindowTransparency: number;
  /** macOS 菜单栏图标文案模板 */
  macosTrayTitleTemplate: string;
  /** 菜单栏主图标：`date` = 日期数字，`calendar` = SF Symbol 日历图标（LunarBar 同款） */
  macosTrayBarIcon: MacosTrayBarIconKind;
  /**
   * macOS 菜单栏日期图标样式（与 LunarBar 一致：实心 = 整面填充 + 数字镂空，描边 = 圆角框 + 数字）
   */
  macosTrayDateIconStyle: MacosTrayDateIconStyle;
  /** 菜单栏日期图标位图宽度（像素），后端限制 16–128，默认 42（与 LunarBar 对齐的 21×18 @2× 画布之宽） */
  macosTrayIconWidth: number;
  /** 菜单栏日期图标位图高度（像素），默认 36（21×18 @2×，与 tray 约 18pt 槽 + LunarBar 15pt 图高一致） */
  macosTrayIconHeight: number;
}
