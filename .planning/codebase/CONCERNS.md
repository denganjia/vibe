# Codebase Concerns

**Analysis Date:** 2025-03-04

## Tech Debt

**Terminal Signaling Mechanism:**
- Issue: Signaling between agents relies on raw text injection into the master pane's stdin and string matching in `vibe wait`.
- Files: `apps/vibe-cli/src/main.rs`, `crates/vibe-core/src/adapter/wezterm.rs`, `crates/vibe-core/src/adapter/tmux.rs`
- Impact: Highly fragile. Signals can be missed if `vibe wait` is not active, garbled by other output, or intercepted by interactive agents. False positives are possible if agents print similar strings.
- Fix approach: Implement a more robust IPC mechanism (e.g., Unix Domain Sockets or a local TCP bus) instead of TTY hijacking.

**Hardcoded Shell Assumptions:**
- Issue: Several adapters hardcode `bash -c` and `exec bash` for command execution and environment setup.
- Files: `crates/vibe-core/src/adapter/wezterm.rs`, `crates/vibe-core/src/adapter/tmux.rs`
- Impact: Prevents execution on systems where `bash` is not available or where users prefer other shells (zsh, fish, pwsh).
- Fix approach: Use the `ShellAdapter` in `crates/vibe-core/src/os/shell.rs` to determine the appropriate shell and formatting for the current platform.

**State Store Locking:**
- Issue: File-based locking (`.lock` file) is used for all state operations without stale lock recovery.
- Files: `crates/vibe-core/src/state/mod.rs`
- Impact: If a process crashes while holding the lock, the entire system blocks until the lock is manually removed. High contention on a single JSON file limits scalability.
- Fix approach: Add PID-based lock validation or use a more robust database like SQLite which handles concurrency and locking more gracefully.

## Known Bugs

**Cmd Environment Variable Injection:**
- Symptoms: Potential command execution when setting environment variables on Windows `cmd.exe`.
- Files: `crates/vibe-core/src/os/shell.rs`
- Trigger: Values containing `&`, `|`, or `>` passed to `build_env_command` for `ShellType::Cmd`.
- Workaround: None currently implemented. Values should be properly quoted or escaped.

**Spurious "No active WezTerm window found":**
- Symptoms: `vibe spawn` may fail even if WezTerm is running if the environment variables aren't correctly passed or detected.
- Files: `crates/vibe-core/src/adapter/wezterm.rs`
- Trigger: Running `vibe spawn` from outside a WezTerm context or in a nested environment.

## Security Considerations

**Command Injection in Shell Adapters:**
- Risk: Unsanitized input from roles or persona templates can be used to inject shell commands.
- Files: `crates/vibe-core/src/os/shell.rs`, `apps/vibe-cli/src/main.rs`
- Current mitigation: Basic escaping for Bash and Pwsh. No escaping for Cmd.
- Recommendations: Implement strict sanitization for all strings being interpolated into shell commands. Use structured argument passing instead of string concatenation where possible.

## Performance Bottlenecks

**State File Contention:**
- Problem: Every `vibe report` or heartbeat requires a full read/write of `panes.json` with a global file lock.
- Files: `crates/vibe-core/src/state/mod.rs`
- Cause: Monolithic state file and coarse-grained locking.
- Improvement path: Split state into individual files per pane or use a transactional database.

## Fragile Areas

**Terminal Detection:**
- Files: `crates/vibe-core/src/env.rs`
- Why fragile: Relies exclusively on environment variables like `WEZTERM_PANE` or `TMUX`, which can be lost in `sudo` sessions, SSH, or certain shell configurations.
- Safe modification: Check for terminal-specific escape sequence responses as a fallback.
- Test coverage: Minimal, ignored tests in `env.rs`.

## Scaling Limits

**Signal Bus (Master Pane):**
- Current capacity: Single master pane for all signals.
- Limit: Becomes a bottleneck and a single point of failure. If the master pane is flooded with output, `vibe wait` performance degrades.
- Scaling path: Decentralized signal distribution or a dedicated background bus process.

## Test Coverage Gaps

**Terminal Adapters:**
- What's not tested: Real interaction with `tmux` or `wezterm` CLIs.
- Files: `crates/vibe-core/src/adapter/tmux.rs`, `crates/vibe-core/src/adapter/wezterm.rs`
- Risk: Changes to CLI output or behavior in new terminal versions can break the adapters without notice.
- Priority: High

**Windows Compatibility:**
- What's not tested: Cross-platform behavior of terminal splitting and command injection on Windows.
- Files: `crates/vibe-core/src/os/windows.rs`, `crates/vibe-core/src/os/shell.rs`
- Risk: Windows-specific bugs remain undetected until manual testing.
- Priority: Medium

---

*Concerns audit: 2025-03-04*
