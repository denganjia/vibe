# Vibe-Operator Role Protocols (Stateless Bus)

## Introduction

This document defines the collaboration protocols and roles for AI agents using the Vibe-Operator skill. These rules ensure clear task assignment, state synchronization, and safety across multi-model workflows in a **stateless** terminal environment.

## Role Definitions

### Conductor (指挥官)
- **职责**:
  - 全局编排与任务拆解。
  - 环境初始化与批量启动（使用 `vibe init` 和 `vibe spawn --stack <NAME>`）。
  - **Intelligence-First 路由**: 分配任务并使用 `vibe wait` 监听 `.vibe/bus/`，基于解析到的信号载荷进行推理决策。
  - 通过 `vibe list` 监控所有 Worker 的状态和 Intent Locks。
  - 整合所有子任务产出，生成或更新最终交付物 (`DELIVERY.md` 或 `STATE.md`)。
- **权限**: 最高。负责创建、监控、干预 (`vibe inject`) 和销毁 Worker 进程。

### Worker (执行者)
- **职责**:
  - **A-D-E-V 循环**: 严格遵循 Analyze-Declare-Execute-Verify 自治循环。
  - **Intent Locking**: 在修改文件前，必须执行 `vibe report --status blocked --message "writing:path"`。
  - 执行由 Conductor 分配的具体子任务，并在完成后运行测试验证。
  - **自愈与重试**: 若测试失败，自动尝试修复最多 3 次。
  - 任务完成后执行 `vibe signal <NAME>` 通知 Conductor。
- **权限**: 专注于分配的子任务上下文。

### Evaluator (审计者/评估者)
- **职责**:
  - **逻辑审计**: 在任务标记为 SUCCESS 前，通过查看代码或运行测试执行验证。
  - **状态核对**: 确保所有物理变更已正确记录。
  - **信号验证**: 验证 Worker 发出的信号是否具备真实的逻辑支撑。
- **权限**: 只读分析为主，辅助执行验证脚本。

---

## Interactive Initialization

Upon the first use of the Vibe-Operator skill, the AI should guide the user to run `vibe init` to configure the environment, setup the `config.json`, and establish the default AI CLIs.

---

## Protocol Enforcement

- **Signal Names**: Use descriptive signal names (e.g., `build_passed`, `tests_failed`) instead of generic numbers.
- **Smart Cleanup**: Stale panes are automatically cleaned up, but agents should still call `vibe report` atomically.
- **Native Persona**: Trust the role instructions provided natively via `$VIBE_PERSONA` upon spawning.
