import { invoke } from '@tauri-apps/api/core';
import { useEffect } from 'react';
import { useConfigSync } from '../../sync/configStore.ts';
import { isWindows } from '../../utils/platform.ts';

/**
 * 任意桌面窗口挂载时，根据已持久化的配置同步 Windows 任务栏自定义时钟。
 * 放在 `App` 根组件，避免依赖「设置页是否打开」。
 */
export function useWindowsTrayClockBootstrap(): void {
  const { data: config, initialized } = useConfigSync();

  useEffect(() => {
    if (!isWindows || !initialized) {
      return;
    }
    if (!config.customTrayClockEnabled || !config.timeFormat || !config.dateFormat) {
      return;
    }
    const text = `${config.timeFormat}\n${config.dateFormat}`;
    void invoke('apply_custom_clock_text', { text });
  }, [initialized, config.customTrayClockEnabled, config.timeFormat, config.dateFormat]);
}
