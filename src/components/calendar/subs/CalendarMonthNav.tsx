import { CaretDownFilled, CaretUpFilled } from '@ant-design/icons';
import { Button, Tooltip } from 'antd';
import type { Dayjs } from 'dayjs';
import type { ReactElement } from 'react';
import { useCalendarViewContext } from '../../../hooks/calender/CalendarViewContext.tsx';
import type { CalendarViewClassNames } from '../../../styles/useCalendarViewStyles.ts';

export interface CalendarMonthNavProps {
  /** 样式 class 映射 */
  styles: CalendarViewClassNames;
  /** 当前面板显示的月份（与网格首行对齐） */
  panelMonth: Dayjs;
  /** 选中今天并跳到当月 */
  onGoToToday: () => void;
  /** 上一个月 */
  onPrevMonth: () => void;
  /** 下一个月 */
  onNextMonth: () => void;
}

/**
 * 年月标题与「今天」、上/下月切换控件（数据来自 `CalendarViewContext`）。
 */
function CalendarMonthNav(): ReactElement {
  const { navProps } = useCalendarViewContext();
  const { styles, panelMonth, onGoToToday, onPrevMonth, onNextMonth } = navProps;

  return (
    <div className={styles.calendarNav}>
      <div className={styles.navTitle}>
        {panelMonth.year()}年{panelMonth.month() + 1}月
      </div>
      <div className={styles.navBtns}>
        <Tooltip title="回到今天">
          <Button
            autoInsertSpace={false}
            // className={styles.todayBtn}
            size="small"
            type="text"
            shape="circle"
            onClick={onGoToToday}
          >
            今
          </Button>
        </Tooltip>
        <Tooltip title="上个月">
          <Button
            // className={styles.navBtn}
            size="small"
            type="text"
            shape="circle"
            onClick={onPrevMonth}
            icon={<CaretUpFilled />}
          />
        </Tooltip>
        <Tooltip title="下个月">
          <Button
            // className={styles.navBtn}
            size="small"
            type="text"
            shape="circle"
            onClick={onNextMonth}
            icon={<CaretDownFilled />}
          />
        </Tooltip>
      </div>
    </div>
  );
}

export default CalendarMonthNav;
