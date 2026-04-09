import type { Dayjs } from 'dayjs';
import { HolidayUtil } from 'lunar-typescript';

/** 距离下一个「非工作日」法定节假日的展示文案所需字段 */
export type HolidayCountdownInfo = {
  name: string;
  date: string;
  days: number;
};

/**
 * 从今天起向后最多扫一年，找到下一个休假日并计算剩余天数（用于页脚倒计时）。
 * `today` 为占位 `dayjs(0)`（后端时间未到）时不展示倒计时。
 * 若**当天**已是休假日（剩余 0 天），则跳过该日，继续找之后的下一个休假日。
 */
export function useHolidayCountdown(today: Dayjs): HolidayCountdownInfo | null {
  if (today.valueOf() === 0) {
    return null;
  }
  const startOf = today.startOf('day');
  const maxSearchDays = 365;
  for (let i = 0; i < maxSearchDays; i++) {
    const date = startOf.add(i, 'day');
    const h = HolidayUtil.getHoliday(date.year(), date.month() + 1, date.date());
    if (h && !h.isWork()) {
      const diff = date.diff(startOf, 'day');
      if (diff === 0) {
        continue;
      }
      return {
        name: h.getName(),
        date: date.format('YYYY年M月D日'),
        days: diff,
      };
    }
  }
  return null;
}
