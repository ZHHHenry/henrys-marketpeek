# Henry's MarketPeek

[中文](#中文) | [English](#english)

> 文档：[AGENTS.md](AGENTS.md)（开发规则）· [PROGRESS.md](PROGRESS.md)（待办与路线）· [CHANGELOG.md](CHANGELOG.md)（版本变更）· [Releases](https://github.com/ZHHHenry/henrys-marketpeek/releases)（下载 exe）

---

## 中文

常驻桌面的行情小挂件：新质感（Neumorphism）圆角卡片，一眼看清主要指数、商品与汇率的点位和涨跌。

### 使用

**[从 Releases 下载](https://github.com/ZHHHenry/henrys-marketpeek/releases)** 最新 `MarketPeek.exe`，双击即可运行 —— 无需安装 Rust 或 Node.js。

- **平台**：Windows 10/11（64 位）。本应用使用 Windows 托盘、无边框圆角窗口与 `winres` 资源编译，**目前不支持 macOS / Linux**。
- **联网**：行情来自公开接口，需保持网络连接；休市期间显示最近一笔收盘数据，断网时保留上一笔（从未取到则显示 `--`）。
- 界面语言、主题、刷新间隔、涨跌配色、置顶与位置记忆均可在设置面板调整；关闭窗口 = 隐藏到托盘。

### 功能特性

- 30 个可选品种：A 股指数（上证/深成/创业板/沪深300/科创50/中证500）、恒指与恒生科技、日经225 与韩国KOSPI、台湾加权、澳洲标普200、英国富时100、德国DAX30、纳指/标普/道指、沪金/沪银/沪铜主连、布伦特/WTI 原油、伦敦金/COMEX 白银、美元人民币/欧元美元/美元港币/欧元人民币/港币人民币/人民币日元
- 2×2 卡片区最多同显 4 张；「品种」面板切换，已选品种置顶并高亮，可用行首拖拽条**拖动排序**（卡片顺序同步），超出上限有提示
- **自选品种**：品种面板标题旁的「+」可直接添加三源（腾讯 / 新浪 / 东方财富）支持的任意接口代码，归入「自选」分组，可随时删除
- 卡片显示：名称 + 点位 + 涨跌额 + 涨跌幅；红涨绿跌 / 绿涨红跌可切换
- **外汇**：卡片名旁 ⇄ 一键交换货币对方向（美元人民币 ↔ 人民币美元）；含美元 / 人民币的货币对按「美元在前、人民币在后」规范显示
- 10 套预设主题配色；刷新间隔 5/10/30 秒可选，行情超过 180 秒未更新则自动降频至 60 秒，隐藏到托盘暂停刷新
- 界面支持中文 / English 切换（设置面板内，托盘菜单同步）
- 无边框圆角（浅色背景边缘干净、无黑边底衬）、**精细拖拽区域**（首页除卡片、品种页除行与滚动条、设置页除按钮，其余区域均可拖动窗口）、位置记忆、置顶可切换、托盘常驻、单实例运行

> 行情数据来自腾讯行情、新浪财经与东方财富公开接口，仅供参考，不构成投资建议。

### 项目结构

```
Henry's_MarketPeek/
├─ README.md                本文件（使用说明与构建）
├─ AGENTS.md                开发规则（面向 Agent / 开发者）
├─ PROGRESS.md              待办与后续路线
├─ CHANGELOG.md             版本变更记录
├─ LICENSE                  MIT 许可证
├─ .gitignore               Git 忽略规则
├─ package.json             npm 脚本（dev / build / start / icon）+ Tauri CLI
├─ package-lock.json        npm 依赖锁
├─ app-icon.png             图标源图（1024px）
├─ MarketPeek.exe           构建产物（npm run build 生成；不入库）
├─ src/                     前端（纯 HTML/CSS/JS，无打包器）
│  ├─ index.html            侧栏 + 2×2 卡片区 + 品种/设置面板
│  ├─ styles.css            新质感样式 + 10 套主题 CSS 变量
│  └─ main.js               状态、渲染、轮询、面板、多语言逻辑
└─ src-tauri/               Rust 后端（Tauri v2）
   ├─ Cargo.toml            依赖与版本号
   ├─ tauri.conf.json       窗口与打包配置
   ├─ .cargo/config.toml.example  cargo 镜像配置模板（复制为 config.toml 启用）
   ├─ capabilities/         权限声明（拖动、置顶）
   ├─ icons/                全平台图标（由 app-icon.png 生成，另含托盘专用 tray.png）
   ├─ vendor/tauri-winres/  上游修补副本：修复含撇号路径的资源编译 bug
   └─ src/
      ├─ main.rs            托盘、窗口位置记忆、显隐事件、语言切换命令
      └─ quotes.rs          多源行情适配器（GBK 解码、按格式解析、中英文名称表、单元测试）
```

### 从源码构建

```bash
npm install
npm run dev     # 开发模式
npm run build   # 发布构建，postbuild 自动把 exe 剪切到项目根目录 MarketPeek.exe
npm start       # 运行根目录已构建的 exe（需先 npm run build）
```

需要 **Windows（64 位）** 与 Rust（MSVC 工具链）、Node.js；国内网络可复制 `src-tauri/.cargo/config.toml.example` 为 `src-tauri/.cargo/config.toml` 启用 rsproxy 镜像。

### 许可

采用 MIT 许可证，详见 [LICENSE](LICENSE)。

版本变更记录见 [CHANGELOG.md](CHANGELOG.md)。

---

## English

A tiny always-on-desktop market widget: neumorphic rounded cards that show quotes and changes of major indices, commodities and FX at a glance.

### Usage

**[Download from Releases](https://github.com/ZHHHenry/henrys-marketpeek/releases)** the latest `MarketPeek.exe` and double-click to run — no Rust or Node.js required.

- **Platform**: Windows 10/11 (64-bit). The app uses the Windows tray, a frameless rounded window and `winres` resource compilation; **macOS / Linux are not supported**.
- **Network**: quotes come from public APIs; an internet connection is required. During market closures the latest close is shown; when offline the last data is kept (or `--` if nothing has been fetched yet).
- Language, theme, refresh interval, up/down colors, always-on-top and position memory are adjustable in Settings; closing the window hides it to the tray.

### Features

- 30 instruments: Chinese A-share indices (SSE Composite, SZSE Component, ChiNext, CSI 300, STAR 50, CSI 500), Hang Seng and Hang Seng Tech, Nikkei 225 and KOSPI, Taiwan Weighted, S&P/ASX 200, FTSE 100, DAX 30, Nasdaq Composite, S&P 500, Dow Jones, SHFE Gold / Silver / Copper, Brent and WTI Crude, London Gold Spot and COMEX Silver, USD/CNY plus EUR/USD, USD/HKD, EUR/CNY, HKD/CNY and CNY/JPY
- 2×2 card grid, up to 4 cards shown; instrument picker panel with selected items pinned on top and **drag-to-reorder** (grip handle; card order follows), over-limit notice
- **Custom instruments**: add any code supported by the three sources (Tencent / Sina / Eastmoney) via the "+" in the picker header; grouped under "Custom" and removable anytime
- Each card shows: name + price + change amount + change percent; CN (red-up) / international (green-up) color schemes
- **FX**: swap the base/quote direction with the ⇄ button on FX cards; USD / CNY pairs are displayed in a normalized order (USD first, CNY last)
- 10 preset themes; refresh interval 5/10/30 s, auto slow-down to 60 s when quotes go stale (>180 s), paused while hidden in tray
- UI language switch between 中文 / English (in Settings; tray menu follows)
- Frameless rounded window (clean edges, no dark underlay on light backgrounds), **fine-grained drag regions** (home: except cards; picker: except rows and scrollbar; settings: except buttons), position memory, toggleable always-on-top, tray resident, single instance

> Quotes come from Tencent, Sina and Eastmoney public APIs. For reference only, not investment advice.

### Project structure

```
Henry's_MarketPeek/
├─ README.md                this file (usage & build)
├─ AGENTS.md                development rules (for agents / developers)
├─ PROGRESS.md              todo & roadmap
├─ CHANGELOG.md             version history
├─ LICENSE                  MIT license
├─ .gitignore               git ignore rules
├─ package.json             npm scripts (dev / build / start / icon) + Tauri CLI
├─ package-lock.json        npm lockfile
├─ app-icon.png             icon source (1024px)
├─ MarketPeek.exe           build artifact (created by npm run build; not committed)
├─ src/                     frontend (plain HTML/CSS/JS, no bundler)
│  ├─ index.html            sidebar + 2×2 card grid + instrument/settings panels
│  ├─ styles.css            neumorphic styles + 10 theme variable sets
│  └─ main.js               state, rendering, polling, panels, i18n
└─ src-tauri/               Rust backend (Tauri v2)
   ├─ Cargo.toml            dependencies and version
   ├─ tauri.conf.json       window and bundle config
   ├─ .cargo/config.toml.example  cargo mirror template (copy to config.toml to enable)
   ├─ capabilities/         permissions (dragging, always-on-top)
   ├─ icons/                platform icon set (+ dedicated tray.png)
   ├─ vendor/tauri-winres/  patched upstream copy: fixes RC compile bug on apostrophe paths
   └─ src/
      ├─ main.rs            tray, window position memory, visibility events, language command
      └─ quotes.rs          multi-source quote adapters (GBK decoding, per-format parsing, zh/en names, unit tests)
```

### Build from source

```bash
npm install
npm run dev     # development
npm run build   # release build; postbuild moves the exe to MarketPeek.exe in the project root
npm start       # run the already-built MarketPeek.exe in the project root (after npm run build)
```

Requires **Windows (64-bit)**, Rust (MSVC toolchain) and Node.js. On slow networks, copy `src-tauri/.cargo/config.toml.example` to `src-tauri/.cargo/config.toml` to enable the rsproxy mirror.

### License

MIT — see [LICENSE](LICENSE). Version history in [CHANGELOG.md](CHANGELOG.md).
