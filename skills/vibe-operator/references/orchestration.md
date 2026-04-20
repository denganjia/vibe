# Orchestration SOP (Stateless Bus)

## Purpose
This SOP defines the logical sequence for setting up and managing a multi-model development environment using the `vibe spawn` and `vibe signal` mechanisms.

## Logical Workflow

### Phase 1: Environment Detection
1. **Tool**: Call `vibe check`.
2. **Analysis**: Note the `terminal` type and ensure it supports pane splitting.

### Phase 2: Autonomous Spawning
1. **Tool**: Call `vibe spawn --role <ROLE>`.
2. **Automation**: `vibe-cli` will:
   - Split the pane.
   - Inject the persona from `.vibe/roles/`.
   - Start the agent CLI.
   - Register the new agent in `.vibe/state/panes.json`.

### Phase 3: Task Synchronization
1. **Tool**: Use `vibe wait <SIGNAL>` to block the Conductor until a Worker task is complete.
2. **Monitoring**: Call `vibe list` to check the `summary` of all workers.
3. **Context Switching**: Use `vibe focus <ID>` to manually inspect a worker's progress if necessary.

### Phase 4: Lifecycle Management
1. **Reporting**: Workers MUST call `vibe report` frequently.
2. **Cleanup**: Use `vibe kill` to terminate all vibe-managed panes when the milestone is complete.

## Best Practices
- **Role Templates**: Ensure `.vibe/roles/` contains the necessary Markdown files before spawning.
- **Master Awareness**: Sub-agents should check for `VIBE_MASTER_ID` in their environment to ensure signals reach the correct destination.
- **Avoid Manual Splits**: Prefer `vibe spawn` over `vibe split` to ensure roles and state are correctly tracked from the start.
