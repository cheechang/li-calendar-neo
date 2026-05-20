import { listen } from '@tauri-apps/api/event';
import { HolidayUtil } from 'lunar-typescript';
import { useConfigSync } from '../sync/configStore.ts';
import type { HolidayPatchItem, HolidayRemoteData } from '../sync/type/configTypes.ts';

/** fetch 超时时间（毫秒） */
const FETCH_TIMEOUT_MS = 10_000;

/** 服务状态 */
export interface HolidayServiceState {
  loading: boolean;
  error: string | null;
  lastResult: { count: number; yearRange: string } | null;
}

/** 将节日补丁记录编码为 HolidayUtil.fix() 所需的字符串格式 */
function encodeHolidayPatch(
  items: HolidayPatchItem[],
  baseNames?: string[],
): { names: string[]; dataStr: string } {
  const names = baseNames ? [...baseNames] : [...HolidayUtil.NAMES];
  let dataStr = '';

  for (const item of items) {
    const day = item.day.replace(/-/g, '');
    const target = item.target.replace(/-/g, '');

    let nameIndex = names.indexOf(item.name);
    if (nameIndex === -1) {
      nameIndex = names.length;
      names.push(item.name);
    }

    const nameChar = String.fromCharCode(nameIndex + '0'.charCodeAt(0));
    const workChar = item.work ? '0' : '1';

    dataStr += day + nameChar + workChar + target;
  }

  return { names, dataStr };
}

/** 将 HolidayUtil.fix() 字符串格式解码为节日补丁记录（调试用） */
function decodeHolidayPatch(dataStr: string, names?: string[]): HolidayPatchItem[] {
  const size = 18;
  const result: HolidayPatchItem[] = [];
  const useNames = names || HolidayUtil.NAMES;

  for (let i = 0; i + size <= dataStr.length; i += size) {
    const segment = dataStr.substring(i, i + size);
    const day = `${segment.substring(0, 4)}-${segment.substring(4, 6)}-${segment.substring(6, 8)}`;
    const nameIndex = segment.charCodeAt(8) - '0'.charCodeAt(0);
    const work = segment.charCodeAt(9) === '0'.charCodeAt(0);
    const target = `${segment.substring(10, 14)}-${segment.substring(14, 16)}-${segment.substring(16, 18)}`;

    result.push({
      day,
      name: useNames[nameIndex] || `未知(${nameIndex})`,
      work,
      target,
    });
  }

  return result;
}

/** 从远程 URL 拉取节假日 JSON 数据 */
export async function fetchRemoteHolidayData(url: string): Promise<HolidayRemoteData> {
  const controller = new AbortController();
  const timeoutId = window.setTimeout(() => controller.abort(), FETCH_TIMEOUT_MS);

  try {
    const response = await fetch(url, { signal: controller.signal });
    window.clearTimeout(timeoutId);

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data = (await response.json()) as unknown;

    if (!data || typeof data !== 'object') {
      throw new Error('响应不是有效的 JSON 对象');
    }

    const remoteData = data as HolidayRemoteData;

    if (!Array.isArray(remoteData.holidays)) {
      throw new Error('holidays 字段必须是数组');
    }

    // 验证每条记录的基本格式
    for (const h of remoteData.holidays) {
      if (!h.day || typeof h.day !== 'string') {
        throw new Error(`节假日记录缺少 day 字段: ${JSON.stringify(h)}`);
      }
      if (!h.name || typeof h.name !== 'string') {
        throw new Error(`节假日记录缺少 name 字段: ${JSON.stringify(h)}`);
      }
      if (typeof h.work !== 'boolean') {
        throw new Error(`节假日记录 work 必须是布尔值: ${JSON.stringify(h)}`);
      }
      if (!h.target || typeof h.target !== 'string') {
        throw new Error(`节假日记录缺少 target 字段: ${JSON.stringify(h)}`);
      }
    }

    return remoteData;
  } catch (error) {
    window.clearTimeout(timeoutId);
    if (error instanceof Error && error.name === 'AbortError') {
      throw new Error('请求超时');
    }
    throw error;
  }
}

