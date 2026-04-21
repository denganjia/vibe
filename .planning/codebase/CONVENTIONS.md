# Coding Conventions

**Analysis Date:** 2025-02-18

## Naming Patterns

**Files:**
- Snake case for Rust source files (e.g., `crates/vibe-core/src/adapter/wezterm.rs`).
- `mod.rs` for module entry points.

**Functions:**
- Snake case: `spawn`, `send_keys`, `get_metadata`.

**Variables:**
- Snake case: `pane_id`, `current_dir`.

**Types:**
- Pascal case for Structs/Enums: `TerminalAdapter`, `TmuxAdapter`.
- Type aliases: `VibeID = String`.

## Code Style

**Formatting:**
- Default `rustfmt` (standard Rust style).
- Configuration: Standard defaults (no custom `rustfmt.toml` detected).

**Linting:**
- Standard `clippy` checks.

## Import Organization

**Order:**
1. Standard library (`std::...`)
2. Internal crate modules (`crate::...`)
3. External dependencies (`serde::...`)

**Path Aliases:**
- Standard Rust `crate::` paths are used.

## Error Handling

**Patterns:**
- Custom `VibeError` enum in `crates/vibe-core/src/error.rs` using `thiserror`.
- `Result<T>` type alias for `std::result::Result<T, VibeError>`.
- Use `?` for error propagation.

**Autonomous Self-Healing (A-D-E-V Verify Phase):**
- Workers MUST attempt to automatically fix verification/test failures up to 3 times before escalating.
- If 3 retries fail, the agent MUST emit a `BLOCKED` status via `vibe report --status blocked`.

## Logging

**Framework:** `println!` for CLI output; `vibe report` for agent status.

**Patterns:**
- Raw stdout is for CLI users.
- Agents MUST use `vibe report --status <STATUS> --message <MSG>` at every milestone to update global state (`.vibe/state/panes.json`).

## Comments

**When to Comment:**
- Use Intent Locking via `vibe report --status blocked --message "writing:path/to/file"` (A-D-E-V Declare phase) instead of standard comments to coordinate multi-agent file modifications.

**JSDoc/TSDoc:**
- Not applicable (Rust codebase). Use standard Rustdoc `///` for public APIs.

## Function Design

**Size:** Keep adapter functions small and focused on one specific CLI interaction.

**Parameters:** Prefer passing context (like `VibeID` or environment maps) explicitly.

**Return Values:** Always return `Result<T>` for fallible terminal operations.

## Module Design

**Exports:**
- Core traits and types in `crates/vibe-core/src/adapter/mod.rs`.
- Concrete implementations in submodules.

**Barrel Files:**
- `crates/vibe-core/src/adapter/mod.rs` re-exports common adapters for ease of use.

## Autonomous Workflow (A-D-E-V)

**Analyze-Declare-Execute-Verify Loop:**
- **Analyze:** Read target files and context documents before taking action.
- **Declare:** State intent locks before modification (`vibe report --status blocked --message "writing:<file>"`).
- **Execute:** Apply code modifications.
- **Verify:** Run verification (e.g., `cargo test`). If failed, apply Self-Healing (max 3 retries). On success, signal completion (`vibe signal task_done`).