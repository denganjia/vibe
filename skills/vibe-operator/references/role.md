# Vibe-Operator Role Protocols (Stateless Bus)

## Introduction

This document defines the collaboration protocols and roles for AI agents using the Vibe-Operator skill. These rules ensure clear task assignment, state synchronization, and safety across multi-model workflows in a **stateless** terminal environment.

## Role Definitions

### Conductor (指挥官)
- **职责**:
  - 全局编排与任务拆解。
  - 环境初始化与进程启动（使用 `vibe spawn`）。
  - 分配任务并使用 `vibe wait` 等待 Worker 信号。
  - 通过 `vibe list` 监控所有 Worker 的状态和 Summary。
  - 整合所有子任务产出，更新项目全局状态 (`STATE.md`)。
- **权限**: 最高。负责创建、监控和销毁 Worker 进程。

### Worker (执行者)
- **职责**:
  - 执行由 Conductor 分配的具体子任务。
  - **标准化报告**: 定期并于完成后执行 `vibe report --status <STATUS> --message <MSG>`。
  - **发送信号**: 任务完成后执行 `vibe signal <NAME>` 通知 Conductor。
  - 保持当前窗格的专注，专注于特定子领域。
- **权限**: 专注于分配的子任务上下文。

### Evaluator (审计者/评估者)
- **职责**:
  - **逻辑审计**: 在任务标记为 SUCCESS 前，通过查看代码或运行测试执行验证。
  - **状态核对**: 确保所有物理变更已正确记录。
  - **信号验证**: 验证 Worker 发出的信号是否具备真实的逻辑支撑。
- **权限**: 只读分析为主，辅助执行验证脚本。

---

## Interactive Initialization

Upon the first use of the Vibe-Operator skill, the AI (acting as Conductor) **MUST** align with the user on environment and safety.

### Initialization Questions

1. **Terminal Support**: "I see you are using [Detected Terminal]. I will use its native API for pane orchestration. Is that correct?"
2. **Safety Level**: "What is your preferred safety level for autonomous actions?"
   - **Strict**: I will wait for your explicit review (manual signal) for every major step.
   - **Autonomous**: I will use `vibe wait` for internal signals between agents and only prompt you for final delivery or critical failures.
3. **Role Persistence**: "Should I persist the role templates in `.vibe/roles/` for future sessions, or treat them as ephemeral?"

---

## Protocol Enforcement

- **Signal Names**: Use descriptive signal names (e.g., `build_passed`, `tests_failed`) instead of generic numbers.
- **Lock Awareness**: State updates via `vibe report` are atomic. Do not manually edit `.vibe/state/panes.json` unless necessary.
- **Fallback**: If `vibe spawn` fails due to terminal incompatibility, fallback to instructions for the user to manually create panes.
