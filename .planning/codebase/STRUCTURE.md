# Codebase Structure

**Analysis Date:** 2024-12-16

## Directory Layout

```
vibe-cli/
├── apps/
│   └── vibe-cli/           # Main CLI Application
│       ├── src/
│       │   ├── main.rs      # CLI Entry point & Command routing
│       │   └── tui.rs       # Terminal UI implementation
│       └── Cargo.toml
├── crates/
│   └── vibe-core/          # Core Library Shared Logic
│       ├── src/
│       │   ├── adapter/     # Terminal Multiplexer Adapters
│       │   ├── env.rs       # Environment & Path Resolution
│       │   ├── error.rs     # Error Definitions
│       │   ├── ipc/         # Communication Protocols
│       │   ├── lib.rs       # Crate Root
│       │   ├── os/          # Platform Specific Helpers
│       │   └── state/       # Persistence & Configuration
│       └── Cargo.toml
├── .vibe/                   # Local Project Configuration (Example)
│   ├── config.json
│   ├── roles/               # Agent Persona Templates
│   └── state/               # Persistence state & locks
├── scripts/                 # Installation & Test Scripts
├── docs/                    # Documentation
└── Cargo.toml               # Workspace Configuration
```

## Directory Purposes

**apps/vibe-cli:**
- Purpose: Contains the end-user CLI tool.
- Contains: CLI command definitions, TUI logic.
- Key files: `src/main.rs`, `src/tui.rs`.

**crates/vibe-core:**
- Purpose: Shared logic and system abstractions used by the CLI.
- Contains: Terminal adapters, state management, IPC protocols.
- Key files: `src/lib.rs`, `src/adapter/mod.rs`, `src/state/mod.rs`.

**crates/vibe-core/src/adapter:**
- Purpose: Abstractions for different terminal multiplexers.
- Contains: WezTerm and Tmux implementations.
- Key files: `mod.rs` (trait definition), `wezterm.rs`, `tmux.rs`.

**crates/vibe-core/src/state:**
- Purpose: Persistence layer for the project.
- Contains: State file management, locking mechanisms, role management.
- Key files: `mod.rs`.

**crates/vibe-core/src/os:**
- Purpose: Low-level operating system and shell interactions.
- Contains: Shell detection, process management helpers.
- Key files: `windows.rs`, `unix.rs`.

**.vibe:**
- Purpose: Project-specific configuration and runtime state.
- Contains: Persona files, configuration JSON, and active pane state.
- Key files: `config.json`, `state/panes.json`.

## Key File Locations

**Entry Points:**
- `apps/vibe-cli/src/main.rs`: CLI entry point.
- `crates/vibe-core/src/lib.rs`: Library entry point.

**Configuration:**
- `Cargo.toml`: Workspace configuration and shared dependencies.
- `.vibe/config.json`: Project-level agent settings.

**Core Logic:**
- `crates/vibe-core/src/adapter/mod.rs`: `TerminalAdapter` trait definition.
- `crates/vibe-core/src/state/mod.rs`: `StateStore` implementation.

**Testing:**
- `crates/vibe-core/tests/`: Integration tests.
- `scripts/e2e_test.sh`: End-to-end testing script.

## Naming Conventions

**Files:**
- Rust files: `snake_case.rs` (e.g., `terminal_adapter.rs`).
- Documentation: `UPPERCASE.md` in `.planning/codebase`, `UPPERCASE.md` or `PascalCase.md` elsewhere.

**Directories:**
- Package directories: `kebab-case` (e.g., `vibe-cli`).
- Rust modules: `snake_case` (e.g., `vibe-core/src/adapter`).

## Where to Add New Code

**New Feature (CLI command):**
- Primary code: `apps/vibe-cli/src/main.rs` (add to `Commands` enum and match block).
- Core logic: Add corresponding methods to `vibe-core` if reusable.

**New Terminal Adapter:**
- Implementation: Create `crates/vibe-core/src/adapter/your_term.rs` and implement `TerminalAdapter`.
- Registration: Export it in `crates/vibe-core/src/adapter/mod.rs`.

**New IPC Message:**
- Implementation: `crates/vibe-core/src/ipc/protocol.rs` (add to `Message` enum).

**Utilities:**
- OS/Shell helpers: `crates/vibe-core/src/os/`.
- General helpers: `crates/vibe-core/src/env.rs`.

## Special Directories

**target/:**
- Purpose: Compiled artifacts.
- Generated: Yes
- Committed: No

**.vibe/state/:**
- Purpose: Active runtime state (panes, reports).
- Generated: Yes (at runtime).
- Committed: No (usually ignored via .gitignore, but local to project).

---

*Structure analysis: 2024-12-16*
