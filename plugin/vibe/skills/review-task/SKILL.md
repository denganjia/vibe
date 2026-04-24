---
name: vibe-review-task
version: 0.1.0
description: Execute the review-task workflow logic.
---

# vibe review-task

## Command Contract Only

This file documents the intended workflow contract and does not claim executable Codex command binding.

## Purpose

Run reviewer checks for a completed executor run before the Conductor marks the
task as complete.

## Inputs

- Task id or run id.
- Optional reviewer Agent id.
- Optional review depth or required evidence from the task's
  `reviewer_requirements`.

## Reads

- `.vibe/tasks/`
- `.vibe/runs/`
- `.vibe/Agents/`
- `plugin/vibe/references/review-protocol.md`
- `plugin/vibe/references/task-contract.md`

## Writes

- Reviewer reports under `.vibe/reviews/`.
- Task status updates to `fix-needed`, `completed`, or `blocked`.
- Links from task or run artifacts to review evidence.

## Completion Gate

Reviewer output must use structured findings as defined by
`references/review-protocol.md`. Unresolved high severity findings block completion, and critical or medium findings also require a fix or explicit
Conductor acceptance according to the review protocol.

The Conductor may accept an unresolved finding only by recording the finding id,
severity, reason, and accepted risk in `.vibe/reviews/` or the task artifact.

## Expected Output

- Review id and linked task or run id.
- Structured findings with severity, evidence, and required fix.
- Completion gate result: pass, fix-needed, blocked, or accepted-risk.
- Paths to `.vibe/reviews/` artifacts.
