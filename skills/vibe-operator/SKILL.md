# Vibe-Operator Skill

## Overview
Vibe-Operator is the core skill for AI agents to interact with the local development environment using Vibe-CLI. It enables multi-agent orchestration by turning the terminal into a physical orchestration room.

## Quick Start
- **INIT**: `vibe_check` -> `vibe_split` -> `vibe_run worker`
- **PLAN**: `vibe_submit_plan` -> `vibe_query_approval`
- **EXEC**: `vibe_inject [ID] "[CMD]"`
- **SYNC**: `vibe_list` -> `vibe_focus [ID]`

## Tool Reference (Compact)
- `vibe_check`: Verify terminal orchestration support.
- `vibe_list`: List all active vibe agents, roles, and status.
- `vibe_split [vertical:bool]`: Split current pane or create new one.
- `vibe_run [command, role]`: Execute command in a new worker agent.
- `vibe_focus [vibeId]`: Switch terminal focus to a specific agent.
- `vibe_inject [vibeId, command]`: Inject command into a running agent.
- `vibe_submit_plan [vibeId, plan]`: Submit MD plan for human approval (blocks).
- `vibe_query_approval [vibeId]`: Query plan status (pending, approved, rejected).

## Operating Protocols
- **Prompt Variable Injection**:
  - Syntax: `$[VARIABLE_NAME]`
  - Protocol: AI must semantically resolve and inject values from history/docs into templates.
  - Fallback: Ask user if a variable (e.g., `$[REFACTOR_TARGET]`) cannot be resolved.
- **Roles**: Agents run within defined roles (e.g., Worker, Evaluator). See [role.md](./role.md).
- **SOPs**:
  - [协作 SOP](./sops/collaboration.md): Task assignment & reporting.
  - [验证 SOP](./sops/verification.md): Logic audit & dead-lock detection.
  - [恢复 SOP](./sops/recovery.md): Precision intervention via `vibe_inject`.
  - See [sops/](./sops/) for approval, orchestration, and state management.
- **Templates**: Structured workflows for SDD or Refactoring. See [templates/](./templates/).

## Metadata
See [SKILL.yaml](./SKILL.yaml) for versioning and routing.
