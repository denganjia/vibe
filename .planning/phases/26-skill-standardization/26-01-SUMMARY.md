---
phase: "26-skill-standardization"
plan: "01"
subsystem: "plugin/vibe"
tags: ["tdd", "validation", "standardization"]
requires: ["SKL-01", "SKL-02"]
provides: ["Skill validation infrastructure"]
affects: ["plugin/vibe/package.json", "plugin/vibe/scripts/skills.test.js"]
tech-stack: ["Node.js", "js-yaml"]
key-files:
  - "plugin/vibe/package.json"
  - "plugin/vibe/scripts/skills.test.js"
decisions:
  - "Adopted TDD approach for skill standardization, ensuring validation fails until migration is complete."
  - "Integrated skills validation into the main test suite."
metrics:
  duration: "4 min"
  tasks_completed: 2
  files_modified: 4
---

# Phase 26 Plan 01: Setup TDD validation for skill standardization Summary

## One-liner
Established a TDD validation framework using `js-yaml` to enforce skill frontmatter standards and directory structure cleanup.

## Key Changes
- Updated `plugin/vibe/package.json` to include `js-yaml` as a devDependency and added the `test:skills` script.
- Modified `plugin/vibe/scripts/manifests.test.js` to align with the updated `package.json` configuration.
- Created `plugin/vibe/scripts/skills.test.js` which validates:
    - Existence of `SKILL.md` in all skill directories.
    - Presence of `name`, `version`, and `description` in `SKILL.md` frontmatter.
    - Removal of legacy directories (`commands`, `roles`) and files (`Conductor.md`).
- Executed initial tests, confirming failures as expected under the TDD "RED" phase.

## Deviations from Plan
- **[Rule 1 - Bug] Fixed regression in manifests.test.js**
    - **Found during:** Task 1
    - **Issue:** Changing `package.json` caused existing manifest tests to fail because they asserted the exact content of `scripts` and `devDependencies`.
    - **Fix:** Updated `manifests.test.js` to match the new `package.json` structure.
    - **Files modified:** `plugin/vibe/scripts/manifests.test.js`
    - **Commit:** `c7f5353`

## Self-Check: PASSED

## Threat Flags
| Flag | File | Description |
|------|------|-------------|
| threat_flag: tampering | plugin/vibe/scripts/skills.test.js | uses js-yaml to parse local files. Mitigated by using default safe load. |
