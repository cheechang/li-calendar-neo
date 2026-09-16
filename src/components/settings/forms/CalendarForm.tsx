import { Col, Form, Row, Switch } from 'antd';
import React from 'react';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';

/** 卡片容器统一样式 */
const cardStyle: React.CSSProperties = {
  background: 'var(--bg-card)',
  borderRadius: 12,
  padding: '16px 20px',
  border: '1px solid var(--border-color)',
};

/** 分类标题 */
const sectionTitle = (text: string): React.ReactNode => (
  <h4
    style={{
      fontSize: 13,
      fontWeight: 600,
      color: 'var(--text-sec)',
      marginBottom: 12,
      marginTop: 0,
    }}
  >
    {text}
  </h4>
);

/** 双列表单项：label 在上、Switch 在下，紧凑排列 */
const SwitchItem: React.FC<{ name: string; label: string; disabled?: boolean }> = ({
  name,
  label,
  disabled,
}) => (
  <Form.Item
    name={name}
    label={label}
    labelCol={{ span: 24 }}
    wrapperCol={{ span: 24 }}
    labelAlign="left"
    colon={false}
    style={{ marginBottom: 8 }}
  >
    <Switch size="small" disabled={disabled} />
  </Form.Item>
);

const CalendarForm: React.FC = () => {
  const { data: config } = useConfigSync();
  /** 底部信息总开关关闭时，子项灰显 */
  const footerDisabled = !config.calendarFooterVisible;

  return (
    <div style={{ marginBottom: 24 }}>
      <h3 style={{ fontSize: 16, fontWeight: 600, marginBottom: 16, color: 'var(--text-main)' }}>
        日历显示
      </h3>
      <div style={cardStyle}>
        <Form
          labelAlign="left"
          colon={false}
          initialValues={config}
          onValuesChange={syncValuesConfig}
          style={{ marginBottom: 0 }}
          layout="horizontal"
        >
          {/* ── 外观 ── */}
          {sectionTitle('外观')}
          <Row gutter={24}>
            <Col span={12}>
              <SwitchItem name="themeFollowSystem" label="自动跟随系统主题" />
            </Col>
          </Row>

          {/* ── 网格 ── */}
          {sectionTitle('网格')}
          <Row gutter={24}>
            <Col span={12}>
              <SwitchItem name="showOverflowDates" label="显示跨越日期" />
            </Col>
            <Col span={12}>
              <SwitchItem name="showWeekNumbers" label="显示周数" />
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={12}>
              <SwitchItem name="weekStartsOnSunday" label="周日作为一周起始" />
            </Col>
          </Row>

          {/* ── 交互 ── */}
          {sectionTitle('交互')}
          <Row gutter={24}>
            <Col span={12}>
              <SwitchItem name="wheelScrollEnabled" label="鼠标滚轮按周滚动" />
            </Col>
          </Row>

          {/* ── 底部信息 ── */}
          {sectionTitle('底部信息')}
          <Row gutter={24}>
            <Col span={12}>
              <SwitchItem name="calendarFooterVisible" label="显示底部信息区域" />
            </Col>
            <Col span={12}>
              <SwitchItem
                name="footerFestivalVisible"
                label="显示节假日"
                disabled={footerDisabled}
              />
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={12}>
              <SwitchItem name="footerYiJiVisible" label="显示宜忌" disabled={footerDisabled} />
            </Col>
            <Col span={12}>
              <SwitchItem
                name="footerCountdownVisible"
                label="显示节日倒计时"
                disabled={footerDisabled}
              />
            </Col>
          </Row>
        </Form>
      </div>
    </div>
  );
};

export default CalendarForm;
