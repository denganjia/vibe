# Testing Patterns

**Analysis Date:** 2024-04-18

## Test Framework

**Runner:**
- Built-in `cargo test`.

**Assertion Library:**
- Rust Standard Library (`assert!`, `assert_eq!`).

**Run Commands:**
```bash
cargo test              # Run all tests
cargo test -- --nocapture # See stdout during testing
```

## Test File Organization

**Location:**
- **Unit & Component Tests:** In-file `mod tests` for logic close to implementation.
- **Example:** `crates/vibe-core/src/ipc/server.rs` contains tests for the Master server and its protocol interaction.

**Naming:**
- Standard Rust `mod tests` pattern with `#[cfg(test)]`.

## Test Structure

**Async Testing:**
- Use `#[tokio::test]` for tests involving IPC, async I/O, or timers.
- Return `crate::error::Result<()>` to allow use of `?` operator.

**IPC Integration Pattern:**
```rust
#[tokio::test]
async fn test_master_interaction() -> Result<()> {
    let dir = tempdir().unwrap();
    let socket_path = dir.path().join("vibe.sock");
    let store = Arc::new(StateStore::new().unwrap());

    // Spawn server in background task
    let s_path = socket_path.clone();
    let store_clone = store.clone();
    tokio::spawn(async move {
        let server = MasterServer { ... };
        server.run().await.unwrap();
    });

    // Wait for server to bind
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Connect and interact via UnixStream
    let stream = UnixStream::connect(&socket_path).await.unwrap();
    // ... assertions ...
}
```

## Mocking

**Framework:** None (prefer temporary real resources).

**Patterns:**
- **Temporary Directories:** Use `tempfile::tempdir()` for filesystem (state files) and Unix domain socket testing.
- **In-memory State:** While `StateStore` persists to disk, tests usually use a temporary directory to ensure isolation.

## Fixtures and Factories

**Test Data:**
- Manual instantiation of protocol structs (e.g., `RegisterInfo`, `Message`).
- JSON strings for testing serialization/deserialization.

## Coverage

**Requirements:** None enforced.

## Test Types

**Unit Tests:**
- Test individual functions or small components (e.g., ANSI stripping in `crates/vibe-core/src/os/shell.rs`).

**Integration Tests (In-file):**
- Located in `crates/vibe-core/src/ipc/server.rs` and `crates/vibe-core/src/ipc/client.rs`.
- Test the full handshake and message exchange between Master and Worker.
- Test server lifecycle including idle timeout and socket cleanup.

**State Persistence Tests:**
- Verify that `StateStore` correctly writes and reads JSON files.
- Verify atomic write behavior (checking for `.tmp` files during/after failure if possible).

## Common Patterns

**Async Testing:**
- Always use `tokio::time::sleep` sparingly; prefer waiting for specific state changes if possible.

**Error Testing:**
- Verify that incorrect messages or connection failures result in expected `VibeError` variants.

---

*Testing analysis: 2024-04-18*
