# SDD Phase 4: Implementation & Testing

## Goal
Execute the approved plan and verify the results.

## Instructions
1. **Assign Workers**:
   - Call `vibe_run` to spawn workers for specific sub-tasks (e.g., "Feature Implementation", "Test Suite Creation").
2. **Execute atomic steps**:
   - Each worker performs its assigned task in its dedicated pane.
   - The Conductor monitors progress via `vibe_list` and provides guidance via `vibe_inject`.
3. **Continuous Testing**:
   - Run unit/integration tests as changes are implemented.
   - Report failures immediately.
4. **Final Review**:
   - Once all steps are complete, perform a final cross-check of the changes.

## SOP Reference
- Follow [Orchestration SOP](../../sops/orchestration.md) for managing multiple workers.
- Follow [State Synchronization SOP](../../sops/state.md) for global coordination.

## Next Step
Summarize the work and signal completion to the user.
