---
phase: 19-autonomous-workflow
reviewed: 2026-04-21T08:24:34Z
depth: standard
files_reviewed: 19
files_reviewed_list:
  - .vibe/roles/Conductor.md
  - .vibe/roles/Worker.md
  - Cargo.toml
  - DELIVERY.md
  - apps/vibe-cli/src/main.rs
  - crates/vibe-core/src/adapter/encoder.rs
  - crates/vibe-core/src/adapter/mod.rs
  - crates/vibe-core/src/adapter/tmux.rs
  - crates/vibe-core/src/adapter/wezterm.rs
  - crates/vibe-core/src/ipc/bus.rs
  - skills/vibe-operator/SKILL.md
  - skills/vibe-operator/references/approval.md
  - skills/vibe-operator/references/benchmarks.md
  - skills/vibe-operator/references/collaboration.md
  - skills/vibe-operator/references/orchestration.md
  - skills/vibe-operator/references/recovery.md
  - skills/vibe-operator/references/role.md
  - skills/vibe-operator/references/state.md
  - skills/vibe-operator/references/verification.md
findings:
  critical: 1
  warning: 4
  info: 2
  total: 7
status: issues_found
---

# Phase 19: Code Review Report

**Reviewed:** 2026-04-21T08:24:34Z
**Depth:** standard
**Files Reviewed:** 19
**Status:** issues_found

## Summary

Reviewed the explicit Phase 19 file list at standard depth. `Cargo.lock` was loaded for context but filtered from review scope as a lock file. The Rust changes compile with `cargo check -q`, but the review found one command-injection risk, several correctness issues in pane lifecycle and file-bus semantics, and two maintainability/test-coverage issues.

## Critical Issues

### CR-01: Unquoted cwd Allows Shell Command Injection During Inject

**File:** `apps/vibe-cli/src/main.rs:355`
**Issue:** `vibe inject --cwd` builds shell text with `format!("cd {} && {}", dir, command)`. A directory containing shell metacharacters, spaces, or `;` changes the injected command. Because this text is sent directly into the target shell, a malicious or malformed cwd can execute unintended commands before the requested command.
**Fix:**
```rust
fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

let cmd_str = if let Some(dir) = cwd {
    format!("cd -- {} && {}", shell_quote(&dir), command)
} else {
    command
};
```

## Warnings

### WR-01: Kill Closes Logical Vibe IDs Instead of Physical Pane IDs

**File:** `apps/vibe-cli/src/main.rs:253`
**Issue:** `vibe kill` calls `adapter.close(&state.vibe_id)` even though terminal adapters close physical pane IDs. Spawned roles are saved as logical IDs like `v-xxxx` with a separate `physical_id`, so close fails and line 256 still removes state, leaving live panes orphaned and invisible to later cleanup.
**Fix:**
```rust
for state in panes {
    println!("Killing pane: {}", state.vibe_id);
    match adapter.close(&state.physical_id) {
        Ok(()) => store.remove_pane(&state.vibe_id)?,
        Err(e) => eprintln!("Failed to close pane {}: {}", state.physical_id, e),
    }
}
```

### WR-02: Signal Consumption Returns Success Even When Delete Fails

**File:** `crates/vibe-core/src/ipc/bus.rs:58`
**Issue:** `FileBus::recv` ignores `remove_file` errors and returns the payload anyway. If two conductors wait for the same signal, both can read it, one delete succeeds, and the other still returns success after a failed delete. That violates the documented consume-on-read contract.
**Fix:**
```rust
match fs::remove_file(&path) {
    Ok(()) => return Ok(signal.payload),
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
    Err(e) => return Err(e.into()),
}
```

### WR-03: `vibe check` Human Output Branch Is Unreachable

**File:** `apps/vibe-cli/src/main.rs:40`
**Issue:** `Check { json }` sets `default_value_t = true` for a boolean flag. With the current CLI shape, `vibe check` defaults to JSON and there is no `--no-json`, so the non-JSON branch at lines 236-240 is effectively unreachable.
**Fix:** Either make human output the default:
```rust
#[arg(short, long)]
json: bool,
```
or add an explicit negative flag using clap support for `--no-json`.

### WR-04: Bus Documentation Describes Metadata Not Present in Signals

**File:** `skills/vibe-operator/references/state.md:17`
**Issue:** The SOP says `vibe signal` creates an envelope containing sender and timestamp, but `SignalInfo` currently stores only `name` and `payload`; the timestamp is only in the filename and no sender exists. Agents following the SOP may parse fields that are never emitted.
**Fix:** Either update `SignalInfo` and `FileBus::send` to include `sender` and `timestamp`, or change the SOP to state that signal files contain only `name` and `payload`.

## Info

### IN-01: Unused Mutable Binding

**File:** `apps/vibe-cli/src/main.rs:410`
**Issue:** `cargo check -q` reports `let mut config_manager` does not need to be mutable.
**Fix:** Change it to `let config_manager = vibe_core::state::ConfigManager::new()?;`.

### IN-02: File Bus Integration Test Is Ignored

**File:** `crates/vibe-core/src/ipc/bus.rs:80`
**Issue:** `test_bus_send_recv` is marked `#[ignore]`, so `cargo test` no longer exercises the core file-bus send/receive path that Phase 19 depends on.
**Fix:** Make the test non-ignored after isolating global cwd mutation, or move the bus directory resolver behind an injectable test path so the test can run reliably by default.

---

_Reviewed: 2026-04-21T08:24:34Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
