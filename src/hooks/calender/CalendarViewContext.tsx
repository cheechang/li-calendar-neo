import { createContext, type ReactElement, type ReactNode, useContext } from 'react';
import type { CalendarViewModel } from './useCalendarViewModel.ts';

const CalendarViewContext = createContext<CalendarViewModel | null>(null);

export interface CalendarViewProviderProps {
  value: CalendarViewModel;
  children: ReactNode;
}

/**
 * 由 `CalendarView` 注入整棵日历子树的 view model，子组件通过 `useCalendarViewContext` 取用各自片段。
 */
export function CalendarViewProvider({ value, children }: CalendarViewProviderProps): ReactElement {
  return <CalendarViewContext.Provider value={value}>{children}</CalendarViewContext.Provider>;
}

export function useCalendarViewContext(): CalendarViewModel {
  const ctx = useContext(CalendarViewContext);
  if (ctx === null) {
    throw new Error('useCalendarViewContext 必须在 CalendarViewProvider 内使用');
  }
  return ctx;
}
