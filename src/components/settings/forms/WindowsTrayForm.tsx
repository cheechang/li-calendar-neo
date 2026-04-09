import { Button, Flex, Form, Input, Radio, Space, Switch } from 'antd';
import dayjs from 'dayjs';
import React, { useEffect } from 'react';
import 'dayjs/locale/zh-cn';
import zhCn from 'dayjs/locale/zh-cn';
import {
  type TrayClockDateFormat,
  type TrayClockTimeFormat,
  trayClockDatePresets,
  trayClockTimePresets,
} from '../../../enums/trayClockEnum.ts';
import { useClockManager } from '../../../hooks/settings/useClockManager.ts';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';

/** 与 Windows 区域格式一致：`tt` 仅区分 上午 / 下午（不用 dayjs 默认的 凌晨/晚上 等） */
const LOCALE_ZH_CN_WIN_CLOCK = 'zh-cn-winclock';

/** 未经过应用内 `updateLocale` 改写的标准 zh-cn 周几文案（与 Windows 托盘一致；`dddd`=全称、`ddd`=缩写） */
const WIN_TRAY_WEEKDAYS_FULL = '星期日_星期一_星期二_星期三_星期四_星期五_星期六'.split('_');
const WIN_TRAY_WEEKDAYS_SHORT = '周日_周一_周二_周三_周四_周五_周六'.split('_');
const WIN_TRAY_WEEKDAYS_MIN = '日_一_二_三_四_五_六'.split('_');

dayjs.locale(
  {
    ...zhCn,
    name: LOCALE_ZH_CN_WIN_CLOCK,
    meridiem: (hour: number, _minute: number) => (hour < 12 ? '上午' : '下午'),
    weekdays: WIN_TRAY_WEEKDAYS_FULL,
    weekdaysShort: WIN_TRAY_WEEKDAYS_SHORT,
    weekdaysMin: WIN_TRAY_WEEKDAYS_MIN,
  } as ILocale & { meridiem: (hour: number, minute: number) => string },
  undefined,
  true,
);

dayjs.locale('zh-cn');

// 格式令牌映射表，将 Windows 格式转换为 dayjs 格式
const formatTokenMap: Record<string, string> = {
  yyyy: 'YYYY',
  dddd: 'dddd',
  ddd: 'ddd',
  HH: 'HH',
  H: 'H',
  mm: 'mm',
  m: 'm',
  ss: 'ss',
  s: 's',
  tt: 'A',
  h: 'h',
  M: 'M',
  d: 'D',
};
// 格式令牌正则表达式
const formatTokenRegex = /yyyy|dddd|ddd|HH|H|mm|m|ss|s|tt|h|M|d/g;

// 处理引号内的字面量
const wrapQuotedLiterals = (input: string): string => {
  let output = '';
  let literal = '';
  let inQuote = false;
  for (const char of input) {
    if (char === "'") {
      if (inQuote) {
        output += `[${literal}]`;
        literal = '';
        inQuote = false;
      } else {
        inQuote = true;
      }
      continue;
    }
    if (inQuote) {
      literal += char;
    } else {
      output += char;
    }
  }
  if (literal) {
    output += `[${literal}]`;
  }
  return output;
};

// 将 Windows 时钟格式转换为 dayjs 格式
const convertToDayjsFormat = (format: string): string => {
  if (!format) {
    return '';
  }
  const withLiterals = wrapQuotedLiterals(format);
  const literals: string[] = [];
  const placeholder = withLiterals.replace(/\[[^\]]*]/g, (match: string) => {
    const index = literals.length;
    literals.push(match);
    return `__LITERAL_${index}__`;
  });
  const converted = placeholder.replace(
    formatTokenRegex,
    (token: string) => formatTokenMap[token] ?? token,
  );
  return literals.reduce((result, literalValue, index) => {
    const placeholderToken = `__LITERAL_${index}__`;
    return result.split(placeholderToken).join(literalValue);
  }, converted);
};

/** Windows 托盘时钟预览：始终用专用 locale（全局 `zh-cn` 被日历改成「周一」短名，会破坏 `dddd` 与 Windows 一致） */
const formatTrayClockPreview = (now: dayjs.Dayjs, windowsFormat: string): string => {
  const dayjsFmt = convertToDayjsFormat(windowsFormat);
  return now.locale(LOCALE_ZH_CN_WIN_CLOCK).format(dayjsFmt);
};

