# Migration Classification

This document classifies the legacy standalone Rust CLI responsibilities for the plugin-first Vibe architecture. Phase 20 uses `apps/` and `crates/` as compatibility reference material while future phases move the product surface into plugin skills, commands, references, scripts, and project-local `.vibe` files.

## Classification Values

- `Migrate-to-script`: Move the responsibility into thin plugin scripts or command-backed file operations where deterministic runtime behavior is still required.
- `Compatibility`: Preserve as legacy Rust CLI or terminal-adapter reference behavior during migration; do not make it the default plugin-first execution path.
- `Remove`: Retire the responsibility when it has no plugin-first owner or compatibility value.

## Old CLI Command Matrix

| Old capability | Category | New owner | Rationale |
| --- | --- | --- | --- |
| `init --force` | `Migrate-to-script` | Plugin init command and scripts runtime | The Rust wizard initializes `.vibe/config.json`, roles, and state directories. Phase 21/22 should recreate this as non-destructive plugin script initialization for `.vibe/Agents`, tasks, runs, locks, reviews, logs, and config files. |
| `run <command>` | `Migrate-to-script` | Plugin run-task command and subprocess launcher script | The old command injects `VIBE_MASTER_ID` and waits on a child process. Future scripts should launch configured Agent commands, capture exit codes, and write run artifacts without requiring pane orchestration. |
| `signal` | `Migrate-to-script` | File-based task/result signaling script | The old command writes FileBus JSON and falls back to terminal injection. Plugin-first Vibe should keep file-based signaling semantics but record structured task or run artifacts instead of depending on TTY fallback. |
| `wait` | `Migrate-to-script` | File-based wait/result collection script | The old command polls FileBus signal files and consumes matches. Scripts should provide deterministic waits over `.vibe/tasks`, `.vibe/runs`, and review outputs, with bounded timeouts and visible files. |
| `report` | `Migrate-to-script` | Result recording script | The old command resolves `VIBE_ID` and updates pane state with status and summary. Plugin-first Vibe should record executor and reviewer results as explicit run/review files. |
| `spawn --role/--stack` | `Migrate-to-script` | Agent launch script using `.vibe/Agents` | The useful behavior is role lookup, configured command selection, and allowlisted context injection. The new owner should launch claude/gemini/codex subprocess Agents from `.vibe/Agents`, not force terminal panes as the main path. |
| `split` | `Compatibility` | Legacy terminal adapter reference | Splitting terminal panes is validated legacy behavior, but D-12 and D-16 make subprocess Agents the default. Keep as optional terminal adapter compatibility only. |
| `focus` | `Compatibility` | Legacy terminal adapter reference | Focus depends on stored pane IDs and WezTerm/Tmux adapters. Preserve as reference for optional pane mode, not for plugin-first task execution. |
| `inject` | `Compatibility` | Legacy terminal adapter reference | TTY injection can help compatibility workflows but is not the default Agent control channel. Future plugin scripts should prefer task files, run artifacts, and explicit subprocess arguments. |
| `kill` | `Compatibility` | Legacy terminal adapter reference | Closing tracked panes is specific to terminal orchestration. Subprocess lifecycle management belongs to scripts, while pane cleanup remains compatibility behavior. |
| `list` | `Compatibility` | Legacy pane-state reference | The old list command reads `.vibe/state/panes.json`. Plugin-first status should read tasks, runs, reviews, and logs; pane listings remain a compatibility view. |
| `check` | `Compatibility` | Legacy environment diagnostic reference | Terminal support checks are useful when optional pane mode is enabled. They are not required for the plugin-first default path. |
| `status TUI` | `Compatibility` | Legacy TUI reference | The TUI polls pane state and logs. Future status should be model-readable files and command output first; the Rust TUI can inform optional compatibility UX. |

## Phase 20 Guardrails

- Phase 20 makes no large deletion, move, or rewrite of `apps/` or `crates/`; those directories remain the compatibility reference for old Rust CLI behavior.
- Do not convert terminal pane orchestration into the default plugin-first runtime path.
- Do not introduce a hidden daemon, database, or opaque state store while migrating old responsibilities.
- Do not treat `.vibe/state/panes.json` as the primary progress source for plugin-first tasks.
