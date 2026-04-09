import { ClockCircleOutlined } from '@ant-design/icons';
import { Tooltip } from 'antd';
import classNames from 'classnames';
import type { Lunar } from 'lunar-typescript';
import React, { type ReactElement } from 'react';
import { useCalendarViewContext } from '../../../hooks/calender/CalendarViewContext.tsx';
import type { HolidayCountdownInfo } from '../../../hooks/calender/useHolidayCountdown.ts';
import type { CalendarViewClassNames } from '../../../styles/useCalendarViewStyles.ts';

export interface CalendarFooterProps {
  /** 样式 class 映射 */
  styles: CalendarViewClassNames;
  /** 是否展示节日列表区块 */
  hasFestivalSection: boolean;
  /** 是否展示宜忌区块 */
  hasYiJiSection: boolean;
  /** 是否展示距离下次休假倒计时 */
  hasCountdownSection: boolean;
  /** 选中日的节日与节气名称列表 */
  selectedFestivals: string[];
  /** 点击某一节日名时外链百科 */
  onFestivalClick: (name: string) => void;
  /** 选中日农历，用于宜忌文案 */
  selectedLunar: Lunar;
  /** 下一个休假倒计时数据；无则倒计时区不渲染 */
  holidayCountdown: HolidayCountdownInfo | null;
}

/**
 * 底部信息区：无页脚内容时不渲染；否则展示节日、宜忌、倒计时（数据来自 `CalendarViewContext`）。
 */
function CalendarFooter(): ReactElement | null {
  const { footerProps } = useCalendarViewContext();
  if (!footerProps) {
    return null;
  }

  const {
    styles,
    hasFestivalSection,
    hasYiJiSection,
    hasCountdownSection,
    selectedFestivals,
    onFestivalClick,
    selectedLunar,
    holidayCountdown,
  } = footerProps;

  return (
    <div className={styles.footerInfo}>
      {hasFestivalSection && (
        <div className={styles.festivalSection}>
          {selectedFestivals.length > 0 ? (
            <div className={styles.festivalList}>
              {selectedFestivals.map((name, index) => (
                <React.Fragment key={`${name}-${index}`}>
                  <Tooltip title={`在百度百科中查看 ${name}`}>
                    <button
                      type="button"
                      className={styles.festivalItem}
                      onClick={() => onFestivalClick(name)}
                    >
                      {name}
                    </button>
                  </Tooltip>
                  {index < selectedFestivals.length - 1 && (
                    <span className={styles.festivalSeparator}>·</span>
                  )}
                </React.Fragment>
              ))}
            </div>
          ) : (
            <div className={styles.festivalEmpty}>当前无节假日</div>
          )}
        </div>
      )}
      {hasFestivalSection && hasYiJiSection && <div className={styles.footerDivider} />}
      {hasYiJiSection && (
        <div className={styles.footerMain}>
          <div className={styles.yiJiContainer}>
            <div className={styles.yiJiItem}>
              <div className={classNames(styles.yiJiBadge, styles.yiBadge)}>宜</div>
              <Tooltip title={selectedLunar.getDayYi().join(' · ')}>
                <div className={styles.yiJiText}>{selectedLunar.getDayYi().join(' · ')}</div>
              </Tooltip>
            </div>
            <div className={styles.yiJiItem}>
              <div className={classNames(styles.yiJiBadge, styles.jiBadge)}>忌</div>
              <Tooltip title={selectedLunar.getDayJi().join(' · ')}>
                <div className={styles.yiJiText}>{selectedLunar.getDayJi().join(' · ')}</div>
              </Tooltip>
            </div>
          </div>
        </div>
      )}
      {(hasFestivalSection || hasYiJiSection) && hasCountdownSection && (
        <div className={styles.footerDivider} />
      )}
      {hasCountdownSection && holidayCountdown && (
        <div className={styles.countdown}>
          <ClockCircleOutlined className={styles.countdownIcon} />
          <span>
            距离 {holidayCountdown.date} {holidayCountdown.name} 还有 {holidayCountdown.days} 天
          </span>
        </div>
      )}
    </div>
  );
}

export default CalendarFooter;
