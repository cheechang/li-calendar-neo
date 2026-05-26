import { Form, Slider, type SliderSingleProps } from 'antd';
import React from 'react';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';

/** 字体大小滑块刻度 */
const marks: SliderSingleProps['marks'] = {
  12: '12px',
  14: '14px',
  16: '16px',
  18: '18px',
  20: '20px',
  22: '22px',
  24: '24px',
};

/** 通用设置：字体大小调节 */
const FontSizeForm: React.FC = () => {
  const { data: config } = useConfigSync();

  return (
    <div style={{ marginBottom: 24 }}>
      <h3 style={{ fontSize: 16, fontWeight: 600, marginBottom: 16, color: 'var(--text-main)' }}>
        字体设置
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
          labelCol={{ span: 5 }}
          wrapperCol={{ span: 19 }}
          labelAlign="left"
          colon={false}
          initialValues={config}
          onValuesChange={syncValuesConfig}
          style={{ marginBottom: 0 }}
        >
          <Form.Item name="fontSize" label="字体大小" style={{ marginBottom: 0 }}>
            <Slider
              marks={marks}
              min={12}
              max={24}
              step={1}
              tooltip={{ formatter: (value: number | undefined): string => `${value ?? 14}px` }}
            />
          </Form.Item>
        </Form>
      </div>
    </div>
  );
};

export default FontSizeForm;
