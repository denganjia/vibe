# Testing Patterns

**Analysis Date:** 2024-03-20

## Test Framework

**Runner:**
- Built-in `cargo test`

**Assertion Library:**
- Rust Standard Library (`assert!`, `assert_eq!`)

**Run Commands:**
```bash
cargo test              # Run all tests
cargo test -- --nocapture # See stdout during testing
```

## Test File Organization

**Location:**
- **Unit Tests:** In-file `mod tests` for logic close to implementation (e.g., `crates/vibe-core/src/ipc/server.rs`).
- **Integration Tests:** `crates/vibe-core/tests/` directory for cross-module or complex scenario testing.

**Naming:**
- Unit tests: `mod tests`
- Integration tests: descriptive filename in `tests/` (e.g., `concurrency_test.rs`).

## Test Structure

**Async Testing:**
- Use `#[tokio::test]` for tests involving async code, IPC, or actors.
- Return `anyhow::Result<()>` or `crate::error::Result<()>` to use `?` in tests.

**Example (Integration Test):**
```rust
#[tokio::test]
async fn test_scenario() -> anyhow::Result<()> {
    // Setup
    let dir = tempdir()?;
    let socket = dir.path().join("vibe.sock");

    // Execution
    let handle = tokio::spawn(async move { ... });

    // Assertion
    assert!(condition);
    
    Ok(())
}
```

## Mocking

**Framework:** None (prefer in-memory or temporary real resources).

**Patterns:**
- **In-memory SQLite:** Use `Connection::open_in_memory()` for database testing.
- **Temporary Directories:** Use `tempfile::tempdir()` for filesystem and Unix domain socket testing.

## Fixtures and Factories

**Test Data:**
- Manual instantiation of protocol structs (e.g., `RegisterInfo`).

**Location:**
- Within test modules or integration test files.

## Concurrency Testing

**Multi-worker Simulation:**
- Spawn multiple `tokio` tasks to simulate concurrent workers (`WorkerClient`).
- Use `tokio::time::sleep` to allow for registration and heartbeat cycles.
- Verify state by querying the `DbHandle`.

**Master Recovery (Restart):**
- Spawn the `MasterServer` as a task.
- Use `AbortHandle` or `abort()` to simulate a crash/restart.
- Re-spawn the server and verify that clients reconnect and state is preserved/recovered.

## Idle Timeout Testing

**Pattern:**
- Initialize the server with a short `idle_timeout` (e.g., 500ms).
- Start the server and `await` its `run()` method directly (not in a spawned task).
- Measure the elapsed time and verify it is greater than or equal to the timeout.
- Verify that cleanup (like socket file removal) occurs after the timeout.

**Example:**
```rust
let start = Instant::now();
server.run().await?;
assert!(start.elapsed() >= Duration::from_millis(500));
```

## Test Types

**Unit Tests:**
- Test individual functions or small components (e.g., `VibeID` generation).

**Integration Tests:**
- `crates/vibe-core/tests/concurrency_test.rs`: Tests IPC server-client communication, multi-worker registration, and master server restart scenarios.
- `crates/vibe-core/src/ipc/server.rs` (internal tests): Tests master handshake, message handling, and idle timeout.

**Error Testing:**
- Verify error propagation through `Result` return values.
- Check specific error variants when necessary.

---

*Testing analysis: 2024-03-20*
