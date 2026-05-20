import { trayClockDateFormats, trayClockTimeFormats } from '../enums/trayClockEnum.ts';
import { createSync } from './base/crossWindowSync.ts';
import type {
  CalendarFooterVisible,
  ConfigItem,
  ConfigMacos,
  ConfigWindows,
  HolidayDataConfig,
  SystemConfig,
} from './type/configTypes.ts';

/** `satisfies`：字面量须符合对应类型，写错字符串会在编译期报错，无需 `as` 断言 */
const systemConfigDefaults = {
  autostart: false,
  theme: 'light',
  themeFollowSystem: false,
  calendarPinned: false,
  fontSize: 14,
} satisfies SystemConfig;

const calendarFooterVisibleDefaults = {
  calendarFooterVisible: true,
  footerFestivalVisible: true,
  footerYiJiVisible: false,
  footerCountdownVisible: true,
} satisfies CalendarFooterVisible;

const configWindowsDefaults = {
  desktopWidgetEnabled: true,
  taskbarWidgetEnabled: true,
  desktopWindowPosition: null,
  customTrayClockEnabled: true,
  timeFormat: trayClockTimeFormats.HhMm,
  dateFormat: trayClockDateFormats.DddYmd,
} satisfies ConfigWindows;

const configMacosDefaults = {
  isWindowsEffect: false,
  macosEffect: 'vibrancy',
  frontendWindowEffectEnabled: false,
  frontendWindowEffect: 'transparent',
  frontendWindowTransparency: 20,
  macosTrayTitleTemplate: '',
  macosTrayBarIcon: 'date',
  macosTrayDateIconStyle: 'filled',
  macosTrayIconWidth: 42,
  macosTrayIconHeight: 36,
} satisfies ConfigMacos;

const holidayDataDefaults = {
  holidayDataSource: 'builtin',
  holidayRemoteUrl: '',
  holidayLastUpdated: '',
  holidayDataCache: [],
} satisfies HolidayDataConfig;

const configDefaults: ConfigItem = {
  ...systemConfigDefaults,
  ...calendarFooterVisibleDefaults,
  ...configWindowsDefaults,
  ...configMacosDefaults,
  ...holidayDataDefaults,
};

export const useConfigSync = createSync<ConfigItem>('liConfig', configDefaults);
