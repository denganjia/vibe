# Architecture Patterns: vibe-cli

**Domain:** Terminal Orchestration for AI Agents
**Researched:** 2024-05-24
**Overall confidence:** HIGH

## Recommended Architecture

`vibe-cli` adopts a **Master-Worker (Orchestrator-Executor)** pattern, utilizing a decoupled **Adapter-based Orchestration Layer** to support multiple terminal multiplexers (Wezterm/Tmux).

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| **CLI Master** | CLI entry point, session management, high-level task splitting. | User, Orchestration Core, State DB, Unix Socket. |
| **Orchestration Core** | Abstract interface for terminal operations (split, send-text, get-text). | Wezterm/Tmux CLI, CLI Master. |
| **Worker Runtime** | The agent execution environment running inside a sub-pane. | Master (via Unix Socket), Local Filesystem, LLM APIs. |
| **State Layer** | Persistent storage of session, pane metadata, and task statuses. | CLI Master, Worker Runtime (indirectly via Master). |
| **IPC Layer (UDS)** | Real-time message bus for intent injection and progress streaming. | CLI Master (Server), Worker Runtime (Client). |

### Data Flow

1.  **Session Initialization**: `vibe init` creates a SQLite database in `.vibe/state.db` and starts a Unix Domain Socket (UDS) server.
2.  **Worker Spawning**:
    - Master calls `vibe split --intent "Build Feature X"`.
    - Orchestration Core translates this to `wezterm cli split-pane` or `tmux split-window`.
    - The new pane executes `vibe worker --session-id <ID> --parent-pid <PID>`.
3.  **Registration & Intent**:
    - The `Worker Runtime` connects to the Master's UDS.
    - Master sends a JSON payload containing the detailed "Intent" and context.
4.  **Execution & Monitoring**:
    - Worker executes the task.
    - Worker streams logs/progress via UDS (JSON messages).
    - Master updates `state.db` and optionally reflects status in the main UI.
5.  **Completion & Cleanup**:
    - Worker sends a `TASK_COMPLETE` signal.
    - Master verifies results (manually or via AI).
    - Master triggers `vibe close-pane` or marks the task as `DONE`.

---

## Patterns to Follow

### Pattern 1: Terminal Adapter (Provider Pattern)
**What:** Decouple the orchestration logic from specific terminal implementations.
**When:** To support both Wezterm and Tmux without duplicating business logic.
**Example:**
```rust
trait TerminalAdapter {
    fn split_pane(&self, direction: Direction, cmd: &str) -> Result<PaneId>;
    fn get_text(&self, id: PaneId) -> Result<String>;
    fn send_keys(&self, id: PaneId, keys: &str) -> Result<()>;
}

struct WeztermAdapter;
impl TerminalAdapter for WeztermAdapter { /* uses `wezterm cli` */ }
```

### Pattern 2: State-Driven Orchestration
**What:** Treat the terminal layout as a projection of the `state.db`.
**When:** Ensuring that if the CLI process crashes, we can "re-attach" to existing panes.
**Instead of:** Relying on in-memory process handles that disappear on exit.

### Pattern 3: Intent Injection via Environment or Pipe
**What:** Passing initial context to the worker.
**When:** Spawning a new worker process.
**Implementation:** Use environment variables (`VIBE_INTENT`) for small metadata and the UDS for large context or ongoing commands.

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Direct PTY Manipulation
**What:** Trying to implement a full terminal emulator or low-level PTY driver in Rust.
**Why bad:** High complexity, fragile across OS versions, reinventing Wezterm/Tmux.
**Instead:** Use the mature CLI tools provided by the multiplexers (`wezterm cli`, `tmux`).

### Anti-Pattern 2: Polling for Status
**What:** Master repeatedly calling `get-text` to see if a worker is done.
**Why bad:** Resource intensive, laggy, hard to distinguish between "working" and "hung".
**Instead:** Use push-based communication via Unix Sockets.

---

## Scalability Considerations

| Concern | At 100 users | At 10K users (per machine) | Notes |
|---------|--------------|--------------|-------------|
| **DB Performance** | SQLite (Fast) | N/A (Local tool) | SQLite is perfect for local state. |
| **IPC Overhead** | Negligible | Socket limit issues | Local UDS can handle thousands of concurrent workers. |
| **Terminal Lag** | None | Resource contention | The bottleneck will be CPU/RAM for the AI agents, not the orchestration layer. |

## Sources

- [tmux_interface (Crates.io)](https://crates.io/crates/tmux_interface)
- [Wezterm CLI Documentation](https://wezfurlong.org/wezterm/cli/index.html)
- [Master-Worker AI coordination patterns](https://google.com)
- [Interprocess Communication in Rust (interprocess crate)](https://crates.io/crates/interprocess)
