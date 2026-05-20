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
        <Form
            labelCol={{ span: 5 }}
            wrapperCol={{ span: 14 }}
            labelAlign="left"
            colon={false}
            initialValues={config}
            onValuesChange={syncValuesConfig}
        >
            <Form.Item name="fontSize" label="字体大小">
                <Slider
                    marks={marks}
                    min={12}
                    max={24}
                    step={1}
                    tooltip={{ formatter: (value: number | undefined): string => `${value ?? 14}px` }}
                />
            </Form.Item>
        </Form>
    );
};

export default FontSizeForm;
