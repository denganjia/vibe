# Architecture

**Analysis Date:** 2024-10-24

## Pattern Overview

**Overall:** Client-Server CLI with Terminal Multiplexer Abstraction

**Key Characteristics:**
- **Terminal Agnostic:** Core logic interacts with terminal multiplexers (WezTerm, Tmux) via a unified `TerminalAdapter` trait.
- **Stateful Agent Tracking:** Maintains a persistent mapping between logical agent IDs (`VIBE_ID`) and physical terminal resource IDs.
- **Environment-Injected Identity:** Uses environment variables to pass identity, context (`VIBE_PERSONA`), and stack info to spawned agent processes.

## Layers

**CLI Application:**
- Purpose: Entry point for user commands and TUI.
- Location: `apps/vibe-cli/`
- Contains: Command parsing (`apps/vibe-cli/src/main.rs`), isolated TUI rendering (`apps/vibe-cli/src/tui.rs`), high-level orchestration logic (`spawn_role`).
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
- Contains: `StateStore` (agent tracking), `ConfigManager` (project settings with Stacks), `RoleManager` (agent personas).
- Depends on: Filesystem

## Data Flow

**Agent Spawning Flow (`spawn_role`):**

1. `vibe-cli` parses spawn request (single role or batch via `Stacks`).
2. Retrieves persona content from `RoleManager`. Determines command (adding auto-approve flags if known CLI).
3. Generates a unique `VIBE_ID` (e.g., `v-a1b2c3d4`).
4. Passes persona securely via environment variables (`VIBE_PERSONA`) and injects into CLI arguments (e.g., `--system-prompt "$VIBE_PERSONA"`) instead of relying on slow TTY keystroke injection.
5. `vibe-cli` calls `adapter.spawn(target, command, env_vars)`.
6. The `TerminalAdapter` creates a new `WindowTarget` and returns the `physical_id`.
7. `StateStore` saves the mapping of `VIBE_ID` to `physical_id` and other metadata in `.vibe/state/panes.json`.

**Batch Deployment (Stacks):**

1. `vibe spawn --stack <name>` loads `.vibe/config.json`.
2. Resolves the list of roles for the stack (e.g., `["Conductor", "Worker"]`).
3. Iterates over roles, invoking the `spawn_role` function for each, with a 500ms delay to stabilize TTY initialization and environment configuration.

**Agent Reporting Flow:**

1. Agent executes `vibe report --status <status> --message <msg>`.
2. `vibe-cli` (report command) reads `VIBE_ID` from the environment.
3. If missing, it queries the `TerminalAdapter` for the current `physical_id` and looks up the `VIBE_ID` in `StateStore`.
4. `StateStore` updates the `PaneRecord` in `.vibe/state/panes.json` with the new status and heartbeat.

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

**Stacks:**
- Purpose: Defines batch deployments of multiple agents configured together.
- Location: `crates/vibe-core/src/state/mod.rs` (`ProjectConfig.stacks`)

**VibeID:**
- Purpose: Logical identifier for an agent instance, stable even if physical terminal IDs change or are reused.
- Location: `crates/vibe-core/src/adapter/mod.rs` (type alias to `String`)

## Entry Points

**CLI Main:**
- Location: `apps/vibe-cli/src/main.rs`
- Triggers: User execution of `vibe` command.
- Responsibilities: Subcommand routing, adapter initialization, `spawn_role` orchestration, handling Stacks.

**TUI:**
- Location: `apps/vibe-cli/src/tui.rs`
- Triggers: User execution of `vibe status` (delegated from `main.rs`).
- Responsibilities: Real-time visualization of agent statuses and interactive management, isolated from main CLI execution path.

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

*Architecture analysis: 2024-10-24*