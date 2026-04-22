# Roadmap: vibe

## Overview

Milestone 6.0 是一次产品形态转型：从独立重型 `vibe-cli` 编排系统，转向 plugin-first 的多模型协作系统。Plugin 负责向当前 AI 终端注入 skills、commands、references 和轻量 scripts runtime；`.vibe` 负责项目级 Agent、任务、运行、锁、审查和日志状态；当前主模型担任 Conductor，按需启动 claude/gemini/codex 等子 Agent 执行与审查任务。

## Past Milestones

- ✓ **v1.0 - v3.0**: 基础窗格管理、早期状态管理和 MCP 方向探索。
- ✓ **v4.0**: 转向 stateless 总线架构，完成 UUID 身份注入、`vibe spawn`、`vibe signal/wait` 和 WezTerm/Tmux 协作基础。
- ✓ **v5.0**: 完成双向通信闭环、CLI 初始化增强、全自动流水线验证和高可靠性交互修复。

## Milestone 6.0: Plugin-first 多模型协作转型

**Milestone Goal:** 让用户通过安装 plugin 启用多模型协作，而不是学习和操作独立重型 CLI；把必要 runtime 能力瘦身为 plugin/scripts，并保留 `.vibe` 作为可观察、可恢复的项目工作区。

## Phases

- [ ] **Phase 20: Plugin-first 架构与迁移边界** - 定义 plugin 包结构、协作协议、skills/commands/references/scripts 分工，并分类旧 CLI 能力的迁移去向。
- [x] **Phase 21: `.vibe` 工作区与 Agent 定义** - 实现 plugin 初始化后的 `.vibe` 目录、Agent 文件格式、项目配置和非破坏性迁移规则。 (completed 2026-04-22)
- [ ] **Phase 22: 轻量 scripts runtime** - 用 JS/Python 实现任务落盘、文件锁、Agent subprocess 启动、日志和结果收集。
- [ ] **Phase 23: 多模型执行与审查闭环** - 让主模型通过 plugin 协议完成澄清、拆分、执行、review、修复和恢复。
- [ ] **Phase 24: Release 总结与 CLI 瘦身收束** - 将 release commit 总结做成 plugin command/script，并按迁移分类瘦身或归档 Rust CLI。

## Phase Details

### Phase 20: Plugin-first 架构与迁移边界
**Goal**: 锁定彻底 plugin-first 的产品架构，明确哪些能力属于 skills、commands、references、scripts、`.vibe`，以及旧 CLI 哪些能力迁移、保留或移除。
**Depends on**: Phase 19
**Requirements**: PLUG-01, PLUG-02, PLUG-03, PLUG-04, PLUG-05
**Success Criteria** (what must be TRUE):
  1. 项目有明确 plugin 包目录设计，说明 skills、commands、references 和 scripts 如何被 AI 终端加载。
  2. references 定义协作协议、任务合同、Agent 合同、review 协议和 `.vibe` 工作区布局。
  3. skills 明确当前主模型作为 Conductor 的行为：澄清需求、拆计划、派任务、调 reviewer、聚合结果。
  4. commands 明确暴露 init、plan、run task、review task、status、release summary 等主要入口。
  5. 旧 Rust CLI 能力被分类为 migrate-to-script、compatibility 或 remove，并说明理由。
**Plans**: 4 plans
Plans:
- [x] 20-01-PLAN.md — scaffold `plugin/vibe/` package, scripts boundary, `.vibe` templates, and examples index.
- [x] 20-02-PLAN.md — create model-readable plugin architecture, workspace, collaboration, task, Agent, and review references.
- [ ] 20-03-PLAN.md — create Conductor skill and init/plan/run-task/review-task/status/release-summary command contracts.
- [x] 20-04-PLAN.md — classify old Rust CLI commands, state, env, and bus concepts for migration.

### Phase 21: `.vibe` 工作区与 Agent 定义
**Goal**: Plugin 启用后可以非破坏性创建项目级 `.vibe` 工作区，并用 `.vibe/Agents` 定义 planner、executor、reviewer、release 等角色和模型命令。
**Depends on**: Phase 20
**Requirements**: VIBE-01, VIBE-02, VIBE-03, VIBE-04, VIBE-05
**Success Criteria** (what must be TRUE):
  1. 初始化会创建 `.vibe/Agents`、`.vibe/tasks`、`.vibe/runs`、`.vibe/locks`、`.vibe/reviews`、`.vibe/logs` 和配置文件。
  2. Agent 文件可以声明角色、模型命令、prompt/reference、允许工具和预期输出。
  3. `.vibe/config.json` 可以记录默认模型、Agent 定义、并发限制、任务路径、锁策略、review 策略和 release 设置。
  4. 初始化不会覆盖用户修改过的 `.vibe` 文件，除非显式 force。
  5. `.vibe` 格式足够直观，当前模型可以直接读取并理解当前协作状态。
