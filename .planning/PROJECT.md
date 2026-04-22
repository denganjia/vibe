# vibe-cli

## Current Milestone: Milestone 6.0 - 智能任务流与配置化状态系统

**Goal:** 将 `vibe-cli` 从可运行的多 Agent 协作总线推进到可规划、可分配、可恢复、可发布的自动化任务流系统。

**Strategic Pivot:**
- **Assignment Intelligence**: 从“能 spawn Worker”升级到“能高效且准确地拆分、匹配、派发任务”。
- **Config as System Contract**: `.vibe` 不只是初始化产物，而是角色、能力、任务流、状态目录和发布流程的项目级配置合同。
- **Filesystem State Machine**: 用文件系统表达任务、Worker、信号、锁、结果和恢复点，让状态可观察、可调试、可版本化。
- **Release Intelligence**: 发布时自动从 git commit 历史生成可信的变更总结，减少手写 release note 成本。

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
- ✓ **双向通信闭环** — v5.0
- ✓ **CLI 初始化增强** — v5.0
- ✓ **全自动流水线验证** — v5.0
- ✓ **高可靠性交互修复** — v5.0

### Active

- [ ] **高效且准确的任务分配**: 根据任务类型、文件范围、依赖关系、Worker 能力和当前负载自动拆分与派发任务。
- [ ] **完善的 `.vibe` 配置系统**: 支持配置 schema、校验、合并、默认值、角色能力、任务流模板和项目级覆盖。
- [ ] **基于文件系统的状态机制**: 将任务队列、租约、锁、心跳、结果、失败原因和恢复点持久化到 `.vibe`。
- [ ] **任务流自动化**: 支持任务生命周期从创建、分配、执行、等待、聚合、重试到完成的自动推进。
- [ ] **GitHub release commit 总结**: 发布时从 commit 区间生成结构化 changelog 和 release notes 草稿。

### Out of Scope

- **命令级强实时审计** — v4.0 转向任务级自治，不再审批每一个输入的 shell 命令。
- **全局同步数据库** — 放弃复杂的 DB 同步，专注于本地项目级 `.vibe` 持久化。

## Context

Shipped Milestone 5.0 (Interaction & Initialization).
Current LOC: Rust core and CLI continue to center on `.vibe`, terminal adapters, file bus, and role templates.
Tech stack: Rust, Tokio, Ratatui, WezTerm/Tmux integration.
Established project-local `.vibe/config.json`, `.vibe/state`, `.vibe/bus`, role templates, `vibe spawn --stack`, `vibe signal/wait`, and autonomous loop SOPs.
Current risk: task assignment is still mostly prompt/SOP-driven rather than enforced by a durable task model and scheduler.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| 战略转型 (Pivot 4.0) | 从“控制”转向“协同”，符合高度自治代理的交互趋势。 | ✓ Good |
| 移除 MCP | AI 代理直接调用 CLI 更加符合 Unix 哲学且性能更高。 | ✓ Good |
| 使用 .vibe 目录 | 简化状态管理，方便版本控制（git）与上下文感知。 | ✓ Good |
| 无状态信号总线 | 移除守护进程，通过终端注入和 stdin 轮询实现轻量级通信。 | ✓ Good |
| 优先强化任务分配准确性 | 多 Agent 数量增加后，错误分配比单 Worker 执行失败更昂贵。 | — Pending |
| 继续使用文件系统状态 | 保持可观察、可恢复、无需守护进程的架构方向。 | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-22 after starting Milestone 6.0*
