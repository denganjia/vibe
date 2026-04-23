# Vibe Scripts

Vibe scripts are thin deterministic helpers for project-local `.vibe` workspace operations. They accept structured workspace and task inputs; they must not execute arbitrary model policy text as shell.

## Deterministic Responsibilities

Scripts own exactly these six deterministic responsibilities:

- `init.js`: Initialize `.vibe` workspace and copy templates.
- `task.js`: Create and update task JSON artifacts.
- `lock.js`: Acquire and release file-scope locks.
- `run-task.js`: Launch configured Agent subprocesses and capture output logs/artifacts.
- `status.js`: Unified task and run status management.
- `release-summary.js` (Phase 24): Generate local release summary drafts.

## Implementation Details

All scripts are written in pure Node.js using native APIs to ensure maximum portability and minimal dependencies. They follow the contracts defined in `plugin/vibe/references/`.

## Not Script Responsibilities

Scripts do not decide task priority, Agent assignment policy, review acceptance, or recovery policy. Those belong to skills/references per D-06.

Scripts also do not act as a standalone scheduler, daemon, database, or hidden planning brain.

## Phase Ownership

Phase 20 documents the runtime boundary only. Later phases may add JS script implementations, but those implementations must stay deterministic, inspectable, and driven by `.vibe` workspace files.
