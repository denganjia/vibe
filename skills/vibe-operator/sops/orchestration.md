# Orchestration SOP

## Purpose
This SOP defines the logical sequence for setting up and managing a multi-model development environment using Vibe-CLI.

## Logical Workflow

### Phase 1: Environment Detection
1. **Tool**: Call `vibe_check`.
2. **Analysis**:
   - Check if `supported` is true.
   - Note the `terminal` type (e.g., WezTerm, Tmux).
   - If unsupported, default to external window orchestration.

### Phase 2: Workspace Setup
1. **Strategy**: Determine the required layout based on the task complexity (e.g., one pane for research, one for implementation, one for testing).
2. **Tool**: Call `vibe_split` to create the necessary panes.
   - Use `vertical: true` for side-by-side comparisons.
   - Use `vertical: false` for stack-based workflows.
3. **Validation**: Call `vibe_list` to confirm all panes are registered and available.

### Phase 3: Worker Initialization
1. **Tool**: Call `vibe_run` for each required worker role.
   - Assign a clear `role` to each worker (e.g., "Researcher", "Implementer").
   - Ensure the command provided to `vibe_run` initializes the worker in the correct directory.
2. **Context Switching**: Call `vibe_focus` to direct attention to the newly created panes as they come online.

### Phase 4: Dynamic Reconfiguration
1. As the task evolves, the Conductor may:
   - Split existing panes further to handle sub-tasks.
   - Use `vibe_focus` to move between active contexts.
   - Close or re-purpose panes as workers complete their assignments.

## Best Practices
- **Abstraction**: Always use the provided tools (`vibe_split`, `vibe_focus`) rather than sending raw terminal escape sequences or hotkeys.
- **Verification**: After any layout change, verify the state with `vibe_list`.
- **User Alignment**: Respect the `UI Focus` preference defined in `role.md` when using `vibe_focus`.
