# Phase 3 CONTEXT: State Persistence & IPC Layer

## 1. 核心目标与范围 (Goal & Scope)

本阶段旨在构建 `vibe-cli` 的“神经系统”——IPC 通信与 Master-Worker 协调层。它必须能够支持跨窗格的实时状态更新，并确保持久化状态的一致性，同时兼容 Windows 与 Unix。

## 2. 关键架构决策 (Architectural Decisions)

### 2.1 Master 服务端生命周期 (Master Lifecycle)
- **启动模式**: **On-demand (按需启动)**。任何 `vibe` 终端指令在发现 UDS 不存在时，应自动 fork 出一个分离的 Master 守护进程。
- **存储路径**: UDS Socket 文件必须存储在跨平台的用户配置目录中（例如 macOS 的 `~/.config/vibe/vibe.sock`），而不是公共的 `/tmp`。
- **自动退出**: 具备“闲置超时”机制。如果没有活跃的 Worker 超过一定时间（建议 10 分钟），Master 应自动清理并安全退出。

### 2.2 IPC 通信协议 (Protocol & Messaging)
- **底层技术**: **Unix Domain Sockets (UDS)**，确保本地低延迟。
- **消息格式**: **带版本的 Newline-delimited JSON (NDJSON)**。
  - 示例头结构: `{"version": "v1", "type": "heartbeat", "vibe_id": "worker-123"}`
- **心跳机制**: 采用 **5 秒低频心跳 (5s Heartbeat)**，权衡实时性与系统开销。

### 2.3 调度与控制逻辑 (Scheduling & Control)
- **中心化调度 (Centralized)**: Master 进程是物理资源的唯一决策者。新窗格的创建由 Master 调用 `TerminalAdapter` 后分配给 Worker 进程。
- **Worker 职责**: Worker 作为一个轻量级的 wrapper 进程，负责监控子进程输出、发送心跳并上报最终退出状态。
- **崩溃恢复**: **保留现场 (Preserve Context)**。Worker 异常退出时，Master 将其在 SQLite 中的状态置为 `FAILED`，但不执行物理销毁（`close-pane`），以便用户调试。

### 2.4 并发处理策略 (Concurrency)
- **串行化写入**: Master 内部使用 `tokio::sync::mpsc` 将所有的数据库写入请求序列化，防止 SQLite 锁竞争。
- **单一事实来源 (SSOT)**: 所有的状态查询（`vibe list`）都必须通过 Master 的 API 或者是直接读取受保护的 SQLite 数据库。

## 3. 技术约束与规范 (Technical Constraints)

- **库选择**: 
  - `tokio::net::UnixListener` (跨平台 UDS)。
  - `serde_json` (协议序列化)。
- **Windows 兼容性**: 针对 Windows 10+ 的 `AF_UNIX` 支持进行适配，并确保 Master 退出时 Job Object 能正确响应。

## 4. 下游代理指令 (Directives for Research/Planning)

- **研究员 (Researcher)**: 重点研究 Rust 如何在 Windows 下稳定地“分离 (Detach)”一个后台守护进程（Daemonization），以及 UDS 在 Windows 下的错误清理机制。
- **规划者 (Planner)**: 任务分解需包含：UDS 服务端实现、消息解析层、Worker wrapper 逻辑、以及 SQLite 模式扩展以支持心跳时间戳。

---
*Last updated: 2026-04-14 after Phase 3 Discussion*
