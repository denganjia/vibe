# RETROSPECTIVE

## Milestone: v4.0 — AI Agent Bus

**Shipped:** 2026-04-20
**Phases:** 4 | **Plans:** 5

### What Was Built
- 架构清理：移除了 MCP 和 SQLite，极大地简化了代码库并提升了启动速度。
- 信号总线：实现了无状态的 `vibe signal` 和 `vibe wait` 机制。
- 自治启动器：实现了 `vibe spawn`，支持从 `.vibe/roles/` 注入 Persona。
- 状态管理：引入了基于文件锁的 `panes.json` 持久化方案，确保并发安全。

### What Worked
- **终端注入通信**：证明了通过终端 buffer 注入特定 marker 是一种极简且跨语言的 AI 通信方式，无需依赖复杂的 IPC 守护进程。
- **文件锁并发控制**：在移除数据库后，简单的 `fd-lock` 成功支撑了多 Agent 并发访问状态文件的需求。

### What Was Inefficient
- **Windows 适配瓶颈**：WezTerm 的 CLI 在不同平台（尤其是 Windows）下的行为差异导致了多次重构和测试。
- **E2E 环境搭建**：Mock 真实的终端行为（如 WezTerm CLI）在非交互式环境下非常具有挑战性，消耗了较多时间。

### Patterns Established
- **Stateless Signaling**: 通过 stdin 轮询和特定格式的 stdout 标记实现异步解耦。
- **Role-based Spawning**: 进程启动即注入上下文，消除了 Agent 启动后的“冷启动”感知。

### Key Lessons
- Unix 哲学（文件、管道、标准流）在 AI 代理编排中依然极其强大，过度设计（如 MCP/UDS）往往会增加维护成本和集成难度。

## Cross-Milestone Trends

| Milestone | Plans | Status | Efficiency |
|-----------|-------|--------|------------|
| v4.0      | 5     | Shipped| High (Bus rewrite) |
