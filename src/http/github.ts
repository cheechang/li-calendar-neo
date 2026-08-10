import { GITHUB_REPO_URL } from '../constants/link';

/** GitHub Release API 返回的核心字段 */
export interface GitHubRelease {
  /** 标签名，如 "v1.2.6" */
  tagName: string;
  /** 版本号，去掉 "v" 前缀 */
  version: string;
  /** 发布说明 / 变更日志 */
  body: string;
  /** Release 页面地址 */
  htmlUrl: string;
}

/**
 * 比较两个语义化版本号，返回 1 (a>b)、-1 (a<b) 或 0 (相等)。
 */
function compareSemver(a: string, b: string): number {
  const pa = a.split('.').map(Number);
  const pb = b.split('.').map(Number);
  for (let i = 0; i < 3; i++) {
    const na = pa[i] ?? 0;
    const nb = pb[i] ?? 0;
    if (na > nb) return 1;
    if (na < nb) return -1;
  }
  return 0;
}

/**
 * 从 GitHub Release API 获取最新发布信息。
 */
async function fetchLatestRelease(): Promise<GitHubRelease> {
  const apiUrl = `${GITHUB_REPO_URL.replace('github.com', 'api.github.com/repos')}/releases/latest`;
  const response = await fetch(apiUrl, {
    headers: { Accept: 'application/vnd.github.v3+json' },
  });

  if (!response.ok) {
    throw new Error(`GitHub API 请求失败: ${response.status}`);
  }

  const data = await response.json();
  const tagName: string = data.tag_name ?? '';
  return {
    tagName,
    version: tagName.replace(/^v/, ''),
    body: data.body ?? '',
    htmlUrl: data.html_url ?? `${GITHUB_REPO_URL}/releases/latest`,
  };
}

/** 检测更新的结果 */
export interface UpdateCheckResult {
  /** 是否有新版本 */
  hasUpdate: boolean;
  /** 最新发布信息 */
  release: GitHubRelease;
  /** 当前版本号 */
  currentVersion: string;
}

/**
 * 检查 GitHub 上是否有新版本发布。
 * @param currentVersion 当前应用版本号（不含 "v" 前缀）
 */
export async function checkForUpdate(currentVersion: string): Promise<UpdateCheckResult> {
  const release = await fetchLatestRelease();
  const hasUpdate = compareSemver(release.version, currentVersion) > 0;
  return { hasUpdate, release, currentVersion };
}
