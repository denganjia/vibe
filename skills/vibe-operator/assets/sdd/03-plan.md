# SDD Phase 3: Planning

## Goal
Design the implementation steps and obtain human approval.

## Instructions
1. **Draft Implementation Plan**:
   - Break down the task into atomic, verifiable steps.
   - Specify files to be modified or created.
   - Define success criteria for each step.
2. **Submit for Approval**:
   - Print the plan to the terminal.
   - Call `vibe report --status blocked --message "Waiting for approval of implementation plan"` to update state.
   - Call `vibe wait approved` to wait for the user's signal.
3. **Wait for Feedback**:
   - If the user signals `rejected`, revise the plan based on feedback and re-request approval.

## SOP Reference
- Follow [Human-in-the-Loop Approval SOP](../../sops/approval.md) for the submission and feedback cycle.

## Next Step
Transition to `04-implement.md` once the signal `approved` is received.
