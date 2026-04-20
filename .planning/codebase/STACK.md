# STACK.md

## Languages & Runtime
- **Rust (v1.82+ or 2024 Edition)**: Core language for both the CLI app and the core library.
- **Bash/PowerShell**: Used for installation scripts (`scripts/install.sh`, `scripts/install.ps1`) and E2E testing.

## Frameworks & Key Dependencies
- **Tokio**: Asynchronous runtime for IPC and signal handling.
- **Clap (v4.5)**: CLI argument parsing with derive macros.
- **Ratatui (v0.29)** & **Crossterm (v0.28)**: Terminal UI for the dashboard (`vibe status`).
- **Serde / Serde_JSON**: Data serialization/deserialization for configuration and IPC.
- **Anyhow / Thiserror**: Structured error handling.
- **Dirs / Which**: OS-specific path resolution and tool detection.
- **Windows-sys**: Low-level Windows API bindings for job objects and process management.

## Configuration
- **.planning/**: GSD-specific project management directory.
- **.vibe/**: Project-local configuration for agents, roles, and state.
- **Cargo.toml**: Workspace-level Rust dependency management.

## Build System
- **Cargo**: Standard Rust build and package manager.
- **GitHub Actions**: Release workflow configured in `.github/workflows/release.yml`.
