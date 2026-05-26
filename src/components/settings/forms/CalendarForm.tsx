import { Form, Switch } from 'antd';
import React from 'react';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';

const CalendarForm: React.FC = () => {
  const { data: config } = useConfigSync();

  return (
    <div style={{ marginBottom: 24 }}>
      <h3 style={{ fontSize: 16, fontWeight: 600, marginBottom: 16, color: 'var(--text-main)' }}>
        日历显示
      </h3>
      <div
        style={{
          background: 'var(--bg-card)',
          borderRadius: 12,
          padding: '20px 24px',
          border: '1px solid var(--border-color)',
        }}
      >
        <Form
          labelCol={{ span: 8 }}
          wrapperCol={{ span: 16 }}
          labelAlign="left"
          colon={false}
          initialValues={config}
          onValuesChange={syncValuesConfig}
          style={{ marginBottom: 0 }}
          layout="horizontal"
        >
          <Form.Item name="themeFollowSystem" label="自动跟随系统主题" style={{ marginBottom: 16 }}>
            <Switch />
          </Form.Item>
          <Form.Item name="calendarFooterVisible" label="显示底部信息区域" style={{ marginBottom: 16 }}>
            <Switch />
          </Form.Item>
          <Form.Item name="footerFestivalVisible" label="显示节假日" style={{ marginBottom: 16 }}>
            <Switch />
          </Form.Item>
          <Form.Item name="footerYiJiVisible" label="显示宜忌" style={{ marginBottom: 16 }}>
            <Switch />
          </Form.Item>
          <Form.Item name="footerCountdownVisible" label="显示节日倒计时" style={{ marginBottom: 0 }}>
            <Switch />
          </Form.Item>
        </Form>
      </div>
    </div>
  );
};

export default CalendarForm;
