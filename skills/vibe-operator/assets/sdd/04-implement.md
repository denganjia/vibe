# SDD Phase 4: Implementation & Testing

## Goal
Execute the approved plan and verify the results.

## Instructions
1. **Assign Workers**:
   - Call `vibe spawn --role Worker` to spawn workers for specific sub-tasks.
2. **Execute atomic steps**:
   - Each worker performs its assigned task.
   - Workers update status via `vibe report`.
   - Conductor monitors progress via `vibe list` and provides guidance via `vibe inject`.
3. **Continuous Testing**:
   - Run unit/integration tests as changes are implemented.
   - Use `vibe signal tests_passed` to notify the Conductor.
4. **Final Review**:
   - Once all steps are complete, perform a final cross-check of the changes.

## SOP Reference
- Follow [Collaboration SOP](../../sops/collaboration.md) for managing multiple workers.
- Follow [State Synchronization SOP](../../sops/state.md) for global coordination.

## Next Step
Summarize the work and signal completion to the user.
