import type { Dayjs } from 'dayjs';

/**
 * 根据面板所在月份，生成月历网格中 42 个单元格对应的公历日期（含上月尾部与下月头部）。
 * @param panelMonth 当前面板显示的月份
 * @param weekStart 'monday'（默认，周一开头）或 'sunday'（周日开头）
 */
export function buildMonthCells(
  panelMonth: Dayjs,
  weekStart: 'monday' | 'sunday' = 'monday',
): Dayjs[] {
  const startOfMonth = panelMonth.startOf('month');
  /** 根据周起始计算第一个格子相对于月初的偏移天数 */
  const startWeekday =
    weekStart === 'sunday'
      ? startOfMonth.day() // 周日(0)→列0, 周一(1)→列1 …
      : (startOfMonth.day() + 6) % 7; // 周一(1)→0, 周日(0)→6 …
  const totalCells = 42;
  const firstCellDate = startOfMonth.subtract(startWeekday, 'day');
  return Array.from({ length: totalCells }, (_, index) => firstCellDate.add(index, 'day'));
}
