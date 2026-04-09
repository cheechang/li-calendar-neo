import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { openUrl } from '@tauri-apps/plugin-opener';
import { Button, Divider, Input, Modal, message, Space } from 'antd';
import React, { useState } from 'react';
import { initEnv, SOFT_INFO } from '../constants/env.ts';
import { SOFT_URL } from '../constants/link';
import { EMAIL, QQ_GROUP, QQ_GROUP_LINK, SOFT_NAME } from '../constants/soft';
import { DebugConst } from '../debugConst.ts';

const { TextArea } = Input;

const About: React.FC = () => {
  const [messageApi, contextHolder] = message.useMessage();
  const [isModalOpen, setIsModalOpen] = useState<boolean>(false);
  const [appInfo, setAppInfo] = useState<string>('');

  return (
    <>
      {contextHolder}
      <Space orientation="vertical" align="center" style={{ width: '100%' }}>
        <div style={{ fontSize: 'x-large' }}>
          <b>
            {SOFT_NAME}
            {DebugConst.IS_DEV && ' -- 开发环境'}
          </b>
        </div>

        <Space separator={<Divider orientation="vertical" />}>
          <Button type="link" onClick={() => void openUrl(SOFT_URL)}>
            官网
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
      </Space>

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
    </>
  );
};
export default About;
