# vibe-cli

## What This Is

`vibe-cli` 是一个基于 Rust 构建的物理调度层，专为终端（Wezterm/Tmux）中的 AI Agent 设计。它将 AI Agent 转化为能够自主操控多窗格协作、共享上下文并实现任务闭环的“终端虚拟操作员”，让开发者通过指挥 AI 团队在真实的窗口与文件系统中“并联作业”。

## Core Value

打破 AI 与本地开发环境之间的“次元壁”，将终端从单纯的字符输入框升级为 AI 协作的物理调度室。

## Requirements

### Validated

<!-- Shipped and confirmed valuable. -->

(None yet — ship to validate)

### Active

<!-- Current scope. Building toward these. -->

- [ ] **窗格编排 (Orchestration)**：支持 Wezterm/Tmux 窗格的自动化切分、聚焦与关闭。
- [ ] **意图注入 (Intent Injection)**：Master AI (如 Claude) 能通过终端输入向 Worker Agent 注入任务意图。
- [ ] **进度监控 (Monitoring)**：实时捕获 Worker 窗格的输出，并向 Master AI 回传关键进度。
- [ ] **状态管理 (State Management)**：通过 Rust 维护 `.vibe/state.db`（或 Unix Socket），实现跨窗格的 AI 角色与任务状态同步。
- [ ] **自动任务闭环 (Autonomous Loop)**：Worker 完成任务后自动触发 Master 审核，并根据反馈进行修正或清理现场。

### Out of Scope

<!-- Explicit boundaries. Includes reasoning to prevent re-adding. -->

- **原生终端 GUI** — 初始版本（MVP）将通过调用 Wezterm/Tmux 的 CLI 工具实现，避免复杂的 GUI 开发。
- **复杂的多主机调度** — 现阶段专注于本地终端工作空间。

## Context

- **技术环境**：本项目使用 Rust 开发，追求极致的性能与安全性。
- **目标用户**：习惯于在终端环境（Wezterm/Tmux）中使用 AI 进行重度开发的开发者。
- **应用场景**：处理复杂的 Electron Monorepo 或涉及多进程协作的现代软件项目。

## Constraints

- **Tech Stack**: Rust — 确保作为系统级工具的稳定性和分发效率。
- **Dependency**: Wezterm/Tmux CLI — MVP 依赖于终端自带的 CLI 工具（如 `wezterm cli`）。
- **Environment**: MacOS/Linux — 终端开发者集中的主流操作系统。

## Key Decisions

<!-- Decisions that constrain future work. Add throughout project lifecycle. -->

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| 使用 CLI Wrapper | 快速跑通 MVP，利用 Wezterm/Tmux 现有的成熟功能。 | — Pending |
| Rust 开发 | 保证调度层的低延迟与高可靠性，适合文件监听与 Socket 通信。 | — Pending |

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
