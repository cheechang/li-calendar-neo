import type { CalendarThemeTokens, CalendarViewStyleProps } from './types.ts';

/** 根据透明 / 深浅色 / 背景不透明度推导壳层用色与阴影 */
export function computeCalendarTheme({
  transparent,
  isDark,
  backgroundOpacity,
}: CalendarViewStyleProps): CalendarThemeTokens {
  const safeOpacity = Math.max(0, Math.min(100, backgroundOpacity ?? 100)) / 100;
  const containerBackground = transparent
    ? isDark
      ? `rgba(32, 32, 32, ${safeOpacity})`
      : `rgba(255, 255, 255, ${safeOpacity})`
    : isDark
      ? '#202020'
      : '#ffffff';
  const backdropFilter = transparent ? 'none' : 'none';
  const borderColor = isDark ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.05)';
  const shadowValue = transparent
    ? 'none'
    : isDark
      ? '0 8px 32px rgba(0, 0, 0, 0.4)'
      : '0 8px 32px rgba(0, 0, 0, 0.12)';
  const overlayBackground = 'none';
  const textureBackground = 'none';

  return {
    containerBackground,
    borderColor,
    shadowValue,
    backdropFilter,
    overlayBackground,
    textureBackground,
  };
}
