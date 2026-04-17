# Vibe-Operator Role Protocols

## Introduction

This document defines the collaboration protocols and roles for AI agents using the Vibe-Operator skill. These rules ensure clear task assignment, state synchronization, and safety across multi-model workflows.

## Role Definitions

### Vibe-Conductor (指挥官/Master)
- **职责 (Responsibilities)**:
  - 全局编排与任务拆解。
  - 环境初始化（拆分窗格、设置 Worker）。
  - 基于推理的任务分配：根据任务复杂度和模型能力分配子任务 (基于 D-01)。
  - 提交高层计划供人工审批。
  - 通过 `vibe_list` 进行全局状态监控和进度追踪。
  - 管理任务移交和冲突解决。
- **权限 (Authority)**: 高。可以创建新 Worker 并重定向关注点。

### Worker (执行者/Executor)
- **职责 (Responsibilities)**:
  - 执行由 Vibe-Conductor 分配的具体子任务。
  - **标准化报告**：通过 `vibe report` 及时汇报状态、产出、阻塞项及后续步骤。
  - 在任务模糊时主动请求澄清。
- **权限 (Authority)**: 受限于分配的上下文/窗格。

### Evaluator (审计者/评估者)
- **职责 (Responsibilities)**:
  - **意图对齐**：验证 Worker 的执行结果是否符合 Conductor 的原始意图。
  - **逻辑审计**：检查任务分解和执行路径的逻辑严密性 (基于 D-01, D-04)。
  - **验证**：在任务标记为完成前执行独立验证，确保满足 success_criteria。
- **权限 (Authority)**: 中等。主要进行只读分析、环境检查和验证反馈。

---

## Interactive Initialization

Upon the first use of the Vibe-Operator skill in a new environment, the AI (acting as Vibe-Conductor) **MUST** present the following questions to the user to align on preferences and safety.

### Initialization Questions

1. **Stack Detection**: "I have detected the following tech stack: [Detected Stack]. Is this correct, or should I consider other libraries/frameworks?"
2. **Terminal Preference**: "Would you prefer I use **WezTerm** or **Tmux** for local pane management? (If neither, I will use external window spawning via `vibe_run`.)"
3. **Safety & Approvals**: "What is your preferred safety level for command execution?"
   - **Strict**: I will submit a plan for approval before *every* significant command.
   - **Moderate**: I will only ask for approval on high-risk commands (e.g., deletions, structural changes).
   - **Relaxed**: I will execute commands autonomously and only report status.
4. **UI Focus**: "Should I automatically switch terminal focus to new panes as I create them, or would you prefer to keep focus on your current view?"
5. **Autonomy Level**: "Should I (the Conductor) manage all task assignments autonomously based on your goals, or would you like to manually assign tasks to specific workers?"

---

## Protocol Enforcement

- These roles and preferences are stored in the session context.
- The Vibe-Conductor must verify environment support using `vibe_check` before proceeding with initialization.
- All high-level plans MUST be submitted via `vibe_submit_plan` if the safety level is not "Relaxed".
