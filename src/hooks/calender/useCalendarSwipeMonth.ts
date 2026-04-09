import type { Dayjs } from 'dayjs';
import { type Dispatch, type SetStateAction, type TouchEvent, useRef } from 'react';

export interface UseCalendarSwipeMonthResult {
  handleTouchStart: (e: TouchEvent) => void;
  handleTouchEnd: (e: TouchEvent) => void;
}

/**
 * 纵向滑动手势：上滑下个月、下滑上个月，用于触摸设备换月。
 */
export function useCalendarSwipeMonth(
  setPanelMonth: Dispatch<SetStateAction<Dayjs>>,
): UseCalendarSwipeMonthResult {
  /** 触摸起点 Y，用于计算滑动方向与距离 */
  const touchStartY = useRef<number | null>(null);

  const handleTouchStart = (e: TouchEvent) => {
    touchStartY.current = e.touches[0].clientY;
  };

  const handleTouchEnd = (e: TouchEvent) => {
    if (touchStartY.current === null) return;

    const touchEndY = e.changedTouches[0].clientY;
    const deltaY = touchStartY.current - touchEndY;
    const threshold = 50;

    if (Math.abs(deltaY) > threshold) {
      if (deltaY > 0) {
        setPanelMonth((prev) => prev.add(1, 'month'));
      } else {
        setPanelMonth((prev) => prev.subtract(1, 'month'));
      }
    }

    touchStartY.current = null;
  };

  return { handleTouchStart, handleTouchEnd };
}
