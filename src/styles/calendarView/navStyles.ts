import type { CalendarViewStyleContext } from './types.ts';

export function createCalendarNavStyles(ctx: CalendarViewStyleContext) {
  const { css, isDark } = ctx;

  return {
    calendarNav: css`
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 12px;
      padding: 0 4px;
    `,
    navTitle: css`
      font-size: 14px;
      font-weight: 600;
      color: var(--text-main);
    `,
    navBtns: css`
      display: flex;
      gap: 8px;
      align-items: center;
      font-size: 10px;
      color: var(--text-sec);
    `,
    todayBtn: css`
      height: 28px;
      padding: 0 10px;
      font-size: 12px;
      font-weight: 500;
    `,
    navBtn: css`
      cursor: pointer;
      width: 28px;
      height: 28px;
      padding: 0;
      border: none;
      background: transparent;
      color: var(--text-sec);
      display: inline-flex;
      align-items: center;
      justify-content: center;

      &:hover {
        background: ${isDark ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.05)'};
      }
    `,
  };
}
