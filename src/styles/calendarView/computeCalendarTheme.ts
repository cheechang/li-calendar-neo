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
  const borderColor = isDark ? 'rgba(255, 255, 255, 0.12)' : 'rgba(0, 0, 0, 0.1)';
  const shadowValue = transparent
    ? 'none'
    : isDark
      ? '0 8px 32px rgba(0, 0, 0, 0.4)'
      : /* 多层弥散阴影，模拟 Windows 11 浮窗的层次感：
         近距微阴影勾勒边缘 + 中距过渡 + 远距大面积弥散 */
        '0 1px 2px rgba(0,0,0,0.06), 0 4px 12px rgba(0,0,0,0.08), 0 12px 40px rgba(0,0,0,0.14)';
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
