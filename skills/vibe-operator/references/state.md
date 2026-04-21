# State Synchronization SOP (Stateless Bus)

## Purpose
This SOP defines how AI agents maintain global state awareness and synchronize context across multiple terminal panes using file-based communication.

## Technical Architecture

`vibe-cli` v5.0 operates on a **Stateless Bus** architecture using local filesystem persistence:

### 1. State Store (`panes.json`)
- All agent metadata (`vibe_id`, `physical_id`, `role`, `status`, `summary`, `cwd`) is stored in `.vibe/state/panes.json`.
- **Smart Cleanup**: `vibe-cli` automatically prunes stale records during `init`, `spawn`, and `list` by cross-referencing active physical terminal panes.
- Agents update their own state via `vibe report`.

### 2. File-based Signal Bus (`.vibe/bus/`)
- Communication is achieved through atomic file writes, eliminating TTY injection lag and buffer clearing issues.
- `vibe signal <NAME> [PAYLOAD]`: Creates a JSON envelope in `.vibe/bus/` containing the sender, timestamp, and optional payload (supports `@path` for large file references).
- `vibe wait <NAME>`: Monitors the `.vibe/bus/` directory. When a matching signal file appears, it reads the payload, consumes (deletes) the file, and outputs the data to the agent.

### 3. Execution Flow
1. **Conductor** spawns a **Worker** via `vibe spawn` (which passes `$VIBE_PERSONA`).
2. **Worker** performs action -> Declares intent and updates status via `vibe report`.
3. **Worker** signals completion via `vibe signal task_done '{"status":"ok"}'`.
4. **Conductor** (running `vibe wait task_done`) receives the file-bus payload, avoiding any prompt injection crashes, and uses Intelligence-First logic to decide the next step.

## Logical Workflow

### 1. Global State Awareness
- **Tool**: Call `vibe list` periodically or after receiving a signal.
- **Goal**: Maintain an up-to-date map of all active vibe agents. The list is always accurate due to Smart Cleanup.

### 2. Intent Locking
- **Requirement**: Workers MUST call `vibe report --status blocked --message "writing:path"` before modifying files to declare intent and prevent race conditions with other agents.

### 3. Context Sharing
- Shared project context is maintained in `.vibe/` (e.g., `config.json`, `bus/`).
- High-level project deliverables (like `DELIVERY.md`) should be aggregated and written by the Conductor after consolidating all Worker signals.
