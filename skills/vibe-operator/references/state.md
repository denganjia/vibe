# State Synchronization SOP (Stateless Bus)

## Purpose
This SOP defines how AI agents maintain global state awareness and synchronize context across multiple terminal panes without a central daemon.

## Technical Architecture

`vibe-cli` operates on a **Stateless Bus** architecture using local filesystem persistence and terminal-based signaling:

### 1. State Store (`panes.json`)
- All agent metadata (`vibe_id`, `role`, `status`, `summary`, `cwd`) is stored in `.vibe/state/panes.json`.
- **Concurrency Safety**: The state store uses file-level locking (`fd-lock`) to ensure multiple agents can safely read and write to the same JSON file simultaneously.
- Agents update their own state via `vibe report`.

### 2. Stateless Bus (Signal/Wait)
- Communication is achieved through **Terminal Text Injection**.
- `vibe signal <NAME>`: Injects a formatted marker `[vibe-signal:NAME]` into the master pane's buffer.
- `vibe wait <NAME>`: Reads from `stdin` and blocks until it detects the specific signal marker. This allows for asynchronous task completion notification.

### 3. Execution Flow
1. **Conductor** spawns a **Worker** via `vibe spawn`.
2. **Worker** performs action -> Updates `.vibe/state/panes.json` via `vibe report`.
3. **Worker** signals completion via `vibe signal done`.
4. **Conductor** (which was running `vibe wait done`) wakes up and reads the worker's status from `vibe list`.

## Logical Workflow

### 1. Global State Awareness
- **Tool**: Call `vibe list` periodically or after receiving a signal.
- **Goal**: Maintain an up-to-date map of all active vibe agents.
- **Protocol**: Use the `summary` field in `vibe list` to quickly understand what each worker accomplished.

### 2. Status Reporting
- **Requirement**: Workers MUST call `vibe report --status <STATUS> --message <MSG>` at key milestones and upon completion.
- **Responsiveness**: If a worker is non-responsive, the Conductor should use `vibe focus` to inspect the pane or `vibe inject` to probe the agent.

### 3. Context Sharing
- Shared project context is maintained in `.vibe/` (e.g., `roles/`, `config.json`).
- High-level project state (like `STATE.md`) should be updated by the Conductor after consolidating worker reports.

## Best Practices
- **Explicit Signals**: Always use unique signal names if multiple workers are running in parallel.
- **Rich Summaries**: Provide meaningful messages in `vibe report` to minimize the need for reading full logs.
- **Master ID**: Ensure `VIBE_MASTER_ID` is set in the environment (automatically handled by `vibe spawn`) for correct signal routing.
