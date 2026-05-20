import type { HolidayRemoteData } from '../sync/type/configTypes.ts';

/**
 * 节假日数据源适配器接口。
 * 预留扩展：未来可支持 ICS、CSV、XML 等多种格式。
 */
export interface HolidayAdapter {
  /** 适配器名称 */
  readonly name: string;
  /** 支持的 MIME 类型或文件扩展名 */
  readonly supportedTypes: string[];
  /** 将原始数据解析为标准 HolidayRemoteData 格式 */
  parse(raw: string): HolidayRemoteData;
}

/** JSON 格式适配器（默认） */
export class JsonHolidayAdapter implements HolidayAdapter {
  readonly name = 'JSON';
  readonly supportedTypes = ['application/json', '.json'];

  parse(raw: string): HolidayRemoteData {
    const data = JSON.parse(raw) as unknown;

    if (!data || typeof data !== 'object') {
      throw new Error('JSON 解析结果不是对象');
    }

    const result = data as HolidayRemoteData;

    if (!Array.isArray(result.holidays)) {
      throw new Error('JSON 中缺少 holidays 数组');
    }

    return result;
  }
}

/** ICS 格式适配器（预留，尚未实现完整解析） */
export class IcsHolidayAdapter implements HolidayAdapter {
  readonly name = 'ICS';
  readonly supportedTypes = ['text/calendar', '.ics'];

  parse(_raw: string): HolidayRemoteData {
    // TODO: 实现 ICS (iCalendar) 格式解析
    // 需要处理 VEVENT、RRULE、EXDATE 等字段
    // 并将调休/补班信息映射到 work 字段
    throw new Error('ICS 适配器尚未实现');
  }
}

/** 适配器注册表 */
const adapters: HolidayAdapter[] = [new JsonHolidayAdapter(), new IcsHolidayAdapter()];

/** 根据内容类型或扩展名获取适配器 */
export function getAdapter(contentType?: string, url?: string): HolidayAdapter {
  const type = (contentType || '').toLowerCase();
  const ext = (url || '').toLowerCase();

  for (const adapter of adapters) {
    for (const supported of adapter.supportedTypes) {
      if (type.includes(supported) || ext.endsWith(supported)) {
        return adapter;
      }
    }
  }

  // 默认返回 JSON 适配器
  return adapters[0];
}

/** 注册自定义适配器 */
export function registerAdapter(adapter: HolidayAdapter): void {
  adapters.push(adapter);
}
