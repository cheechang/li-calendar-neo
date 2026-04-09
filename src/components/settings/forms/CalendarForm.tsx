import { Form, Switch } from 'antd';
import React from 'react';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';

const CalendarForm: React.FC = () => {
  const { data: config } = useConfigSync();

  return (
    <Form
      labelCol={{ span: 5 }}
      wrapperCol={{ span: 14 }}
      labelAlign="left"
      colon={false}
      initialValues={config}
      onValuesChange={syncValuesConfig}
    >
      <Form.Item name="calendarFooterVisible" label="显示底部信息区域">
        <Switch />
      </Form.Item>
      <Form.Item name="footerFestivalVisible" label="显示节假日">
        <Switch />
      </Form.Item>
      <Form.Item name="footerYiJiVisible" label="显示宜忌">
        <Switch />
      </Form.Item>
      <Form.Item name="footerCountdownVisible" label="显示节日倒计时">
        <Switch />
      </Form.Item>
    </Form>
  );
};

export default CalendarForm;
