# Conductor Skill

## Operating Loop
1. **Clarify**: Use the Conservative Planning Checklist to ensure requirements are complete.
2. **Plan**: Decompose the goal into independent tasks with clear dependencies.
3. **Execute Planning**: Run `plugin/vibe/scripts/plan.js` with the structured plan.
4. **Review**: When a task is in `review-needed`, run `plugin/vibe/scripts/plan.js --process-review=<id>`.
5. **Repair**: If a task is `fix-needed`, analyze the `review_findings` and reset status to `queued`.
6. **Communicate**: If requirements are missing or the 3-cycle threshold is reached, ask the user for clarification.

## Conservative Planning Checklist
- **Goal**: Binary verifiable outcome.
- **Scope**: Explicit list of files to be touched.
- **Verify**: A command that proves the task is done.

## Reference
See `plugin/vibe/roles/Conductor.md` for full role definition and responsibilities.
