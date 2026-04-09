import type { Dayjs } from 'dayjs';

/**
 * 根据面板所在月份，生成月历网格中 42 个单元格对应的公历日期（含上月尾部与下月头部）。
 */
export function buildMonthCells(panelMonth: Dayjs): Dayjs[] {
  const startOfMonth = panelMonth.startOf('month');
  /** 将周日(0)映射到列索引 6，周一(1)→0 … 与表头「一二三四五六日」对齐 */
  const startWeekday = (startOfMonth.day() + 6) % 7;
  const totalCells = 42;
  const firstCellDate = startOfMonth.subtract(startWeekday, 'day');
  return Array.from({ length: totalCells }, (_, index) => firstCellDate.add(index, 'day'));
}
