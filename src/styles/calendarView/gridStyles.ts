import type { CalendarViewStyleContext } from './types.ts';

export function createCalendarGridStyles(ctx: CalendarViewStyleContext) {
  const { css, cx, isDark } = ctx;

  const lunar = css`
    font-size: 10px;
    color: ${isDark ? '#999999' : '#707070'};
    line-height: 1;
    margin-top: 1px;
  `;
  const term = css`
    color: ${isDark ? '#81c784' : '#2e7d32'};
    font-weight: 600;
  `;

  return {
    calendarGrid: css`
      display: grid;
      grid-template-columns: repeat(7, 1fr);
      gap: 1px;
      justify-items: center;
      align-items: center;
    `,
    weekday: css`
      text-align: center;
      font-size: 13px;
      font-weight: 400;
      color: var(--text-main);
      padding-bottom: 12px;
      height: 24px;
    `,
    cell: css`
      width: 40px;
      height: 40px;
      background: transparent;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      border: none;
      border-radius: 50%;
      transition: background 0.1s ease;
      cursor: pointer;
      position: relative;
      padding: 0;
      color: var(--text-main);

      &:hover {
        background: ${isDark ? 'rgba(255, 255, 255, 0.1)' : 'rgba(0, 0, 0, 0.05)'};
      }
    `,
    otherMonth: css`
      color: ${isDark ? '#666666' : '#bfbfbf'};
    `,
    today: css`
      background: var(--accent);
      color: ${isDark ? '#000000' : '#ffffff'};

      .${cx(lunar)} {
        color: ${isDark ? 'rgba(0, 0, 0, 0.7)' : 'rgba(255, 255, 255, 0.9)'};
      }

      .${cx(term)} {
        color: ${isDark ? '#000000' : '#ffffff'};
      }

      /* 覆盖 .cell 的灰底 hover：保持强调色底，整体略提亮作为反馈（不叠灰底） */
      &:hover {
        background: var(--accent);
        filter: brightness(1.07);
      }
    `,
    selected: css`
      box-shadow: inset 0 0 0 1px var(--accent);
      border-radius: 50%;
    `,
    dateText: css`
      font-size: 13px;
      font-weight: 400;
      line-height: 1.1;
    `,
    lunar,
    term,
    tag: css`
      position: absolute;
      top: 4px;
      right: 4px;
      font-size: 8px;
      width: 12px;
      height: 12px;
      display: flex;
      align-items: center;
      justify-content: center;
      font-weight: bold;
      z-index: 1;
      border-radius: 50%;
    `,
    tagWork: css`
      background: ${isDark ? '#4d2d2f' : '#fde7e9'};
      color: ${isDark ? '#ff9999' : '#a80000'};
    `,
    tagRest: css`
      background: ${isDark ? '#2d4d2d' : '#dff6dd'};
      color: ${isDark ? '#99ff99' : '#107c10'};
    `,
  };
}
