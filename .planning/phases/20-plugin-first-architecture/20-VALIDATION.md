---
phase: 20
slug: plugin-first-architecture
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-04-22
---

# Phase 20 - Validation Strategy

> Per-phase validation contract for plugin-first architecture planning and scaffold work.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Filesystem/content smoke checks plus existing Cargo tests when Rust files are touched |
| **Config file** | `Cargo.toml`; no JS test framework required in Phase 20 |
| **Quick run command** | `test -f plugin/vibe/.codex-plugin/plugin.json && test -f plugin/vibe/skills/conductor/SKILL.md` |
| **Full suite command** | `cargo test` plus all scaffold smoke checks listed below |
| **Estimated runtime** | ~60 seconds |

---

## Sampling Rate

- **After every task commit:** Run scaffold smoke checks for files created or modified by that task.
- **After every plan wave:** Run all scaffold smoke checks; run `cargo test` only if `apps/`, `crates/`, or `Cargo.toml` changed.
- **Before `$gsd-verify-work`:** Full scaffold smoke suite must pass, and `cargo test` must pass if legacy Rust code changed.
- **Max feedback latency:** 120 seconds.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 20-01-01 | 01 | 1 | PLUG-01 | T-20-01 | Plugin manifest and package layout are explicit and local | filesystem | `test -f plugin/vibe/.codex-plugin/plugin.json` | no | pending |
| 20-01-02 | 01 | 1 | PLUG-01 | T-20-01 | Required plugin surfaces exist | filesystem | `test -d plugin/vibe/skills && test -d plugin/vibe/references && test -d plugin/vibe/scripts` | no | pending |
| 20-02-01 | 02 | 1 | PLUG-02 | T-20-02 | Protocol references are model-readable files, not hidden state | filesystem | `test -f plugin/vibe/references/collaboration-protocol.md && test -f plugin/vibe/references/task-contract.md` | no | pending |
| 20-02-02 | 02 | 1 | PLUG-02 | T-20-02 | Agent and review contracts are explicit | filesystem | `test -f plugin/vibe/references/agent-contract.md && test -f plugin/vibe/references/review-protocol.md` | no | pending |
| 20-03-01 | 03 | 1 | PLUG-03 | T-20-03 | Conductor skill exists with Codex skill frontmatter | content | `rg '^name:|^description:' plugin/vibe/skills/conductor/SKILL.md` | no | pending |
| 20-03-02 | 03 | 1 | PLUG-03 | T-20-03 | Conductor skill references clarify-plan-task-review loop | content | `rg 'clarify|task|review|aggregate|Conductor' plugin/vibe/skills/conductor/SKILL.md` | no | pending |
| 20-04-01 | 04 | 1 | PLUG-04 | T-20-04 | Command stubs for core workflow entry points exist | filesystem | `test -f plugin/vibe/commands/init.md && test -f plugin/vibe/commands/plan.md && test -f plugin/vibe/commands/run-task.md && test -f plugin/vibe/commands/review-task.md && test -f plugin/vibe/commands/status.md && test -f plugin/vibe/commands/release-summary.md` | no | pending |
| 20-05-01 | 05 | 1 | PLUG-05 | T-20-05 | Legacy CLI migration classification exists | content | `rg 'Migrate-to-script|Compatibility|Remove' plugin/vibe/references/migration-classification.md` | no | pending |

*Status: pending / green / red / flaky*

---

## Wave 0 Requirements

- [ ] `plugin/vibe/.codex-plugin/plugin.json` - covers PLUG-01.
- [ ] `plugin/vibe/skills/conductor/SKILL.md` - covers PLUG-03.
- [ ] `plugin/vibe/references/collaboration-protocol.md` - covers PLUG-02.
- [ ] `plugin/vibe/references/task-contract.md` - covers PLUG-02.
- [ ] `plugin/vibe/references/agent-contract.md` - covers PLUG-02.
- [ ] `plugin/vibe/references/review-protocol.md` - covers PLUG-02.
- [ ] `plugin/vibe/references/workspace-layout.md` - covers PLUG-02.
- [ ] `plugin/vibe/references/migration-classification.md` - covers PLUG-05.
- [ ] `plugin/vibe/commands/init.md`, `plan.md`, `run-task.md`, `review-task.md`, `status.md`, `release-summary.md` - covers PLUG-04.
- [ ] `plugin/vibe/templates/.vibe/` scaffold - supports Phase 21 planning boundary.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Command file semantics are intentionally conservative | PLUG-04 | Codex command file schema is currently treated as assumed in research | Read `plugin/vibe/commands/*.md` and verify they are safe documented command contracts, not unsupported executable assumptions |
| Legacy CLI migration classification is reasonable | PLUG-05 | Classification is a product/architecture judgment | Review `migration-classification.md` and confirm each old CLI capability is categorized as migrate-to-script, compatibility, or remove with rationale |

---

## Validation Sign-Off

- [ ] All tasks have automated scaffold/content checks or explicit manual verification.
- [ ] No 3 consecutive tasks lack an automated verify command.
- [ ] Wave 0 covers all missing scaffold references.
- [ ] No watch-mode flags.
- [ ] Feedback latency < 120s.
- [ ] `nyquist_compliant: true` set in frontmatter.

**Approval:** pending
