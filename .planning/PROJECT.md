# vibe-cli

## Current Milestone: Milestone 5.0 - 多 Agent 交互增强与 CLI 初始化标准化

**Goal:** 完善多智能体间的双向通信机制，实现高自治的开发流水线，并标准化项目级的环境初始化流程。

**Strategic Pivot:**
- **Autonomous Flow**: 从“手动干预”转向“信号驱动的自动化”。
- **Config-First**: 强化 `.vibe` 配置文件的核心地位，通过配置定义角色行为和交互逻辑。
- **Closed Loop**: 确保主会话与 Worker 之间的“启动-任务-回复-关闭”链路完全打通。

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
- ✓ **按角色配置命令** — v4.0 (v5.0 Pre-work)

### Active

- [ ] **双向通信闭环**: 完善 Worker 对主会话的自动化回复与数据传递机制。
- [ ] **CLI 初始化增强**: 实现项目根目录 `.vibe` 配置文件驱动的快速环境初始化。
- [ ] **全自动流水线**: 实现从任务分配到最终交付的端到端自动化。
- [ ] **高可靠性交互**: 解决交互式 CLI（如 Claude/Gemini）在不同环境下的输入输出解析问题。

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
