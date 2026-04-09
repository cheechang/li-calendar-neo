/**
 * 托盘时钟时间 / 日期格式（与 Windows 区域格式令牌一致）。
 * - `trayClock*Formats`：具名映射，供 `xxx.HhMm` 等调用；
 * - `TrayClock*Format`：合法格式字符串联合类型；
 * - `trayClock*Presets`：`Object.values(formats)`，供列表遍历（与映射单一数据源）。
 */
export const trayClockTimeFormats = {
  HhMm: 'HH:mm',
  TtH12: 'tt h:mm:ss',
  HhMmSs: 'HH:mm:ss',
} as const;

export const trayClockDateFormats = {
  DddYmd: 'ddd yyyy-M-d',
  YmdDddd: 'yyyy/M/d dddd',
  MonthDayDdd: 'M月d日 ddd',
  MonthDayDddd: 'M月d日 dddd',
} as const;

export type TrayClockTimeFormat = (typeof trayClockTimeFormats)[keyof typeof trayClockTimeFormats];
export type TrayClockDateFormat = (typeof trayClockDateFormats)[keyof typeof trayClockDateFormats];

export const trayClockTimePresets = Object.values(
  trayClockTimeFormats,
) as readonly TrayClockTimeFormat[];

export const trayClockDatePresets = Object.values(
  trayClockDateFormats,
) as readonly TrayClockDateFormat[];
