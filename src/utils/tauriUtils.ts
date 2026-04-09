import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { isMobile } from './platform.ts';

/** 移动端日历页与设置页之间切换时使用的前端事件名。 */
const mobileNavigationEvent = 'li-calendar:navigation';

/** 移动端应用当前支持的两种主视图。 */
export type MobileAppView = 'calendar' | 'settings';

/** 从当前 URL 里解析移动端正在展示的是日历页还是设置页。 */
export function getMobileAppViewFromLocation(): MobileAppView {
  return new URLSearchParams(window.location.search).get('view') === 'settings'
    ? 'settings'
    : 'calendar';
}

/** 更新移动端 URL 状态，并主动广播视图切换事件。 */
function setMobileAppView(view: MobileAppView): void {
  /** 基于当前地址构造下一次切换后的 URL。 */
  const nextUrl = new URL(window.location.href);
  if (view === 'calendar') {
    nextUrl.searchParams.delete('view');
  } else {
    nextUrl.searchParams.set('view', view);
  }
  window.history.pushState({}, '', `${nextUrl.pathname}${nextUrl.search}${nextUrl.hash}`);
  window.dispatchEvent(new Event(mobileNavigationEvent));
}

/** 切回移动端日历主页面。 */
export function openMobileCalendarView(): void {
  setMobileAppView('calendar');
}

/** 订阅移动端内部导航变化，并返回取消订阅函数。 */
export function addMobileNavigationListener(listener: () => void): () => void {
  window.addEventListener(mobileNavigationEvent, listener);
  return () => window.removeEventListener(mobileNavigationEvent, listener);
}

/**
 * 从当前页面 URL 查询参数读取窗口类型（如 popup、macos-popup），用于区分关闭行为。
 */
export function getCalendarWindowKindFromLocation(): string | null {
  return new URLSearchParams(window.location.search).get('window');
}

/**
 * 弹窗类窗口执行 hide，主窗口等执行 close。
 */
export async function closeOrHideCalendarWindow(windowKind: string | null): Promise<void> {
  const currentWindow = getCurrentWindow();
  if (windowKind === 'popup' || windowKind === 'macos-popup') {
    await currentWindow.hide();
    return;
  }
  await currentWindow.close();
}

/** 通过 Tauri 命令打开应用主设置窗口 */
export async function openMainApplicationWindow(): Promise<void> {
  /** 移动端不弹新窗口，而是在当前页面内切到设置视图。 */
  if (isMobile) {
    setMobileAppView('settings');
    return;
  }
  await invoke('open_main_window');
}

/**
 * 在空白区域按下指针时启动窗口拖拽（需在可拖拽区域且非交互控件上触发）。
 */
export async function startCalendarShellDragging(): Promise<void> {
  await getCurrentWindow().startDragging();
}
