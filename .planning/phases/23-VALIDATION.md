# Phase 23 Validation Architecture

**Phase:** 23 - 多模型执行与审查闭环
**Role:** Nyquist Validator
**Standard:** Automated E2E Shell Testing

## Overview
第 23 阶段的验证重点是 Conductor 的 Operating Loop，即从计划生成到多模型执行、审查反馈及最终完成的完整生命周期。我们将使用 Node.js 编写集成测试，模拟 Agent 的各种输出情况。

## Test Strategy

### 1. Integration Tests (`test-conductor.js`)
- **T1: Single Task Happy Path**: `plan` -> `run` -> `review` (pass) -> `completed`.
- **T2: Dependency Chain**: `plan` (2 tasks) -> `run` Task B (should be `blocked`) -> `run` Task A -> `run` Task B (should now be `queued`/`running`).
- **T3: Review-Fix Loop**: `plan` -> `run` -> `review` (fail) -> `fix-needed` -> Conductor resets to `queued` -> `run` -> `review` (pass) -> `completed`.
- **T4: Max Retries**: `plan` -> 3 cycles of `fix-needed` -> `failed` (Max review cycles reached).
- **T5: Recovery**: `plan` -> Start `run` -> Simulate crash (kill process, leave lock) -> `sync` -> Task status marked as `interrupted` or resumed.

### 2. Manual Verification
- 使用 `vibe init` 初始化测试工作区。
- 手动触发 `vibe plan`（模拟用户输入），检查生成的 `plan.json` 和 `tasks/*.json`。
- 运行 `vibe run-task` 并检查日志。

## Truths to Verify

| Truth | Verification Method |
|-------|---------------------|
| Conductor 仅在需求清晰（包含 goal, scope, verification）时生成任务。 | `test-conductor.js` (Clarification Check) |
| 任务 A 依赖任务 B 时，B 未完成前 A 保持 `blocked`。 | `test-conductor.js` (Dependency Check) |
| Reviewer 发现严重问题时，任务状态转为 `fix-needed`。 | `test-conductor.js` (Review Fail Check) |
| 修复循环上限为 3 次。 | `test-conductor.js` (Max Retry Check) |
| `sync` 脚本能识别并报告运行中断的任务。 | `test-conductor.js` (Recovery Check) |

## Artifact Check
- [ ] `.vibe/plan.json` 包含正确的任务列表和依赖图。
- [ ] `.vibe/planning_notes.md` 包含澄清细节。
- [ ] `.vibe/reviews/*.json` 包含结构化的 findings。
