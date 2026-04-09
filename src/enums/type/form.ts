import * as React from 'react';

export type OptionType = {
  value: string;
  label: string | React.ReactNode;
  // 下面的是前端显示时才需要的
  disabled?: boolean;
  title?: string;
};
