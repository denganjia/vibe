# Testing Patterns

**Analysis Date:** 2025-01-24

## Test Framework

**Runner:**
- Rust `cargo test` for unit and integration tests.
- Bash for E2E integration tests: `scripts/e2e_test.sh`.

**Assertion Library:**
- Standard Rust `assert!`, `assert_eq!`.

## Test File Organization

**Location:**
- Unit tests: Co-located in source files within `#[cfg(test)] mod tests` blocks.
- Integration tests: `crates/vibe-core/tests/` (if applicable).
- E2E tests: `scripts/e2e_test.sh`.

## Test Structure

**Unit Test Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_feature() -> Result<()> {
        let dir = tempdir()?;
        // Test logic...
        Ok(())
    }
}
```

## Mocking

**Terminal Orchestration Mocking:**
- Strategy: Shadow the terminal binary (e.g., `wezterm`, `tmux`) by placing a mock shell script in a directory at the front of the `PATH`.
- Implementation: The mock script intercepts calls, records arguments/input, and returns pre-defined outputs (like a fake `pane_id`).
- Example: `scripts/e2e_test.sh` creates a fake `wezterm` script.

**Agent Mocking:**
- `mock_agent.sh`: Simulates an autonomous agent that reads its persona, performs a delay, and then uses `vibe report` and `vibe signal` to communicate back.

## Terminal Orchestration Testing Strategy

**Blackbox Integration:**
The E2E test (`scripts/e2e_test.sh`) validates the interaction between the CLI tool and the terminal environment without requiring a real terminal multiplexer.

1.  **Environment Setup**: Creates a temporary work directory with `.vibe/config.json`.
2.  **Binary Mocking**: Creates fake binaries for terminal tools in a local `bin/` directory and adds it to `PATH`.
3.  **Signal Interception**: Uses named pipes (`mkfifo`) to capture signals emitted by the tool (`vibe wait`).
4.  **Background Execution**: Runs the `vibe spawn` command and then manually triggers the "spawned" agent (the mock agent script) to simulate the terminal executing the injected command.
5.  **State Verification**:
    - Checks `.vibe/state/panes.json` for correct status and summary updates.
    - Verifies that signals are correctly received and payloads match expectations.

## Coverage

**Requirements:** None enforced, but critical paths (spawn, signal, wait) must have E2E coverage.

## Test Types

**Unit Tests:**
- Logic in `env.rs`, `state/mod.rs`, `ipc/protocol.rs`.
- Filesystem interactions using `tempfile`.

**Integration Tests:**
- Library-level interaction in `vibe-core`.

**E2E Tests:**
- Full lifecycle tests from command invocation to signal resolution.

## Common Patterns

**Async Testing:**
- Use `#[tokio::test]` for functions using async/await.

**Error Testing:**
- Assert that specific errors (like `VibeError::TerminalDetectionFailed`) are returned when expected.

---

*Testing analysis: 2025-01-24*
