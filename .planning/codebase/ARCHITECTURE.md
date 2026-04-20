# Architecture

**Analysis Date:** 2024-12-16

## Pattern Overview

**Overall:** CLI-driven Terminal Orchestrator with Adapter Abstraction.

**Key Characteristics:**
- **Adapter-based Terminal Integration:** Uses a trait-based system to support multiple terminal multiplexers (WezTerm, Tmux).
- **Decoupled Core Logic:** Core functionality is isolated in a separate crate (`vibe-core`) from the CLI interface (`vibe-cli`).
- **File-based State Management:** Uses JSON files with file-level locking for multi-process safe state tracking.
- **In-band Signaling:** Communicates between the master and worker panes using terminal text injection (stdin/stdout markers).

## Layers

**CLI Layer:**
- Purpose: High-level user interface and command orchestration.
- Location: `apps/vibe-cli`
- Contains: Command-line interface definitions, TUI implementation, and high-level workflow logic.
- Depends on: `vibe-core`
- Used by: End users

**Core Layer:**
- Purpose: Domain logic and system abstractions.
- Location: `crates/vibe-core`
- Contains: Terminal adapters, state persistence logic, environment detection, and OS abstractions.
- Depends on: Standard library, tokio, serde
- Used by: `vibe-cli`

**Protocol/IPC Layer:**
- Purpose: Standardized message structures for inter-agent communication.
- Location: `crates/vibe-core/src/ipc`
- Contains: Message enums and NDJSON serialization logic.
- Depends on: `serde`
- Used by: Core and CLI layers for signaling.

**Adapter Layer:**
- Purpose: Abstracting terminal multiplexer commands.
- Location: `crates/vibe-core/src/adapter`
- Contains: `TerminalAdapter` trait and implementations for WezTerm and Tmux.
- Depends on: `std::process::Command`
- Used by: Core and CLI layers.

## Data Flow

**Command Execution Flow:**

1. User invokes `vibe <command>` in `vibe-cli`.
2. `vibe-cli` parses the command and detects the current terminal type using `vibe-core::env`.
3. `vibe-cli` instantiates the appropriate `TerminalAdapter` (e.g., `WezTermAdapter`).
4. Logic in `vibe-cli` calls adapter methods (e.g., `split`, `send_keys`) to manipulate the terminal environment.
5. `StateStore` in `vibe-core` records the changes (e.g., new pane ID, role) to `.vibe/state/panes.json`.

**Signaling Flow:**

1. Master process calls `vibe signal <name> <payload>`.
2. `vibe-cli` identifies the target pane (usually the Master ID from env or state).
3. `TerminalAdapter::inject_text` sends a formatted signal string `[vibe-signal:...]` into the target pane's stdin.
4. The worker process (often `vibe wait`) reads stdin, detects the marker, and parses the payload.

## Key Abstractions

**TerminalAdapter:**
- Purpose: Provides a unified interface for terminal multiplexer operations like splitting panes and sending keys.
- Examples: `crates/vibe-core/src/adapter/mod.rs`
- Pattern: Strategy/Adapter Pattern.

**StateStore:**
- Purpose: Manages persistent state of active panes, roles, and status.
- Examples: `crates/vibe-core/src/state/mod.rs`
- Pattern: Repository Pattern with file-based persistence.

**VibeID:**
- Purpose: An abstraction over physical terminal pane IDs to identify vibe-managed workers uniquely.
- Examples: `crates/vibe-core/src/adapter/mod.rs`

## Entry Points

**vibe-cli main:**
- Location: `apps/vibe-cli/src/main.rs`
- Triggers: User CLI execution.
- Responsibilities: Routing CLI commands to appropriate core logic and managing terminal adapters.

**TUI Status Dashboard:**
- Location: `apps/vibe-cli/src/tui.rs`
- Triggers: `vibe status`
- Responsibilities: Real-time monitoring of agent states using `ratatui`.

## Error Handling

**Strategy:** Centralized error enum with `thiserror` for library-level errors and `anyhow` for CLI-level context.

**Patterns:**
- `VibeError` enum in `crates/vibe-core/src/error.rs` covers IO, terminal detection, and serialization errors.
- Automatic conversion from `std::io::Error` and `serde_json::Error` using `#[from]`.

## Cross-Cutting Concerns

**Logging:** Currently primarily relies on stdout/stderr and file-based state updates. Logs directory is resolved via `resolve_logs_dir` in `crates/vibe-core/src/env.rs`.
**Validation:** CLI arguments validated by `clap`.
**State Locking:** Inter-process locking for state files using `.lock` files in `crates/vibe-core/src/state/mod.rs`.

---

*Architecture analysis: 2024-12-16*
