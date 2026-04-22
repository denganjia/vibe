---
phase: 20-plugin-first-architecture
plan: 02
subsystem: architecture
tags: [plugin, references, contracts, vibe-workspace, agents]

requires:
  - phase: 20-plugin-first-architecture
    provides: "Phase context, research, validation strategy, and plugin-first architecture decisions"
provides:
  - "Model-readable plugin architecture and ownership boundary"
  - ".vibe workspace layout contract"
  - "Collaboration, task, Agent, and review protocols"
affects: [phase-21-vibe-workspace, phase-22-scripts-runtime, phase-23-multi-model-workflow]

tech-stack:
  added: []
  patterns:
    - "Markdown references as durable model-readable contracts"
    - "Plugin-first split: skills judge, scripts execute, .vibe stores state"

key-files:
  created:
    - plugin/vibe/references/plugin-architecture.md
    - plugin/vibe/references/workspace-layout.md
    - plugin/vibe/references/collaboration-protocol.md
    - plugin/vibe/references/task-contract.md
    - plugin/vibe/references/agent-contract.md
    - plugin/vibe/references/review-protocol.md
  modified: []

key-decisions:
  - "Reference contracts are Markdown so the current model can inspect collaboration state and rules directly."
  - "Subprocess Agents are the default execution path; terminal panes remain compatibility only."
  - "Task file scopes must be repo-root-relative and reject path traversal using `..`."

patterns-established:
  - "Contracts link collaboration handoff to task JSON and review requirements."
  - "Agent definitions expose model_command, allowed tools, expected output, and review policy."
  - "Review completion requires findings to be resolved or explicitly accepted by the Conductor."

requirements-completed: [PLUG-02]

duration: 4min
completed: 2026-04-22
---

# Phase 20 Plan 02: Reference Contracts Summary

**Model-readable Vibe contracts for plugin ownership, `.vibe` workspace state, task handoff, Agent subprocess boundaries, and review completion gates**

## Performance

- **Duration:** 4 min
- **Started:** 2026-04-22T08:09:19Z
- **Completed:** 2026-04-22T08:13:20Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Defined the plugin-first ownership map across skills, commands, references, scripts, `.vibe`, and legacy compatibility code.
- Documented the required `.vibe` workspace entries and non-destructive initialization rule.
- Added collaboration, task, Agent, and review contracts with explicit file scope, subprocess, secret, and completion boundaries.

## Task Commits

Each task was committed atomically:

1. **Task 1: Define architecture and workspace references** - `71b86f4` (feat)
2. **Task 2: Define collaboration, task, Agent, and review contracts** - `0f2d123` (feat)

**Plan metadata:** committed separately after summary creation.

## Files Created/Modified

- `plugin/vibe/references/plugin-architecture.md` - Plugin-first product shape, ownership map, Phase 20 boundary, and compatibility boundary.
- `plugin/vibe/references/workspace-layout.md` - Required `.vibe` directories, config contract, non-destructive initialization, and model readability expectations.
- `plugin/vibe/references/collaboration-protocol.md` - Clarify, plan, split, execute, review, and aggregate protocol.
- `plugin/vibe/references/task-contract.md` - Required task JSON fields, repo-root-relative file scope rules, verification, and status values.
- `plugin/vibe/references/agent-contract.md` - Agent definition fields, subprocess boundary, secret handling, and compatibility notes.
- `plugin/vibe/references/review-protocol.md` - Review timing, finding fields, severity, required fixes, and completion gate.

## Decisions Made

- Kept all Phase 20 reference contracts as Markdown rather than executable schema files.
- Made `.vibe` artifacts the synchronization source for task, run, review, lock, and log state.
- Treated environment variables crossing into subprocess Agents as allowlisted and non-dumpable into artifacts.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

Task 2 acceptance checks required exact continuous strings for `terminal panes are compatibility only` and `explicitly accepted by the Conductor`; wording was adjusted before commit and the full gate was re-run successfully.

## Verification

- `test -f` checks passed for all six reference files.
- `rg 'file_scope|reviewer_requirements|repo-root-relative' plugin/vibe/references/task-contract.md` passed.
- `rg 'model_command|Secret Handling|allowlisted' plugin/vibe/references/agent-contract.md` passed.
- `rg 'severity|required_fix|Completion Gate' plugin/vibe/references/review-protocol.md` passed.
- No Rust files were changed, so `cargo test` was not required by the Phase 20 validation strategy.

## Known Stubs

None.

## Threat Flags

None - security-relevant surfaces documented in this plan match the plan threat model.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 21 can use `workspace-layout.md` and `agent-contract.md` to implement `.vibe/Agents`, `config.json`, and non-destructive initialization. Phase 22 can use `task-contract.md` and `collaboration-protocol.md` to implement task writing, locks, subprocess launch, logs, and run artifacts.

## Self-Check: PASSED

- Created files exist on disk.
- Task commits `71b86f4` and `0f2d123` exist in git history.
- Plan verification commands passed.

---
*Phase: 20-plugin-first-architecture*
*Completed: 2026-04-22*
