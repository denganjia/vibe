# Vibe

## Current Milestone: v7.0 Universal Plugin & MCP Integration

**Goal:** 将 Vibe 彻底升级为全面支持三大主流 AI 终端（Gemini CLI、Claude Code、Codex CLI）的标准化插件，引入 Model Context Protocol (MCP) 将底层协作原语暴露给大语言模型，实现开箱即用。

**Target features:**
- 统一插件入口与包管理 (Universal Manifests & Packaging)
- 技能元数据标准化 (Skill Standardization)
- 底层脚本向 MCP 重构 (MCP Server Integration)
- 协作工作流与文档对齐 (Workflow & Documentation Update)

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
- ✓ **Plugin-first 架构合同** — v6.0
- ✓ **`.vibe` 项目工作区初始化** — v6.0
- ✓ **轻量 scripts runtime** — v6.0
- ✓ **多模型执行与审查闭环** — v6.0
- ✓ **GitHub release commit 总结** — v6.0

### Active

- [ ] **统一插件入口与包管理**: 在 plugin/vibe/ 中初始化 package.json，并补充 Gemini CLI、Claude Code 和 Codex CLI 所需的扩展清单文件。
- [ ] **技能元数据标准化**: 确保所有协作角色均符合各平台兼容的 YAML Frontmatter 解析规范，统一 `skills/` 目录。
- [ ] **底层脚本向 MCP 重构**: 实现轻量级本地 MCP Server，将任务落盘、加锁、生成总结等操作重构为 Tools，摒弃脆弱的 shell 注入方式。
- [ ] **协作工作流与文档对齐**: 引导主控模型 (Conductor) 使用 MCP Tools 操作工作区，并同步更新相关的系统和产品文档。

### Out of Scope

- **命令级强实时审计** — v4.0 转向任务级自治，不再审批每一个输入的 shell 命令。
- **全局同步数据库** — 放弃复杂的 DB 同步，专注于本地项目级 `.vibe` 持久化。
- **独立重型 CLI 作为主产品形态** — v6.0 转向 plugin-first，CLI 能力只保留为 scripts runtime 或迁移兼容层。
- **终端 pane 编排作为唯一执行方式** — 子 Agent 优先通过 shell/subprocess 启动，terminal adapter 仅作为可选兼容能力。

## Context

Shipped Milestone 6.0 (Plugin-first 多模型协作转型).
Existing JS scripts and `.vibe` workspace demonstrated the viability of a lightweight runtime for plugin-based collaboration.
New v7.0 direction: Universal Plugin & MCP Integration. We are replacing the raw shell script invocation pattern with a standardized Model Context Protocol (MCP) server. This makes Vibe directly compatible with major AI CLIs (Gemini, Claude, Codex) without requiring the Conductor model to write fragile shell commands to manipulate the workspace.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| 战略转型 (Pivot 4.0) | 从“控制”转向“协同”，符合高度自治代理的交互趋势。 | ✓ Good |
| 移除 MCP | AI 代理直接调用 CLI 更加符合 Unix 哲学且性能更高。 | ✗ Reverted in v7.0 |
| 使用 .vibe 目录 | 简化状态管理，方便版本控制（git）与上下文感知。 | ✓ Good |
| 无状态信号总线 | 移除守护进程，通过终端注入和 stdin 轮询实现轻量级通信。 | ✓ Good |
| 转向 plugin-first | 主流 AI CLI 已支持 skills/commands/plugins，用户不应先学习独立编排 CLI。 | ✓ Good (v6.0) |
| CLI 瘦身为 plugin/scripts runtime | 初始化、任务落盘、锁、Agent 启动和结果收集代码量小，适合放进 plugin scripts。 | ✓ Good (v6.0) |
| 采用 MCP 替代 raw scripts | 提升跨端兼容性与工具调用安全性，避免 shell 注入脆弱性。 | — Pending (v7.0) |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-23 after v7.0 MCP integration pivot*
