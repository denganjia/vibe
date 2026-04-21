# Codebase Concerns

**Analysis Date:** 2024-05-24

## Tech Debt

**FileBus Concurrency:**
- Issue: `FileBus::recv` uses file polling and deletion without file locks. 
- Files: `crates/vibe-core/src/ipc/bus.rs`
- Impact: Potential race condition if multiple consumers listen for the same signal. Both might read the JSON, but only one removes it. The other ignores the remove error (`let _ = fs::remove_file(...)`), leading to duplicate processing. Also, `fs::read_dir` order is not guaranteed, breaking FIFO processing.
- Fix approach: Implement file locking (e.g., using `fs2` or `fs3` crates) or use a proper IPC mechanism (sockets/named pipes). Sort directory entries by timestamp/filename before processing to enforce FIFO.

**TTY ANSI Stripping:**
- Issue: `strip_ansi` uses a basic regex that covers CSI (Control Sequence Introducer) but might miss OSC (Operating System Command), DCS (Device Control String), or other escape sequences.
- Files: `crates/vibe-core/src/os/shell.rs`
- Impact: Terminal output containing complex ANSI sequences (e.g., window titles, hyperlinks) may not be properly cleaned, breaking text parsing logic downstream.
- Fix approach: Use a dedicated ANSI parsing crate like `vte` or `strip-ansi-escapes` instead of maintaining a custom Regex.

## Fragile Areas

**Terminal Text Injection & Throttling:**
- Files: `crates/vibe-core/src/adapter/encoder.rs`, `crates/vibe-core/src/adapter/tmux.rs`, `crates/vibe-core/src/adapter/wezterm.rs`
- Why fragile: `TTYEncoder::throttle_inject` hardcodes a 64-byte chunk size and 5ms delay. Different OSes and terminals (Tmux vs WezTerm) might have varying PTY buffer limits, potentially causing dropped characters or stuttering on high-latency systems.
- Safe modification: Make chunk size and delay configurable or dynamically adaptive. 
- Test coverage: `throttle_inject` has a basic unit test, but lacks integration tests verifying end-to-end injection reliability with `tmux` or `wezterm` CLIs.

## Known Bugs

**Tmux Availability on Windows:**
- Symptoms: Tmux adapter will fail on native Windows environments since Tmux is Unix-centric.
- Files: `crates/vibe-core/src/adapter/tmux.rs`
- Trigger: Running Vibe with a Tmux target on Windows natively (outside WSL/MSYS2).
- Workaround: Gracefully disable the Tmux adapter on Windows and default to WezTerm, or surface a clear environment error.

## Performance Bottlenecks

**FileBus Polling:**
- Problem: `FileBus::recv` sleeps for 100ms in a busy loop repeatedly reading the `.vibe/bus/` directory.
- Files: `crates/vibe-core/src/ipc/bus.rs`
- Cause: Lack of event-driven file system monitoring.
- Improvement path: Migrate to the `notify` crate to watch the directory for `Create` events instead of manually polling, drastically reducing CPU and Disk I/O.

## Security Considerations

**Shell Command Escaping:**
- Risk: `ShellAdapter::build_env_command` and `build_cd_command` use basic string replacement for escaping (`'\'\''` for Bash, `''` for Pwsh).
- Files: `crates/vibe-core/src/os/shell.rs`
- Current mitigation: Basic single-quote escaping is applied.
- Recommendations: Complex nested quotes or unescaped variables might lead to syntax errors or command injection. Whenever possible, pass environment variables directly to the `std::process::Command` builder via `.env()` instead of serializing them into shell evaluation strings.

## Missing Critical Features

**Cross-Platform Injection Edge Cases:**
- Problem: `wezterm cli send-text --no-paste` handles literal text well, but injecting large payloads with mixed line endings (CRLF vs LF) hasn't been normalized before dispatching.
- Blocks: Consistent behavior of multiline script injection across Windows (CRLF) and Unix (LF) targets.
