import { createStyles } from 'antd-style';

import { computeCalendarTheme } from './calendarView/computeCalendarTheme.ts';
import { createCalendarFooterStyles } from './calendarView/footerStyles.ts';
import { createCalendarGridStyles } from './calendarView/gridStyles.ts';
import { createCalendarHeaderStyles } from './calendarView/headerStyles.ts';
import { createCalendarNavStyles } from './calendarView/navStyles.ts';
import { createCalendarShellStyles } from './calendarView/shellStyles.ts';
import type { CalendarViewStyleContext, CalendarViewStyleProps } from './calendarView/types.ts';

/**
 * 日历窗口全套样式（Mica 壳、网格、页脚等），随透明开关、深浅色与背景透明度变化。
 * 仅 `--calendar-radius` / `--calendar-shadow` 由窗口根节点注入，用于与系统窗口圆角、无阴影一致。
 */
export const useCalendarViewStyles = createStyles(({ css, cx }, props: CalendarViewStyleProps) => {
  const theme = computeCalendarTheme(props);
  const isDark = props.isDark ?? false;

  const ctx: CalendarViewStyleContext = {
    ...theme,
    css,
    cx,
    isDark,
  };

  return {
    ...createCalendarShellStyles(ctx),
    ...createCalendarHeaderStyles(ctx),
    ...createCalendarNavStyles(ctx),
    ...createCalendarGridStyles(ctx),
    ...createCalendarFooterStyles(ctx),
  };
});

/** 子组件接收的 `styles` 对象类型，便于 props 标注 */
export type CalendarViewClassNames = ReturnType<typeof useCalendarViewStyles>['styles'];
