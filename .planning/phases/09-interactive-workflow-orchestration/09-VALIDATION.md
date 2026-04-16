---
phase: 09
slug: interactive-workflow-orchestration
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-04-16
---

# Phase 09 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | `cargo test` |
| **Config file** | `Cargo.toml` |
| **Quick run command** | `cargo test -p vibe-cli --lib mcp` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p vibe-cli`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 30 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 09-01-01 | 01 | 0 | SCO-01 | — | N/A | unit | `cargo test test_mcp_routes` | ❌ W0 | ⬜ pending |
| 09-01-02 | 01 | 0 | SCO-01 | — | N/A | unit | `cargo test test_protocol_serialization` | ❌ W0 | ⬜ pending |
| 09-02-01 | 01 | 1 | SCO-01 | — | N/A | unit | `cargo test test_mcp_submit_plan` | ❌ W0 | ⬜ pending |
| 09-02-02 | 01 | 1 | SCO-01 | — | N/A | unit | `cargo test test_state_approval_transition` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `apps/vibe-cli/src/mcp.rs` — tests for the new JSON-RPC routes
- [ ] `crates/vibe-core/src/ipc/protocol.rs` — serialization tests for new events

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| TUI "Waiting for Approval" visual state | SCO-01 | Visual/UI behavior | Launch vibe, submit a plan, verify TUI displays the pending state correctly. |
| Worker pane stdin approval | SCO-01 | Interactive behavior | Launch vibe, submit a plan, go to the worker pane, and verify you can approve/reject via stdin. |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 30s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending 2026-04-16
