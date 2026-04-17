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

- **Roles**: Agents operate within defined roles (e.g., Vibe-Conductor for orchestration, Worker for execution). See [role.md](./role.md).
- **SOPs**: Standard Operating Procedures for common tasks. See [sops/](./sops/).
- **Templates**: Structured workflow templates for various development modes (e.g., SDD). See [templates/](./templates/).

## Metadata

See [SKILL.yaml](./SKILL.yaml) for versioning, dependencies, and routing configurations.
