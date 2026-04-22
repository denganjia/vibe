# Vibe Scripts

Vibe scripts are thin deterministic helpers for project-local `.vibe` workspace operations. They accept structured workspace and task inputs; they must not execute arbitrary model policy text as shell.

## Deterministic Responsibilities

Scripts own exactly these six deterministic responsibilities:

- initialize `.vibe` (initialize .vibe)
- write task JSON
- acquire and release file locks
- launch configured Agent subprocesses
- capture stdout/stderr/exit code/timestamps
- generate local release summary drafts

## Not Script Responsibilities

Scripts do not decide task priority, Agent assignment policy, review acceptance, or recovery policy. Those belong to skills/references per D-06.

Scripts also do not act as a standalone scheduler, daemon, database, or hidden planning brain.

## Phase Ownership

Phase 20 documents the runtime boundary only. Later phases may add JS script implementations, but those implementations must stay deterministic, inspectable, and driven by `.vibe` workspace files.
