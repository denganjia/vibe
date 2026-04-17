# AI Agent Bus Implementation Research Summary

## Executive Summary

`vibe-cli` 的 “AI 代理总线 (AI Agent Bus)” 旨在构建一个轻量级、去中心化的协调层，用于支持多代理协作流程。该方案摒弃了复杂的中心化编排器，转而采用 **轻量级守护进程 (Daemon)** + **项目本地上下文 (.vibe)** 的模式。核心技术栈基于 Rust/Tokio，利用 Unix Domain Sockets (UDS) 实现高性能的跨进程通信。

核心实现策略包括：
1.  **Signal/Wait 机制**: 通过 UDS 连接挂起模拟异步信号，解决代理间的同步问题。
2.  **角色注入 (Role Injection)**: 利用 Stdin 管道在代理启动时注入 Persona 提示词，实现角色的快速切换与定制。
3.  **本地大脑 (.vibe)**: 将角色定义、配置和临时状态持久化在项目目录下，确保 AI 代理具备“自我管理”意识且易于团队共享。

主要风险点在于 UDS 路径长度限制（108 字节）以及交互式程序（如 SSH/Vim）在 Stdin 重定向时的 TTY 冲突。

---

## Key Findings

### From STACK.md (技术栈)
*   **核心通信**: Rust + Tokio (异步运行时) + NDJSON (流式协议) + UDS (进程间通信)。
*   **进程管理**: `tokio::process` 用于异步子进程启动与管道处理。
*   **持久化**: Markdown/TOML。优先考虑易读性和 Git 友好性，替代重量级的数据库。

### From FEATURES.md (功能图景)
*   **核心功能**: `vibe spawn` (角色启动), `vibe signal` (信号发送), `vibe wait` (阻塞等待), `.vibe/roles` (模板管理)。
*   **差异化优势**: 跨窗口/面板信号同步、项目本地上下文共享。
*   **原则**: 避免全局注册中心，坚持项目本地化 (Project-Local)。

### From ARCHITECTURE.md (架构模式)
*   **去中心化总线**: 守护进程仅负责消息转发和信号挂起，不维护复杂的任务状态机。
*   **Stdin "预热" 模式**: 在子进程启动后立即通过 Stdin 写入角色定义，随后接管终端 IO。
*   **路径哈希化**: 为避免 UDS 路径限制，将 Socket 存放在 `/tmp` 并基于项目路径哈希命名。

### From PITFALLS.md (潜在坑点)
*   **路径溢出**: UDS 路径在 Unix 系统下有 108 字符硬限制。
*   **僵尸等待者**: 客户端崩溃可能导致服务端资源泄漏，需实现心跳检查和强制超时。
*   **TTY 丢失**: 重定向 Stdin 会破坏某些交互式工具的 TTY 特性，建议在高级阶段使用终端适配器（WezTerm/Tmux CLI）。

---

## Implications for Roadmap

### Suggested Phase Structure

1.  **Phase 1: 核心总线与信号机制 (Foundation)**
    *   **Rationale**: 建立通信基础，信号同步是所有协作的前提。
    *   **Deliverables**: `vibe daemon`, `vibe signal`, `vibe wait`。
    *   **Pitfalls**: 需立即解决 UDS 路径限制和连接清理逻辑。

2.  **Phase 2: 代理启动与角色注入 (Spawning)**
    *   **Rationale**: 实现代理的生命周期管理。
    *   **Deliverables**: `vibe spawn --role`, `.vibe/roles/` 支持。
    *   **Pitfalls**: 处理 Stdin 写入的竞态条件。

3.  **Phase 3: 上下文与持久化 (Memory)**
    *   **Rationale**: 提供跨代理的“记忆”。
    *   **Deliverables**: `active_context.md` 自动同步。

4.  **Phase 4: 终端适配与 PTY 优化 (Advanced)**
    *   **Rationale**: 解决交互式代理的体验问题。
    *   **Deliverables**: WezTerm/Tmux 专用 `send-text` 适配。

### Research Flags
*   **需深入调研**: Phase 4 的 PTY 无损注入方案（可能需要调研 `nix` 库的 PTY 处理）。
*   **标准模式**: Phase 1 & 2 均有成熟的 Rust/Tokio 模式可循，可快速推进。

---

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| **Stack** | HIGH | 基于 Rust 工业级实践，方案稳健。 |
| **Features** | HIGH | 需求明确，MVP 范围可控。 |
| **Architecture** | HIGH | 去中心化模式符合 vibe-cli 轻量化原则。 |
| **Pitfalls** | MEDIUM | TTY 冲突是通用技术难题，需要特定终端适配。 |

**Gaps to Address:**
*   Windows 环境下的命名管道 (Named Pipes) 适配尚未详细调研。
*   多用户环境下 `/tmp` 路径下的 Socket 安全权限隔离需在实施中明确。

---

## Sources
*   .planning/research/AGENT_BUS_IMPLEMENTATION.md
*   .planning/research/STACK.md
*   .planning/research/FEATURES.md
*   .planning/research/ARCHITECTURE.md
*   .planning/research/PITFALLS.md