const WindowsTrayForm: React.FC = () => {
  const { data: config } = useConfigSync();
  const [form] = Form.useForm();
  const { handleApplyClock, handleRestoreClock, clockPending } = useClockManager();
  const [previewNow, setPreviewNow] = React.useState<dayjs.Dayjs>(() => dayjs());

  // 定时更新预览时间
  useEffect(() => {
    const timer = setInterval(() => {
      setPreviewNow(dayjs());
    }, 1000);
    return () => {
      clearInterval(timer);
    };
  }, []);

  return (
    <Form form={form} initialValues={config} onValuesChange={syncValuesConfig}>
      <Form.Item name="customTrayClockEnabled" label="自定义托盘时钟">
        <Switch
          onChange={(checked: boolean) => {
            if (checked) {
              void handleApplyClock(config.timeFormat, config.dateFormat);
              return;
            }
            void handleRestoreClock();
          }}
        />
      </Form.Item>
      <Form.Item name="timeFormat" label="时间预设">
        <Radio.Group
          disabled={!config.customTrayClockEnabled}
          onChange={async (e) => {
            const value = e.target.value as TrayClockTimeFormat;
            const picked = trayClockTimePresets.find((tf) => tf === value);
            if (picked !== undefined) {
              form.setFieldValue('timeFormat', picked);
              await handleApplyClock(picked, config.dateFormat);
            }
          }}
          optionType="button"
          buttonStyle="solid"
        >
          {trayClockTimePresets.map((timeFormat) => {
            const preview = formatTrayClockPreview(previewNow, timeFormat);
            return (
              <Radio.Button key={timeFormat} value={timeFormat}>
                {preview}
              </Radio.Button>
            );
          })}
        </Radio.Group>
      </Form.Item>
      <Form.Item name="dateFormat" label="日期预设">
        <Radio.Group
          disabled={!config.customTrayClockEnabled}
          onChange={async (e) => {
            const value = e.target.value as TrayClockDateFormat;
            const picked = trayClockDatePresets.find((df) => df === value);
            if (picked !== undefined) {
              form.setFieldValue('dateFormat', picked);
              await handleApplyClock(config.timeFormat, picked);
            }
          }}
          optionType="button"
          buttonStyle="solid"
        >
          {trayClockDatePresets.map((dateFormat) => {
            const preview = formatTrayClockPreview(previewNow, dateFormat);
            return (
              <Radio.Button key={dateFormat} value={dateFormat}>
                {preview}
              </Radio.Button>
            );
          })}
        </Radio.Group>
      </Form.Item>
      <Form.Item label="时间格式">
        <Flex gap="small" align="flex-start">
          <Form.Item name="timeFormat" noStyle>
            <Input
              disabled={!config.customTrayClockEnabled}
              value={config.timeFormat}
              placeholder="如: HH:mm"
              style={{ flex: 1 }}
            />
          </Form.Item>
          <div
            style={{
              minWidth: 120,
              textAlign: 'center',
              padding: '4px 8px',
              background: '#f5f5f5',
            }}
          >
            {formatTrayClockPreview(previewNow, config.timeFormat)}
          </div>
        </Flex>
      </Form.Item>
      <Form.Item label="日期格式">
        <Flex gap="small" align="flex-start">
          <Form.Item name="dateFormat" noStyle>
            <Input
              disabled={!config.customTrayClockEnabled}
              value={config.dateFormat}
              placeholder="如: M月d日"
              style={{ flex: 1 }}
            />
          </Form.Item>
          <div
            style={{
              minWidth: 120,
              textAlign: 'center',
              padding: '4px 8px',
              background: '#f5f5f5',
            }}
          >
            {formatTrayClockPreview(previewNow, config.dateFormat)}
          </div>
        </Flex>
      </Form.Item>
      <Form.Item>
        <Space>
          <Button
            disabled={!config.customTrayClockEnabled}
            type="primary"
            onClick={() => handleApplyClock(config.timeFormat, config.dateFormat)}
            loading={clockPending}
          >
            应用
          </Button>
        </Space>
      </Form.Item>
    </Form>
  );
};

export default WindowsTrayForm;
