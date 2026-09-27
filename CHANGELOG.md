# Changelog

Henry's MarketPeek 版本变更记录。**最新版本在最上方。**

- 功能说明：[README.md](README.md)（中英双语）
- 发布页：[Releases](https://github.com/ZHHHenry/henrys-marketpeek/releases)（每版附便携 exe）

## [v0.1.1] - 2026-09-27

### 新增
- **品种排序**：品种页「已显示」组新增拖拽条（三横线样式）；按住拖动可在品种间换位，卡片顺序同步更新
- **拖动特效**：被拖行纵向跟手（限制在「已显示」区域内），其它行平滑让位，落点所见即所得

### 改进
- **拖拽范围细化**：首页（除四张卡片）、品种页（除品种行与滚动条）、设置页（除按钮）其余区域均可拖动窗口，缝隙等「空心」区域不再漏拖
- **外观**：移除窗口在浅色背景下的黑色底衬（阴影残留），圆角边缘更干净

### 工程
- 构建规则（exe **剪切**到根目录、脚本一律相对路径、发版同步 README）写入 `AGENTS.md`
- 新增 `AGENTS.md`（开发规则）与 `PROGRESS.md`（进展与路线）
- 清理项目迁移遗留的旧路径构建缓存

## [v0.1.0] - 2026-09-16

### 新增
- 16 个品种：A 股五大指数（上证 / 深成 / 创业板 / 沪深300 / 科创50）、恒指与恒生科技、日经 225、韩国 KOSPI、纳指 / 标普 / 道指、沪金主连、布伦特原油、伦敦金、美元人民币
- 2×2 卡片区（最多同显 4 张），品种面板分类展示、已选置顶高亮、超限提示
- 红涨绿跌 / 绿涨红跌切换；10 套新质感主题；刷新间隔 5/10/30 秒 + 休市降频 + 托盘隐藏暂停
- 中文 / English 界面切换（设置面板、托盘菜单同步）
- 无边框圆角、整窗拖动、位置记忆、置顶切换、托盘常驻、单实例运行
- 数据源：腾讯行情 / 新浪财经 / 东方财富公开接口（仅供参考，不构成投资建议）

### 工程
- Tauri v2 + 零构建前端（纯 HTML/CSS/JS，无打包器）；含撇号路径 winres 补丁；构建后自动拷贝根目录 exe

---

## English Summary

- **v0.1.1** — Drag-to-reorder instruments with a grip handle (vertical follow effect, rows shifting to preview the drop); fine-grained window drag regions; removed the dark underlay behind the rounded window; build rules (move exe, relative paths, README sync) documented in AGENTS.md.
- **v0.1.0** — Initial release: 16 instruments, 2×2 cards, 10 themes, CN/EN UI, tray, position memory, single instance.
