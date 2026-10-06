# AGENTS.md — Henry's MarketPeek 项目规则

> 面向后续 Agent / 开发者的项目守则。待办与路线见 [PROGRESS.md](PROGRESS.md)，功能说明见 [README.md](README.md)。

## 项目速览

- 常驻桌面的行情小挂件：**Tauri v2**（Rust 后端）+ **纯 HTML/CSS/JS 前端**（无打包器、无框架，`src/` 即产物，经 `frontendDist` 直接加载，前端经 `withGlobalTauri` 使用全局 API）。
- 窗口：无边框圆角、透明、置顶可切换、常驻托盘、单实例；关闭 = 隐藏到托盘，不退出。
- 行情源：腾讯（GBK）、新浪（GBK）、东方财富（JSON，全球指数）；失败时对应卡片降级显示 `--`，不影响其他卡片。
- 仓库：GitHub `ZHHHenry/henrys-marketpeek`。

## 目录结构

- `src/index.html` / `styles.css` / `main.js`：侧栏 + 2×2 卡片区 + 品种/设置面板；状态、渲染、轮询、i18n。
- `src-tauri/src/main.rs`：托盘、窗口位置记忆、显隐事件、语言命令、单实例唤起。
- `src-tauri/src/quotes.rs`：品种表 `POOL`、多源适配器（GBK 解码、按源分格式解析、中英名称）与自选品种命令 `fetch_custom_quotes`（复用同一组解析器），解析器带单元测试。
- `src-tauri/vendor/tauri-winres`：上游修补副本——修复**含撇号路径**的资源编译 bug，**不可删**。
- `src-tauri/.cargo/config.toml`：本地 rsproxy 镜像（个人配置被 `.gitignore` 排除，不入库；模板见同目录 `config.toml.example`）。

## 数据流（改行情相关代码前先读）

1. 内置品种：前端 `invoke('fetch_quotes', { ids })` → Rust `fetch_quotes`；自选品种：前端 `invoke('fetch_custom_quotes', { items })` → Rust 按 `source` 路由同一组解析器（`*_with` 变体以自定义 id 映射）。
2. Rust 按品种 `source` 分组请求：腾讯（GBK）/ 新浪（GBK）/ 东方财富（JSON），逐源解析为 `Quote { id, price, change, pct, ts }`，`ts` 为 epoch 毫秒。
3. 返回 `Vec<Quote>` → 前端写入 `quotes` Map → `renderCards()` 刷新卡片；某源失败只影响该源品种（卡片保留上一笔或显示 `--`），不影响其他源。
4. 轮询：`tick()` 按所选间隔执行；当所有卡片 `ts` 均超过 180 秒未更新时，间隔自动放宽到 60 秒——**这是数据新鲜度判断，不是交易日历**。
5. 为何多源：各接口品种覆盖不同（如各全球指数走东方财富、日经 / 沪金走新浪），按品种选择可用的源；新增品种必须确认「源 + 解析器」匹配（见下「新增 / 修改品种清单」）。

## 强制同步项（改动必查）

### 1. 中英双语必须同步

- 前端文案：`src/main.js` 的 `STR.zh` / `STR.en`（HTML 侧用 `data-i18n` / `data-i18n-title` 挂接）。
- 品种名称：`quotes.rs` 的 `name / name_en / short / short_en / category / category_en`（前端按 `state.lang` 取用）。
- 托盘菜单：`main.rs` 的 `build_menu()`（显示/退出 ↔ Show/Quit），由 `set_language` 命令随前端语言切换。
- 任何新增用户可见文本都必须提供 zh / en 两份。

### 2. 新增 / 修改品种清单

- `quotes.rs` 的 `POOL`：id、中英名称与简称、中英分类、`decimals`、`code`、`source` 六类字段；
- 确认所属源的解析器与格式匹配：腾讯 `parse_std / parse_hf / parse_wh`、新浪 `parse_nf / parse_int`、东方财富 `parse_eastmoney`（时间戳统一为 epoch 毫秒，时区走 `ts_from`：A 股/港股 Asia/Shanghai、美股 America/New_York）；
- 前端无需登记：品种列表由 `get_instruments` 命令动态返回。

### 3. 主题同步清单

- `main.js` 的 `THEMES` 数组（id、中英名、bg、accent）与 `styles.css` 对应 `[data-theme="..."]` 变量组一一对应；
- 亮色系文字对比由 `textColorFor()` 自动计算；主题选择即时持久化（localStorage `mp-settings-v1`）。

