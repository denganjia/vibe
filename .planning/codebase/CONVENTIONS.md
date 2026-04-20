# CONVENTIONS.md

## Code Style
- **Rust Idioms**: Adheres to standard Rust conventions (snake_case for functions/variables, PascalCase for types).
- **Asynchronous Code**: Uses `tokio` for async/await patterns, especially in CLI and IPC layers.

## Naming Conventions
- **Adapters**: Named `*Adapter` (e.g., `WezTermAdapter`, `TmuxAdapter`) implementing the `TerminalAdapter` trait.
- **State Management**: `*Manager` or `*Store` (e.g., `ConfigManager`, `StateStore`) for handling persistence.

## Error Handling
- **Core Library**: Uses a custom `VibeError` enum (via `thiserror`) and a `Result<T>` type alias.
- **CLI App**: Uses `anyhow::Result` for flexible error reporting in the top-level application.
- **Fail Fast**: Environment checks (`ensure_project_vibe`) are performed early in command execution.

## Patterns
- **Trait-based Orchestration**: The `TerminalAdapter` trait abstracts away specific terminal multiplexer logic.
- **Local State**: State is persisted in JSON files (`panes.json`) with file-level locking for concurrency safety.
- **Terminal Injection**: Communication via injecting specific text markers (`[vibe-signal:NAME]`) into terminal buffers.
