# Testing Patterns

**Analysis Date:** 2025-02-18

## Test Framework

**Runner:**
- Rust `cargo test` for unit and integration tests.
- Bash for E2E integration tests.
- Config: `Cargo.toml` in workspace members.

**Assertion Library:**
- Standard Rust `assert!`, `assert_eq!`.

**Run Commands:**
```bash
cargo test             # Run all tests
./scripts/e2e_test.sh  # Run E2E integration and stress tests
```

## Test File Organization

**Location:**
- Co-located in source files within `#[cfg(test)] mod tests` blocks (e.g., `crates/vibe-core/src/ipc/bus.rs`).

**Naming:**
- E2E scripts: `scripts/e2e_test.sh`
- Rust tests: `#[test] fn test_*()`

**Structure:**
```
[project-root]/
├── crates/vibe-core/src/**/*.rs  # Co-located unit tests
└── scripts/e2e_test.sh           # Automated E2E stress tests
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_feature() -> anyhow::Result<()> {
        let dir = tempdir()?;
        // setup...
        // assert_eq!(...);
        Ok(())
    }
}
```

**Patterns:**
- **Setup:** Create temporary environments using `tempfile::tempdir`.
- **Teardown:** `tempdir` automatically cleans up when dropped.
- **Assertion:** Standard `assert_eq!` on returned Results.

## Mocking

**Framework:** Bash scripts (for E2E) and `tempfile` (for unit tests).

**Patterns:**
```bash
# E2E Mocking Pattern (scripts/e2e_test.sh)
cat << 'WEZ' > "$TEST_DIR/bin/wezterm"
#!/bin/bash
if [ "$1" == "cli" ] && [ "$2" == "split-pane" ]; then
    echo "81"
elif [ "$1" == "cli" ] && [ "$2" == "send-text" ] && [ "$3" == "--pane-id" ] && [ "$4" == "81" ]; then
    TEXT="$5"
    echo -n "$TEXT" >> "$TEST_DIR/injected.txt"
    exit 0
fi
WEZ
```

**What to Mock:**
- Terminal multiplexer binaries (`wezterm`, `tmux`) during E2E testing.
- Agent scripts (`mock_agent.sh`) to simulate autonomous agent behavior.

**What NOT to Mock:**
- File-based IPC (bus). Use actual temporary directories to test read/write file locks.

## Fixtures and Factories

**Test Data:**
```bash
echo "{}" > "$TEST_DIR/.vibe/state/panes.json"
echo "# Scanner Persona" > "$TEST_DIR/.vibe/roles/Scanner.md"
```

**Location:**
- Generated dynamically within `scripts/e2e_test.sh` in the temporary `e2e_test_workdir`.

## Coverage

**Requirements:** Critical paths (spawn, signal, wait) must have E2E coverage. The E2E stress test validates the autonomous loop and intelligence-first decision making.

**View Coverage:**
```bash
cargo test
```

## Test Types

**Unit Tests:**
- Tests core logic like IPC bus reading/writing, protocol formatting, and environment parsing inside `crates/vibe-core/src/*.rs`.

**Integration Tests:**
- Tests library-level boundaries.

**E2E Tests:**
- **Automated E2E Stress Testing**: `scripts/e2e_test.sh` automates terminal interaction without manual keyboard intervention. It spawns a mock agent, simulates text injection via a fake terminal adapter, waits for file-bus signals using named pipes (`mkfifo`), and asserts on the final JSON state (`.vibe/state/panes.json`). This strictly validates the autonomous A-D-E-V loop orchestration.

## Common Patterns

**Async Testing:**
```rust
#[tokio::test]
async fn test_async_behavior() {
    // Execute async functions
}
```

**Error Testing:**
```rust
assert!(result.is_err());
```