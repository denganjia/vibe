# Coding Conventions

**Analysis Date:** 2024-03-20

## Naming Patterns

**Files:**
- snake_case for all Rust source files (e.g., `error.rs`, `db.rs`).

**Functions:**
- snake_case for function names (standard Rust).

**Variables:**
- snake_case for variable names.

**Types:**
- PascalCase for structs, enums, and traits (standard Rust).
- `VibeError`: Central error type for the codebase.
- `Result<T>`: Type alias for `std::result::Result<T, VibeError>`.

## Code Style

**Formatting:**
- `rustfmt` (standard).

**Linting:**
- `clippy` (standard).

## Import Organization

**Order:**
1. Standard library imports (`std::*`).
2. External dependency imports (e.g., `tokio::*`, `serde::*`).
3. Internal module imports (`crate::*`).

**Path Aliases:**
- None detected.

## Error Handling

**Patterns:**
- Use `thiserror` for defining custom error types in `crates/vibe-core/src/error.rs`.
- Propagate errors using the `?` operator.
- Avoid `unwrap()` and `expect()` in core logic; use `Result` instead.

**Common Errors:**
- `VibeError::Internal(String)` for unexpected internal failures.
- `VibeError::Io(std::io::Error)` for file/socket operations.
- `VibeError::Database(rusqlite::Error)` for persistence issues.

## Logging

**Framework:** `println!` and `eprintln!`

**Patterns:**
- `println!` for informational output in the CLI.
- `eprintln!` for error reporting and debugging.

## Concurrency Pattern (Serialized DB Actor)

**Purpose:**
- To provide thread-safe access to non-thread-safe resources (like SQLite) from multiple async tasks.

**Implementation:**
- **Request Enum:** Define a `DbRequest` enum in `crates/vibe-core/src/state/db.rs` where each variant includes a `oneshot::Sender<Result<T>>` for the response.
- **Actor:** A `DbActor` struct that holds the resource and an `mpsc::Receiver<DbRequest>`. It processes requests sequentially in a loop.
- **Handle:** A `DbHandle` struct that wraps an `mpsc::Sender<DbRequest>`. It is cloneable and provides `async` methods that encapsulate creating a `oneshot` channel and waiting for the response.

**Example:**
```rust
pub async fn register_pane(&self, info: RegisterInfo) -> Result<()> {
    let (tx, rx) = oneshot::channel();
    self.sender.send(DbRequest::RegisterPane(info, tx)).await
        .map_err(|e| VibeError::Internal(format!("Send error: {}", e)))?;
    rx.await.map_err(|e| VibeError::Internal(format!("Recv error: {}", e)))?
}
```

## Module Design

**Exports:**
- `pub` for public interface.
- Use `pub(crate)` for internal library visibility.

**Barrel Files:**
- `mod.rs` for organizing exports within a directory.
- `lib.rs` for the crate's entry point and public API definition.

---

*Convention analysis: 2024-03-20*
