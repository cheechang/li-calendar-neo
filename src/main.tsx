import { type ReactElement, useEffect, useState } from 'react';
import ReactDOM from 'react-dom/client';
import { isMobile } from './utils/platform.ts';
import { addMobileNavigationListener, getMobileAppViewFromLocation } from './utils/tauriUtils.ts';
import DesktopWindow from './windows/DesktopWindow.tsx';
import MacosPopupWindow from './windows/MacosPopupWindow.tsx';
import MainWindow from './windows/MainWindow.tsx';
import MobileWindow from './windows/mobile/MobileWindow.tsx';
import PopupWindow from './windows/PopupWindow.tsx';
import './global.css';
import { useWindowsTrayClockBootstrap } from './hooks/settings/useWindowsTrayClockBootstrap.ts';
import { prepareSync } from './sync/base/crossWindowSync.ts';

/** 根据当前平台和 URL 参数，决定应用根节点应该渲染哪一种窗口视图。 */
const resolveWindow = (
  mobileView: ReturnType<typeof getMobileAppViewFromLocation>,
): ReactElement => {
  /** 读取 URL 查询参数中的窗口类型。 */
  const params = new URLSearchParams(window.location.search);
  /** 桌面端会通过 window 参数区分 popup、desktop 等视图。 */
  const kind = params.get('window');
  if (kind === 'popup') {
    return <PopupWindow />;
  }
  if (kind === 'macos-popup') {
    return <MacosPopupWindow />;
  }
  if (kind === 'desktop') {
    return <DesktopWindow />;
  }
  if (isMobile) {
    if (mobileView === 'settings') {
      return <MainWindow mobileCalendarOnly />;
    }
    return <MobileWindow />;
  }
  return <MainWindow />;
};

/** 应用根组件，负责监听移动端“日历/设置”两种内嵌视图切换。 */
function App(): ReactElement {
  /** 当前移动端主视图，桌面端虽然不会用到，但保持统一入口。 */
  const [mobileView, setMobileView] = useState(getMobileAppViewFromLocation);

  /** 各 WebView 独立挂载时，按配置同步任务栏时钟（不依赖设置页是否打开）。 */
  useWindowsTrayClockBootstrap();

  useEffect(() => {
    /** 当 URL 或内部导航事件变化时，同步更新当前移动端视图。 */
    const handleLocationChange = () => {
      setMobileView(getMobileAppViewFromLocation());
    };

    /** 监听自定义导航事件，处理应用内切页。 */
    const removeNavigationListener = addMobileNavigationListener(handleLocationChange);
    /** 监听浏览器历史记录变化，支持返回手势或系统返回。 */
    window.addEventListener('popstate', handleLocationChange);
    return () => {
      removeNavigationListener();
      window.removeEventListener('popstate', handleLocationChange);
    };
  }, []);

  return resolveWindow(mobileView);
}

/** 等配置同步系统准备完成后，再挂载 React 根节点。 */
prepareSync().then(() => {
  ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(<App />);
});
