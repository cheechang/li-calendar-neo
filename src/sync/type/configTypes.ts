import type { TrayClockDateFormat, TrayClockTimeFormat } from '../../enums/trayClockEnum.ts';

/** 节假日数据源类型 */
export type HolidayDataSource = 'builtin' | 'remote';

/** 单条节假日补丁记录 */
export interface HolidayPatchItem {
  /** 日期 YYYY-MM-DD */
  day: string;
  /** 节日名称 */
  name: string;
  /** true=调休/补班(班), false=休假(休) */
  work: boolean;
  /** 目标节日日期 YYYY-MM-DD */
  target: string;
}

/** 远程节假日 JSON 数据结构 */
export interface HolidayRemoteData {
  version?: string;
  updatedAt?: string;
  /** 节日名称表，若包含内置 NAMES 中没有的新节日则需提供 */
  names?: string[];
  holidays: HolidayPatchItem[];
}

/** 节假日数据配置项 */
export interface HolidayDataConfig {
  holidayDataSource: HolidayDataSource;
  holidayRemoteUrl: string;
  holidayLastUpdated: string;
  holidayDataCache: HolidayPatchItem[];
}

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
    ConfigMacos,
    HolidayDataConfig {}

export interface SystemConfig {
  // 开机自启动
  autostart: boolean;
  theme: CalendarTheme;
  /** 自动跟随系统主题切换 */
  themeFollowSystem: boolean;
  /** 日历小窗是否固定在最前（与顶栏图钉一致，持久化） */
  calendarPinned: boolean;
  /** 日历界面字体大小（px） */
  fontSize: number;
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
