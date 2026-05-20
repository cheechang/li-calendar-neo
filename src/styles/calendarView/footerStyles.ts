import type { CalendarViewStyleContext } from './types.ts';

export function createCalendarFooterStyles(ctx: CalendarViewStyleContext) {
  const { css, isDark } = ctx;

  return {
    footerInfo: css`
      margin-top: 16px;
      padding: 12px;
      background: ${isDark ? 'rgba(255, 255, 255, 0.05)' : 'rgba(0, 0, 0, 0.03)'};
      display: flex;
      flex-direction: column;
      gap: 12px;
      min-width: 0;
    `,
    footerMain: css`
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      min-width: 0;
    `,
    lunarInfo: css`
      display: flex;
      flex-direction: column;
      gap: 2px;
    `,
    lunarDay: css`
      font-size: calc(16px * var(--font-scale));
      font-weight: 500;
      color: var(--text-main);
    `,
    lunarYear: css`
      font-size: calc(12px * var(--font-scale));
      color: var(--text-sec);
    `,
    yiJiContainer: css`
      display: flex;
      flex-direction: column;
      gap: 6px;
      flex: 1;
      min-width: 0;
    `,
    yiJiItem: css`
      display: flex;
      align-items: flex-start;
      gap: 8px;
      font-size: calc(13px * var(--font-scale));
      min-width: 0;
    `,
    yiJiBadge: css`
      width: 18px;
      height: 18px;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: calc(11px * var(--font-scale));
      font-weight: bold;
      flex-shrink: 0;
      margin-top: 1px;
      border-radius: 50%;
    `,
    yiBadge: css`
      background: #e6f4ea;
      color: #1e8e3e;
      ${isDark &&
      css`
        background: #1e3a2f;
        color: #81c784;
      `
      }
    `,
    jiBadge: css`
      background: #fce8e6;
      color: #d93025;
      ${isDark &&
      css`
        background: #3c1e1e;
        color: #f28b82;
      `
      }
    `,
    yiJiText: css`
      color: var(--text-main);
      line-height: 1.5;
      flex: 1;
      min-width: 0;
      white-space: nowrap;
      text-overflow: ellipsis;
      overflow: hidden;
    `,
    footerDivider: css`
      height: 1px;
      background: ${isDark ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.06)'};
      margin: 0 -4px;
    `,
    countdown: css`
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: calc(12px * var(--font-scale));
      color: var(--text-sec);
    `,
    countdownIcon: css`
      font-size: calc(14px * var(--font-scale));
      opacity: 0.8;
    `,
  };
}
