---
phase: 25
slug: universal-manifests-packaging
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-04-24
---

# Phase 25 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Node.js built-in `node:test` + `node:assert` |
| **Config file** | none — use Node.js built-ins |
| **Quick run command** | `node --test plugin/vibe/scripts/manifests.test.js` |
| **Full suite command** | `npm test` from `plugin/vibe/` after `package.json` exists |
| **Estimated runtime** | ~2 seconds |

---

## Sampling Rate

- **After every task commit:** Run `node --test plugin/vibe/scripts/manifests.test.js`
- **After every plan wave:** Run `npm test` from `plugin/vibe/`
- **Before `$gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 2 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 25-01-01 | 01 | 1 | MAN-01 | T-25-01 / T-25-02 | No unexpected dependency install or publish path | smoke | `node --test plugin/vibe/scripts/manifests.test.js` | no W0 | pending |
| 25-01-02 | 01 | 1 | MAN-02 | T-25-01 | Gemini manifest identity matches `package.json` | smoke | `node --test plugin/vibe/scripts/manifests.test.js` | no W0 | pending |
| 25-01-03 | 01 | 1 | MAN-03 | T-25-01 | Claude manifest uses plugin-root-safe layout | smoke | `node --test plugin/vibe/scripts/manifests.test.js` | no W0 | pending |
| 25-01-04 | 01 | 1 | MAN-04 | T-25-01 / T-25-03 | Codex manifest and local discovery stay inside repo root | smoke | `node --test plugin/vibe/scripts/manifests.test.js` | partial W0 | pending |

*Status: pending, green, red, flaky*

---

## Wave 0 Requirements

- [ ] `plugin/vibe/package.json` — package skeleton and test entrypoint.
- [ ] `plugin/vibe/gemini-extension.json` — Gemini manifest.
- [ ] `plugin/vibe/.claude-plugin/plugin.json` — Claude Code manifest.
- [ ] `plugin/vibe/scripts/manifests.test.js` — smoke test covering MAN-01 through MAN-04.
- [ ] `plugin/vibe/.codex-plugin/plugin.json` — existing file aligned with `package.json`.
- [ ] `.agents/plugins/marketplace.json` — existing discovery file verified for `./plugin/vibe`.

---

## Manual-Only Verifications

All phase behaviors have automated verification. Real provider CLI loading is intentionally excluded from Phase 25 and can be added later as an optional integration check.

---

## Validation Sign-Off

- [x] All tasks have automated verify commands or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 2 seconds
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-04-24
