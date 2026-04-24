# Architecture

**Analysis Date:** 2024-10-24

## Pattern Overview

**Overall:** Plugin-first MCP Server with AI Agent Integration

**Key Characteristics:**
- **Terminal & Language Agnostic:** The core interactions are exposed via the Model Context Protocol (MCP), meaning any compliant AI agent can use the tools regardless of the underlying terminal.
- **Stateless Tooling:** The MCP server provides atomic tools for task management, locking, and execution.
- **Environment-Injected Identity:** Tasks and context are managed within a project-local `.vibe/` directory.

## Layers

**MCP Server Layer:**
- Purpose: Exposes Vibe functionalities as standard MCP tools (`vibe_create_task`, `vibe_get_status`, `vibe_list_tasks`, `vibe_acquire_lock`, `vibe_release_lock`).
- Location: `plugin/vibe/mcp-server.js`
- Depends on: `@modelcontextprotocol/sdk`
- Used by: AI Agents (Conductor, Worker)

**Script Layer:**
- Purpose: Implements the core logic for managing tasks, state, and locks.
- Location: `plugin/vibe/scripts/`
- Contains: Task planning, review, execution, and locking logic in Node.js.
- Used by: The MCP Server

**State Management:**
- Purpose: Persistence of task definitions, statuses, and locks.
- Location: `.vibe/` (Project root)
- Contains: 
  - `.vibe/tasks/`: JSON task definitions
  - `.vibe/locks/`: Lock files for resources
  - `.vibe/roles/`: Agent SOPs (Conductor, Worker)

## Data Flow

**Task Spawning Flow:**

1. AI Agent (Conductor) uses the `vibe_create_task` MCP tool.
2. The MCP server processes the request and calls the underlying script.
3. A JSON task file is created in `.vibe/tasks/`.

**Agent Execution & Locking:**

1. AI Agent (Worker) uses `vibe_acquire_lock` before modifying files.
2. The MCP server creates a lock in `.vibe/locks/`.
3. Worker executes changes and runs tests.
4. Worker uses `vibe_release_lock` to free the resource.
5. Worker uses `vibe_get_status` or other tools to report completion.

**State Management:**
- Handled via file-based JSON and lock files in the `.vibe/` directory.
- Ensuring atomic operations through file locking mechanisms in the scripts.

## Key Abstractions

**MCP Tools:**
- Purpose: Standardized interface for AI models to interact with the Vibe ecosystem.

**Task Definitions:**
- Purpose: JSON-based declaration of work, tracking status and results.

## Legacy Components
- The Rust implementations in `crates/vibe-core` and `apps/vibe-cli` are deprecated and maintained for historical reference (Archive/Legacy). The system is now fully driven by the `plugin/vibe/` architecture.

---

*Architecture analysis: Updated for Phase 28*