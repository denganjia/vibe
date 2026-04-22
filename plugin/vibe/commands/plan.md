# vibe plan

## Command Contract Only

This file documents the intended workflow contract and does not claim executable Codex command binding.

## Purpose

Turn a user request into one or more `.vibe` task contracts that executor and
reviewer Agents can run through the Vibe collaboration protocol.

## Inputs

- User goal, constraints, expected output, and verification expectations.
- Optional file ownership hints or explicit task count.
- Optional reviewer requirements.

## Reads

- `plugin/vibe/skills/conductor/SKILL.md`
- `plugin/vibe/references/collaboration-protocol.md`
- `plugin/vibe/references/task-contract.md`
- `.vibe/config.json`
- `.vibe/Agents/`

## Writes

- Task JSON files under `.vibe/tasks/`.
- Planning notes or decisions only when they are needed for later executor or
  reviewer context.

## Expected Output

The Conductor clarifies before writing task JSON. When enough detail is known,
it uses `references/task-contract.md` to write task files under `.vibe/tasks/`
with repo-root-relative `file_scope`, verification, and reviewer requirements.

If the request is not ready to plan, the expected output is a focused
clarification question instead of a partial task artifact.
