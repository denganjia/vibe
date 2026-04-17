# 外部集成 (External Integrations)

**分析日期：** 2025-05-22

## 终端复用器 (Terminal Multiplexers)

**TMUX:**
- 用途：创建/分割 Pane、发送命令、聚焦窗口。
- 实现：通过调用 `tmux` 命令行工具 (`Command::new("tmux")`)。
- 关键集成文件：`crates/vibe-core/src/adapter/tmux.rs`

**WezTerm:**
- 用途：利用 WezTerm CLI 执行 Pane 管理和命令注入。
- 实现：通过调用 `wezterm cli` 子命令。
- 关键集成文件：`crates/vibe-core/src/adapter/wezterm.rs`

## 进程间通信 (IPC)

**Unix Domain Sockets (UDS):**
- 用途：Master 服务与 Worker 进程之间的实时通信。
- 协议：NDJSON (Newline Delimited JSON) 序列化消息。
- 实现：使用 `tokio::net::UnixListener` 和 `UnixStream`。
- 关键集成文件：
  - `crates/vibe-core/src/ipc/server.rs` (Master 端)
  - `crates/vibe-core/src/ipc/client.rs` (Worker 端)
  - `crates/vibe-core/src/ipc/protocol.rs` (协议定义)

## 数据存储 (Data Storage)

**本地文件存储：**
- 数据库：无 (已移除 SQLite)。
- 状态存储：项目目录下的 `.vibe/state/panes.json` 文件。
- 机制：通过 `serde_json` 实现持久化，采用原子重命名方式保证数据一致性。
- 关键集成文件：`crates/vibe-core/src/state/mod.rs`

## 操作系统集成 (OS Integration)

**Unix/macOS:**
- 通过 `daemonize` crate 实现 Master 进程的后台脱离。
- 使用标准 UDS 文件路径进行通信。

**Windows:**
- 通过 `windows-sys` 处理基础系统调用。
- (当前状态) UDS 理论上在现代 Windows 上受支持，但 Master 后台化机制在 Windows 上可能与 Unix 不同。

## 环境变量 (Environment Configuration)

**关键变量：**
- `TMUX_PANE`: 用于检测是否在 TMUX 环境及获取当前 Pane ID。
- `WEZTERM_PANE`: 用于检测是否在 WezTerm 环境。

---

*集成审计更新：2025-05-22*
