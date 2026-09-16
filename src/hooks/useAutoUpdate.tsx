import { getVersion } from '@tauri-apps/api/app';
import { Modal } from 'antd';
import { type ReactElement, useEffect } from 'react';
import { checkForUpdate } from '../http/github.ts';
import { useConfigSync } from '../sync/configStore.ts';
import type { UpdateCheckFrequency } from '../sync/type/configTypes.ts';
import { isDesktop } from '../utils/platform.ts';

/** 频率 → 毫秒间隔映射 */
const FREQUENCY_MS: Record<UpdateCheckFrequency, number> = {
  daily: 24 * 60 * 60 * 1000,
  weekly: 7 * 24 * 60 * 60 * 1000,
  monthly: 30 * 24 * 60 * 60 * 1000,
  yearly: 365 * 24 * 60 * 60 * 1000,
};

/** 判断是否到达了下次检查时间 */
function shouldCheckNow(frequency: UpdateCheckFrequency, lastCheck: string): boolean {
  if (!lastCheck) return true;
  const elapsed = Date.now() - new Date(lastCheck).getTime();
  return elapsed >= FREQUENCY_MS[frequency];
}

/**
 * 自动更新检测 hook。
 *
 * 仅在桌面端主窗口（Settings）生效，避免弹窗/桌面组件重复检查。
 * 启动时按用户配置的频率检测新版本，发现更新后弹窗引导一键下载→安装→重启。
 * 若 updater 插件未配置（pubkey 为空），自动回退到 GitHub API + 浏览器下载。
 */
export function useAutoUpdate(): ReactElement {
  const { data: config, sync: syncConfig } = useConfigSync();
  const [modal, modalContextHolder] = Modal.useModal();

  useEffect(() => {
    // 仅桌面端主窗口（Settings 页）执行自动检查
    if (!isDesktop) return;
    const params = new URLSearchParams(window.location.search);
    if (params.get('window') !== null) return;

    if (!config.autoCheckUpdate) return;
    if (!shouldCheckNow(config.updateCheckFrequency, config.lastUpdateCheck)) return;

    // 同一会话内只检查一次，防止多窗口重复弹窗
    if (sessionStorage.getItem('li-update-checked')) return;
    sessionStorage.setItem('li-update-checked', '1');

    void syncConfig('lastUpdateCheck', new Date().toISOString());

    const run = async (): Promise<void> => {
      const currentVersion = await getVersion();

      /* ---- 方式一：updater 插件（支持下载→安装→重启） ---- */
      try {
        const { check } = await import('@tauri-apps/plugin-updater');
        const update = await check();
        if (update?.available) {
          showUpdaterModal(modal, update, currentVersion);
          return;
        }
        return; // 已是最新
      } catch {
        // updater 插件未配置或失败 → 回退 GitHub API
      }

      /* ---- 方式二：GitHub API 回退（仅提示前往下载） ---- */
      try {
        const result = await checkForUpdate(currentVersion);
        if (result.hasUpdate) {
          showGithubFallbackModal(modal, result.release.tagName, result.release.htmlUrl);
        }
      } catch {
        /* 静默：网络不通时不打扰用户 */
      }
    };

    void run();
  }, [
    config.autoCheckUpdate,
    config.updateCheckFrequency,
    config.lastUpdateCheck,
    syncConfig,
    modal,
  ]);

  return modalContextHolder;
}

/* ------------------------------------------------------------------ */
/*  弹窗辅助                                                           */
/* ------------------------------------------------------------------ */

/** updater 插件可用时：显示更新日志 + 一键“立即更新”（下载→安装→重启） */
function showUpdaterModal(
  modal: ReturnType<typeof Modal.useModal>[0],
  update: {
    version: string;
    body?: string;
    downloadAndInstall: (
      onEvent?: (e: { event: string; data?: Record<string, unknown> }) => void,
    ) => Promise<void>;
  },
  currentVersion: string,
): void {
  let progressText = '';
  let downloadedBytes = 0;
  let totalBytes = 0;

  modal.confirm({
    title: `发现新版本 v${update.version}`,
    content: (
      <div>
        <div style={{ color: '#888', marginBottom: 8 }}>当前版本 v{currentVersion}</div>
        <div style={{ whiteSpace: 'pre-wrap', maxHeight: 240, overflowY: 'auto' }}>
          {update.body || '暂无更新说明'}
        </div>
        <div id="update-progress" style={{ marginTop: 8, color: '#1677ff' }} />
      </div>
    ),
    okText: '立即更新',
    cancelText: '稍后',
    width: 520,
    onOk: async () => {
      try {
        await update.downloadAndInstall((e) => {
          if (e.event === 'Started') {
            totalBytes = (e.data as { contentLength?: number })?.contentLength ?? 0;
            progressText = '正在下载…';
          } else if (e.event === 'Progress') {
            const chunk = (e.data as { chunkLength?: number })?.chunkLength ?? 0;
            downloadedBytes += chunk;
            if (totalBytes > 0) {
              const pct = Math.round((downloadedBytes / totalBytes) * 100);
              progressText = `正在下载… ${pct}%`;
            } else {
              progressText = `正在下载… ${(downloadedBytes / 1024 / 1024).toFixed(1)} MB`;
            }
          } else if (e.event === 'Finished') {
            progressText = '下载完成，正在安装…';
          }
          const el = document.getElementById('update-progress');
          if (el) el.textContent = progressText;
        });
        // 安装成功 → 提示重启
        modal.info({
          title: '更新已安装',
          content: '新版本已安装完成，需要重启应用以生效。',
          okText: '立即重启',
          onOk: async () => {
            const { relaunch } = await import('@tauri-apps/plugin-process');
            await relaunch();
          },
        });
      } catch (err) {
        modal.error({
          title: '更新失败',
          content: `下载或安装更新时出错：${err instanceof Error ? err.message : String(err)}`,
        });
      }
    },
  });
}

/** updater 插件不可用时：回退到 GitHub API，引导用户浏览器下载 */
function showGithubFallbackModal(
  modal: ReturnType<typeof Modal.useModal>[0],
  tagName: string,
  htmlUrl: string,
): void {
  modal.confirm({
    title: `发现新版本 ${tagName}`,
    content: '点击"前往下载"将在浏览器中打开下载页面。',
    okText: '前往下载',
    cancelText: '稍后',
    onOk: async () => {
      const { openUrl } = await import('@tauri-apps/plugin-opener');
      await openUrl(htmlUrl);
    },
  });
}
