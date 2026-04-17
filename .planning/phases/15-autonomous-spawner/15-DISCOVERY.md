# Phase 15 Discovery: Autonomous Spawner

## Persona Protocol

### 1. Directory Structure
Roles will be stored in `.vibe/roles/` as Markdown files.
- `.vibe/roles/Conductor.md`
- `.vibe/roles/Worker.md`
- `.vibe/roles/Evaluator.md`

### 2. Injection Protocol
The injection will follow the "Stateless Terminal Injection" model established in Phase 14.
- `vibe spawn --role <ROLE>` will:
  1. Split a new pane using the terminal adapter.
  2. Inject the contents of `.vibe/roles/<ROLE>.md` into the new pane's stdin via `adapter.inject_text`.
  3. Send the command to launch the agent via `adapter.send_keys`.

### 3. Default Agent Command
A new configuration field `agent_command` will be added to `.vibe/config.json`.
- Default value: `a-agent` (or `vibe run a-agent` if we want to track it).
- Can be overridden via command line: `vibe spawn --role Worker --cmd "aider"`.

## Implementation Details

### Environment Variables
The following environment variables MUST be propagated to the spawned pane:
- `VIBE_MASTER_ID`: The physical ID of the pane that spawned this worker.
- `VIBE_ID`: The logical ID of the newly created pane.

### WezTerm Env Propagation
Since `wezterm cli split-pane` does not support `--env`, we will wrap the command:
```bash
wezterm cli split-pane -- env VIBE_MASTER_ID=... VIBE_ID=... bash
```

### Tmux Env Propagation
Tmux supports `-e` flag for environment variables in `split-window`.
```bash
tmux split-window -e VIBE_MASTER_ID=... -e VIBE_ID=...
```

## Security & Reliability
- Injection should be literal (`-l` in tmux, `send-text` in wezterm) to avoid accidental command execution from persona content.
- `vibe-core` should handle the auto-initialization of `.vibe/roles/` with safe defaults.
