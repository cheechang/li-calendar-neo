import type { SerializeCSS } from 'antd-style/lib/core';
import type { ClassNamesUtil } from 'antd-style/lib/types/css';

/** createStyles 回调入参（与日历组件传入的 props 一致） */
export interface CalendarViewStyleProps {
  transparent?: boolean;
  isDark?: boolean;
  backgroundOpacity?: number;
}

/** 由主题与透明选项推导出的纯值，供各样式块共用 */
export interface CalendarThemeTokens {
  containerBackground: string;
  borderColor: string;
  shadowValue: string;
  backdropFilter: string;
  overlayBackground: string;
  textureBackground: string;
}

/** 子模块样式工厂收到的完整上下文 */
export interface CalendarViewStyleContext extends CalendarThemeTokens {
  css: SerializeCSS;
  cx: ClassNamesUtil;
  isDark: boolean;
}
