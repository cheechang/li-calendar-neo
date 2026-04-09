import { openUrl } from '@tauri-apps/plugin-opener';

/**
 * 在系统默认浏览器中打开百度百科对应节日词条。
 */
export async function openFestivalBaike(name: string): Promise<void> {
  const url = `https://baike.baidu.com/item/${encodeURIComponent(name)}`;
  try {
    await openUrl(url);
  } catch (error) {
    console.error('Failed to open browser', error);
  }
}