/** 应用节假日补丁到 HolidayUtil */
export function applyHolidayPatch(data: HolidayRemoteData): void {
  if (!data.holidays || data.holidays.length === 0) {
    return;
  }

  const { names, dataStr } = encodeHolidayPatch(data.holidays, data.names);
  HolidayUtil.fix(names, dataStr);

  console.log('[holidayService] 已应用节假日补丁:', {
    count: data.holidays.length,
    namesCount: names.length,
  });
}

/** 从当前配置缓存中应用节假日数据 */
export function applyCachedHolidayData(): void {
  const { holidayDataCache, holidayDataSource } = useConfigSync.getState().data;

  if (holidayDataSource === 'remote' && holidayDataCache && holidayDataCache.length > 0) {
    applyHolidayPatch({ holidays: holidayDataCache });
  }
}

/** 手动刷新节假日数据 */
export async function refreshHolidayData(url?: string): Promise<HolidayServiceState> {
  const targetUrl = url || useConfigSync.getState().data.holidayRemoteUrl;

  if (!targetUrl?.trim()) {
    return { loading: false, error: '未配置远程 URL', lastResult: null };
  }

  try {
    const remoteData = await fetchRemoteHolidayData(targetUrl.trim());
    applyHolidayPatch(remoteData);

    // 更新缓存和最后更新时间
    const now = new Date().toISOString();
    const cache = remoteData.holidays;
    await useConfigSync.getState().sync('holidayDataCache', cache);
    await useConfigSync.getState().sync('holidayLastUpdated', now);

    // 计算覆盖年份范围
    const years = new Set(cache.map((h) => Number(h.day.substring(0, 4))));
    const yearRange = years.size > 0 ? `${Math.min(...years)}-${Math.max(...years)}` : '-';

    return {
      loading: false,
      error: null,
      lastResult: { count: cache.length, yearRange },
    };
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    console.error('[holidayService] 刷新节假日数据失败:', message);
    return {
      loading: false,
      error: message,
      lastResult: null,
    };
  }
}

/** 跨窗口同步监听器是否已设置 */
let syncListenerSetup = false;

/** 应用启动时初始化节假日数据 */
export async function initializeHolidayData(): Promise<void> {
  const { holidayDataSource, holidayRemoteUrl, holidayDataCache } = useConfigSync.getState().data;

  // 1. 如果配置了远程且有缓存，先应用缓存（保证离线时也能使用上次的数据）
  if (holidayDataSource === 'remote' && holidayDataCache && holidayDataCache.length > 0) {
    applyHolidayPatch({ holidays: holidayDataCache });
    console.log('[holidayService] 已从缓存应用节假日数据:', holidayDataCache.length, '条');
  }

  // 2. 如果配置了远程 URL，尝试静默拉取最新数据
  if (holidayDataSource === 'remote' && holidayRemoteUrl && holidayRemoteUrl.trim()) {
    try {
      await refreshHolidayData(holidayRemoteUrl.trim());
    } catch {
      // 静默失败，已应用缓存数据或内置数据
      console.error('[holidayService] 启动时拉取远程数据失败，使用缓存/内置数据');
    }
  }

  // 3. 设置跨窗口同步监听（只设置一次）
  if (!syncListenerSetup) {
    syncListenerSetup = true;
    void listen('sync:liConfig', (event) => {
      const payload = event.payload as Record<string, unknown>;
      if (payload?.delta && typeof payload.delta === 'object') {
        const delta = payload.delta as Record<string, unknown>;
        if ('holidayDataCache' in delta || 'holidayDataSource' in delta) {
          // 延迟执行，确保配置状态已更新
          setTimeout(() => {
            applyCachedHolidayData();
          }, 0);
        }
      }
    });
  }
}

/** 调试用：将当前内置 DATA 解码为人类可读格式 */
export function debugDecodeBuiltinData(): HolidayPatchItem[] {
  return decodeHolidayPatch(HolidayUtil.DATA, HolidayUtil.NAMES);
}
