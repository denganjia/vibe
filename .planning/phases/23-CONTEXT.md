# Phase 23: 多模型执行与审查闭环 - Context

**Status:** DECIDED
**Last Updated:** 2026-04-23
**Phase Goal:** 实现 Conductor 自动澄清、任务拆分、依赖管理、多模型执行、审查及中断恢复的完整闭环。

## Locked Decisions

### 1. 澄清机制 (Clarification Strategy)
- **保守计划优先**：Conductor 必须确保 `goal`, `file_scope` 和 `verification` 明确后才写入 `task.json`。
- **持久化上下文**：重要的澄清背景应写入 `.vibe/planning_notes.md`，Executor 启动时需将其作为参考文档读取。

### 2. 任务拆分与依赖管理 (Task Decomposition & Dependencies)
- **显式依赖字段**：在 `task.json` 中引入 `dependencies: string[]`（存储任务 ID）。
- **静态拓扑生成**：Conductor 倾向于在 `plan` 阶段一次性生成所有任务 JSON，以便用户和模型能预览完整流程。
- **阻塞逻辑**：运行时脚本（或 `run.js` 的前置检查）若发现依赖任务未达到 `completed` 状态，应拒绝执行并将当前任务标记为 `blocked`。

### 3. 审查与修复循环 (Review & Fix Loop)
- **结构化反馈**：Reviewer 产出的 Findings 存放在 `.vibe/reviews/<task-id>_run_<run-id>.json`。
- **状态驱动修复**：
    - Reviewer 发现问题时将任务设为 `fix-needed`。
    - Conductor 负责将 Findings 汇总到 `task.json` 的 `context` 或 `constraints` 中，并重置状态为 `queued` 以触发重跑。
- **安全阈值**：单个任务的修复循环上限暂定为 **3 次**。达到上限后状态转为 `failed`，需人工干预。

### 4. 中断恢复与计划管理 (Recovery & Plan Management)
- **Plan Manifest**：引入 `.vibe/plan.json` 记录当前 Plan 的元数据（ID, tasks[], goal）。
- **自愈逻辑**：Conductor 启动时需对比 `plan.json`、`tasks/*.json` 和 `locks/`。若发现任务状态为 `running` 但没有对应进程或锁，应将其标记为 `interrupted` 并提示恢复。

## User Constraints

- 严禁引入外部数据库或中心化 Daemon。
- 所有协作状态必须对齐 `task-contract.md` 和 `agent-contract.md`。

## Gray Areas (OUT OF SCOPE for now)

- 跨项目的任务依赖。
- 复杂的动态计划调整（执行中途大幅度增删任务）。

## Next Steps

1. **Phase 23 Research**: 研究如何编写 Conductor Skill 以及具体的任务状态转移逻辑。
2. **Phase 23 Planning**: 制定具体的脚本更新和 Skill 编写计划。
