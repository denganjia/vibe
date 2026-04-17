# Codebase Structure

**Analysis Date:** 2025-03-04

## Directory Layout

```
vibe-cli/
├── apps/               # Application binaries
│   └── vibe-cli/       # CLI tool entry and UI logic
│       ├── src/
│       │   ├── main.rs # CLI Entry point
│       │   └── tui.rs  # Dashboard TUI
├── crates/             # Shared libraries
│   └── vibe-core/      # Core logic and abstractions
│       └── src/
│           ├── adapter/# Terminal multiplexer integrations
│           ├── ipc/    # AI Agent Bus implementation
│           ├── os/     # OS-specific helpers
│           ├── state/  # JSON-based StateStore
│           ├── env.rs  # Path and environment resolution
│           ├── error.rs# Global error types
│           └── lib.rs  # Core library exports
├── docs/               # Technical documentation
├── scripts/            # Install and utility scripts
├── skills/             # Agent role definitions and templates
└── .vibe/              # Project-local runtime state (git-ignored)
    ├── logs/           # Worker process logs
    └── state/          # panes.json persistence
```

## Directory Purposes

**`apps/vibe-cli/`:**
- Purpose: The main CLI application that users interact with.
- Contains: Command-line interface definitions and TUI code.
- Key files: `apps/vibe-cli/src/main.rs`, `apps/vibe-cli/src/tui.rs`.

**`crates/vibe-core/src/ipc/`:**
- Purpose: The AI Agent Bus communication logic.
- Contains: Protocol definitions, master server, and worker client.
- Key files: `protocol.rs`, `server.rs`, `client.rs`.

**`crates/vibe-core/src/state/`:**
- Purpose: Persistent state management for panes and agents.
- Contains: The JSON-based `StateStore`.
- Key files: `mod.rs`.

**`crates/vibe-core/src/adapter/`:**
- Purpose: Multiplexer abstraction layer.
- Contains: Tmux and WezTerm specific implementations.
- Key files: `wezterm.rs`, `tmux.rs`.

## Key File Locations

**Entry Points:**
- `apps/vibe-cli/src/main.rs`: CLI Entry Point.
- `crates/vibe-core/src/ipc/server.rs`: Master Server Entry Point.
- `crates/vibe-core/src/ipc/client.rs`: Worker Client Entry Point.

**Configuration:**
- `Cargo.toml`: Workspace configuration.
- `crates/vibe-core/src/env.rs`: Paths and environment detection.

**Core Logic:**
- `crates/vibe-core/src/ipc/protocol.rs`: IPC message definitions.
- `crates/vibe-core/src/state/mod.rs`: Persistence logic.

**Testing:**
- `crates/vibe-core/src/ipc/server.rs` (internal tests).
- `crates/vibe-core/tests/` (integration tests).

## Naming Conventions

**Files:**
- Snake case: `vibe_core`, `master_server.rs`.

**Directories:**
- Kebab case for apps/crates: `vibe-cli`, `vibe-core`.
- Snake case for modules: `ipc`, `adapter`.

## Where to Add New Code

**New Agent Capability:**
- Primary logic: `crates/vibe-core/src/ipc/client.rs` (handle new intents).
- Protocol update: `crates/vibe-core/src/ipc/protocol.rs` (add message variant).

**New Terminal Multiplexer:**
- Implementation: `crates/vibe-core/src/adapter/new_multiplexer.rs`.
- Integration: Update `detect_current_terminal` in `crates/vibe-core/src/env.rs`.

**New CLI Command:**
- Definition: `apps/vibe-cli/src/main.rs`.

## Special Directories

**`.vibe/`:**
- Purpose: Stores local project state and logs.
- Generated: Yes, via `ensure_project_vibe()` in `state/mod.rs`.
- Committed: No (managed by `.gitignore`).

**`skills/`:**
- Purpose: Definitions for agent behaviors and roles.
- Committed: Yes.

---

*Structure analysis: 2025-03-04*
