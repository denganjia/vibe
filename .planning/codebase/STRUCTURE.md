# STRUCTURE

## File Tree

/Users/anjia/Documents/part-time/vibe-cli/
├───Cargo.lock
├───Cargo.toml          # Workspace root, defines shared dependencies
├───apps/
│   └───vibe-cli/       # Main user-facing CLI application
│       ├───Cargo.toml
│       └───src/
│           ├───main.rs # CLI command routing (split, run, master, status)
│           └───tui.rs  # Interactive dashboard (Ratatui implementation)
├───crates/
│   └───vibe-core/      # Core logic and shared library
│       ├───Cargo.toml
│       ├───src/
│       │   ├───adapter/ # Terminal drivers (WezTerm, Tmux)
│       │   ├───env.rs   # OS-specific paths and env detection
│       │   ├───error.rs # Unified error types (VibeError)
│       │   ├───ipc/     # Networking (Protocol, Master Server, Worker Client)
│       │   ├───os/      # Low-level OS helpers (Shell, Windows Job Objects)
│       │   └───state/   # Persistence (SQLite, DB Actor)
│       └───tests/      # Integration and concurrency tests
└───.planning/          # Project roadmaps and codebase documentation
    ├───STATE.md
    └───codebase/
        ├───ARCHITECTURE.md
        ├───CONCERNS.md
        └───STRUCTURE.md

## Module Dependencies
1. `apps/vibe-cli` depends on `crates/vibe-core`.
2. `vibe-core/ipc` depends on `vibe-core/state` (via DB actor).
3. `vibe-core/adapter` depends on `vibe-core/env`.

## Data Storage
- **Database**: `~/Library/Application Support/vibe/state.db` (macOS)
- **Sockets**: `~/Library/Application Support/vibe/vibe.sock`
- **Logs**: `~/Library/Application Support/vibe/logs/*.log`
