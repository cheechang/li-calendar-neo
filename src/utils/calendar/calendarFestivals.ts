import type { Dayjs } from 'dayjs';
import { Solar } from 'lunar-typescript';

/**
 * 需要从 `Lunar.getOtherFestivals()` 中额外纳入的重要传统节日。
 * 库内默认仅展示 FESTIVAL 级别，这些传统节日被归为 OTHER_FESTIVAL，
 * 但对用户而言具有较高认知度，因此选择性补充。
 */
const EXTRA_TRADITIONAL_FESTIVALS = new Set([
  '上巳节', // 农历三月初三
  '寒食节', // 清明前一天（动态计算）
  '中元节', // 农历七月十五
  '下元节', // 农历十月十五
]);

/**
 * 收集某日阳历、农历的主要节日名称，并选择性纳入重要传统节日。
 */
export function getAllFestivals(date: Dayjs): string[] {
  const solar = Solar.fromDate(date.toDate());
  const lunar = solar.getLunar();
  const festivals: string[] = [];

  // 阳历节日 + 农历主要节日
  festivals.push(...solar.getFestivals());
  festivals.push(...lunar.getFestivals());

  // 补充重要传统节日（来自 OTHER_FESTIVAL，排除小众纪念日）
  for (const name of lunar.getOtherFestivals()) {
    if (EXTRA_TRADITIONAL_FESTIVALS.has(name)) {
      festivals.push(name);
    }
  }

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

/** 表头星期行：周一至周日单字（默认） */
export const WEEKDAYS_MONDAY_FIRST = ['一', '二', '三', '四', '五', '六', '日'];

/** 表头星期行：周日至周六单字（周日开头） */
export const WEEKDAYS_SUNDAY_FIRST = ['日', '一', '二', '三', '四', '五', '六'];

/** @deprecated 兼容旧引用，请使用 WEEKDAYS_MONDAY_FIRST */
export const weekdays = WEEKDAYS_MONDAY_FIRST;

/** 顶栏完整星期名称，索引与 dayjs().day() 一致（0=周日） */
export const weekdayNames = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六'];
