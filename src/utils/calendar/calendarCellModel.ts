import { type Dayjs } from 'dayjs';
import { HolidayUtil, Solar } from 'lunar-typescript';
import { getAllFestivals } from './calendarFestivals.ts';

/** 单元格右上角「休 / 班」角标类型：休假日或调休工作日 */
export type CalendarCellBadgeVariant = 'rest' | 'work';

/**
 * 单日格子的展示数据（由纯函数算出，供 CalendarMonthGrid 只做渲染）。
 */
export type CalendarCellViewModel = {
  /** 该格对应的公历日期，用于点击回传 */
  date: Dayjs;
  /** React key 与 Tooltip 等用的稳定字符串 */
  dateKey: string;
  /** 农历行展示文案（日期、初一月份、短节日名或节气） */
  displayText: string;
  /** 是否为节气日（控制农历行样式 class） */
  hasJieQi: boolean;
  /** Tooltip 全文，无节日/节气时为 undefined */
  tooltipTitle: string | undefined;
  /** 是否为当天 */
  isToday: boolean;
  /** 是否为当前选中日期 */
  isSelected: boolean;
  /** 是否不属于面板月份（灰色其它月） */
  isOtherMonth: boolean;
  /** 角标「休」「班」或空字符串 */
  badgeText: string;
  /** 角标配色分支；null 表示不显示角标 */
  badgeVariant: CalendarCellBadgeVariant | null;
};

/**
 * 汇总某日格在月视图中的农历文案、节假日角标、选中态等与 UI 相关的数据。
 */
export function getCalendarCellViewModel(
  date: Dayjs,
  panelMonth: Dayjs,
  selectedDate: Dayjs,
  calendarToday: Dayjs,
): CalendarCellViewModel {
  const solar = Solar.fromDate(date.toDate());
  const lunar = solar.getLunar();
  const jieQi = lunar.getJieQi();
  const allFestivals = getAllFestivals(date);

  let displayText = lunar.getDayInChinese();
  if (jieQi) {
    displayText = jieQi;
  } else if (lunar.getDay() === 1) {
    displayText = `${lunar.getMonthInChinese()}月`;
  } else {
    const shortFestival = allFestivals.find((festival) => festival.length < 4);
    if (shortFestival) {
      displayText = shortFestival;
    }
  }

  const isToday = date.isSame(calendarToday, 'date');
  const isSelected = selectedDate.isSame(date, 'date');
  const isOtherMonth = date.month() !== panelMonth.month();
  const h = HolidayUtil.getHoliday(date.get('year'), date.get('month') + 1, date.get('date'));
  const isHoliday = h?.isWork() === false;
  const isWorkday = h?.isWork() === true;
  const isWeekend = date.day() === 0 || date.day() === 6;
  const badgeText = isHoliday ? '休' : isWorkday && isWeekend ? '班' : '';
  const badgeVariant: CalendarCellBadgeVariant | null = badgeText
    ? isHoliday
      ? 'rest'
      : 'work'
    : null;

  const cellFestivals = [...allFestivals];
  if (jieQi) cellFestivals.unshift(jieQi);

  return {
    date,
    dateKey: date.format('YYYY-MM-DD'),
    displayText,
    hasJieQi: Boolean(jieQi),
    tooltipTitle: cellFestivals.length > 0 ? cellFestivals.join(' · ') : undefined,
    isToday,
    isSelected,
    isOtherMonth,
    badgeText,
    badgeVariant,
  };
}
