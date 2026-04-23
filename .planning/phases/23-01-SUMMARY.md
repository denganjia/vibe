---
phase: 23
plan: 01
subsystem: plugin-vibe
tags: [manifest, dependencies, workflow]
requirements: [FLOW-01, FLOW-02]
requires: []
provides: [PLAN-MANIFEST, TASK-DEPENDENCIES]
affects: [run-task.js, status.js]
tech-stack: [Node.js]
key-files: [plugin/vibe/templates/plan.json, plugin/vibe/scripts/run-task.js, plugin/vibe/scripts/status.js]
decisions:
  - 核心脚本 run.js 重命名为 run-task.js 以对齐计划命名并更准确描述其功能。
  - 任务依赖检查失败时，任务状态将自动更新为 'blocked'。
metrics:
  duration: 45m
  completed_date: "2026-04-23"
---

# Phase 23 Plan 01: Implement task dependencies and plan manifest tracking Summary

## 一句话总结
实现了基于 `.vibe/plan.json` 的计划清单机制和任务间的显式依赖强制检查逻辑。

## 主要变更

### 1. 计划清单 (Plan Manifest)
- 创建了 `plugin/vibe/templates/plan.json` 模板。
- 脚本现在可以读取 `.vibe/plan.json` 来识别当前活跃的任务集合和整体目标。

### 2. 任务依赖强制执行
- 实现了 `plugin/vibe/scripts/run-task.js` (由 `run.js` 重命名并升级)。
- 执行任务前会自动检查 `dependencies` 字段。
- 若依赖未满足（缺失或未完成），任务将被标记为 `blocked` 并拒绝执行。

### 3. 状态显示增强
- 更新了 `plugin/vibe/scripts/status.js`。
- 支持 `list` 命令展示包含依赖信息的任务列表。
- 增加了 `--check-schema` 标志用于验证。

## 偏离记录

### 1. [Rule 3 - Blocking Issue] 重命名 run.js 为 run-task.js
- **原因**: 计划中引用的是 `run-task.js`，而现有代码中只有 `run.js`。为了保持一致性并更清晰地表达其作为任务运行器的职责，决定进行重命名。
- **影响**: 更新了 `README.md` 和 `test-runtime.js` 中的引用。
- **提交**: `refactor(23-01): rename run.js to run-task.js to align with plan`

## 验证结果
- **依赖阻塞验证**: 创建依赖任务 B 依赖 A。当 A 未完成时运行 B，B 变为 `blocked`。成功。
- **依赖通过验证**: 将 A 标记为 `completed` 后运行 B，B 进入 `running` 状态。成功。
- **Schema 验证**: `node status.js --check-schema` 返回 OK。

## 自我检查：PASSED
- [x] 创建了 `plugin/vibe/templates/plan.json`
- [x] 实现了 `run-task.js` 的依赖检查逻辑
- [x] 更新了 `status.js` 以支持计划清单和依赖展示
- [x] 所有变更已提交
