# vibe-cli

## What This Is

`vibe-cli` 是一个基于 Rust 构建的物理调度层，专为终端（Wezterm/Tmux）中的 AI Agent 设计。它将 AI Agent 转化为能够自主操控多窗格协作、共享上下文并实现任务闭环的“终端虚拟操作员”，让开发者通过指挥 AI 团队在真实的窗口与文件系统中“并联作业”。

## Core Value

打破 AI 与本地开发环境之间的“次元壁”，将终端从单纯的字符输入框升级为 AI 协作的物理调度室。

## Requirements

### Validated

- [x] **窗格编排 (Orchestration)**：支持 Wezterm/Tmux 窗格的自动化切分、聚焦与关闭 (Phase 1-2).
- [x] **意图注入 (Intent Injection)**：Master AI 能向 Worker Agent 注入任务意图 (Phase 4).
- [x] **状态管理 (State Management)**：通过 Rust 维护 SQLite 状态，实现跨窗格同步 (Phase 3).
- [x] **实时监控 (Monitoring)**：TUI 仪表盘实时展示 Agent 状态与日志快照 (Phase 6).
- [x] **AI 原生集成 (MCP)**：提供标准的 MCP Server 接口供 LLM 调用 (Phase 7).

### Active

- [ ] **受控编排 (Controlled Workflow)**：实现“计划-确认-执行”的任务节点流，关键节点必须人工二次确认。
- [ ] **数据库演进 (State Evolution)**：引入自动迁移机制，支持无损更新数据库 Schema。
- [ ] **工程化分发 (Distribution)**：实现跨平台二进制打包与一键安装脚本。

### Out of Scope

- **复杂多主机调度** — 现阶段依然专注于本地终端工作空间。
- **日志深度检索** — TUI 仅保持最近快照显示，不进行全文搜索实现。

## Context

- **当前状态**：Wave 1 已完成，系统具备 Master-Worker-TUI 核心架构。
- **Wave 2 重心**：从“功能可用”向“生产可靠”和“受控编排”演进。

## Constraints

- **Compatibility**: 必须保证在数据库结构变化时，用户的旧数据能自动迁移或平滑处理。

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| 使用 CLI Wrapper | 快速跑通 MVP，利用 Wezterm/Tmux 现有的成熟功能。 | Validated |
| MCP 协议 | 标准化 AI 接入层，降低集成本地工具的门槛。 | Validated |
| 引入 Migration 框架 | 避免 Schema 变更导致的删库操作，提升生产环境稳定性。 | Planned (W2) |
| GitHub Actions 打包 | 自动化发布流程，提供各平台预编译二进制。 | Planned (W2) |


## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-14 after Initial Questioning*
