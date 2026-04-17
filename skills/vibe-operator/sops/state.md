# State Synchronization SOP

## Purpose
This SOP defines how AI agents maintain global state awareness and synchronize context across multiple terminal panes/vibe agents.

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
