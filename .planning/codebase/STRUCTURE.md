# Codebase Structure

**Analysis Date:** 2024-03-20

## Directory Layout

```
vibe-cli/
├── src/            # Source code
│   └── main.rs     # CLI Entry point
├── Cargo.toml      # Build and dependency manifest
└── .gitignore      # Git ignore patterns
```

## Directory Purposes

**src/:**
- Purpose: Contains all source code for the project.
- Contains: Rust files (.rs).
- Key files: `src/main.rs`.

## Key File Locations

**Entry Points:**
- `src/main.rs`: Primary entry point for the CLI binary.

**Configuration:**
- `Cargo.toml`: Package definition and dependencies.

**Core Logic:**
- `src/main.rs`: Currently contains the entire application logic.

**Testing:**
- Not implemented yet.

## Naming Conventions

**Files:**
- Snake case: `main.rs`.

**Directories:**
- Snake case (expected): `src`.

## Where to Add New Code

**New Feature:**
- Primary code: `src/` directory. Create new modules as needed.
- Tests: Either in-file `#[cfg(test)]` modules or `tests/` directory.

**New Component/Module:**
- Implementation: `src/` directory. Create subdirectories for complex modules.

**Utilities:**
- Shared helpers: `src/utils.rs` or `src/utils/` (not yet present).

## Special Directories

**.planning/:**
- Purpose: Documentation for codebase mapping and project planning.
- Generated: Yes.
- Committed: Yes.

---

*Structure analysis: 2024-03-20*
