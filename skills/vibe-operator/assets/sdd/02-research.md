# SDD Phase 2: Research

## Goal
Perform technical discovery and environment verification.

## Instructions
1. **Environment Check**: Call `vibe check` to verify orchestration capabilities.
2. **Setup Workspace**:
   - Call `vibe spawn --role Worker` to start a dedicated research agent if necessary.
3. **Explore Codebase**:
   - Locate relevant files, schemas, and API definitions.
   - Identify potential side effects of the proposed changes.
4. **Document Findings**: Summarize the findings in the agent's context or a temporary research file.
5. **Sync**: Use `vibe report` to share findings with the Conductor.

## SOP Reference
- Follow [Collaboration SOP](../../sops/collaboration.md) for agent spawning.
- Follow [State Synchronization SOP](../../sops/state.md) for reporting progress.

## Next Step
Transition to `03-plan.md` to design the solution.
