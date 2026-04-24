---
name: vibe-plan
version: 0.1.0
description: Execute the plan workflow logic.
---

# vibe plan

## Command Contract Only

This file documents the intended workflow contract and does not claim executable Codex command binding.

## Purpose

Turn a user request into one or more `.vibe` task contracts that executor and
reviewer Agents can run through the Vibe collaboration protocol.

## Execution Pattern

1. **Clarify**: The Conductor uses the Conservative Planning Checklist (in `plugin/vibe/roles/Conductor.md`) to ensure the goal, scope, and verification are clear.
2. **Decompose**: Split the objective into discrete tasks with explicit dependencies.
3. **Artifact Generation**: Call `plugin/vibe/scripts/plan.js` with a structured JSON payload containing the goal, tasks, and planning notes.

## Inputs

- User goal, constraints, expected output, and verification expectations.
- Optional file ownership hints or explicit task count.
- Optional reviewer requirements.

## Reads

- `plugin/vibe/roles/Conductor.md`
- `plugin/vibe/skills/Conductor.md`
- `plugin/vibe/references/task-contract.md`
- `.vibe/config.json`

## Writes

- `.vibe/plan.json`: The master plan manifest.
- `.vibe/tasks/*.json`: Individual task contracts.
- `.vibe/planning_notes.md`: Contextual notes for executors.

## Expected Output

When enough detail is known, the Conductor calls `plan.js` to write the plan and task artifacts. 
If the request is not ready to plan, the expected output is a focused clarification question.
