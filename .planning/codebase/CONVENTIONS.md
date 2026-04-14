# Coding Conventions

**Analysis Date:** 2024-03-20

## Naming Patterns

**Files:**
- snake_case for all Rust source files.

**Functions:**
- snake_case for function names (standard Rust).

**Variables:**
- snake_case for variable names.

**Types:**
- PascalCase for structs, enums, and traits (standard Rust).

## Code Style

**Formatting:**
- `rustfmt` (standard).

**Linting:**
- `clippy` (recommended standard).

## Import Organization

**Order:**
1. Standard library imports.
2. External dependency imports.
3. Internal module imports.

**Path Aliases:**
- None detected.

## Error Handling

**Patterns:**
- Standard Rust Result and Option types.

## Logging

**Framework:** Standard Output

**Patterns:**
- `println!` for CLI output.

## Comments

**When to Comment:**
- Complexity documentation.

**JSDoc/TSDoc:**
- Rust doc comments (`///` and `//!`) for API documentation.

## Function Design

**Size:** Concise.

**Parameters:** Prefer passing references.

**Return Values:** Prefer `Result<T, E>` for operations that can fail.

## Module Design

**Exports:** `pub` for public interface.

**Barrel Files:** `mod.rs` or named modules for organizing exports.

---

*Convention analysis: 2024-03-20*
