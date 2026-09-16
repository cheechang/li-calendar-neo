import type { CalendarViewStyleContext } from './types.ts';

export function createCalendarGridStyles(ctx: CalendarViewStyleContext) {
  const { css, cx, isDark } = ctx;

  const lunar = css`
    font-size: calc(10px * var(--font-scale));
    color: ${isDark ? '#999999' : '#707070'};
    line-height: 1;
    margin-top: 1px;
  `;
  const term = css`
    color: ${isDark ? '#81c784' : '#2e7d32'};
    font-weight: 600;
  `;
  const today = css`
    position: relative;
    background: transparent;
    color: ${isDark ? '#000000' : '#ffffff'};

    /* 圆形聚焦点：居中、略小于格子，参考 Windows 11 原生日历 */
    &::before {
      content: '';
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: calc(100% - 4px);
      height: calc(100% - 4px);
      background: var(--accent);
      border-radius: 50%;
      z-index: -1;
      transition: filter 0.1s ease;
    }

    &:hover::before {
      filter: brightness(1.07);
    }

    /* 覆盖 .cell 的灰底 hover */
    &:hover {
      background: transparent;
    }

    /* today 圆圈内的阴历文字：与日期数字使用同一套高对比度颜色 */
    .${cx(lunar)} {
      color: ${isDark ? '#000000' : '#ffffff'};
    }

    .${cx(term)} {
      color: ${isDark ? '#000000' : '#ffffff'};
    }
  `;

  return {
    /** 外层容器：普通 block 布局，表头行 + 格子区域上下排列 */
    calendarGrid: css`
      display: block;
    `,
    /** 格子区域：CSS Grid，仅此区域参与滑动动画 */
    gridCellsContainer: css`
      display: grid;
      gap: 1px;
      justify-items: center;
      align-items: center;
    `,
    weekday: css`
      text-align: center;
      font-size: calc(13px * var(--font-scale));
      font-weight: 400;
      color: var(--text-main);
      padding-bottom: 12px;
      height: 24px;
    `,
    weekdayWeekend: css`
      color: ${isDark ? '#64b5f6' : '#1976d2'};
      font-weight: 600;
    `,
    weekNumberHeader: css`
      height: 24px;
    `,
    weekNumberCell: css`
      font-size: calc(10px * var(--font-scale));
      color: ${isDark ? '#666666' : '#bfbfbf'};
      display: flex;
      align-items: center;
      justify-content: center;
      width: 100%;
      height: 100%;
    `,
    cellPlaceholder: css`
      width: calc(40px + (var(--font-size-base) - 14px) * 1.5);
      height: calc(40px + (var(--font-size-base) - 14px) * 1.5);
    `,
    cell: css`
      width: calc(40px + (var(--font-size-base) - 14px) * 1.5);
      height: calc(40px + (var(--font-size-base) - 14px) * 1.5);
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
    today,
    selected: css`
      box-shadow: inset 0 0 0 1px var(--accent);
      border-radius: 50%;
    `,
    dateText: css`
      font-size: calc(13px * var(--font-scale));
      font-weight: 400;
      line-height: 1.1;
    `,
    weekend: css`
      .${cx(lunar)} {
        color: ${isDark ? '#90caf9' : '#42a5f5'};
      }

      &.${cx(today)} {
        color: ${isDark ? '#000000' : '#ffffff'};

        /* 周末且 today：阴历文字同样用高对比度（覆盖周末蓝色） */
        .${cx(lunar)} {
          color: ${isDark ? '#000000' : '#ffffff'};
        }
      }
    `,
    lunar,
    term,
    tag: css`
      position: absolute;
      top: 1px;
      right: 1px;
      font-size: calc(8px * var(--font-scale));
      min-width: 13px;
      min-height: 13px;
      padding: 0 1px;
      display: flex;
      align-items: center;
      justify-content: center;
      font-weight: bold;
      z-index: 1;
      border-radius: 3px;
      line-height: 1;
    `,
    tagWork: css`
      background: ${isDark ? '#5c3335' : '#fde7e9'};
      color: ${isDark ? '#ffb3b3' : '#a80000'};
    `,
    tagRest: css`
      background: ${isDark ? '#335c33' : '#dff6dd'};
      color: ${isDark ? '#b3ffb3' : '#107c10'};
    `,
  };
}
