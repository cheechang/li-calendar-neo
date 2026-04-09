import { invoke } from '@tauri-apps/api/core';
import { Form, Segmented, Slider, SliderSingleProps, Switch } from 'antd';
import React, { useEffect, useState } from 'react';
import { DebugConst } from '../../../debugConst.ts';
import { syncValuesConfig } from '../../../sync/base/syncValuesConfig.ts';
import { useConfigSync } from '../../../sync/configStore.ts';
import type { ConfigItem, MacosVibrancyEffect } from '../../../sync/type/configTypes.ts';
import { isDesktop, isMacos, isWindows } from '../../../utils/platform.ts';

/** 各窗口毛玻璃/材质效果在 Segmented 中展示的文案与取值 */
const effectOptionMap: Record<MacosVibrancyEffect, { label: string; value: MacosVibrancyEffect }> =
  {
    blur: { label: 'blur', value: 'blur' },
    acrylic: { label: 'acrylic', value: 'acrylic' },
    popover: { label: 'popover', value: 'popover' },
    sidebar: { label: 'sidebar', value: 'sidebar' },
    mica: { label: 'mica', value: 'mica' },
    'mica-dark': { label: 'mica-dark', value: 'mica-dark' },
    'mica-light': { label: 'mica-light', value: 'mica-light' },
    'hud-window': { label: 'hud-window', value: 'hud-window' },
    tabbed: { label: 'tabbed', value: 'tabbed' },
    'tabbed-dark': { label: 'tabbed-dark', value: 'tabbed-dark' },
    'tabbed-light': { label: 'tabbed-light', value: 'tabbed-light' },
    'header-view': { label: 'header-view', value: 'header-view' },
    vibrancy: { label: 'vibrancy', value: 'vibrancy' },
    'liquid-glass': { label: 'liquid-glass', value: 'liquid-glass' },
    'under-window-background': {
      label: 'under-window-background',
      value: 'under-window-background',
    },
  };

/** macOS 上默认提供的窗口效果枚举顺序（后端不可用时回退用） */
const macosEffectValues: MacosVibrancyEffect[] = [
  'popover',
  'sidebar',
  'hud-window',
  'header-view',
  'under-window-background',
];

/** Windows 上默认提供的窗口效果枚举顺序（后端不可用时回退用） */
const windowsEffectValues: MacosVibrancyEffect[] = [
  'blur',
  'acrylic',
  'mica',
  'mica-dark',
  'mica-light',
  'tabbed',
  'tabbed-dark',
  'tabbed-light',
];

/** 判断字符串是否为已知的窗口效果键 */
const isSupportedMacosEffect = (value: string): value is MacosVibrancyEffect =>
  value in effectOptionMap;

/**
 * 将历史/跨平台旧枚举映射到当前 macOS 侧支持的效果值，避免配置里残留无效项。
 * @param effect 当前配置中的效果
 */
const normalizeMacosEffect = (effect: MacosVibrancyEffect): MacosVibrancyEffect => {
  switch (effect) {
    case 'blur':
    case 'vibrancy':
      return 'popover';
    case 'acrylic':
      return 'sidebar';
    case 'mica':
    case 'mica-dark':
    case 'mica-light':
      return 'hud-window';
    case 'tabbed':
    case 'tabbed-dark':
    case 'tabbed-light':
      return 'header-view';
    case 'liquid-glass':
      return 'under-window-background';
    default:
      return effect;
  }
};

