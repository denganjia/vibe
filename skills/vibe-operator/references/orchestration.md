# Orchestration SOP (Stateless Bus)

## Purpose
This SOP defines the logical sequence for setting up and managing a multi-model development environment using `vibe init`, `vibe spawn`, and file-based signals.

## Logical Workflow

### Phase 1: Environment Initialization
1. **Tool**: Call `vibe init`.
2. **Automation**: Opens an interactive wizard to scan for AI CLIs (`claude`, `gemini`), configure defaults, and generate `.vibe/config.json`.

### Phase 2: Autonomous Spawning
1. **Tool**: Call `vibe spawn --role <ROLE>` or `vibe spawn --stack <NAME>`.
2. **Automation**: `vibe-cli` will:
   - Perform smart cleanup of stale panes.
   - Create a new terminal Tab or Pane.
   - Pass the persona securely via the `$VIBE_PERSONA` environment variable.
   - Start the agent CLI with the correct auto-approve flags (e.g., `--dangerously-skip-permissions` for Claude, `-y` for Gemini).
   - Register the new agent in `.vibe/state/panes.json`.

### Phase 3: Task Synchronization
1. **Tool**: Use `vibe wait <SIGNAL>` to block the Conductor until a Worker writes to the file bus (`.vibe/bus/`).
2. **Monitoring**: Call `vibe list` to check the `summary` and intent locks of all workers.
3. **Context Switching**: Use `vibe focus <ID>` to manually inspect a worker's physical pane.
4. **Intervention**: Use `vibe inject <ID> <CMD>` to send emergency instructions to stalled workers.

### Phase 4: Lifecycle Management
1. **Reporting**: Workers MUST call `vibe report` to log Intent Locks and progress.
2. **Cleanup**: Use `vibe kill` to terminate all vibe-managed panes when the milestone is complete.
