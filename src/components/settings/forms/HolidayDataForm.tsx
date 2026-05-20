import { Button, Form, Input, message, Radio, Space, Typography } from 'antd';
import React, { useState } from 'react';
import { refreshHolidayData } from '../../../services/holidayService.ts';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';

const { Text } = Typography;

const HolidayDataForm: React.FC = () => {
  const { data: config } = useConfigSync();
  const [loading, setLoading] = useState(false);
  const [refreshResult, setRefreshResult] = useState<string | null>(null);

  const { holidayDataSource, holidayRemoteUrl, holidayLastUpdated, holidayDataCache } = config;

  const handleRefresh = async () => {
    if (!holidayRemoteUrl?.trim()) {
      message.warning('请先输入远程 URL');
      return;
    }
    setLoading(true);
    setRefreshResult(null);
    const result = await refreshHolidayData();
    setLoading(false);
    if (result.error) {
      message.error(`刷新失败: ${result.error}`);
    } else if (result.lastResult) {
      message.success(`已更新 ${result.lastResult.count} 条节假日数据`);
      setRefreshResult(`覆盖年份: ${result.lastResult.yearRange}`);
    }
  };

  const formatLastUpdated = (iso: string) => {
    if (!iso) return '从未更新';
    try {
      return new Date(iso).toLocaleString('zh-CN');
    } catch {
      return iso;
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
      <Form.Item name="holidayDataSource" label="数据来源">
        <Radio.Group>
          <Radio value="builtin">使用内置数据</Radio>
          <Radio value="remote">使用远程 JSON</Radio>
        </Radio.Group>
      </Form.Item>

      {holidayDataSource === 'remote' && (
        <>
          <Form.Item
            name="holidayRemoteUrl"
            label="远程 URL"
            rules={[{ type: 'url', message: '请输入有效的 URL' }]}
          >
            <Input placeholder="https://example.com/holidays.json" />
          </Form.Item>

          <Form.Item label=" ">
            <Space>
              <Button type="primary" loading={loading} onClick={handleRefresh}>
                立即刷新
              </Button>
              {refreshResult && <Text type="success">{refreshResult}</Text>}
            </Space>
          </Form.Item>

          <Form.Item label="状态">
            <Space direction="vertical" size={0}>
              <Text type="secondary">上次更新: {formatLastUpdated(holidayLastUpdated)}</Text>
              <Text type="secondary">缓存条数: {holidayDataCache?.length || 0}</Text>
            </Space>
          </Form.Item>
        </>
      )}
    </Form>
  );
};

export default HolidayDataForm;
