import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';
import type { TrayClockDateFormat, TrayClockTimeFormat } from '../../enums/trayClockEnum.ts';
import { isWindows } from '../../utils/platform.ts';

export interface UseClockManagerResult {
  /** 应用自定义时钟文本。 */
  handleApplyClock: (
    timeFormatToApply: TrayClockTimeFormat,
    dateFormatToApply: TrayClockDateFormat,
  ) => Promise<void>;
  /** 恢复系统默认时钟文本。 */
  handleRestoreClock: () => Promise<void>;
  /** 当前是否正在和后端同步时钟设置。 */
  clockPending: boolean;
}

/** 封装任务栏时钟设置逻辑，并在非 Windows 平台自动跳过。 */
export const useClockManager = (): UseClockManagerResult => {
  /** 防止重复点击时钟设置操作的加载状态。 */
  const [clockPending, setClockPending] = useState<boolean>(false);

  /** 将时间格式与日期格式拼成后端所需的时钟模板文本。 */
  const handleApplyClock = async (
    timeFormatToApply: TrayClockTimeFormat,
    dateFormatToApply: TrayClockDateFormat,
  ): Promise<void> => {
    if (!isWindows) {
      return;
    }
    try {
      setClockPending(true);
      /** 当前后端命令仍要求一个合并后的文本参数。 */
      const text = `${timeFormatToApply}\n${dateFormatToApply}`;
      await invoke('apply_custom_clock_text', { text });
    } finally {
      setClockPending(false);
    }
  };

  /** 恢复 Windows 任务栏默认时钟样式。 */
  const handleRestoreClock = async (): Promise<void> => {
    if (!isWindows) {
      return;
    }
    try {
      setClockPending(true);
      await invoke('restore_default_clock');
    } finally {
      setClockPending(false);
    }
  };

  return {
    handleApplyClock,
    handleRestoreClock,
    clockPending,
  };
};
