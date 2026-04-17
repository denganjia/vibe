# Coding Conventions

**Analysis Date:** 2024-04-18

## Naming Patterns

**Files:**
- snake_case for all Rust source files (e.g., `error.rs`, `protocol.rs`).

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
- None.

## Error Handling

**Patterns:**
- Use `thiserror` for defining custom error types in `crates/vibe-core/src/error.rs`.
- Propagate errors using the `?` operator.
- Avoid `unwrap()` and `expect()` in core logic; use `Result` instead.

**Common Errors:**
- `VibeError::Internal(String)`: Unexpected internal failures.
- `VibeError::Io(std::io::Error)`: File/socket operations.
- `VibeError::Serialization(serde_json::Error)`: State persistence issues.
- `VibeError::Codec(tokio_util::codec::LinesCodecError)`: IPC framing issues.

## State Persistence Pattern (JSON)

**Pattern:**
- State is persisted as JSON in `.vibe/state/panes.json`.
- Managed by `StateStore` in `crates/vibe-core/src/state/mod.rs`.

**Key Characteristics:**
- **In-memory Cache:** `StateStore` holds an `Arc<Mutex<HashMap<VibeID, PaneRecord>>>`.
- **Atomic Writes:** Saves are performed by writing to a `.tmp` file and then renaming it to the target path to prevent corruption.
- **Serialization:** Uses `serde` and `serde_json` with pretty-printing for human readability.

**Implementation Example:**
```rust
fn save(&self) -> Result<()> {
    let panes = self.panes.lock().unwrap();
    let content = serde_json::to_string_pretty(&*panes)?;
    
    // Atomic write
    let tmp_file = self.state_file.with_extension("tmp");
    fs::write(&tmp_file, content)?;
    fs::rename(tmp_file, &self.state_file)?;
    
    Ok(())
}
```

## Logging

**Framework:** `println!` and `eprintln!`.

**Patterns:**
- `println!` for informational output in the CLI and worker logs.
- `eprintln!` for error reporting and debugging.
- Log files are stored in `.vibe/logs/` named by `{vibe_id}.log`.

## Cross-Platform Design

**Patterns:**
- Use `#[cfg(windows)]` and `#[cfg(not(windows))]` for OS-specific implementations (e.g., in `crates/vibe-core/src/os/mod.rs`).
- Abstract shell differences using a `ShellAdapter` (e.g., in `crates/vibe-core/src/os/shell.rs`).
- Use `PathBuf` for all file paths to ensure cross-platform compatibility.

## Module Design

**Exports:**
- `pub` for public interface.
- Use `pub(crate)` for internal library visibility.

**Organization:**
- `mod.rs` for organizing exports within a directory.
- `lib.rs` for the crate's entry point and public API definition.

---

*Convention analysis: 2024-04-18*
