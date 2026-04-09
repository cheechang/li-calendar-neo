import { invoke } from '@tauri-apps/api/core';
import { PhysicalSize } from '@tauri-apps/api/dpi';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { type RefObject, useLayoutEffect, useRef } from 'react';

/** 任务栏弹窗在 `setSize` 后按新高度重新贴边，防抖避免 ResizeObserver 连发 */
const REPOSITION_NEAR_TASKBAR_MS = 150;

/**
 * 将日历根节点内容尺寸同步为当前 Tauri 窗口物理像素大小（含缩放因子）。
 */
export function useTauriCalendarResize(
  autoResizeWindow: boolean,
): RefObject<HTMLDivElement | null> {
  /** 挂载在日历最外层 div，供 ResizeObserver 测量 */
  const containerRef = useRef<HTMLDivElement | null>(null);
  const repositionTaskbarPopupTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useLayoutEffect(() => {
    if (!autoResizeWindow) {
      return;
    }
    const el = containerRef.current;
    if (!el) return;

    const scheduleRepositionTaskbarPopup = (): void => {
      const w = getCurrentWindow();
      if (w.label !== 'calendar') {
        return;
      }
      if (repositionTaskbarPopupTimerRef.current != null) {
        clearTimeout(repositionTaskbarPopupTimerRef.current);
      }
      repositionTaskbarPopupTimerRef.current = setTimeout(() => {
        repositionTaskbarPopupTimerRef.current = null;
        requestAnimationFrame(() => {
          void invoke('show_calendar').catch(() => {});
        });
      }, REPOSITION_NEAR_TASKBAR_MS);
    };

    const resizeWindow = async (): Promise<void> => {
      const window = getCurrentWindow();
      const factor = await window.scaleFactor();
      const width = el.offsetWidth;
      const height = el.offsetHeight;

      const physicalWidth = Math.ceil(width * factor);
      const physicalHeight = Math.ceil(height * factor);

      await window.setSize(new PhysicalSize(physicalWidth, physicalHeight));
      scheduleRepositionTaskbarPopup();
    };

    const observer = new ResizeObserver(() => {
      resizeWindow().catch(() => {});
    });

    observer.observe(el);
    resizeWindow().catch(() => {});

    return () => {
      observer.disconnect();
      if (repositionTaskbarPopupTimerRef.current != null) {
        clearTimeout(repositionTaskbarPopupTimerRef.current);
        repositionTaskbarPopupTimerRef.current = null;
      }
    };
  }, [autoResizeWindow]);

  return containerRef;
}
