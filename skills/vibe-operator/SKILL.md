# Vibe-Operator Skill

## Overview

Vibe-Operator is the core skill for AI agents to interact with the local development environment using Vibe-CLI. It enables multi-agent orchestration by turning the terminal into a physical orchestration room, allowing AI to manage panes, execute commands autonomously, and collaborate through structured protocols.

The primary goal of this skill is to break the "dimensional wall" between AI reasoning and local execution, providing a safe and efficient way for AI to perform complex development tasks.

## Core Capabilities

### 1. Environment Detection & Management
- **vibe_check**: Detects if the current terminal supports advanced features like pane splitting and focusing.
- **vibe_split**: Dynamically organizes the workspace by creating new terminal panes or external windows.
- **vibe_focus**: Directs the user's or AI's attention to specific task contexts.

### 2. Autonomous Execution & Tracking
- **vibe_run**: Spawns and tracks background tasks as "vibe agents."
- **vibe_list**: Provides a global view of all active agents, their roles, and current statuses.
- **vibe_inject**: Allows direct interaction with running agents (e.g., sending input or stopping processes).

### 3. Safety & Human-in-the-Loop
- **vibe_submit_plan**: Submits complex plans for human review before execution, ensuring alignment and safety.
- **vibe_query_approval**: Tracks the status of submitted plans (Pending, Approved, or Rejected with reasons).

## Operating Protocols

Before using this skill, agents must align with the user on environment preferences and safety levels.

- **Roles**: Agents operate within defined roles (e.g., Vibe-Conductor for orchestration, Worker for execution). See [role.md](./role.md).
- **SOPs**: Standard Operating Procedures for common tasks. See [sops/](./sops/).
- **Templates**: Structured workflow templates for various development modes (e.g., SDD). See [templates/](./templates/).

## Metadata

See [SKILL.yaml](./SKILL.yaml) for versioning, dependencies, and routing configurations.
