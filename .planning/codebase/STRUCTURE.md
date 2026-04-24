# Codebase Structure

**Analysis Date:** 2024-10-24

## Directory Layout

```
vibe-cli/
├── plugin/
│   └── vibe/               # Active Core: MCP Server and Scripts
│       ├── mcp-server.js   # MCP Server Entry Point
│       ├── package.json    # Node.js dependencies
│       └── scripts/        # Core logic scripts (Node.js)
├── .vibe/                  # Project-local runtime data
│   ├── config.json         # Project Config
│   ├── roles/              # Role templates (Markdown, Conductor/Worker SOPs)
│   ├── tasks/              # Generated JSON tasks
│   └── locks/              # Resource locks
├── apps/                   # [Archive/Legacy] Old Rust CLI Application
├── crates/                 # [Archive/Legacy] Old Rust Core Library
└── scripts/                # Utility scripts
```

## Directory Purposes

**plugin/vibe/:**
- Purpose: The active core of the Vibe project. Implements the MCP server and underlying logic.
- Key files: 
  - `plugin/vibe/mcp-server.js`: Exposes MCP tools.
  - `plugin/vibe/scripts/`: Implementations for tasks, locks, and summaries.

**.vibe/:**
- Purpose: Project-specific configuration and runtime state.
- Contains: Task JSONs, lock files, and Markdown persona templates.

**apps/ & crates/:**
- Purpose: Archived legacy Rust implementation of the original vibe-cli.

## Key File Locations

**Entry Points:**
- `plugin/vibe/mcp-server.js`: MCP Server entry point.

**Testing:**
- `plugin/vibe/scripts/test-mcp.js`: Basic tests for the MCP server.

## Special Directories

**.vibe/:**
- Purpose: Stores project-local state, roles, tasks, and locks.
- Committed: `config.json` and `roles/` should be committed; `tasks/` and `locks/` might be ignored depending on project needs.

---

*Structure analysis: Updated for Phase 28*