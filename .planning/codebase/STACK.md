# Technology Stack

**Analysis Date:** 2025-02-12

## Languages

**Primary:**
- Rust 2024 - Core logic, CLI, and terminal orchestration. Used in `apps/vibe-cli/` and `crates/vibe-core/`.

**Secondary:**
- Bash - Installation and E2E testing scripts (`scripts/install.sh`, `scripts/e2e_test.sh`).
- PowerShell - Windows installation script (`scripts/install.ps1`).

## Runtime

**Environment:**
- Tokio 1.0 - Asynchronous runtime for CLI operations and IPC tasks.

**Package Manager:**
- Cargo - Standard Rust package manager.
- Lockfile: `Cargo.lock` present.

## Frameworks

**Core:**
- Clap 4.5 - Command-line argument parsing with derive macros.
- Ratatui 0.29 - TUI framework for the dashboard (`vibe status`).

**Testing:**
- Cargo Test - Unit and integration tests in `crates/vibe-core/src/` and `crates/vibe-core/tests/`.
- Custom E2E - Shell-based testing in `scripts/e2e_test.sh`.

**Build/Dev:**
- Windows-sys 0.59 - Low-level Windows API for process management.
- Daemonize 0.5.0 - Unix-specific daemonization support.

## Key Dependencies

**Critical:**
- `uuid` 1.8 - Generates unique `vibe_id` for tracking terminal panes independently of physical IDs.
- `serde` / `serde_json` 1.0 - Serialization for project state and "Stateless Bus" messages.
- `crossterm` 0.28 - Low-level terminal handling and event-stream support.

**Infrastructure:**
- `anyhow` / `thiserror` - Unified error management across the workspace.
- `chrono` 0.4 - Time tracking for heartbeats and state updates.
- `tokio-util` 0.7 - Used for framing/codecs in communication protocols.
- `dirs` 5.0 - Cross-platform configuration directory resolution.

## Configuration

**Environment:**
- Local environment variables (e.g., `VIBE_ID`, `VIBE_MASTER_ID`).
- Configuration files in `.vibe/`.

**Build:**
- `Cargo.toml` (Workspace and crate level).

## Platform Requirements

**Development:**
- Rust Toolchain (stable).
- Terminal with WezTerm or Tmux installed.

**Production:**
- Cross-platform support for Windows (via `windows-sys`) and Unix (via `daemonize`).

---

*Stack analysis: 2025-02-12*
