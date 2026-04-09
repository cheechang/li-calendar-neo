import { invoke } from '@tauri-apps/api/core';
import { Form, Switch } from 'antd';
import React, { useState } from 'react';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';
import { isDesktop, isWindows } from '../../../utils/platform.ts';

const WidgetShowForm: React.FC = () => {
  const { data: config } = useConfigSync();

  /** 桌面组件开关的提交加载态。 */
  const [desktopWidgetLoading, setDesktopWidgetLoading] = useState<boolean>(false);
  /** 任务栏弹窗开关的提交加载态。 */
  const [taskbarWidgetLoading, setTaskbarWidgetLoading] = useState<boolean>(false);

  if (!isDesktop) {
    return null;
  }

  // 处理桌面组件开关变化
  const handleDesktopWidgetEnabledChange = async (checked: boolean): Promise<void> => {
    setDesktopWidgetLoading(true);
    try {
      await invoke('set_desktop_widget_enabled', { enabled: checked });
    } catch (err) {
      console.error('设置桌面组件开关失败:', err);
    } finally {
      setDesktopWidgetLoading(false);
    }
  };

  // 处理任务栏弹窗组件开关变化
  const handleTaskbarWidgetEnabledChange = async (checked: boolean): Promise<void> => {
    setTaskbarWidgetLoading(true);
    try {
      await invoke('set_taskbar_widget_enabled_command', { enabled: checked });
    } catch (err) {
      console.error('设置任务栏弹窗开关失败:', err);
    } finally {
      setTaskbarWidgetLoading(false);
    }
  };

  return (
    <Form
      labelCol={{ span: 5 }}
      wrapperCol={{ span: 14 }}
      labelAlign="left"
      colon={false}
      initialValues={config}
      onValuesChange={syncValuesConfig}
    >
      {isWindows && (
        <div>
          <Form.Item name="desktopWidgetEnabled" label="桌面组件">
            <Switch
              loading={desktopWidgetLoading}
              disabled={desktopWidgetLoading}
              onChange={handleDesktopWidgetEnabledChange}
            />
          </Form.Item>
          <Form.Item name="taskbarWidgetEnabled" label="替换任务栏日历">
            <Switch
              loading={taskbarWidgetLoading}
              disabled={taskbarWidgetLoading}
              onChange={handleTaskbarWidgetEnabledChange}
            />
          </Form.Item>
        </div>
      )}
    </Form>
  );
};

export default WidgetShowForm;
