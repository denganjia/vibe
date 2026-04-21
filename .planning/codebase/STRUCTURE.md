# Codebase Structure

**Analysis Date:** 2024-10-24

## Directory Layout

```
vibe-cli/
├── apps/
│   └── vibe-cli/           # Main CLI Application
│       └── src/
│           ├── main.rs     # CLI Entry point, Subcommands, spawn_role logic
│           └── tui.rs      # Ratatui-based Monitoring UI (isolated)
├── crates/
│   └── vibe-core/          # Core logic and abstractions
│       └── src/
│           ├── adapter/    # Terminal Multiplexer Adapters
│           ├── ipc/        # Communication Protocols
│           ├── os/         # OS-specific helpers
│           ├── state/      # Persistence, Config & Stacks Management
│           ├── env.rs      # Path & Environment Resolution
│           ├── error.rs    # Error types
│           └── lib.rs      # Core Library Entry
├── .vibe/                  # Project-local runtime data
│   ├── config.json         # Project Config (includes Stacks definitions)
│   ├── roles/              # Role templates (Markdown)
│   └── state/
│       └── panes.json      # Active Agent State Mapping
└── scripts/                # Installation & Test scripts
```

## Directory Purposes

**apps/vibe-cli/:**
- Purpose: The user-facing command-line tool.
- Contains: Command logic, separated UI rendering, and orchestration of core features (like batch spawning via `Stacks`).
- Key files: 
  - `apps/vibe-cli/src/main.rs`: Subcommands and `spawn_role` centralized refactoring.
  - `apps/vibe-cli/src/tui.rs`: Isolated Ratatui UI components and state logic.

**crates/vibe-core/src/adapter/:**
- Purpose: Abstraction layer for different terminal multiplexers.
- Contains: `TerminalAdapter` trait and implementations for WezTerm and Tmux.
- Key files: `crates/vibe-core/src/adapter/mod.rs`, `crates/vibe-core/src/adapter/wezterm.rs`, `crates/vibe-core/src/adapter/tmux.rs`.

**crates/vibe-core/src/state/:**
- Purpose: Persistent storage for agent metadata and project configuration.
- Contains: `StateStore` for tracking agents, `ConfigManager` (managing default commands and Stacks), `RoleManager` (persona management).
- Key files: `crates/vibe-core/src/state/mod.rs`.

**crates/vibe-core/src/ipc/:**
- Purpose: Logic for inter-process communication between agents and the controller.
- Contains: Protocol definitions and messaging structures.
- Key files: `crates/vibe-core/src/ipc/protocol.rs`.

**.vibe/:**
- Purpose: Project-specific configuration and runtime state.
- Contains: JSON state files, Stack declarations, and Markdown persona templates.
- Key files: `.vibe/state/panes.json`, `.vibe/config.json`.

## Key File Locations

**Entry Points:**
- `apps/vibe-cli/src/main.rs`: Main CLI entry point.

**Configuration:**
- `Cargo.toml`: Workspace definition and dependencies.
- `.vibe/config.json`: Project-level settings, default commands, and Stack definitions (batch deployments).

**Core Logic:**
- `crates/vibe-core/src/adapter/mod.rs`: Definition of `WindowTarget` and `TerminalAdapter`.
- `crates/vibe-core/src/state/mod.rs`: `StateStore`, `ProjectConfig` (with `stacks` HashMap).
- `apps/vibe-cli/src/main.rs`: `spawn_role` function for robust environment/persona injection.

**Testing:**
- `crates/vibe-core/tests/`: Integration tests for core logic.
- `scripts/e2e_test.sh`: End-to-end testing script.

## Naming Conventions

**Files:**
- Rust Source: `snake_case.rs`
- Configuration: `snake_case.json`
- Roles: `PascalCase.md`

**Directories:**
- Rust Crates/Apps: `kebab-case`
- Modules: `snake_case`

## Where to Add New Code

**New CLI Subcommand:**
- Primary code: `apps/vibe-cli/src/main.rs` (add to `Commands` enum and `match` block).

**New UI Views/Components:**
- Implementation: `apps/vibe-cli/src/tui.rs` (extend the `App` state and `ui` rendering function).

**New Terminal Adapter:**
- Implementation: `crates/vibe-core/src/adapter/[name].rs`.
- Registration: Add to `TerminalAdapter` trait and update factory logic in `vibe-cli`.

**New Stack Configuration:**
- Definition: Add a new key-value pair to the `stacks` object in `.vibe/config.json`.

**New State Metadata:**
- Implementation: Update `PaneRecord` in `crates/vibe-core/src/state/mod.rs`.

**Utilities:**
- Shared helpers: `crates/vibe-core/src/os/` (for OS-level) or `crates/vibe-core/src/lib.rs` (for general).

## Special Directories

**.vibe/:**
- Purpose: Stores project-local state, roles, and stack config.
- Generated: Yes (via `vibe init` or implicitly on spawn).
- Committed: `config.json` and `roles/` should be committed; `state/` should be ignored.

**target/:**
- Purpose: Rust build artifacts.
- Generated: Yes.
- Committed: No.

---

*Structure analysis: 2024-10-24*