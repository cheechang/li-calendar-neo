import type { CalendarViewStyleContext } from './types.ts';

export function createCalendarShellStyles(ctx: CalendarViewStyleContext) {
  const {
    css,
    containerBackground,
    borderColor,
    shadowValue,
    backdropFilter,
    overlayBackground,
    textureBackground,
    isDark,
  } = ctx;

  return {
    micaContainer: css`
      --mica-bg: ${containerBackground};
      --card-bg: ${isDark ? 'rgba(45, 45, 45, 0.5)' : 'rgba(255, 255, 255, 0.5)'};
      --accent: ${isDark ? '#60cdff' : '#0067c0'};
      --text-main: ${isDark ? '#ffffff' : '#1c1c1c'};
      --text-sec: ${isDark ? '#cccccc' : '#5d5d5d'};
      width: 320px;
      background: var(--mica-bg);
      backdrop-filter: ${backdropFilter};
      -webkit-backdrop-filter: ${backdropFilter};
      border: 1px solid ${borderColor};
      box-shadow: var(--calendar-shadow, ${shadowValue});
      border-radius: var(--calendar-radius, 16px);
      clip-path: inset(0 round var(--calendar-radius, 16px));
      overflow: hidden;
      padding: 8px;
      color: var(--text-main);
      margin: 0;
      touch-action: none;
      user-select: none;
      position: relative;
      isolation: isolate;

      &::before,
      &::after {
        content: '';
        position: absolute;
        inset: 0;
        pointer-events: none;
        border-radius: inherit;
      }

      &::before {
        z-index: 0;
        background: ${overlayBackground};
        opacity: 0;
      }

      &::after {
        z-index: 0;
        background: ${textureBackground};
        opacity: 0;
        mix-blend-mode: normal;
      }

      & > * {
        position: relative;
        z-index: 1;
      }
    `,
  };
}
