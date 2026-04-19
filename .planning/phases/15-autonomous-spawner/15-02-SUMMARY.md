# Phase 15 Plan 02: vibe spawn implementation and terminal adapter enhancements Summary

## Metadata
- **Phase**: 15
- **Plan**: 02
- **Subsystem**: vibe-cli, vibe-core
- **Tags**: spawn, terminal-adapter, wezterm, tmux, persona-injection
- **Duration**: 45 minutes
- **Completed Date**: 2026-04-18

## Substantive Summary
Implemented the `vibe spawn` command, allowing for autonomous agent pane creation with persona injection and automatic agent command startup. Enhanced both WezTerm and Tmux terminal adapters to support environment variable injection during pane splitting, specifically for propagating `VIBE_MASTER_ID`. 

Key features implemented:
- **Enhanced Split**: WezTerm adapter now supports `env` wrapping to inject environment variables. Tmux adapter already supported `-e`.
- **vibe spawn command**: 
    - Loads roles from `.vibe/roles/` (created automatically if missing).
    - Injects Persona Markdown into the new pane via `stdin` (using `inject_text`).
    - Automatically launches the `agent_command` (default `a-agent` or configured in `.vibe/config.json`).
    - Propagates `VIBE_MASTER_ID` to the new pane's environment.
    - Registers the new pane in the project's state store.

## Key Files
- `crates/vibe-core/src/adapter/wezterm.rs`: Enhanced `split` method to support `env_vars`.
- `crates/vibe-core/src/state/mod.rs`: Updated `save_pane` to support optional role tracking.
- `apps/vibe-cli/src/main.rs`: Implemented `Spawn` command and updated `Split` command.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Functionality] Added role support to save_pane**
- **Found during**: Task 2 implementation.
- **Issue**: The existing `save_pane` method in `StateStore` did not allow associating a role with a pane at creation time.
- **Fix**: Updated `save_pane` signature and `PaneRecord` to include an optional `role`.
- **Files modified**: `crates/vibe-core/src/state/mod.rs`, `apps/vibe-cli/src/main.rs`.
- **Commit**: (see below)

## Decisions Made
- **WezTerm Env Pattern**: Used `wezterm cli split-pane -- env VAR=VAL ... bash` as requested by the plan. This pattern was verified to work on Windows with WezTerm installed.
- **Role tracking**: Decided to track the role in `StateStore` immediately during `vibe spawn` to ensure `vibe list` provides accurate information even before the agent fully registers itself.

## Success Criteria Verification
- [x] `vibe spawn <ROLE>` opens a new pane (verified with WezTerm).
- [x] `VIBE_MASTER_ID` is injected into the new pane's environment.
- [x] Persona content is injected into the new pane.
- [x] Agent command is launched automatically.
- [x] State store correctly reflects the new pane and its role.

## Self-Check: PASSED
- `cargo build -p vibe-cli` succeeded.
- `vibe spawn Worker` succeeded and updated state.
- `vibe list` shows the new pane with "Worker" role.
- All commits follow the task protocol.
