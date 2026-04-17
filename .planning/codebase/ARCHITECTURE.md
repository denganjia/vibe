# Architecture

**Analysis Date:** 2025-03-04

## Pattern Overview

**Overall:** Centralized Agent Bus with JSON State Persistence.

**Key Characteristics:**
- **Master/Worker Topology:** A central master process (`vibe master`) coordinates multiple worker processes running in terminal panes.
- **Decoupled Communication:** Agents communicate via an NDJSON-based IPC bus rather than direct process calls.
- **Stateless CLI:** The CLI interacts with either the `StateStore` (for queries) or the `MasterServer` (for actions), keeping the CLI logic focused on UI/UX.

## Layers

**IPC Layer (AI Agent Bus):**
- Purpose: Provides the communication backbone for the entire system.
- Location: `crates/vibe-core/src/ipc/`
- Contains: `protocol.rs` (Message types), `server.rs` (Master), `client.rs` (Worker).
- Depends on: `tokio`, `serde_json`, `tokio-util` (codec).
- Used by: `vibe-cli` (Master/Run/Inject/Report commands).

**State Layer:**
- Purpose: Persists pane and agent state to disk.
- Location: `crates/vibe-core/src/state/`
- Contains: `StateStore` (JSON-based persistence).
- Depends on: `serde`, `serde_json`, `std::fs`.
- Used by: `MasterServer` for tracking workers, CLI for listing and status.

**Adapter Layer:**
- Purpose: Abstracts terminal multiplexer commands (WezTerm, Tmux).
- Location: `crates/vibe-core/src/adapter/`
- Contains: `wezterm.rs`, `tmux.rs`.
- Depends on: CLI tools of respective multiplexers.
- Used by: CLI for splitting, focusing, and metadata retrieval.

## Data Flow

**Worker Registration:**
1. `vibe run` starts a `WorkerClient`.
2. Client connects to `MasterServer` socket (resolved via `crates/vibe-core/src/env.rs`).
3. Client sends `Register` message with `vibe_id`, `pid`, and metadata.
4. `MasterServer` updates `StateStore` and broadcasts new state to all `Subscribers`.

**Command Injection (Intent):**
1. `vibe inject <vibe_id> <cmd>` sends `ExecuteIntent` to `MasterServer`.
2. `MasterServer` lookups the connection for `<vibe_id>`.
3. `MasterServer` forwards the intent to the corresponding `WorkerClient`.
4. `WorkerClient` executes the command and sends `Ack` or `Report` back.

**State Management:**
- **In-Memory:** `MasterServer` maintains an `Arc<Mutex<HashMap>>` of active worker connections and their states.
- **Persistence:** `StateStore` writes state to `panes.json` on every change using atomic write-and-rename.
- **Actors:** The previous Actor pattern for DB/State has been **removed** in favor of direct shared-state management within the async Master server for simplicity and performance.

## Key Abstractions

**`Message` (IPC Protocol):**
- Purpose: Uniform message format for all agent communications.
- Examples: `crates/vibe-core/src/ipc/protocol.rs`
- Pattern: Enums with `#[serde(tag = "type")]` for NDJSON serialization.

**`TerminalAdapter`:**
- Purpose: Common interface for terminal operations.
- Examples: `crates/vibe-core/src/adapter/mod.rs`
- Pattern: Trait-based abstraction.

## Entry Points

**CLI Main:**
- Location: `apps/vibe-cli/src/main.rs`
- Triggers: User commands via `clap`.
- Responsibilities: Command parsing, orchestration, starting master/workers.

**Master Server Loop:**
- Location: `crates/vibe-core/src/ipc/server.rs`
- Triggers: `vibe master` or automatic spawn by `vibe run`.
- Responsibilities: Connection management, message routing, state persistence.

## Error Handling

**Strategy:** Centralized `VibeError` enum in `vibe-core`.

**Patterns:**
- `Result<T, VibeError>` used throughout core.
- `anyhow::Result` used in CLI for convenient top-level error reporting.

## Cross-Cutting Concerns

**Logging:** Output is directed to `.vibe/logs/<vibe_id>.log` for worker tasks.
**Validation:** CLI arguments validated by `clap`; IPC messages validated by `serde`.
**Authentication:** Relies on OS-level permissions for Unix Domain Sockets/Named Pipes.

---

*Architecture analysis: 2025-03-04*
