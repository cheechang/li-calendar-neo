import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app';
import { arch, platform, version } from '@tauri-apps/plugin-os';

interface SoftInfo {
  appName: string;
  appVersion: string;
  tauriVersion: string;
  os: string;
  osArch: string;
  osVersion: string;
}

export let SOFT_INFO: SoftInfo;

export async function initEnv() {
  SOFT_INFO = {
    appName: await getName(),
    appVersion: await getVersion(),
    tauriVersion: await getTauriVersion(),
    os: platform(),
    osArch: arch(),
    osVersion: version(),
  };
}
