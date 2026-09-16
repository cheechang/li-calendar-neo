import { Tooltip } from 'antd';
import classNames from 'classnames';
import type { Dayjs } from 'dayjs';
import type { ReactElement } from 'react';
import { useLayoutEffect, useRef } from 'react';
import { useCalendarViewContext } from '../../../hooks/calender/CalendarViewContext.tsx';
import type { CalendarViewClassNames } from '../../../styles/useCalendarViewStyles.ts';
import type { CalendarCellViewModel } from '../../../utils/calendar/calendarCellModel.ts';

/** 一行 7 格，共 6 行 */
const CELLS_PER_ROW = 7;

export interface CalendarMonthGridProps {
  /** antd-style 生成的 className 映射 */
  styles: CalendarViewClassNames;
  /** 已由逻辑层算好的 42 格展示模型 */
  cellModels: CalendarCellViewModel[];
  /** 当前周起始对应的星期表头数组 */
  weekdays: string[];
  /** 用户点击某一格时回传该格公历日期 */
  onSelectDate: (date: Dayjs) => void;
  /** 是否显示其它月份的灰色日期 */
  showOverflowDates: boolean;
  /** 是否在左侧显示周数列 */
  showWeekNumbers: boolean;
  /** 当前面板月份，用于触发滑动动画 */
  panelMonth: Dayjs;
}

/**
 * 月历主体：星期表头 + 日期格子（数据来自 `CalendarViewContext`）。
 */
function CalendarMonthGrid(): ReactElement {
  const { gridProps } = useCalendarViewContext();
  const {
    styles,
    cellModels,
    weekdays,
    onSelectDate,
    showOverflowDates,
    showWeekNumbers,
    panelMonth,
  } = gridProps;

  /** 格子区域 ref，用于 Web Animations API 滑动动画（表头不参与动画） */
  const cellsRef = useRef<HTMLDivElement>(null);
  /** 上一次面板月份，用于判断滑动方向 */
  const prevMonthRef = useRef<Dayjs | null>(null);

  /** 面板月份变化时，播放滑动动画（参考 Windows 11 原生日历行为） */
  useLayoutEffect(() => {
    const el = cellsRef.current;
    const prev = prevMonthRef.current;
    prevMonthRef.current = panelMonth;

    // 首次挂载不播放动画
    if (!el || !prev) return;

    const monthDiff = panelMonth.diff(prev, 'month');
    if (monthDiff === 0) return;

    // 格子区域实际高度，用作动画滑动距离
    const gridH = el.offsetHeight || 280;
    const direction = monthDiff > 0 ? -1 : 1;
    const startOffset = direction * gridH * 0.55;

    el.animate(
      [
        { transform: `translateY(${startOffset}px)`, opacity: 0.4 },
        { transform: 'translateY(0)', opacity: 1 },
      ],
      { duration: 150, easing: 'ease-out' },
    );
  }, [panelMonth]);

  /** 动态网格列：有周数时左侧多一列 */
  const gridColumns = showWeekNumbers ? 'auto repeat(7, 1fr)' : 'repeat(7, 1fr)';

  return (
    <div className={styles.calendarGrid}>
      {/* 表头行：固定在顶部，不参与滑动动画 */}
      <div style={{ display: 'grid', gridTemplateColumns: gridColumns, gap: '1px' }}>
        {showWeekNumbers && <div className={styles.weekNumberHeader} />}
        {weekdays.map((day, index) => (
          <div
            className={classNames(styles.weekday, {
              [styles.weekdayWeekend]: day === '六' || day === '日',
            })}
            key={`${index}-${day}`}
          >
            {day}
          </div>
        ))}
      </div>

      {/* 格子区域：仅此区域参与滑动动画 */}
      <div
        ref={cellsRef}
        className={styles.gridCellsContainer}
        style={{ gridTemplateColumns: gridColumns }}
      >
        {/* 6 行 × 7 列日期格子 */}
        {Array.from({ length: 6 }, (_, rowIndex) => {
          const rowCells = cellModels.slice(
            rowIndex * CELLS_PER_ROW,
            (rowIndex + 1) * CELLS_PER_ROW,
          );
          const rowKey = rowCells[0]?.dateKey ?? rowIndex;

          return (
            <div key={rowKey} style={{ display: 'contents' }}>
              {/* 周数列 */}
              {showWeekNumbers && (
                <div className={styles.weekNumberCell}>{rowCells[0]?.date.isoWeek()}</div>
              )}

              {/* 日期格子 */}
              {rowCells.map((cell) => {
                /** 「休」「班」角标对应不同背景色 class */
                const badgeClass =
                  cell.badgeVariant === 'rest'
                    ? styles.tagRest
                    : cell.badgeVariant === 'work'
                      ? styles.tagWork
                      : '';

                if (!showOverflowDates && cell.isOtherMonth) {
                  return <div key={cell.dateKey} className={styles.cellPlaceholder} />;
                }

                return (
                  <Tooltip key={cell.dateKey} title={cell.tooltipTitle} mouseEnterDelay={0.5}>
                    <button
                      type="button"
                      className={classNames(styles.cell, {
                        [styles.otherMonth]: cell.isOtherMonth,
                        [styles.today]: cell.isToday,
                        [styles.selected]: cell.isSelected && !cell.isToday,
                        [styles.weekend]: cell.date.day() === 0 || cell.date.day() === 6,
                      })}
                      onClick={() => onSelectDate(cell.date)}
                    >
                      {cell.badgeText && (
                        <span className={classNames(styles.tag, badgeClass)}>{cell.badgeText}</span>
                      )}
                      <span className={styles.dateText}>{cell.date.date()}</span>
                      <span className={classNames(styles.lunar, { [styles.term]: cell.hasJieQi })}>
                        {cell.displayText}
                      </span>
                    </button>
                  </Tooltip>
                );
              })}
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default CalendarMonthGrid;
