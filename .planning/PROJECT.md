# vibe-cli

## Current Milestone: Milestone 4.0 - AI Agent Bus

**Goal:** 将 `vibe-cli` 从“受控编排层”重构为“AI 代理协同总线”，实现高度自治的代理协作流。

**Strategic Pivot:**
- **From Control to Bus**: 从“主从命令注入”转向“分布式自治代理通信”。
- **Zero Friction**: 信任子 Agent 行为，仅在任务交付时进行人工 Review。
- **Lightweight**: 移除 MCP 与 SQLite DB，转而使用项目本地 `.vibe/` 目录。

**Target features:**
- `vibe spawn`: 自动开窗、指派角色并启动子 AI 进程。
- `vibe signal`: 代理向总线发送状态信号。
- `vibe wait`: 阻塞等待特定信号返回，实现工作流闭环。
- `.vibe/` context: 管理角色模板、项目进度与全局配置。

## What This Is

`vibe-cli` 是一个基于 Rust 构建的 **AI 代理协同总线**。它将终端窗格转化为受控的“工作站”，让 AI 能够自主操控多窗格协作、通过 `.vibe` 目录管理项目进度，并利用 `signal/wait` 机制实现复杂的任务闭环。

## Core Value

打破 AI 与本地开发环境之间的“次元壁”，将终端从单纯的字符输入框升级为分布式 AI 协作的物理调度室。

## Requirements

### Validated

- [x] **窗格编排 (Orchestration)**：支持 Wezterm/Tmux 窗格的自动化切分、聚焦与关闭 (Phase 1-2).
- [x] **状态管理 (State Management)**：初步通过 SQLite 实现跨窗格同步 (Phase 3 - 将在 v4.0 简化).
- [x] **实时监控 (Monitoring)**：TUI 仪表盘实时展示 Agent 状态与日志快照 (Phase 6).
- [x] **受控编排 (Controlled Workflow)**：实现“计划-确认-执行”的任务节点流 (Phase 9 - 将在 v4.0 升级为自治 Review).
- [x] **Vibe-Operator Skill**: 定义了多模型协作 SOP 与重构工作流模板 (Phase 10-12).

### Active

- [ ] **架构极简化**：移除 MCP、移除沉重的 SQLite 数据库。
- [ ] **物理层编排 (Spawner)**：实现 `vibe spawn --role <ROLE>`。
- [ ] **信号总线 (Messaging)**：实现 `vibe signal` 与 `vibe wait`。
- [ ] **轻量级上下文 (.vibe)**：使用本地 `.vibe/` 目录管理项目进度。

### Out of Scope

- **命令级强实时审计** — v4.0 转向任务级自治，不再审批每一个输入的 shell 命令。
- **全局同步数据库** — 放弃复杂的 DB 同步，专注于本地项目级 `.vibe` 持久化。

## Constraints

- **Minimalism**: 核心二进制文件必须极其轻量，作为纯粹的通信总线。
- **Autonomous First**: 流程设计必须优先考虑 AI 的自治能力，减少人类干预频率。

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| 战略转型 (Pivot 4.0) | 从“控制”转向“协同”，符合高度自治代理的交互趋势。 | Approved |
| 移除 MCP | AI 代理直接调用 CLI 更加符合 Unix 哲学且性能更高。 | Approved |
| 使用 .vibe 目录 | 简化状态管理，方便版本控制（git）与上下文感知。 | Approved |

---
*Last updated: 2026-04-17 after Strategic Pivot to Milestone 4.0*
