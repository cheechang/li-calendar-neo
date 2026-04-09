import { useConfigSync } from '../../sync/configStore.ts';
import type { CalendarTheme } from '../../sync/type/configTypes.ts';

export interface UseCalendarThemeResult {
  theme: CalendarTheme;
  toggleTheme: () => Promise<void>;
  isDark: boolean;
}

/**
 * 读取 / 切换日历窗口深浅色主题，并与全局配置同步。
 */
export function useCalendarTheme(): UseCalendarThemeResult {
  const { data: config, sync } = useConfigSync();
  const theme = config.theme;

  const toggleTheme = async (): Promise<void> => {
    const newTheme = theme === 'light' ? 'dark' : 'light';
    await sync('theme', newTheme);
  };

  return { theme, toggleTheme, isDark: theme === 'dark' };
}
