import { getVersion } from '@tauri-apps/api/app';
import { Button, Form, message, Select, Space, Switch } from 'antd';
import React, { useState } from 'react';
import { checkForUpdate } from '../../../http/github.ts';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';
import type { UpdateCheckFrequency } from '../../../sync/type/configTypes.ts';

const FREQUENCY_OPTIONS: { value: UpdateCheckFrequency; label: string }[] = [
  { value: 'daily', label: '每天一次' },
  { value: 'weekly', label: '每周一次' },
  { value: 'monthly', label: '每月一次' },
  { value: 'yearly', label: '每年一次' },
];

const UpdateForm: React.FC = () => {
  const { data: config } = useConfigSync();
  const [messageApi, contextHolder] = message.useMessage();
  const [checking, setChecking] = useState(false);

  /** 手动检查更新（与自动检查共用同一回退逻辑） */
  const handleManualCheck = async (): Promise<void> => {
    setChecking(true);
    try {
      const currentVersion = await getVersion();
      /* 优先尝试 updater 插件 */
      try {
        const { check } = await import('@tauri-apps/plugin-updater');
        const update = await check();
        if (update?.available) {
          // updater 可用，触发自动更新弹窗（通过重新触发 hook）
          // 这里直接引导下载→安装→重启
          await update.downloadAndInstall((e) => {
            if (e.event === 'Started') messageApi.loading('正在下载更新…');
            else if (e.event === 'Finished') messageApi.success('下载完成，正在安装…');
          });
          messageApi.success('更新已安装，即将重启…');
          setTimeout(async () => {
            const { relaunch } = await import('@tauri-apps/plugin-process');
            await relaunch();
          }, 1500);
          return;
        }
        messageApi.success('当前已是最新版本');
        return;
      } catch {
        // updater 不可用 → 回退
      }

      /* GitHub API 回退 */
      const result = await checkForUpdate(currentVersion);
      if (result.hasUpdate) {
        const { openUrl } = await import('@tauri-apps/plugin-opener');
        messageApi.info(`发现新版本 ${result.release.tagName}，正在打开下载页面…`);
        await openUrl(result.release.htmlUrl);
      } else {
        messageApi.success('当前已是最新版本');
      }
    } catch {
      messageApi.error('检查更新失败，请检查网络连接');
    } finally {
      setChecking(false);
    }
  };

  return (
    <div style={{ marginBottom: 24 }}>
      {contextHolder}
      <h3 style={{ fontSize: 16, fontWeight: 600, marginBottom: 16, color: 'var(--text-main)' }}>
        软件更新
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
          <Form.Item name="autoCheckUpdate" label="自动检测更新" style={{ marginBottom: 16 }}>
            <Switch />
          </Form.Item>
          <Form.Item name="updateCheckFrequency" label="检测频率" style={{ marginBottom: 16 }}>
            <Select options={FREQUENCY_OPTIONS} disabled={!config.autoCheckUpdate} />
          </Form.Item>
          <Form.Item label="手动检查" style={{ marginBottom: 0 }}>
            <Space>
              <Button loading={checking} onClick={() => void handleManualCheck()}>
                立即检查更新
              </Button>
              {config.lastUpdateCheck && (
                <span style={{ color: '#888', fontSize: 12 }}>
                  上次检查：{new Date(config.lastUpdateCheck).toLocaleString()}
                </span>
              )}
            </Space>
          </Form.Item>
        </Form>
      </div>
    </div>
  );
};

export default UpdateForm;
