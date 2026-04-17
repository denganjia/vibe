# Vibe-Operator Skill

## Overview

Vibe-Operator is the core skill for AI agents to interact with the local development environment using Vibe-CLI. It enables multi-agent orchestration by turning the terminal into a physical orchestration room, allowing AI to manage panes, execute commands autonomously, and collaborate through structured protocols.

The primary goal of this skill is to break the "dimensional wall" between AI reasoning and local execution, providing a safe and efficient way for AI to perform complex development tasks.

## Tool Reference

### 1. Environment & Discovery
- **vibe_check**
  - Description: Check if the current terminal environment supports physical orchestration (split/focus).
  - Parameters: None.
- **vibe_list**
  - Description: List all active vibe agents and their current status (role, status, summary, cwd, approval).
  - Parameters: None.

### 2. Orchestration & Control
- **vibe_split**
  - Description: Split the current pane or create a new one externally if local orchestration is not available.
  - Parameters:
    - `vertical` (boolean, optional): Split vertically instead of horizontally.
- **vibe_run**
  - Description: Run a command in a tracked vibe agent. Spawns an external window if current environment is not supported.
  - Parameters:
    - `command` (string, required): The command to execute.
    - `role` (string, optional): Role for the agent (e.g., "worker", "evaluator").
- **vibe_focus**
  - Description: Switch terminal focus to a specific vibe agent's pane.
  - Parameters:
    - `vibeId` (string, required): Target vibe ID.
- **vibe_inject**
  - Description: Inject a command into a running worker agent.
  - Parameters:
    - `vibeId` (string, required): Target vibe ID.
    - `command` (string, required): The command to inject.

### 3. Workflow & Approvals
- **vibe_submit_plan**
  - Description: Submit a multi-step plan for human approval before execution. Blocks execution until approved.
  - Parameters:
    - `vibeId` (string, required): The target vibe ID.
    - `plan` (string, required): The plan in Markdown format.
- **vibe_query_approval**
  - Description: Query the approval status of a previously submitted plan.
  - Parameters:
    - `vibeId` (string, required): The target vibe ID.
  - Returns: `status` (pending, approved, rejected) and `reason` (if rejected).

## Operating Protocols

Before using this skill, agents must align with the user on environment preferences and safety levels.

- **Roles (角色定义)**: 代理在定义明确的角色内运行（如指挥官 Vibe-Conductor、执行者 Worker、审计者 Evaluator）。详见 [role.md](./role.md)。
- **SOPs (标准作业程序)**:
  - [协作 SOP (Collaboration SOP)](./sops/collaboration.md): 定义多模型间的任务分配、标准化报告 (vibe report) 和上下文传递策略。
  - [验证 SOP (Verification SOP)](./sops/verification.md): 定义任务后的逻辑审计流程、意图对齐检查表及死锁检测规则 (M=3)。
  - [恢复 SOP (Recovery SOP)](./sops/recovery.md): 定义通过 `vibe_inject` 进行精准干预的故障恢复序列及升级协议。
  - 其他基础 SOP（审批、编排、状态管理）详见 [sops/](./sops/) 目录。
- **Templates (工作流模板)**: 针对不同开发模式（如 SDD）的结构化工作流模板。详见 [templates/](./templates/)。

## Metadata

See [SKILL.yaml](./SKILL.yaml) for versioning, dependencies, and routing configurations.
