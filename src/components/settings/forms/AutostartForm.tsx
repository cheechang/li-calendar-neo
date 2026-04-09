import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
import { Form, Switch } from 'antd';
import React, { useEffect, useState } from 'react';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';
import type { ConfigItem } from '../../../sync/type/configTypes.ts';
import { isDesktop } from '../../../utils/platform.ts';

/** 系统相关设置：自启动、窗口透明与纯前端透明度等 */
const AutostartForm: React.FC = () => {
  /** 当前同步配置快照 */
  const { data: config, sync: syncConfig, initialized } = useConfigSync();
  /** 自启动开关与系统通信中时为 true，用于禁用 Checkbox 防重复点击 */
  const [autostartLoading, setAutostartLoading] = useState<boolean>(false);

  /** 桌面端启动时从系统读取自启动是否已开启并写回配置 */
  useEffect(() => {
    if (!isDesktop || !initialized) {
      return;
    }

    setAutostartLoading(true);
    isEnabled()
      .then((enabled: boolean) => {
        // enabled：系统当前是否已启用开机自启动
        syncConfig('autostart' as keyof ConfigItem, enabled);
      })
      .catch((err: unknown) => {
        console.error('获取自启动状态失败:', err);
      })
      .finally(() => {
        setAutostartLoading(false);
      });
  }, [initialized, syncConfig]);

  /** 用户切换「开机自启动」时调用系统 API 并同步配置 */
  const handleAutostartChange = async (checked: boolean): Promise<void> => {
    setAutostartLoading(true);
    try {
      if (checked) {
        await enable();
      } else {
        await disable();
      }
      await syncConfig('autostart' as keyof ConfigItem, checked);
    } catch (err) {
      console.error('设置自启动失败:', err);
    } finally {
      setAutostartLoading(false);
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
      <Form.Item name="autostart" label="开机自启动">
        <Switch disabled={autostartLoading} onChange={handleAutostartChange} />
      </Form.Item>
    </Form>
  );
};

export default AutostartForm;
