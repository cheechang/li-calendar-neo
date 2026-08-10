import { Tooltip } from 'antd';
import classNames from 'classnames';
import type { Dayjs } from 'dayjs';
import type { ReactElement } from 'react';
import { useCalendarViewContext } from '../../../hooks/calender/CalendarViewContext.tsx';
import type { CalendarViewClassNames } from '../../../styles/useCalendarViewStyles.ts';
import type { CalendarCellViewModel } from '../../../utils/calendar/calendarCellModel.ts';
import { weekdays } from '../../../utils/calendar/calendarFestivals.ts';

/** 一行 7 格，共 6 行 */
const CELLS_PER_ROW = 7;

export interface CalendarMonthGridProps {
  /** antd-style 生成的 className 映射 */
  styles: CalendarViewClassNames;
  /** 已由逻辑层算好的 42 格展示模型 */
  cellModels: CalendarCellViewModel[];
  /** 用户点击某一格时回传该格公历日期 */
  onSelectDate: (date: Dayjs) => void;
  /** 是否显示其它月份的灰色日期 */
  showOverflowDates: boolean;
  /** 是否在左侧显示周数列 */
  showWeekNumbers: boolean;
}

/**
 * 月历主体：星期表头 + 日期格子（数据来自 `CalendarViewContext`）。
 */
function CalendarMonthGrid(): ReactElement {
  const { gridProps } = useCalendarViewContext();
  const { styles, cellModels, onSelectDate, showOverflowDates, showWeekNumbers } = gridProps;

  /** 动态网格列：有周数时左侧多一列 */
  const gridColumns = showWeekNumbers ? 'auto repeat(7, 1fr)' : 'repeat(7, 1fr)';

  return (
    <div className={styles.calendarGrid} style={{ gridTemplateColumns: gridColumns }}>
      {/* 表头：可选周数空位 + 星期 */}
      {showWeekNumbers && <div className={styles.weekNumberHeader} />}
      {weekdays.map((day) => (
        <div className={styles.weekday} key={day}>
          {day}
        </div>
      ))}

      {/* 6 行 × 7 列日期格子 */}
      {Array.from({ length: 6 }, (_, rowIndex) => {
        const rowCells = cellModels.slice(rowIndex * CELLS_PER_ROW, (rowIndex + 1) * CELLS_PER_ROW);
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
  );
}

export default CalendarMonthGrid;
