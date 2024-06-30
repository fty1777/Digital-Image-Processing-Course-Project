# 数字图像处理框架

这是一个使用Tauri + React开发的数字图像处理框架。

## 安装

1. 安装rust工具链
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装node和yarn
```bash
# 安装nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
# 安装node
nvm use stable
# 安装yarn
npm install -g yarn
```

3. 启动程序
```bash
cd dip
yarn tauri dev
```

4. （可选）构建可分发程序
```bash
cd dip
yarn tauri build
```