import { getVersion } from '@tauri-apps/api/app';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { openUrl } from '@tauri-apps/plugin-opener';
import { Button, Divider, Input, Modal, message, Space, Tag } from 'antd';
import React, { useEffect, useState } from 'react';
import { initEnv, SOFT_INFO } from '../constants/env.ts';
import { GITHUB_RELEASES_URL, GITHUB_REPO_URL, SOFT_URL } from '../constants/link';
import { EMAIL, QQ_GROUP, QQ_GROUP_LINK, SOFT_NAME } from '../constants/soft';
import { DebugConst } from '../debugConst.ts';
import { checkForUpdate, type UpdateCheckResult } from '../http/github.ts';

const { TextArea } = Input;

const About: React.FC = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const [isModalOpen, setIsModalOpen] = useState<boolean>(false);
  const [appInfo, setAppInfo] = useState<string>('');

  /** 当前应用版本号 */
  const [currentVersion, setCurrentVersion] = useState<string>('');
  /** 是否正在检查更新 */
  const [checking, setChecking] = useState<boolean>(false);
  /** 检测结果 */
  const [updateResult, setUpdateResult] = useState<UpdateCheckResult | null>(null);

  useEffect(() => {
    void getVersion().then(setCurrentVersion);
  }, []);

  /** 检查更新 */
  const handleCheckUpdate = async () => {
    if (!currentVersion) return;
    setChecking(true);
    try {
      const result = await checkForUpdate(currentVersion);
      setUpdateResult(result);
    } catch {
      messageApi.error('检查更新失败，请检查网络连接');
    } finally {
      setChecking(false);
    }
  };

  return (
    <>
      {contextHolder}
      <Space orientation="vertical" align="center" style={{ width: '100%' }}>
        <div style={{ fontSize: 'x-large', display: 'flex', alignItems: 'center', gap: 8 }}>
          <b>
            {SOFT_NAME}
            {DebugConst.IS_DEV && ' -- 开发环境'}
          </b>
          {currentVersion && (
            <Tag style={{ margin: 0, fontSize: 12, lineHeight: '18px' }}>v{currentVersion}</Tag>
          )}
        </div>

        <Space separator={<Divider orientation="vertical" />}>
          <Button type="link" onClick={() => void openUrl(GITHUB_REPO_URL)}>
            GitHub
          </Button>
          <Button type="link" onClick={() => void openUrl(SOFT_URL)}>
            官网
          </Button>
          <Button type="link" onClick={() => void openUrl(GITHUB_RELEASES_URL)}>
            发布记录
          </Button>
          <Button
            type="link"
            onClick={async () => {
              await initEnv();
              setAppInfo(JSON.stringify(SOFT_INFO, null, 2));
              setIsModalOpen(true);
            }}
          >
            软件信息
          </Button>
        </Space>

        <Space separator={<Divider orientation="vertical" />}>
          <div>
            客服邮箱:
            <Button type="link" onClick={() => void openUrl(`mailto:${EMAIL}`)}>
              {EMAIL}
            </Button>
          </div>
          <div>
            QQ群:
            <Button type="link" onClick={() => void openUrl(QQ_GROUP_LINK)}>
              {QQ_GROUP}
            </Button>
          </div>
        </Space>

        <Button loading={checking} onClick={() => void handleCheckUpdate()}>
          检查更新
        </Button>
      </Space>

      {/* 软件信息弹窗 */}
      <Modal
        title="软件信息"
        centered={true}
        open={isModalOpen}
        mask={{ closable: false }}
        onCancel={() => {
          setIsModalOpen(false);
        }}
        footer={[
          <Button
            key="copy"
            type="primary"
            style={{ width: '100%' }}
            onClick={async () => {
              await writeText(appInfo);
              messageApi.success('复制成功');
            }}
          >
            复制
          </Button>,
        ]}
      >
        <TextArea value={appInfo} autoSize readOnly />
      </Modal>

      {/* 更新结果弹窗 */}
      {updateResult?.hasUpdate && (
        <Modal
          title={`发现新版本 ${updateResult.release.tagName}`}
          centered={true}
          open={true}
          onCancel={() => setUpdateResult(null)}
          okText="前往下载"
          onOk={() => void openUrl(updateResult.release.htmlUrl)}
        >
          <div style={{ whiteSpace: 'pre-wrap', maxHeight: 320, overflowY: 'auto' }}>
            {updateResult.release.body || '暂无更新说明'}
          </div>
        </Modal>
      )}

      {updateResult && !updateResult.hasUpdate && (
        <Modal
          title="已是最新版本"
          centered={true}
          open={true}
          onCancel={() => setUpdateResult(null)}
          footer={null}
        >
          <p>当前版本 v{updateResult.currentVersion} 已是最新版本，无需更新。</p>
        </Modal>
      )}
    </>
  );
};
export default About;
