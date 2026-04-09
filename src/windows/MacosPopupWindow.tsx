import { invoke } from '@tauri-apps/api/core';
import { PhysicalSize } from '@tauri-apps/api/dpi';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { type CSSProperties, type ReactElement, useEffect, useLayoutEffect, useRef } from 'react';
import CalendarView from '../components/calendar/CalendarView.tsx';
import { WINDOW_RADIUS } from '../constants/window.ts';
import { useWindowCornerMask } from '../hooks/useWindowCornerMask.ts';
import { useConfigSync } from '../sync/configStore.ts';

const MacosPopupWindow = (): ReactElement => {
  const containerRef = useRef<HTMLDivElement>(null);
  const { data } = useConfigSync();
  const { isWindowsEffect, macosEffect } = data;

  useEffect(() => {
    const html = document.documentElement;
    const body = document.body;
    const root = document.getElementById('root');
    const previous = {
      htmlBackground: html.style.background,
      bodyBackground: body.style.background,
      rootBackground: root?.style.background ?? '',
      htmlOverflow: html.style.overflow,
      bodyOverflow: body.style.overflow,
      rootOverflow: root?.style.overflow ?? '',
      htmlMinHeight: html.style.minHeight,
      bodyMinHeight: body.style.minHeight,
      rootMinHeight: root?.style.minHeight ?? '',
      htmlHeight: html.style.height,
      bodyHeight: body.style.height,
      rootHeight: root?.style.height ?? '',
    };

    html.style.background = 'transparent';
    body.style.background = 'transparent';
    if (root) {
      root.style.background = 'transparent';
    }
    html.style.overflow = 'hidden';
    body.style.overflow = 'hidden';
    if (root) {
      root.style.overflow = 'hidden';
    }
    html.style.minHeight = '0';
    body.style.minHeight = '0';
    if (root) {
      root.style.minHeight = '0';
    }
    html.style.height = 'auto';
    body.style.height = 'auto';
    if (root) {
      root.style.height = 'auto';
    }
    return () => {
      html.style.background = previous.htmlBackground;
      body.style.background = previous.bodyBackground;
      html.style.overflow = previous.htmlOverflow;
      body.style.overflow = previous.bodyOverflow;
      html.style.minHeight = previous.htmlMinHeight;
      body.style.minHeight = previous.bodyMinHeight;
      html.style.height = previous.htmlHeight;
      body.style.height = previous.bodyHeight;
      if (root) {
        root.style.background = previous.rootBackground;
        root.style.overflow = previous.rootOverflow;
        root.style.minHeight = previous.rootMinHeight;
        root.style.height = previous.rootHeight;
      }
    };
  }, []);

  useLayoutEffect(() => {
    const resizeWindow = async (): Promise<void> => {
      const el = containerRef.current;
      if (!el) return;

      const window = getCurrentWindow();
      const factor = await window.scaleFactor();
      const width = el.offsetWidth;
      const height = el.offsetHeight;

      if (width === 0 || height === 0) return;

      const physicalWidth = Math.ceil(width * factor);
      const physicalHeight = Math.ceil(height * factor);

      await window.setSize(new PhysicalSize(physicalWidth, physicalHeight));
    };

    const observer = new ResizeObserver(() => {
      resizeWindow().catch(console.error);
    });
    if (containerRef.current) {
      observer.observe(containerRef.current);
    }

    return () => {
      observer.disconnect();
    };
  }, []);

  useWindowCornerMask();

  useEffect(() => {
    invoke('set_macos_vibrancy', {
      enabled: isWindowsEffect,
      effect: macosEffect,
    }).catch(console.error);
  }, [isWindowsEffect, macosEffect]);

  /** 仅包裹尺寸测量，日历视觉与桌面/任务栏窗口共用 `useCalendarViewStyles` 默认值 */
  const containerStyle = {
    boxSizing: 'border-box',
    padding: 0,
    overflow: 'hidden',
    display: 'inline-block',
    width: 'fit-content',
    background: 'transparent',
  } as CSSProperties;

  const calendarStyle = {
    '--calendar-radius': `${WINDOW_RADIUS}px`,
    '--calendar-shadow': 'none',
  } as CSSProperties;

  return (
    <div style={containerStyle} ref={containerRef}>
      <CalendarView
        autoResizeWindow={false}
        transparent={isWindowsEffect}
        backgroundOpacity={isWindowsEffect ? 8 : 100}
        style={calendarStyle}
      />
    </div>
  );
};

export default MacosPopupWindow;