**Plans**: 2 plans
Plans:
- [x] 21-01-PLAN.md — Update the `.vibe` workspace templates to use a pure JSON schema.
- [x] 21-02-PLAN.md — Implement the plugin initialization script to safely scaffold `.vibe` workspaces.

### Phase 22: 轻量 scripts runtime
**Goal**: 用小型 JS/Python scripts 提供 plugin 必需的 runtime 原语，替代独立重型 CLI 的核心执行职责。
**Depends on**: Phase 20, Phase 21
**Requirements**: RUN-01, RUN-02, RUN-03, RUN-04, RUN-05
**Success Criteria** (what must be TRUE):
  1. scripts 可以创建包含 goal、context、file scope、constraints、expected output、verification command 和 reviewer requirements 的 task JSON。
  2. scripts 可以用项目本地 lock 文件获取和释放任务拥有路径。
  3. scripts 可以按 `.vibe/Agents` 配置启动 claude、gemini、codex 或其他 Agent 命令作为 subprocess。
  4. scripts 可以把 stdout、stderr、exit code、时间戳和结果 artifact 写入 `.vibe/runs` 与 `.vibe/logs`。
  5. runtime 不需要独立 server、数据库或 daemon，代码保持可读、可移植、可由 plugin 调用。
**Plans**: TBD

### Phase 23: 多模型执行与审查闭环
**Goal**: 当前主模型通过 plugin 协议完成从用户澄清到 executor/reviewer 协作、修复循环和中断恢复的完整任务闭环。
**Depends on**: Phase 21, Phase 22
**Requirements**: FLOW-01, FLOW-02, FLOW-03, FLOW-04, FLOW-05
**Success Criteria** (what must be TRUE):
  1. 主模型会先与用户多轮澄清，再把计划持久化到 `.vibe/tasks`。
  2. 主模型可以把计划拆成独立或有序任务，并尊重文件范围与依赖顺序。
  3. 主模型可以根据 Agent 定义、模型命令、任务类型和文件所有权选择 executor。
  4. Reviewer Agent 可以检查 executor 输出，产出结构化 findings，并在完成前触发修复。
  5. 中断后可以从 `.vibe` 恢复，区分 queued、running、blocked、review-needed、failed 和 completed。
**Plans**: TBD

### Phase 24: Release 总结与 CLI 瘦身收束
**Goal**: 将 release commit 总结作为 plugin command/script 提供，并按迁移分类瘦身或归档旧 Rust CLI 能力。
**Depends on**: Phase 22
**Requirements**: REL-01, REL-02, REL-03, REL-04, REL-05
**Success Criteria** (what must be TRUE):
  1. release command 可以从最新 tag 或显式 `--from/--to` 参数确定 commit 区间。
  2. release 总结可以用确定性规则把 commits 分为 features、fixes、docs、tests、refactors 和 internal changes。
  3. release 总结可以在可用时包含 changed files、phase references 和 task/review artifacts。
  4. release command 可以无网络生成本地 GitHub release notes 草稿。
  5. Rust CLI 被按 Phase 20 的分类瘦身或归档，保留功能必须能解释为什么不能迁入 plugin scripts。
**Plans**: TBD

## Progress

**Execution Order:** Phase 20 -> Phase 21 -> Phase 22 -> Phase 23 -> Phase 24

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 20. Plugin-first 架构与迁移边界 | v6.0 | 3/4 | In Progress | - |
| 21. `.vibe` 工作区与 Agent 定义 | v6.0 | 2/2 | Complete   | 2026-04-22 |
| 22. 轻量 scripts runtime | v6.0 | 0/TBD | Not started | - |
| 23. 多模型执行与审查闭环 | v6.0 | 0/TBD | Not started | - |
| 24. Release 总结与 CLI 瘦身收束 | v6.0 | 0/TBD | Not started | - |