### 4. 版本号同步清单

- 发版必须同步三处 version：`package.json`、`src-tauri/tauri.conf.json`、`src-tauri/Cargo.toml`；
- 界面版本号通过 `app.getVersion()` 动态读取，无需硬编码；发版另需 git tag + GitHub Release（附根目录 exe）。

### 5. 发版文档同步（README / CHANGELOG）

- `README.md`：**功能特性与当前版本状态**必须随版本同步更新（中文与 English 两个区块都要查），禁止留旧版本描述。
- `CHANGELOG.md`：每个版本补条目（最新最上，遵循 Keep a Changelog / SemVer，含日期与 compare 链接），Release 说明与之一致。

## 环境与构建

- 路径含撇号（`Henry's_MarketPeek`）的规避配置（winres 补丁等）**不可删**；网络受限时参考 `src-tauri/.cargo/config.toml.example` 配置 cargo 镜像，并保持 npm 镜像。
- 构建：`npm install` → `npm run dev` / `npm run build`；**postbuild 会自动把 `src-tauri/target/release/marketpeek.exe` 剪切到根目录 `MarketPeek.exe`**（成品只放根目录、源位置不留副本、不入库）。
- **构建脚本一律使用相对路径**（以项目根为基准，如 `postbuild` 中对 `src-tauri\target\release\...` 的引用），不得写死绝对路径；项目整体移动后若构建报旧路径相关错误，先清理/改名 `target` 中相关构建脚本缓存再重建。
- 质量门槛：Rust 改动跑 `cd src-tauri && cargo check`；解析层单测跑 `cd src-tauri && cargo test`（新增 / 修改解析器必须补用例）；行情解析类改动仍须在 `npm run dev` 下实测对应品种（含休市 / 断网表现）。

## 常用命令

```bash
npm install                  # 安装前端依赖（首次）
npm run dev                  # 开发模式（tauri dev）
npm run build                # 发布构建；postbuild 自动剪切 exe 到根目录
npm start                    # 运行根目录已构建的 exe（需先 npm run build）
cd src-tauri && cargo check  # Rust 质量门槛
cd src-tauri && cargo test   # 解析层单元测试
```

## 发版清单（每版）

1. 三处 version 同步：`package.json`、`src-tauri/tauri.conf.json`、`src-tauri/Cargo.toml`；
2. 有功能变化时同步更新 `README.md`（中英两个区块）与 `CHANGELOG.md`（条目 + 日期 + compare 链接）；
3. `npm run build` 并冒烟根目录 exe（托盘、置顶、隐藏 / 恢复）；
4. 提交推送 → 打 tag → `gh release create`（附根目录 exe）。

## 行为约束

- 窗口生命周期约定（改动时不得破坏）：关闭 = 隐藏 + `mp-hidden` 事件；托盘左键切换显隐；单实例二次启动唤起主窗口；位置记忆写 `app_config_dir/window.json`（400ms 节流）。
- 轮询约定：默认 10s（可选 5/10/30s）；数据不新鲜（>180s）自动降频至 60s；托盘隐藏时暂停、恢复时立即刷新。
- 免责声明：设置面板与 README 中的行情来源披露**必须覆盖全部实际数据源**（腾讯 / 新浪 / 东方财富），且「仅供参考，不构成投资建议」字样不得移除。新增数据源时必须同步 `src/main.js` 的 zh / en 词典与 `src/index.html` 默认文本。
- 自选品种与外汇偏好：自选列表存 localStorage（`mp-settings-v1` 的 `custom`），外汇交换显式值存 `swapped`；内置外汇默认显示方向——报价端为 USD 或基准端为 CNY 时翻转（满足「美元在前、人民币在后」）。
- 前端保持零依赖、零构建：不要引入打包器 / 框架，除非用户明确要求。

## 横切事项

| 方向 | 约定 |
|---|---|
| 双语 | 前端 STR、品种名称表、托盘菜单全部 zh / en 同改 |
| 版本 | package.json / tauri.conf.json / Cargo.toml 三处同步 + tag + Release |
| 测试 | 解析层有单测（`cargo test`）；解析器改动 → 补用例 + dev 实测；Rust 改动 → `cargo check` |
| 数据 | 仅用公开行情接口；来源与免责声明保留；失败降级 `--` |
| 发布 | `npm run build` 自动把 exe **剪切**到根目录（源位置不留副本、不入库）；发版同步 README；Release 附 exe |
