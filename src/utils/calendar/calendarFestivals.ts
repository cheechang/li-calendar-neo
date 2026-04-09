import type { Dayjs } from 'dayjs';
import { Solar } from 'lunar-typescript';

/**
 * 收集某日阳历、农历的主要节日名称（不含其它小众节日接口）。
 */
export function getAllFestivals(date: Dayjs): string[] {
  const solar = Solar.fromDate(date.toDate());
  const lunar = solar.getLunar();
  const festivals: string[] = [];

  // 仅获取主要节日，排除 getOtherFestivals() (小众/次要节日)
  festivals.push(...solar.getFestivals());
  festivals.push(...lunar.getFestivals());

  return festivals;
}

/**
 * 底部节日区：在 `getAllFestivals` 结果前插入当日节气名（若有），与原先展示顺序一致。
 */
export function getSelectedFestivalsWithJieQi(selectedDate: Dayjs): string[] {
  const solar = Solar.fromDate(selectedDate.toDate());
  const lunar = solar.getLunar();
  const list = getAllFestivals(selectedDate);
  const jieQi = lunar.getJieQi();
  if (jieQi) {
    list.unshift(jieQi);
  }
  return list;
}

/** 表头星期行：周一至周日单字 */
export const weekdays = ['一', '二', '三', '四', '五', '六', '日'];

/** 顶栏完整星期名称，索引与 dayjs().day() 一致（0=周日） */
export const weekdayNames = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六'];
