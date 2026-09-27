# AGENTS.md — Henry's MarketPeek 项目规则

> 面向后续 Agent / 开发者的项目守则。项目进展与路线见 [PROGRESS.md](PROGRESS.md)，功能说明见 [README.md](README.md)。

## 项目速览

- 常驻桌面的行情小挂件：**Tauri v2**（Rust 后端）+ **纯 HTML/CSS/JS 前端**（无打包器、无框架，`src/` 即产物，经 `frontendDist` 直接加载，前端经 `withGlobalTauri` 使用全局 API）。
- 窗口：无边框圆角、透明、置顶可切换、常驻托盘、单实例；关闭 = 隐藏到托盘，不退出。
- 行情源：腾讯（GBK）、新浪（GBK）、东方财富（KOSPI 用）；失败时对应卡片降级显示 `--`，不影响其他卡片。
- 仓库：GitHub 私有 `ZHHHenry/henrys-marketpeek`。

## 目录结构

- `src/index.html` / `styles.css` / `main.js`：侧栏 + 2×2 卡片区 + 品种/设置面板；状态、渲染、轮询、i18n。
- `src-tauri/src/main.rs`：托盘、窗口位置记忆、显隐事件、语言命令、单实例唤起。
- `src-tauri/src/quotes.rs`：品种表 `POOL` 与多源适配器（GBK 解码、按源分格式解析、中英名称）。
- `src-tauri/vendor/tauri-winres`：上游修补副本——修复**含撇号路径**的资源编译 bug，**不可删**。
- `src-tauri/.cargo/config.toml`：本地 rsproxy 镜像（已被 `.gitignore` 排除，不入库）。

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
- `CHANGELOG.md`：每个版本补条目（最新最上），Release 说明与之一致。

## 环境与构建

- 路径含撇号（`Henry's_MarketPeek`）的规避配置（winres 补丁等）**不可删**；网络受限时保持 npm 镜像与 cargo rsproxy 镜像。
- 构建：`npm install` → `npm run dev` / `npm run build`；**postbuild 会自动把 `src-tauri/target/release/marketpeek.exe` 剪切到根目录 `MarketPeek.exe`**（成品只放根目录、源位置不留副本、不入库）。
- **构建脚本一律使用相对路径**（以项目根为基准，如 `postbuild` 中对 `src-tauri\target\release\...` 的引用），不得写死绝对路径；项目整体移动后若构建报旧路径相关错误，先清理/改名 `target` 中相关构建脚本缓存再重建。
- 质量门槛：Rust 改动跑 `cargo check`；当前无自动化测试——行情解析类改动必须在 `npm run dev` 下实测对应品种（含休市 / 断网表现）。
- 发布流程：同步三处版本号 → 同步更新 `README.md` → `npm run build` → 冒烟运行根目录 exe → 提交推送 → 打 tag → `gh release create`（附根目录 exe）。

## 行为约束

- 窗口生命周期约定（改动时不得破坏）：关闭 = 隐藏 + `mp-hidden` 事件；托盘左键切换显隐；单实例二次启动唤起主窗口；位置记忆写 `app_config_dir/window.json`（400ms 节流）。
- 轮询约定：默认 10s（可选 5/10/30s）；数据不新鲜（>180s）自动降频至 60s；托盘隐藏时暂停、恢复时立即刷新。
- 免责声明：设置面板与 README 中的"行情来自腾讯 / 新浪公开接口，仅供参考"不得移除。
- 前端保持零依赖、零构建：不要引入打包器 / 框架，除非用户明确要求。

## 横切事项

| 方向 | 约定 |
|---|---|
| 双语 | 前端 STR、品种名称表、托盘菜单全部 zh / en 同改 |
| 版本 | package.json / tauri.conf.json / Cargo.toml 三处同步 + tag + Release |
| 测试 | 无自动化测试；解析器改动 → dev 实测；Rust 改动 → `cargo check` |
| 数据 | 仅用公开行情接口；来源与免责声明保留；失败降级 `--` |
| 发布 | `npm run build` 自动把 exe **剪切**到根目录（源位置不留副本、不入库）；发版同步 README；Release 附 exe |
