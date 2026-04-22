# Vibe

## Current Milestone: Milestone 6.0 - Plugin-first 多模型协作转型

**Goal:** 将项目从独立重型 CLI 编排系统转型为 plugin-first 的多模型协作系统，把必要的运行时代码瘦身后整合进 plugin/scripts。

**Strategic Pivot:**
- **Plugin as Product**: 用户入口从 `vibe-cli` 命令迁移到当前 AI CLI 可安装的 plugin，由 skills/commands/references 注入协作协议。
- **Scripts as Thin Runtime**: 原 CLI 中必要的初始化、任务落盘、锁、Agent 启动、结果收集能力迁移为轻量 JS/Python scripts。
- **`.vibe` as Project Workspace**: `.vibe/Agents`、`.vibe/tasks`、`.vibe/runs`、`.vibe/locks`、`.vibe/reviews` 成为 plugin-first 协作状态与配置目录。
- **Current Model as Conductor**: 当前主会话模型通过 plugin skill 自动多轮澄清、拆分任务、启动 claude/gemini/codex 子 Agent、调用 Reviewer 并聚合修复。
- **Release Intelligence**: 发布时从 git commit 历史生成可信的变更总结，作为 plugin command/script 能力提供。

## What This Is

`vibe` 是一个 plugin-first 的 **多模型协作协议与轻量运行时**。它让当前 AI 终端模型成为 Conductor，通过 plugin 注入的 skills、commands、references 和 scripts，在项目 `.vibe` 目录中定义 Agent、任务、状态、审查和发布流程，并按需启动 claude/gemini/codex 等子 Agent 处理任务。

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

- [ ] **Plugin-first 架构合同**: 明确 plugin、references、skills、commands、scripts 与 `.vibe` 工作区的边界。
- [ ] **`.vibe` 项目工作区初始化**: plugin 安装/启用后可生成 `.vibe/Agents`、配置、任务目录和轻量运行时脚本入口。
- [ ] **轻量 scripts runtime**: 用 JS/Python 实现任务落盘、锁、状态、Agent 启动、日志和结果收集，替代独立重型 CLI。
- [ ] **多模型执行与审查闭环**: 主模型自动澄清需求、拆分任务、启动 executor、调用 reviewer、触发修复并汇总交付。
- [ ] **GitHub release commit 总结**: 作为 plugin command/script，从 commit 区间生成结构化 changelog 和 release notes 草稿。

### Out of Scope

- **命令级强实时审计** — v4.0 转向任务级自治，不再审批每一个输入的 shell 命令。
- **全局同步数据库** — 放弃复杂的 DB 同步，专注于本地项目级 `.vibe` 持久化。
- **独立重型 CLI 作为主产品形态** — v6.0 转向 plugin-first，CLI 能力只保留为 scripts runtime 或迁移兼容层。
- **终端 pane 编排作为唯一执行方式** — 子 Agent 优先通过 shell/subprocess 启动，terminal adapter 仅作为可选兼容能力。

## Context

Shipped Milestone 5.0 (Interaction & Initialization).
Existing Rust CLI proved the core ideas: `.vibe` project directory, role templates, file bus, state files, stack spawning, signal/wait, and autonomous loop SOPs.
New v6.0 direction: preserve those validated concepts but move the product surface into plugin skills/commands/references and implement only the necessary runtime primitives as small JS/Python scripts inside the plugin.
Current risk: if runtime scripts are too thin, execution and recovery become unreliable; if they grow too large, the project recreates the heavy CLI in another language.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| 战略转型 (Pivot 4.0) | 从“控制”转向“协同”，符合高度自治代理的交互趋势。 | ✓ Good |
| 移除 MCP | AI 代理直接调用 CLI 更加符合 Unix 哲学且性能更高。 | ✓ Good |
| 使用 .vibe 目录 | 简化状态管理，方便版本控制（git）与上下文感知。 | ✓ Good |
| 无状态信号总线 | 移除守护进程，通过终端注入和 stdin 轮询实现轻量级通信。 | ✓ Good |
| 转向 plugin-first | 主流 AI CLI 已支持 skills/commands/plugins，用户不应先学习独立编排 CLI。 | — Pending |
| CLI 瘦身为 plugin/scripts runtime | 初始化、任务落盘、锁、Agent 启动和结果收集代码量小，适合放进 plugin scripts。 | — Pending |
| `.vibe/Agents` 定义角色与模型 | 角色定义、模型选择和执行策略应成为项目可读配置，而不是硬编码 CLI 状态。 | — Pending |
| 产品名保留为 Vibe | `.vibe`、`vibe` 命令和既有心智都围绕 Vibe；`vibe-cli` 只作为旧实现名。 | — Pending |

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
*Last updated: 2026-04-22 after plugin-first v6.0 pivot*
