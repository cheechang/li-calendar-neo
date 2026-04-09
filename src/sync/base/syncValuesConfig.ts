import { useConfigSync } from '../configStore.ts';
import type { ConfigItem } from '../type/configTypes.ts';

/**
 * 当表单配置发生变化时，同步更新到状态管理中。并且保存到本地
 * @param changedValues 变更的配置值
 * @returns Promise<void>
 */
export async function syncValuesConfig(
  changedValues: Partial<Record<keyof ConfigItem, unknown>>,
): Promise<void> {
  await Promise.all(
    Object.entries(changedValues).map(([key, value]) =>
      useConfigSync.getState().sync(key as keyof ConfigItem, value as ConfigItem[keyof ConfigItem]),
    ),
  );
}
