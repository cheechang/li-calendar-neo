import { useEffect } from 'react';
import { useConfigSync } from '../../sync/configStore.ts';
import type { CalendarTheme } from '../../sync/type/configTypes.ts';

export interface UseCalendarThemeResult {
  theme: CalendarTheme;
  toggleTheme: () => Promise<void>;
  isDark: boolean;
}

/**
 * 读取 / 切换日历窗口深浅色主题，并与全局配置同步。
 * 当 `themeFollowSystem` 为 true 时，自动监听系统主题变化并同步。
 */
export function useCalendarTheme(): UseCalendarThemeResult {
  const { data: config, sync } = useConfigSync();
  const { theme, themeFollowSystem } = config;

  /** 监听系统主题变化，自动同步到配置 */
  useEffect(() => {
    if (!themeFollowSystem) return;

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

    // 立即同步当前系统主题
    const systemTheme = mediaQuery.matches ? 'dark' : 'light';
    if (systemTheme !== useConfigSync.getState().data.theme) {
      void sync('theme', systemTheme);
    }

    const handleChange = (e: MediaQueryListEvent) => {
      const newSystemTheme = e.matches ? 'dark' : 'light';
      void useConfigSync.getState().sync('theme', newSystemTheme);
    };

    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, [themeFollowSystem, sync]);

  const toggleTheme = async (): Promise<void> => {
    const newTheme = theme === 'light' ? 'dark' : 'light';
    // 手动切换时，自动关闭跟随系统主题，避免冲突
    if (themeFollowSystem) {
      await sync('themeFollowSystem', false);
    }
    await sync('theme', newTheme);
  };

  return { theme, toggleTheme, isDark: theme === 'dark' };
}
