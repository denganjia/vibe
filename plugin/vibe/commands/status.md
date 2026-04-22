# vibe status

## Command Contract Only

This file documents the intended workflow contract and does not claim executable Codex command binding.

## Purpose

Report the current Vibe workspace state from durable `.vibe` artifacts so the
Conductor and user can see queued work, running work, review state, logs, and
locks.

## Reads

- `.vibe/tasks/`
- `.vibe/runs/`
- `.vibe/reviews/`
- `.vibe/logs/`
- `.vibe/locks/`
- `.vibe/config.json`

## Output

- Task counts by status.
- Active run ids with exit status or last update time.
- Review findings that still block completion.
- Locked file scopes and owning task ids.
- Recent log artifact paths that are safe to inspect.

## Compatibility

Pane-backed status from the legacy Rust CLI is compatibility only. The
plugin-first status contract reads `.vibe` task, run, review, log, and lock
artifacts as the default source of truth.
