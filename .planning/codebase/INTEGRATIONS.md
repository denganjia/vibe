# INTEGRATIONS.md

## Terminal Multiplexers
- **WezTerm**: Integrated via `wezterm cli` commands. Supports pane splitting (`split-pane`), text injection (`send-text`), and metadata retrieval (`list --format json`).
- **Tmux**: Integrated via `tmux` commands for similar orchestration capabilities.

## External APIs & Services
- **None (Local First)**: The system is designed to be a "Stateless Bus" and does not currently have external SaaS integrations. All communication is local via terminal injection and filesystem-based state.

## State Management
- **Local Filesystem**: Uses `.vibe/state/panes.json` for tracking active agent sessions.
- **IPC**: Communication via terminal text injection (`vibe-signal:NAME` markers) and stdin polling.

## Development Tools
- **GSD (Get Shit Done)**: Integration via the `.planning/` directory for autonomous development workflows.
- **Rustup**: Toolchain management, potentially referenced by `rustup-init.exe` in the root.
