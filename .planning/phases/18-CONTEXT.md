# Phase 18 Context: Standardization & Init

## Implementation Decisions

### 1. Interactive Initialization (`vibe init`)
- **Mode**: Interactive Wizard (using `dialoguer` or similar).
- **Functionality**: 
  - Scan `PATH` for `claude`, `gemini`, `codex`.
  - Ask user which CLI to use as default.
  - Generate `.vibe/config.json` with user choices.
  - Seed `.vibe/roles/` with default templates if missing.
  - Support a `--force` flag to reset existing configurations.

### 2. Batch Spawning with Stacks
- **Configuration**: Add a `stacks` object to `ProjectConfig`.
  ```json
  "stacks": {
    "default": ["Conductor", "Worker"],
    "full": ["Conductor", "Worker", "Evaluator"]
  }
  ```
- **CLI Command**: `vibe spawn --stack <NAME>`.
- **Behavior**: Iterates through the list of roles in the selected stack and performs a sequential `spawn` (creating a new Tab for each).

### 3. Automatic State Lifecycle Management
- **Trigger**: Automatic cleanup occurs during `vibe init` and before any `vibe spawn`.
- **Logic**: 
  - Query the terminal adapter (WezTerm/Tmux) for a list of all active physical IDs.
  - Cross-reference with `.vibe/state/panes.json`.
  - Delete any records from the JSON store where the physical ID is no longer present in the terminal emulator.
- **Goal**: Prevent Vibe ID reuse issues and keep the dashboard clean without manual intervention.

## Configuration Schema Changes
- **ProjectConfig**:
  - `roles`: Map<String, String> (already exists).
  - `default_command`: String (already exists).
  - `stacks`: Map<String, Vec<String>> (new).

## Locked Constraints
- Do not overwrite existing role templates in `.vibe/roles/` unless explicitly forced.
- The `init` wizard must handle cases where no AI CLIs are found gracefully (warn the user).
- Maintain backward compatibility for single-role spawning (`vibe spawn --role <NAME>`).

## Next Steps
- Implement the `init` wizard in `vibe-cli`.
- Update `ProjectConfig` and `ensure_project_vibe` in `vibe-core`.
- Implement stack-based spawning logic.
- Integrate the silent state cleanup into the core lifecycle.
