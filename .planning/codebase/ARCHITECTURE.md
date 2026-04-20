# Architecture

**Analysis Date:** 2024-05-23

## Pattern Overview

**Overall:** Client-Server CLI with Terminal Multiplexer Abstraction

**Key Characteristics:**
- **Terminal Agnostic:** Core logic interacts with terminal multiplexers (WezTerm, Tmux) via a unified `TerminalAdapter` trait.
- **Stateful Agent Tracking:** Maintains a persistent mapping between logical agent IDs (`VIBE_ID`) and physical terminal resource IDs.
- **Environment-Injected Identity:** Uses environment variables to pass identity and context to spawned agent processes.

## Layers

**CLI Application:**
- Purpose: Entry point for user commands and TUI.
- Location: `apps/vibe-cli/`
- Contains: Command parsing, TUI rendering, high-level orchestration logic.
- Depends on: `vibe-core`
- Used by: End users

**Core Library:**
- Purpose: Shared logic and abstractions.
- Location: `crates/vibe-core/`
- Contains: Terminal adapters, state management, IPC protocols, environment handling.
- Depends on: External crates (`ratatui`, `serde`, `uuid`, etc.)
- Used by: `vibe-cli`

**State Management:**
- Purpose: Persistence of agent metadata and workspace configuration.
- Location: `crates/vibe-core/src/state/`
- Contains: `StateStore` (agent tracking), `ConfigManager` (project settings), `RoleManager` (agent personas).
- Depends on: Filesystem

## Data Flow

**Agent Spawning Flow:**

1. `vibe-cli` generates a unique `VIBE_ID` (e.g., `v-a1b2c3d4`).
2. `vibe-cli` calls `adapter.spawn(target, command, env_vars)` where `env_vars` includes `VIBE_ID`.
3. The `TerminalAdapter` (WezTerm or Tmux) creates a new `WindowTarget` (Pane or Tab) and returns its `physical_id`.
4. `StateStore` saves the mapping of `VIBE_ID` to `physical_id` and other metadata (role, cwd) in `.vibe/state/panes.json`.
5. The spawned process (agent) reads `VIBE_ID` from its environment.

**Agent Reporting Flow:**

1. Agent executes `vibe report --status <status> --message <msg>`.
2. `vibe-cli` (report command) reads `VIBE_ID` from the environment.
3. If missing, it queries the `TerminalAdapter` for the current `physical_id` and looks up the `VIBE_ID` in `StateStore`.
4. `StateStore` updates the `PaneRecord` in `panes.json` with the new status and heartbeat.

**State Management:**
- Handled by `StateStore` in `crates/vibe-core/src/state/mod.rs`.
- Uses a file-based lock (`panes.lock`) to ensure atomic updates to `panes.json`.
- State is reloaded before each read/write operation to ensure consistency across multiple processes.

## Key Abstractions

**TerminalAdapter:**
- Purpose: Abstract interface for terminal multiplexers.
- Examples: `crates/vibe-core/src/adapter/wezterm.rs`, `crates/vibe-core/src/adapter/tmux.rs`
- Pattern: Strategy Pattern

**WindowTarget:**
- Purpose: Defines where a new agent process should be spawned.
- Location: `crates/vibe-core/src/adapter/mod.rs`
- Variants: `Pane(SplitDirection)` or `Tab`.

**VibeID:**
- Purpose: Logical identifier for an agent instance, stable even if physical terminal IDs change or are reused.
- Location: `crates/vibe-core/src/adapter/mod.rs` (type alias to `String`)

## Entry Points

**CLI Main:**
- Location: `apps/vibe-cli/src/main.rs`
- Triggers: User execution of `vibe` command.
- Responsibilities: Subcommand routing, adapter initialization, state orchestration.

**TUI:**
- Location: `apps/vibe-cli/src/tui.rs`
- Triggers: `vibe monitor` or `vibe ui`.
- Responsibilities: Real-time visualization of agent statuses and interactive management.

## Error Handling

**Strategy:** Result-based error propagation using `anyhow` (in CLI) and custom `VibeError` (in core).

**Patterns:**
- Custom `Result` type in `crates/vibe-core/src/error.rs`.
- `?` operator for concise propagation.
- `anyhow::Result` in `apps/vibe-cli` for flexible error context.

## Cross-Cutting Concerns

**Logging:** Currently uses `println!` and `eprintln!` for CLI output.
**Validation:** CLI arguments validated by `clap`.
**Authentication:** N/A (Local execution model).
**Path Resolution:** Centralized in `crates/vibe-core/src/env.rs` to handle project-local `.vibe` directories and global state.

---

*Architecture analysis: 2024-05-23*
