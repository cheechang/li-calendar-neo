import { isTauri as isTauriCore } from '@tauri-apps/api/core';
import { type } from '@tauri-apps/plugin-os';

/** 当前是否运行在 Tauri 容器内。 */
export const isTauri = isTauriCore();
/** 当前是否运行在普通 Web 浏览器环境。 */
export const isWeb = !isTauri;

/** 安全获取运行平台，避免移动端插件尚未就绪时直接抛错。 */
const resolvePlatformType = (): ReturnType<typeof type> | 'unknown' => {
  if (!isTauri) {
    return 'unknown';
  }

  try {
    return type();
  } catch {
    return 'unknown';
  }
};

/** 当前平台名称，无法识别时回退为 unknown。 */
export const currentPlatform = resolvePlatformType();

/** 是否为 Windows 桌面端。 */
export const isWindows = currentPlatform === 'windows';
/** 是否为 macOS 桌面端。 */
export const isMacos = currentPlatform === 'macos';
/** 是否为 Linux 桌面端。 */
export const isLinux = currentPlatform === 'linux';
/** 是否为任意桌面平台。 */
export const isDesktop = isWindows || isMacos || isLinux;

/** 是否为 iOS 移动端。 */
export const isIos = currentPlatform === 'ios';
/** 是否为 Android 移动端。 */
export const isAndroid = currentPlatform === 'android';
/** 是否为任意移动平台。 */
export const isMobile = isIos || isAndroid;
