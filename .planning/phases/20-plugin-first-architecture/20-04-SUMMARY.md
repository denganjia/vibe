---
phase: 20-plugin-first-architecture
plan: 04
subsystem: migration
tags: [plugin-first, migration, rust-cli, vibe-workspace]

requires:
  - phase: 20-plugin-first-architecture
    provides: plugin-first architecture context and legacy Rust CLI research
provides:
  - Legacy Rust CLI command migration matrix
  - Legacy state, bus, pane lifecycle, and environment variable classification
  - Phase 20 guardrails for preserving apps/ and crates/ as compatibility references
affects: [phase-21-vibe-workspace, phase-22-scripts-runtime, phase-24-cli-slimming]

tech-stack:
  added: []
  patterns: [markdown migration matrix, plugin-first compatibility boundary]

key-files:
  created:
    - plugin/vibe/references/migration-classification.md
  modified: []

key-decisions:
  - "Classified script-owned CLI behavior separately from optional terminal pane compatibility behavior."
  - "Kept apps/ and crates/ as Phase 20 compatibility references with no Rust deletion or rewrite."
  - "Documented allowlisted subprocess environment passing to avoid secret leakage."

patterns-established:
  - "Migration matrix rows use Old capability, Category, New owner, and Rationale."
  - "Legacy pane lifecycle remains Compatibility while task/result files become the plugin-first default."

requirements-completed: [PLUG-05]

duration: 3min
completed: 2026-04-22
---

# Phase 20 Plan 04: Migration Classification Summary

**Legacy Rust CLI command, state, bus, pane, and env responsibilities classified for plugin-first Vibe migration.**

## Performance

- **Duration:** 3 min
- **Started:** 2026-04-22T08:09:53Z
- **Completed:** 2026-04-22T08:13:10Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Created `plugin/vibe/references/migration-classification.md` with the required classification values: `Migrate-to-script`, `Compatibility`, and `Remove`.
- Mapped all old CLI capabilities from `apps/vibe-cli/src/main.rs` to script or compatibility ownership with rationale.
- Classified `.vibe` role/config/state, FileBus, terminal lifecycle, and `VIBE_*` subprocess environment behavior with security guardrails.
- Added validation checklist covering no Phase 20 Rust deletion, no terminal pane default, no hidden daemon/database state, no arbitrary env dump, and no path traversal.

## Task Commits

Each task was committed atomically:

1. **Task 1: Classify old CLI commands** - `db8923f` (docs)
2. **Task 2: Classify legacy state, env, and bus concepts** - `c419944` (docs)

**Plan metadata:** created by the final `docs(20-04)` summary commit

## Files Created/Modified

- `plugin/vibe/references/migration-classification.md` - Migration classification matrix and guardrails for old Rust CLI responsibilities.
- `.planning/phases/20-plugin-first-architecture/20-04-SUMMARY.md` - Execution summary for this plan.

## Decisions Made

- Classified `init --force`, `run <command>`, `signal`, `wait`, `report`, and `spawn --role/--stack` as `Migrate-to-script`.
- Classified pane-oriented capabilities (`split`, `focus`, `inject`, `kill`, `list`, `check`, `status TUI`) as `Compatibility`.
- Treated `apps/` and `crates/` as Phase 20 compatibility reference material, not deletion or rewrite targets.
- Required allowlisted environment passing for subprocess Agents to avoid secret leakage.

## Deviations from Plan

None - plan executed exactly as written.

**Total deviations:** 0 auto-fixed.
**Impact on plan:** No scope changes.

## Issues Encountered

- The first Task 1 commit initially picked up staged files from parallel work. The commit was amended immediately so it contains only `plugin/vibe/references/migration-classification.md`; the unrelated files were preserved in the worktree and not included in this plan's commits.
- Per orchestration instructions, this executor did not modify `.planning/STATE.md` or `.planning/ROADMAP.md`; those files are owned by the orchestrator for this parallel phase run.

## Verification

- `test -f plugin/vibe/references/migration-classification.md` passed.
- `rg 'Migrate-to-script|Compatibility|Remove' plugin/vibe/references/migration-classification.md` passed.
- `rg 'init --force|run <command>|signal|wait|report|spawn --role/--stack|split|focus|inject|kill|list|check|status TUI' plugin/vibe/references/migration-classification.md` passed.
- `rg 'VIBE_ID|VIBE_MASTER_ID|VIBE_PERSONA|allowlisted|secret leakage' plugin/vibe/references/migration-classification.md` passed.
- Stub scan for placeholder/TODO/FIXME/hardcoded empty UI values found no matches.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 21 and Phase 22 can use the migration classification to decide which legacy Rust concepts become `.vibe` templates and plugin scripts. Phase 24 can use the same document as the deletion guard before any Rust CLI slimming.

## Self-Check: PASSED

- Found `plugin/vibe/references/migration-classification.md`.
- Found `.planning/phases/20-plugin-first-architecture/20-04-SUMMARY.md`.
- Found task commit `db8923f`.
- Found task commit `c419944`.

---
*Phase: 20-plugin-first-architecture*
*Completed: 2026-04-22*
