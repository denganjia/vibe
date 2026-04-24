---
phase: 25-universal-manifests-packaging
plan: 01
subsystem: infra
tags: [plugin, manifests, npm, codex, claude, gemini]
requires:
  - phase: 20-plugin-first-architecture
    provides: plugin package root, local discovery layout, and deterministic Node.js script patterns
provides:
  - shared plugin identity in `plugin/vibe/package.json`
  - minimal Gemini and Claude manifests aligned with Codex identity
  - deterministic smoke coverage for manifest and marketplace consistency
affects: [26-skill-standardization, 27-mcp-server-integration, plugin-discovery]
tech-stack:
  added: [npm package metadata, Node.js built-in node:test]
  patterns: [package-json-source-of-truth, minimal-provider-manifests, local-discovery-smoke-test]
key-files:
  created:
    - plugin/vibe/package.json
    - plugin/vibe/gemini-extension.json
    - plugin/vibe/.claude-plugin/plugin.json
    - plugin/vibe/scripts/manifests.test.js
  modified: []
key-decisions:
  - "Use `plugin/vibe/package.json` as the only shared source for name, version, and description."
  - "Keep Gemini and Claude manifests identity-only in Phase 25 while preserving Codex-specific `skills` and `interface` fields."
  - "Validate marketplace and manifest consistency with pure Node.js tests instead of provider CLI loading."
patterns-established:
  - "Shared identity pattern: `package.json` defines the core plugin identity and provider manifests copy it verbatim."
  - "Boundary pattern: provider manifests stay minimal until Phase 27 introduces real MCP runtime fields."
  - "Verification pattern: local discovery contracts are enforced with a deterministic `node:test` smoke test."
requirements-completed: [MAN-01, MAN-02, MAN-03, MAN-04]
duration: 2 min
completed: 2026-04-24
---

# Phase 25 Plan 01: Universal Manifests & Packaging Summary

**Shared `plugin/vibe` package identity with Gemini/Claude manifests and a deterministic local-discovery smoke test**

## Performance

- **Duration:** 2 min
- **Started:** 2026-04-24T03:49:22Z
- **Completed:** 2026-04-24T03:51:53Z
- **Tasks:** 3
- **Files modified:** 4

## Accomplishments
- Added `plugin/vibe/package.json` as the single source of truth for plugin identity, package privacy, Node runtime, and test entrypoints.
- Added minimal `gemini-extension.json` and `.claude-plugin/plugin.json` manifests aligned to the shared package identity.
- Added a pure Node.js smoke test that verifies package metadata, provider manifest consistency, and repo-local marketplace discovery without invoking provider CLIs.
- Preserved the existing Codex manifest and repo-local marketplace contract unchanged, while validating both through automated tests.

## Task Commits

Each task was committed atomically:

1. **Task 1: Create `plugin/vibe/package.json` as the manifest source of truth** - `79df992` (feat)
2. **Task 2: Add minimal Gemini and Claude manifests and align the Codex manifest** - `aa4f1ab` (feat)
3. **Task 3: Lock marketplace discovery and manifest consistency behind a Node.js smoke test** - `c1d4784` (test)

## Files Created/Modified
- `plugin/vibe/package.json` - Shared plugin identity, private package guardrail, Node engine contract, and `npm test` entrypoint.
- `plugin/vibe/gemini-extension.json` - Gemini CLI minimal identity manifest for Phase 25.
- `plugin/vibe/.claude-plugin/plugin.json` - Claude Code minimal identity manifest for Phase 25.
- `plugin/vibe/scripts/manifests.test.js` - Deterministic smoke test for package, manifests, and local marketplace discovery.

## Decisions Made
- `plugin/vibe/package.json` now owns the canonical `name`, `version`, and `description` values for all provider manifests.
- Gemini and Claude stay on the smallest valid identity surface in this phase; future MCP or directory override fields remain deferred.
- Codex local discovery remains repo-local through `.agents/plugins/marketplace.json` and is enforced by test rather than by provider-specific runtime checks.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- A one-line `node -e` acceptance probe for Task 2 referenced `path` before declaration; rerunning the corrected verification confirmed the manifests were already valid. No code change was required.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 25 plan output is complete and green; `plugin/vibe` now has a stable package/manifests surface for later skill standardization and MCP registration work.
- Phase 26 can standardize skill metadata without revisiting package identity, and Phase 27 can add real MCP fields on top of the current smoke-tested baseline.

## Self-Check: PASSED
- Verified `plugin/vibe/package.json`, `plugin/vibe/gemini-extension.json`, `plugin/vibe/.claude-plugin/plugin.json`, and `plugin/vibe/scripts/manifests.test.js` exist on disk.
- Verified task commits `79df992`, `aa4f1ab`, and `c1d4784` are present in git history.
