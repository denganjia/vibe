# Research Summary: vibe-cli

**Project:** vibe-cli
**Context:** AI-powered Terminal Orchestrator (WezTerm/Tmux)
**Status:** Research Complete

## Executive Summary

`vibe-cli` 是一个面向开发者的 AI 终端编排工具，旨在通过物理操控终端窗格（WezTerm/Tmux）来实现多智能体协作。该产品的核心逻辑是“Master-Worker”模式：一个 Master 进程负责任务拆分与资源调度，多个 Worker 进程在独立的终端窗格中执行具体任务（如编译、测试、日志监控）。

研究表明，成功的实现依赖于对终端抽象层（Adapter Pattern）的严谨设计，以及对“物理状态”与“逻辑状态”的强一致性管理。项目的关键挑战在于如何防止 AI 被冗长的终端日志（Token Flooding）淹没，以及如何处理由于终端交互产生的阻塞（Interactive Prompts）和僵尸进程。推荐采用 Rust 生态系统，利用其高性能异步处理和成熟的终端控制库。

## Key Findings

### [STACK.md](./STACK.md)
- **核心语言:** Rust 1.75+ (高性能、类型安全、卓越的终端生态)。
- **异步运行时:** Tokio (处理并发监控与 IPC 的行业标准)。
- **持久化层:** SQLite (rusqlite) 用于存储会话、窗格元数据及任务状态。
- **终端对接:** `tmux_interface` 对接 Tmux；通过 `tokio::process` 调用 `wezterm cli` 获取 JSON 输出对接 WezTerm。
- **通信层:** Unix Domain Sockets (UDS) 用于 Master 与 Worker 间的实时意图注入与进度流式传输。

### [FEATURES.md](./FEATURES.md)
- **基础功能 (Table Stakes):** 窗格生命周期管理、命令注入 (`send-keys`)、输出捕获、人类确认网关 (HITL)。
- **核心差异化:** 层级化意图注入 (Master 调度 Worker)、状态感知的物理调度（自动弹出错误窗格）、跨窗格上下文同步。
- **非功能约束:** 坚持“本地优先”，不开发自定义终端模拟器，专注于物理调度而非通用聊天。

### [ARCHITECTURE.md](./ARCHITECTURE.md)
- **模式:** Master-Worker (Orchestrator-Executor)。
- **适配器模式:** 抽象 `TerminalAdapter` 接口，解耦业务逻辑与 WezTerm/Tmux 的具体 CLI 实现。
- **状态驱动:** 终端布局应视为 `state.db` 的投影，确保崩溃后可重新挂载。
- **IPC 设计:** 使用 UDS 建立 Master (Server) 与 Worker (Client) 的长连接，避免轮询日志。

### [PITFALLS.md](./PITFALLS.md)
- **上下文爆炸:** 原始日志（ANSI 码、进度条）会迅速耗尽 Token。必须在 Rust 层实现摘要过滤器。
- **僵尸进程:** 窗格关闭不等于进程终止。必须使用进程组管理（Process Group）确保清理。
- **交互阻塞:** 交互式提示（如 `[y/N]`）会导致自动化停滞。需强制非交互参数并配合超时检测。
- **定位漂移:** 严禁使用索引（如 `0:1.0`）定位窗格，必须使用持久化的唯一 ID。

## Implications for Roadmap

### Suggested Phase Structure

1.  **Phase 1: 终端适配层 (Terminal Abstraction)**
    - **Rationale:** 屏蔽 WezTerm 和 Tmux 的差异是后续所有功能的基础。
    - **Deliverables:** `TerminalAdapter` 接口，支持基础的 Split, SendKeys, GetOutput。
    - **Avoid Pitfalls:** 统一使用 Unique IDs 而非 Index。

2.  **Phase 2: 状态机与 IPC 通信 (Core Service)**
    - **Rationale:** 建立 Master 与 Worker 的双向通信链路。
    - **Deliverables:** SQLite 状态存储，Unix Socket 握手协议，`vibe worker` 引导程序。
    - **Features:** 意图注入 (Intent Injection)。

3.  **Phase 3: 监控与输出流处理 (Observability)**
    - **Rationale:** 解决 Token 消耗和反馈环路问题。
    - **Deliverables:** ANSI 剥离器，基于滑动窗口的日志摘要器，退出码捕获。
    - **Avoid Pitfalls:** 实现上下文摘要以防止 Token 溢出。

4.  **Phase 4: 安全与生命周期管理 (Safety & Cleanup)**
    - **Rationale:** 确保工具的鲁棒性和用户信任。
    - **Deliverables:** HITL 确认网关，进程组收割机制 (Zombie cleanup)，超时自动检测。
    - **Features:** `--confirm` 模式。

5.  **Phase 5: 高级调度与 TUI (UX Enhancements)**
    - **Rationale:** 提升多智能体协作的直观性。
    - **Deliverables:** Ratatui 仪表盘，多窗格上下文自动同步逻辑。

### Research Flags
- **需要深入研究:** 复杂的 ANSI 转义码处理（尤其是进度条）可能需要专门的解析库；在不同 OS 上对进程组管理的细微差异。
- **成熟模式 (可跳过研究):** SQLite 架构、Tokio 异步模式、Clap 参数解析。

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| **Stack** | HIGH | Rust 在终端工具领域的统治地位非常明确，选型标准。 |
| **Features** | HIGH | 明确了“调度器”而非“聊天机器人”的定位。 |
| **Architecture** | HIGH | Master-Worker 模式在 Agent 领域已验证，适配器模式解决了多终端支持。 |
| **Pitfalls** | HIGH | 识别出了终端自动化特有的边缘情况（ANSI、僵尸进程、交互阻塞）。 |

### Gaps to Address
- **环境变量持久化:** 需要在 Requirements 阶段详细定义如何在不同 Shell 会话间可靠地传播变量。
- **Worker 退出策略:** 当 Master 进程被 `SIGKILL` 时，Worker 应该采取何种降级策略？

## Sources

- [WezTerm CLI Documentation](https://wezfurlong.org/wezterm/cli/index.html)
- [Tmux Man Pages (Control Mode)](https://man7.org/linux/man-pages/man1/tmux.1.html)
- [Rust `tmux_interface` Crate](https://crates.io/crates/tmux_interface)
- [Multi-Agent Design Patterns (Master-Worker)](https://github.com/microsoft/autogen)
- [Process Management Pitfalls (Nix Crate Docs)](https://docs.rs/nix/latest/nix/)
