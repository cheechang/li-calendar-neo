import { invoke } from '@tauri-apps/api/core';
import { PhysicalPosition } from '@tauri-apps/api/dpi';
import { availableMonitors, getCurrentWindow } from '@tauri-apps/api/window';
import { type CSSProperties, type ReactElement, useEffect, useRef } from 'react';
import CalendarView from '../components/calendar/CalendarView.tsx';
import { WINDOW_RADIUS } from '../constants/window.ts';
import { useWindowCornerMask } from '../hooks/useWindowCornerMask.ts';
import { useConfigSync } from '../sync/configStore.ts';
import { isWindows } from '../utils/platform.ts';

const DesktopWindow = (): ReactElement => {
  const { data, sync, initialized } = useConfigSync();
  const {
    desktopWindowPosition,
    isWindowsEffect: windowTransparency,
    macosEffect: windowEffect,
  } = data;
  const lastPositionRef = useRef<{ x: number; y: number } | null>(null);
  const applyingPositionRef = useRef(false);
  const pendingPositionRef = useRef<{ x: number; y: number } | null>(null);
  const syncTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const windowShownRef = useRef(false);

  useEffect(() => {
    if (isWindows) {
      void getCurrentWindow().setShadow(false);
    }
  }, []);

  useEffect(() => {
    if (!initialized) {
      return;
    }
    applyingPositionRef.current = true;
    void (async () => {
      try {
        let position = desktopWindowPosition;
        if (!position) {
          const monitors = await availableMonitors();
          const monitor = monitors[0];
          const windowSize = await getCurrentWindow().outerSize();
          const margin = 40;
          if (monitor) {
            position = {
              x: monitor.size.width - windowSize.width - margin,
              y: margin,
            };
          } else {
            position = { x: 40, y: 40 };
          }
        }
        // 若 Rust 侧已将窗口定位到持久化坐标，lastPositionRef 可能为空但实际位置已正确。
        // 先读取当前实际物理坐标，只有确实不同时才移动，避免重复 setPosition 触发闪烁。
        if (!lastPositionRef.current) {
          const current = await getCurrentWindow().outerPosition();
          lastPositionRef.current = { x: current.x, y: current.y };
        }
        const last = lastPositionRef.current;
        if (!last || last.x !== position.x || last.y !== position.y) {
          await getCurrentWindow().setPosition(new PhysicalPosition(position.x, position.y));
        }
        lastPositionRef.current = position;
        if (!windowShownRef.current) {
          windowShownRef.current = true;
          if (!desktopWindowPosition) {
            await sync({ desktopWindowPosition: position });
          }
          // Note: Avoid calling getCurrentWindow().show() or focus() here
          // because Desktop Window is meant to be passively displayed
          // on the desktop and shouldn't steal focus from Taskbar Popup on startup.
        }
      } catch (error) {
        console.error(error);
      } finally {
        applyingPositionRef.current = false;
      }
    })();
  }, [initialized, desktopWindowPosition, sync]);

  // 统一应用前端圆角遮罩
  useWindowCornerMask();

  useEffect(() => {
    invoke('set_macos_vibrancy', {
      enabled: windowTransparency,
      effect: windowEffect,
      windowLabel: 'desktop_calendar',
    }).catch(console.error);
  }, [windowEffect, windowTransparency]);

  useEffect(() => {
    const window = getCurrentWindow();
    let unlisten: (() => void) | undefined;
    let mounted = true;
    const flushPendingPosition = (): void => {
      const pending = pendingPositionRef.current;
      if (!pending) {
        return;
      }
      pendingPositionRef.current = null;
      void sync({ desktopWindowPosition: pending }).catch(console.error);
    };
    const init = async (): Promise<void> => {
      unlisten = await window.onMoved(({ payload }) => {
        if (applyingPositionRef.current) {
          return;
        }
        const next = { x: payload.x, y: payload.y };
        const last = lastPositionRef.current;
        if (last && last.x === next.x && last.y === next.y) {
          return;
        }
        lastPositionRef.current = next;
        pendingPositionRef.current = next;
        if (syncTimerRef.current) {
          clearTimeout(syncTimerRef.current);
        }
        syncTimerRef.current = setTimeout(() => {
          flushPendingPosition();
          syncTimerRef.current = null;
        }, 120);
      });
      const current = await window.outerPosition();
      if (!mounted) {
        return;
      }
      lastPositionRef.current = { x: current.x, y: current.y };
    };
    init().catch(console.error);
    return () => {
      mounted = false;
      if (syncTimerRef.current) {
        clearTimeout(syncTimerRef.current);
        syncTimerRef.current = null;
      }
      flushPendingPosition();
      if (unlisten) {
        unlisten();
      }
    };
  }, [sync]);

  return (
    <CalendarView
      enableDrag
      dragRegion
      transparent={isWindows ? windowTransparency : true}
      backgroundOpacity={windowTransparency ? 72 : 100}
      style={
        {
          '--calendar-radius': `${WINDOW_RADIUS}px`,
          '--calendar-shadow': 'none',
        } as CSSProperties
      }
    />
  );
};

export default DesktopWindow;
