# Henry's MarketPeek

[中文](#中文) | [English](#english)

---

## 中文

常驻桌面的行情小挂件：新质感（Neumorphism）圆角卡片，一眼看清主要指数、商品与汇率的点位和涨跌。

### 功能特性

- 16 个可选品种：A 股指数（上证/深成/创业板/沪深300/科创50）、恒指与恒生科技、日经225 与韩国KOSPI、纳指/标普/道指、沪金主连、布伦特原油、伦敦金、美元人民币
- 2×2 卡片区最多同显 4 张；「品种」面板切换，已选品种置顶并高亮，超出上限有提示
- 卡片显示：名称 + 点位 + 涨跌额 + 涨跌幅；红涨绿跌 / 绿涨红跌可切换
- 10 套预设主题配色；刷新间隔 5/10/30 秒可选，休市自动降频，隐藏到托盘暂停刷新
- 界面支持中文 / English 切换（设置面板内，托盘菜单同步）
- 无边框圆角、整窗可拖动、位置记忆、置顶可切换、托盘常驻、单实例运行

> 行情数据来自腾讯行情与新浪财经公开接口，仅供参考，不构成投资建议。

### 项目结构

```
Henry's_MarketPeek/
├─ package.json             npm 脚本（dev/build）+ Tauri CLI
├─ app-icon.png             图标源图（1024px）
├─ src/                     前端（纯 HTML/CSS/JS，无打包器）
│  ├─ index.html            侧栏 + 2×2 卡片区 + 品种/设置面板
│  ├─ styles.css            新质感样式 + 10 套主题 CSS 变量
│  └─ main.js               状态、渲染、轮询、面板、多语言逻辑
└─ src-tauri/               Rust 后端（Tauri v2）
   ├─ tauri.conf.json       窗口与打包配置
   ├─ capabilities/         权限声明（拖动、置顶）
   ├─ icons/                全平台图标（由 app-icon.png 生成，另含托盘专用 tray.png）
   ├─ vendor/tauri-winres/  上游修补副本：修复含撇号路径的资源编译 bug
   └─ src/
      ├─ main.rs            托盘、窗口位置记忆、显隐事件、语言切换命令
      └─ quotes.rs          双源行情适配器（GBK 解码、按格式解析、中英文名称表）
```

### 构建

```bash
npm install
npm run dev     # 开发模式
npm run build   # 发布构建，产物在 src-tauri/target/release/
```

构建完成后会自动复制一份到项目根目录：`MarketPeek.exe`（双击即可运行，可自由改名/移动）。

需要 Rust（MSVC 工具链）与 Node.js；国内网络建议为 cargo 配置 crates.io 镜像加速。

### 许可

采用 MIT 许可证，详见 [LICENSE](LICENSE)。

---

## English

A tiny always-on-desktop market widget: neumorphic rounded cards that show quotes and changes of major indices, commodities and FX at a glance.

### Features

- 16 instruments: Chinese A-share indices (SSE Composite, SZSE Component, ChiNext, CSI 300, STAR 50), Hang Seng and Hang Seng Tech, Nikkei 225 and KOSPI, Nasdaq Composite, S&P 500, Dow Jones, SHFE Gold, Brent Crude, London Gold Spot, USD/CNY
- 2×2 card grid, up to 4 cards shown; instrument picker panel with selected items pinned on top, over-limit notice
- Each card shows: name + price + change amount + change percent; CN (red-up) / international (green-up) color schemes
- 10 preset themes; refresh interval 5/10/30 s, auto slow-down when markets are closed, paused while hidden in tray
- UI language switch between 中文 / English (in Settings; tray menu follows)
- Frameless rounded window, fully draggable, position memory, toggleable always-on-top, tray resident, single instance

> Quotes come from Tencent and Sina public APIs. For reference only, not investment advice.

### Project structure

```
Henry's_MarketPeek/
├─ package.json            npm scripts (dev/build) + Tauri CLI
├─ app-icon.png            icon source (1024px)
├─ src/                    frontend (plain HTML/CSS/JS, no bundler)
│  ├─ index.html           sidebar + 2×2 card grid + instrument/settings panels
│  ├─ styles.css           neumorphic styles + 10 theme variable sets
│  └─ main.js              state, rendering, polling, panels, i18n
└─ src-tauri/              Rust backend (Tauri v2)
   ├─ tauri.conf.json      window and bundle config
   ├─ capabilities/        permissions (dragging, always-on-top)
   ├─ icons/               platform icon set (+ dedicated tray.png)
   ├─ vendor/tauri-winres/ patched upstream copy: fixes RC compile bug on apostrophe paths
   └─ src/
      ├─ main.rs           tray, window position memory, visibility events, language command
      └─ quotes.rs         dual-source quote adapters (GBK decoding, per-format parsing, zh/en names)
```

### Build

```bash
npm install
npm run dev     # development
npm run build   # release build, output under src-tauri/target/release/
```

The build also copies the executable to the project root as `MarketPeek.exe` (double-click to run; rename or move it freely).

Requires Rust (MSVC toolchain) and Node.js. A crates.io mirror is recommended on slow networks.

### License

MIT — see [LICENSE](LICENSE).
