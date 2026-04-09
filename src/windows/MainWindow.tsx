import { ConfigProvider } from 'antd';
import type { ReactElement } from 'react';
import Settings from '../components/settings/Settings.tsx';

/** 主设置窗口的入参；移动端可切到仅展示“日历内容”的精简模式。 */
interface MainWindowProps {
  mobileCalendarOnly?: boolean;
}

/** 桌面端主设置窗口，同时兼容移动端内嵌设置视图。 */
function MainWindow({ mobileCalendarOnly = false }: MainWindowProps): ReactElement {
  return (
    <ConfigProvider>
      <Settings mobileCalendarOnly={mobileCalendarOnly} />
    </ConfigProvider>
  );
}

export default MainWindow;
