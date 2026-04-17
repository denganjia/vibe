# State Synchronization SOP

## Purpose
This SOP defines how AI agents maintain global state awareness and synchronize context across multiple terminal panes/vibe agents.

## Technical Architecture (IPC & State)

`vibe-cli` operates on a distributed physical layer using local persistence and real-time communication:

### 1. Persistent State (SQLite)
- All agent metadata (vibe_id, role, status, summary, CWD) and workflow plans are stored in a local **SQLite** database.
- This allows for state recovery after CLI crashes and provides a unified "Source of Truth" for all windows and panes.
- AI agents interact with this state via `vibe_list`, which queries the database.

### 2. Real-time Communication (UDS)
- **Unix Domain Sockets (UDS)** are used for high-performance, low-latency IPC between the Master server and Worker clients.
- When you use `vibe_inject`, the Master sends the command through the socket directly to the target Worker's listener.
- Approval notifications (`vibe_submit_plan`) also flow through UDS to the Master to trigger UI updates in the TUI dashboard.

### 3. Execution Flow
1. **Worker** performs action -> Updates **SQLite**.
2. **Master** polls SQLite or receives **UDS** message.
3. **TUI** (Ratatui) renders the latest state from **SQLite**.

## Logical Workflow

### 1. Global State Awareness
- **Tool**: call `vibe_list` periodically.
- **Goal**: Maintain an up-to-date internal map of all active vibe agents, their `vibe_id`, `role`, and `status`.
- **Constraint**: Avoid tight polling loops. Polling should occur after significant task transitions or when waiting for a worker to finish.

### 2. Status Reporting
- **Requirement**: Workers must update their `status` and `summary` frequently.
- **Deadlock Avoidance**: The Conductor must never wait indefinitely for a worker. If a worker's status remains unchanged for a prolonged period, the Conductor should:
  - Use `vibe_focus` to inspect the pane.
  - Use `vibe_inject` to probe for status if the worker is non-responsive.
  - Alert the user if a worker appears stuck.

### 3. Context Sharing
- When a worker completes a task, it should provide a concise summary that the Conductor can read via `vibe_list`.
- The Conductor is responsible for aggregating these summaries and updating the global "Source of Truth" (e.g., a shared PLAN.md or STATE.md file).

## Best Practices
- **Explicit Transitions**: Update status to "Running", "Blocked", or "Completed" immediately upon state change.
- **Rich Summaries**: Use the `summary` field in `vibe_list` to provide high-value context to other agents.
