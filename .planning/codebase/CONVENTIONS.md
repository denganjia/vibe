# Coding Conventions

**Analysis Date:** 2025-01-24

## Naming Patterns

**Files:**
- Snake case for Rust source files: `tmux.rs`, `wezterm.rs`.
- `mod.rs` for module entry points.

**Functions:**
- Snake case: `spawn`, `send_keys`, `get_metadata`.

**Variables:**
- Snake case: `pane_id`, `current_dir`.

**Types:**
- Pascal case for Structs/Enums: `TerminalAdapter`, `TmuxAdapter`, `WindowTarget`.
- Type aliases: `VibeID = String`.

## Code Style

**Formatting:**
- Default `rustfmt` (standard Rust style).

**Linting:**
- Standard `clippy` checks.

## Import Organization

**Order:**
1. Standard library (`std::...`)
2. Internal crate modules (`crate::...`)
3. External dependencies (`serde::...`)

## Error Handling

**Patterns:**
- Custom `VibeError` enum in `crates/vibe-core/src/error.rs` using `thiserror`.
- `Result<T>` type alias for `std::result::Result<T, VibeError>`.
- Use `?` for error propagation.
- For CLI failures, include `stderr` in the error message for debugging.

## Logging

**Framework:** `println!` for CLI output; custom logging not yet fully implemented for core.

## New Adapter Guidelines

**Implementation:**
- Must implement the `TerminalAdapter` trait in `crates/vibe-core/src/adapter/mod.rs`.
- Use `std::process::Command` to invoke the terminal multiplexer's CLI.
- Prefer machine-readable output formats (e.g., JSON) from CLI if available (see `wezterm.rs`).
- Fallback to string parsing (trim/split) for simple outputs (see `tmux.rs`).

**Environment Detection:**
- Use environment variables (e.g., `TMUX_PANE`, `WEZTERM_PANE`) to detect the current context.
- Provide meaningful error messages when detection fails.

**Command Execution:**
- When spawning, use `exec bash` or similar to keep the new pane/window alive after the initial command finishes.
- Use `env` command or adapter-specific flags to pass environment variables.

## Module Design

**Exports:**
- Core traits and types in `crates/vibe-core/src/adapter/mod.rs`.
- Concrete implementations in submodules.

**Barrel Files:**
- `crates/vibe-core/src/adapter/mod.rs` re-exports common adapters for ease of use.

---

*Convention analysis: 2025-01-24*
