# 技术栈 (Technology Stack)

**分析日期：** 2025-05-22

## 语言 (Languages)

**主要：**
- Rust (Edition 2024) - 核心逻辑、CLI、TUI 及 IPC 实现。

## 运行时 (Runtime)

**环境：**
- Tokio 1.x - 异步运行时，处理多线程、IPC 通信及异步 IO。

**包管理器：**
- Cargo
- Lockfile: `Cargo.lock` 已存在。

## 框架 (Frameworks)

**核心架构：**
- `vibe-core` (Crate) - 封装核心逻辑、IPC 协议、状态存储及终端适配器。
- `vibe-cli` (App) - 提供命令行交互界面。

**TUI (终端 UI)：**
- Ratatui 0.29 - TUI 渲染框架。
- Crossterm 0.28 - 终端操作、事件处理库。

**CLI (命令行工具)：**
- Clap 4.5 - 命令行参数解析。
- Dialoguer 0.11 - 交互式提示工具。

**测试：**
- Rust 内置测试框架 (`#[cfg(test)]`)。
- `tempfile` 3.10 - 测试中的临时文件处理。

## 关键依赖 (Key Dependencies)

**序列化：**
- Serde 1.0 - 对象序列化框架。
- Serde_JSON 1.0 - JSON 数据处理。

**IPC 通信：**
- Tokio-util 0.7 - 提供 `Codec` 和 `Framed` 抽象，用于 NDJSON 协议流。
- Futures 0.3 - 异步流处理。

**错误处理：**
- Anyhow 1.0 - 顶层应用错误处理。
- Thiserror 2.0 - 库级别的结构化错误定义。

**系统集成：**
- Dirs 5.0 - 处理平台相关的标准目录。
- Which 6.0 - 寻找可执行文件路径。
- Daemonize 0.5 - (非 Windows) 实现 Master 进程后台化。
- Windows-sys 0.59 - Windows 系统调用接口。

## 配置与存储 (Configuration & Storage)

**环境配置：**
- 通过环境变量 (`TMUX_PANE`, `WEZTERM_PANE`) 自动检测终端环境。
- 使用 `.vibe` 目录存储项目相关状态。

**状态存储：**
- 本地 JSON 文件 (`panes.json`) - 存储活动的 Pane 记录，替代了原有的 SQLite。
- 原子写入机制（写入临时文件后重命名）。

## 平台要求 (Platform Requirements)

**开发与运行：**
- 支持 Unix-like 系统 (macOS, Linux)。
- 提供 Windows 兼容性支持 (通过 `windows-sys`)。

---

*技术栈分析更新：2025-05-22*
