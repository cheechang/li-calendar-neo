import { invoke } from '@tauri-apps/api/core';
import { useEffect, useRef } from 'react';
import { useConfigSync } from '../../sync/configStore.ts';
import { isDesktop } from '../../utils/platform.ts';

export interface UseCalendarPinResult {
  isPinned: boolean;
  togglePin: () => Promise<void>;
}

/**
 * 日历小窗是否置顶：与全局配置 `calendarPinned` 同步并持久化，经 Tauri 与系统窗口层一致。
 */
export function useCalendarPin(): UseCalendarPinResult {
  const { data: config, sync, initialized } = useConfigSync();
  const isPinned = config.calendarPinned;
  /** 本窗口刚通过 `togglePin` 调过 `invoke` 时，跳过一次 effect，避免重复调用 */
  const skipNextEffectInvokeRef = useRef(false);

  useEffect(() => {
    if (!initialized || !isDesktop) {
      return;
    }
    if (skipNextEffectInvokeRef.current) {
      skipNextEffectInvokeRef.current = false;
      return;
    }
    void invoke('set_calendar_pin', { pin: config.calendarPinned }).catch((e: unknown) => {
      console.error('Failed to set pin status:', e);
    });
  }, [initialized, config.calendarPinned]);

  const togglePin = async (): Promise<void> => {
    const prev = config.calendarPinned;
    const next = !prev;
    await sync('calendarPinned', next);
    if (!isDesktop) {
      return;
    }
    skipNextEffectInvokeRef.current = true;
    try {
      await invoke('set_calendar_pin', { pin: next });
    } catch (e) {
      console.error('Failed to set pin status:', e);
      skipNextEffectInvokeRef.current = false;
      await sync('calendarPinned', prev);
    }
  };

  return { isPinned, togglePin };
}