/** 系统相关设置：自启动、窗口透明与纯前端透明度等 */
const TransparentEffectForm: React.FC = () => {
  /** 当前同步配置快照 */
  const { data: config, sync: syncConfig } = useConfigSync();
  /** 当前环境（或后端）声明支持的窗口效果列表 */
  const [supportedEffects, setSupportedEffects] = useState<Array<MacosVibrancyEffect>>(
    isWindows ? windowsEffectValues : macosEffectValues,
  );

  /** Segmented 用的选项列表（随 supportedEffects 过滤） */
  const effectOptions = supportedEffects.map((effect) => effectOptionMap[effect]);
  /** 当前列表为空或配置无效时的默认效果 */
  const defaultEffect: MacosVibrancyEffect =
    effectOptions[0]?.value ?? (isWindows ? 'acrylic' : 'popover');
  /** 配置中的效果经 macOS 归一化后的值 */
  const currentEffect = isMacos ? normalizeMacosEffect(config.macosEffect) : config.macosEffect;
  /** 最终传给原生层与表单的选中效果（若不在列表中则回落到 defaultEffect） */
  const selectedMacosEffect =
    isSupportedMacosEffect(currentEffect) &&
    effectOptions.some((option) => option.value === currentEffect)
      ? currentEffect
      : defaultEffect;

  /** macOS/Windows 下从 Rust 侧拉取本机支持的窗口效果，失败则用平台默认列表 */
  useEffect(() => {
    const supportsWindowVibrancy = isMacos || isWindows;
    if (!supportsWindowVibrancy) {
      return;
    }

    /** 后端不可用或返回空时的回退列表 */
    const fallbackEffects = isWindows ? windowsEffectValues : macosEffectValues;

    invoke<string[]>('get_supported_window_effects')
      .then((effects) => {
        // 仅保留 UI 能识别的效果键
        const nextEffects = effects.filter(isSupportedMacosEffect);
        setSupportedEffects(nextEffects.length > 0 ? nextEffects : fallbackEffects);
      })
      .catch((error: unknown) => {
        console.error('获取窗口效果列表失败:', error);
        setSupportedEffects(fallbackEffects);
      });
  }, []);

  /** macOS：若配置里存的是旧枚举，自动迁移为当前支持的 effect 并回写 */
  useEffect(() => {
    if (!isMacos) {
      return;
    }

    const normalizedEffect = normalizeMacosEffect(config.macosEffect);
    if (normalizedEffect !== config.macosEffect) {
      syncConfig('macosEffect' as keyof ConfigItem, normalizedEffect);
    }
  }, [config.macosEffect, syncConfig]);

  /** 开关「启用窗口半透明」并通知原生层应用/关闭 vibrancy */
  const handleMacosTransparencyChange = (checked: boolean): void => {
    syncConfig('isWindowsEffect' as keyof ConfigItem, checked);
    if (isMacos || isWindows) {
      invoke('set_macos_vibrancy', {
        enabled: checked,
        effect: selectedMacosEffect,
        windowLabel: 'calendar',
      }).catch(console.error);
    }
  };

  /** 仅同步「纯前端透明效果」开关，不涉及原生窗口 */
  const handleFrontendWindowEffectEnabledChange = (checked: boolean): void => {
    syncConfig('frontendWindowEffectEnabled' as keyof ConfigItem, checked);
  };

  /**
   * Form 任意字段变更：先走通用同步，若改了窗口效果则刷新原生 vibrancy。
   * @param changed 本帧变化的字段
   * @param allValues 表单当前全部值（用于合并计算是否启用半透明等）
   */
  const handleFormValuesChange = async (
    changed: Partial<ConfigItem>,
    allValues: Partial<ConfigItem>,
  ): Promise<void> => {
    await syncValuesConfig(changed);
    const nextEffect = changed.macosEffect;
    if (nextEffect !== undefined && (isMacos || isWindows)) {
      invoke('set_macos_vibrancy', {
        enabled: (allValues.isWindowsEffect ?? config.isWindowsEffect) as boolean,
        effect: nextEffect,
        windowLabel: 'calendar',
      }).catch(console.error);
    }
  };

  /** 透明度滑块刻度文案 */
  const marks: SliderSingleProps['marks'] = {
    0: '0%',
    100: '100%',
  };

  return (
    <Form
      labelCol={{ span: 5 }}
      wrapperCol={{ span: 14 }}
      labelAlign="left"
      colon={false}
      initialValues={config}
      onValuesChange={handleFormValuesChange}
    >
      <Form.Item name="frontendWindowEffectEnabled" label="透明效果">
        <Switch
          checked={config.frontendWindowEffectEnabled}
          onChange={handleFrontendWindowEffectEnabledChange}
        />
      </Form.Item>
      {config.frontendWindowEffectEnabled && (
        <Form.Item name="frontendWindowTransparency" label="透明度">
          <Slider
            marks={marks}
            min={0}
            max={100}
            step={1}
            tooltip={{ formatter: (value: number | undefined): string => `${value ?? 0}%` }}
          />
        </Form.Item>
      )}
      {isDesktop && DebugConst.IS_DEV && (
        <Form.Item name="isWindowsEffect" label="启用窗口半透明">
          <Switch checked={config.isWindowsEffect} onChange={handleMacosTransparencyChange} />
        </Form.Item>
      )}
      {config.isWindowsEffect && (
        <Form.Item name="macosEffect" label="窗口效果" layout="vertical">
          <Segmented options={effectOptions} block />
        </Form.Item>
      )}
    </Form>
  );
};

export default TransparentEffectForm;
