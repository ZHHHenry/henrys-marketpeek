# Changelog

Henry's MarketPeek 版本变更记录。**最新版本在最上方。**

本文件遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循 [Semantic Versioning](https://semver.org/lang/zh-CN/)。

- 功能说明：[README.md](README.md)（中英双语）
- 发布页：[Releases](https://github.com/ZHHHenry/henrys-marketpeek/releases)（每版附便携 exe）

## [Unreleased]

## [v0.1.3] - 2026-11-01

### 新增

- 统一应用内提示层的主题样式、动效与边缘避让：行情状态点显示原因与上次更新时间，汇率交换、侧栏按钮和添加自选品种使用同款提示；拖动排序与主题颜色块不再弹出提示
- 品种、自选对话框与设置项补齐键盘操作、焦点管理和辅助功能语义
- 自选品种对话框按数据源显示代码格式示例，并在请求前校验当前解析器支持的格式

### 修复

- 行情请求改用 HTTPS，并增加连接 / 总超时、重定向与响应体限制；多数据源并发请求且独立降级
- 修复损坏设置数据、重复刷新、未来时间戳、已取消选择行情残留及离屏窗口位置恢复等边界情况
- 自选品种输入在前后端执行一致的数量、字符与来源格式校验，避免无效代码进入请求
- 应用退出前刷新保存最新窗口位置；位置文件写入失败会保留诊断信息
- 更新 Rust TLS 锁定依赖，修复 RustSec 报告的连接与证书校验安全公告

### 改进

- 窗口置顶选择在重启后继续生效
- “德国DAX30 / DAX 30”统一为官方名称“德国DAX / DAX”
- 收紧 Tauri CSP，限制页面资源与 IPC 连接范围
- 禁用非编辑区域的 WebView 默认右键菜单；自选品种文本输入框保留复制、粘贴等标准编辑菜单

## [v0.1.2] - 2026-10-06

### 新增

- **品种扩充至 30 个**（新增 14）：
  - 指数 ×5：中证500、台湾加权、澳大利亚标普200、英国富时100、德国DAX30
  - 商品 ×4：WTI原油、COMEX白银、沪银主连、沪铜主连
  - 外汇 ×5：欧元美元、美元港币、欧元人民币、港币人民币、人民币日元
- **自选品种**：品种面板「+」可添加当前解析器覆盖的腾讯标准 / `hf_` / `wh` 行情代码、新浪 `nf_` / `int_` 代码及东方财富 `market.symbol` 代码，归入「自选」分组；添加前自动探测行情，支持删除
- **外汇交换**：卡片名旁 ⇄ 一键交换货币对方向（美元人民币 ↔ 人民币美元），价格 / 涨跌按倒数精确换算，偏好持久化

### 修复

- **外汇解析路由错误**：`parse_wh` 的路由条件与品种代码不匹配，导致「美元人民币」等外汇品种的涨跌额 / 涨跌幅取错字段（价格不受影响）；已修复并补充真实响应测试
- **行情来源披露补全**：设置面板与 README 的行情来源说明由「腾讯 / 新浪」更正为「腾讯 / 新浪 / 东方财富」（KOSPI 实际使用东方财富接口）

### 改进

- **外汇名称规范**：含美元 / 人民币的货币对按「美元在前、人民币在后」统一显示（如 欧元美元 → 美元欧元、人民币日元 → 日元人民币），报价方向同步翻转
- README 新增「使用」节：从 Releases 下载即用、平台与联网要求
- README 结构树补全根目录文件、新增文档导览；构建说明补充 `npm start` 与 cargo 镜像模板（`config.toml.example`）
- 术语修正：「双源」→「多源」；「休市自动降频」→「行情超过 180 秒未更新自动降频」

### 内部整理

- CHANGELOG 遵循 Keep a Changelog / SemVer，新增版本 compare 链接
- PROGRESS.md 收敛为待办与路线，发版清单并入 AGENTS.md
- `quotes.rs` 解析层单元测试增至 22 例（本版新增 14 例），并补充模块说明注释

## [v0.1.1] - 2026-09-27

### 新增

- **品种排序**：品种页「已显示」组新增拖拽条（三横线样式）；按住拖动可在品种间换位，卡片顺序同步更新
- **拖动特效**：被拖行纵向跟手（限制在「已显示」区域内），其它行平滑让位，落点所见即所得

### 改进

- **拖拽范围细化**：首页（除四张卡片）、品种页（除品种行与滚动条）、设置页（除按钮）其余区域均可拖动窗口，缝隙等「空心」区域不再漏拖
- **外观**：移除窗口在浅色背景下的黑色底衬（阴影残留），圆角边缘更干净

### 内部整理

- 构建规则写入 AGENTS.md；新增 AGENTS.md / PROGRESS.md；清理旧路径构建缓存

## [v0.1.0] - 2026-09-16

### 新增

- 16 个品种：A 股五大指数（上证 / 深成 / 创业板 / 沪深300 / 科创50）、恒指与恒生科技、日经 225、韩国 KOSPI、纳指 / 标普 / 道指、沪金主连、布伦特原油、伦敦金、美元人民币
- 2×2 卡片区（最多同显 4 张），品种面板分类展示、已选置顶高亮、超限提示
- 红涨绿跌 / 绿涨红跌切换；10 套新质感主题；刷新间隔 5/10/30 秒 + 数据超时自动降频 + 托盘隐藏暂停
- 中文 / English 界面切换（设置面板、托盘菜单同步）
- 无边框圆角、整窗拖动、位置记忆、置顶切换、托盘常驻、单实例运行
- 数据源：腾讯行情 / 新浪财经 / 东方财富公开接口（仅供参考，不构成投资建议）

### 内部整理

- Tauri v2 + 零构建前端（纯 HTML/CSS/JS，无打包器）；含撇号路径 winres 补丁；构建后自动剪切根目录 exe

---

## English Summary

- **v0.1.3** — Hardened concurrent quote requests, persisted state, polling and window restoration; unified theme-aware status/control tooltips; keyboard and accessibility support; source-specific custom-code guidance; persisted always-on-top preference; official DAX naming; tighter CSP; non-editable WebView context menus disabled while text-field editing menus remain available.
- **v0.1.2** — 30 instruments (14 new); custom instruments (+ button, three sources); FX swap button and normalized USD-first / CNY-last display; FX parsing route fix; quote-source disclosure completed (Tencent / Sina / Eastmoney); README "Usage" section; docs cleanup (Keep a Changelog, PROGRESS trimmed to todo & roadmap); 22 parser unit tests and module docs.
- **v0.1.1** — Drag-to-reorder instruments with a grip handle (vertical follow effect, rows shifting to preview the drop); fine-grained window drag regions; removed the dark underlay behind the rounded window; build rules (move exe, relative paths, README sync) documented in AGENTS.md.
- **v0.1.0** — Initial release: 16 instruments, 2×2 cards, 10 themes, CN/EN UI, tray, position memory, single instance.

[Unreleased]: https://github.com/ZHHHenry/henrys-marketpeek/compare/v0.1.3...HEAD
[v0.1.3]: https://github.com/ZHHHenry/henrys-marketpeek/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/ZHHHenry/henrys-marketpeek/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/ZHHHenry/henrys-marketpeek/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/ZHHHenry/henrys-marketpeek/releases/tag/v0.1.0
