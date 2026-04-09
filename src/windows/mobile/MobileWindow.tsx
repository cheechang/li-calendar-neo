import type { CSSProperties, ReactElement } from 'react';
import CalendarView from '../../components/calendar/CalendarView.tsx';

/** 移动端页面壳层样式，负责处理安全区与整体居中布局。 */
const shellStyle: CSSProperties = {
  minHeight: '100dvh',
  boxSizing: 'border-box',
  paddingTop: 'max(env(safe-area-inset-top), 12px)',
  paddingRight: 'max(env(safe-area-inset-right), 12px)',
  paddingBottom: 'max(env(safe-area-inset-bottom), 12px)',
  paddingLeft: 'max(env(safe-area-inset-left), 12px)',
  display: 'flex',
  alignItems: 'stretch',
  justifyContent: 'center',
  background: 'transparent',
};

/** 移动端日历卡片样式，控制最大宽度与圆角表现。 */
const calendarStyle = {
  width: '100%',
  maxWidth: '420px',
  margin: '0 auto',
  '--calendar-radius': '24px',
  '--calendar-shadow': 'none',
} as CSSProperties;

/** 移动端专用窗口，只展示适合手机使用的日历主体。 */
function MobileWindow(): ReactElement {
  return (
    <div style={shellStyle}>
      <CalendarView
        autoResizeWindow={false}
        transparent={false}
        backgroundOpacity={100}
        showPinButton={false}
        showCloseButton={false}
        style={calendarStyle}
      />
    </div>
  );
}

export default MobileWindow;
