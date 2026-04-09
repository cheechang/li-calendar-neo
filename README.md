# 松鼠日历

跨平台的桌面日历，支持显示农历、节假日、调休、节气等信息

<img src="./README.assets/windows-light.png" width="400" alt="windows-light"> <img src="./README.assets/macos-light.png" width="400" alt="macos-light">

## 主要功能

- 支持 macOS、Windows 等操作系统
- 自定义 Windows 任务栏时钟，可显示**星期**等信息
- 替换 Windows 任务栏的系统日历
- 在 Windows 桌面上显示日历

## 技术栈

- Tauri 2
- React 19
- Ant Design 6
- lunar-typescript

## 开发与构建

```bash
pnpm install
pnpm tauri dev
```

## 🤔常见问题

### macOS版本无法打开问题

安装完成后，复制以下命令到终端，然后按回⻋键

```
sudo xattr -r -d com.apple.quarantine /Applications/liCalendar.app
```
