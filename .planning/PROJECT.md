# vibe-cli

## Current Milestone: Milestone 5.0 - [Planning Next Milestone]

**Goal:** 持续优化 AI 代理协同总线，提升稳定性并探索多代理复杂协作场景。

**Strategic Pivot:**
- **From Control to Bus**: 从“主从命令注入”转向“分布式自治代理通信”。 (Completed in v4.0)
- **Zero Friction**: 信任子 Agent 行为，仅在任务交付时进行人工 Review。
- **Lightweight**: 移除 MCP 与 SQLite DB，转而使用项目本地 `.vibe/` 目录。

## What This Is

`vibe-cli` 是一个基于 Rust 构建的 **AI 代理协同总线**。它将终端窗格转化为受控的“工作站”， 让 AI 能够自主操控多窗格协作、通过 `.vibe` 目录管理项目进度，并利用 `signal/wait` 机制实现复杂的任务闭环。

## Core Value

打破 AI 与本地开发环境之间的“次元壁”，将终端从单纯的字符输入框升级为分布式 AI 协作的物理调度室。

## Requirements

### Validated

- ✓ **窗格编排 (Orchestration)** — v1.0
- ✓ **状态管理 (State Management)** — v1.0 (Refactored to JSON in v4.0)
- ✓ **实时监控 (Monitoring)** — v1.0
- ✓ **Vibe-Operator Skill** — v4.0
- ✓ **架构极简化 (Cleanup)** — v4.0 (Removed MCP & SQLite)
- ✓ **物理层编排 (Spawner)** — v4.0 (Implemented vibe spawn)
- ✓ **信号总线 (Bus Core)** — v4.0 (Implemented vibe signal/wait)
- ✓ **全流程集成 (E2E Integration)** — v4.0

### Active

- [ ] **多平台适配 (OS Compatibility)**: 进一步优化 Windows 下的终端适配稳定性。
- [ ] **性能监控 (Performance Metrics)**: 统计代理任务耗时与资源消耗。
- [ ] **调试工具 (Debugger)**: 提供更直观的信号总线调试视图。

### Out of Scope

- **命令级强实时审计** — v4.0 转向任务级自治，不再审批每一个输入的 shell 命令。
- **全局同步数据库** — 放弃复杂的 DB 同步，专注于本地项目级 `.vibe` 持久化。

## Context

Shipped Milestone 4.0 (AI Agent Bus).
Current LOC: ~1600 (Core Rust code reduced after removing MCP/SQLite).
Tech stack: Rust, Tokio, Ratatui, WezTerm/Tmux integration.
Established a stateless signaling bus based on terminal injection.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| 战略转型 (Pivot 4.0) | 从“控制”转向“协同”，符合高度自治代理的交互趋势。 | ✓ Good |
| 移除 MCP | AI 代理直接调用 CLI 更加符合 Unix 哲学且性能更高。 | ✓ Good |
| 使用 .vibe 目录 | 简化状态管理，方便版本控制（git）与上下文感知。 | ✓ Good |
| 无状态信号总线 | 移除守护进程，通过终端注入和 stdin 轮询实现轻量级通信。 | ✓ Good |

---
*Last updated: 2026-04-20 after v4.0 milestone completion*
