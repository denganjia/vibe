# Technology Stack

**Analysis Date:** 2025-02-12

## Languages

**Primary:**
- JavaScript / Node.js - Core logic, MCP server implementation, and plugin execution.

**Secondary:**
- Bash - Installation and E2E testing scripts (`scripts/install.sh`, `scripts/e2e_test.sh`).
- PowerShell - Windows installation script (`scripts/install.ps1`).

## Runtime

**Environment:**
- Node.js - Runtime for MCP server and plugins.

**Package Manager:**
- npm - Standard Node.js package manager.
- Lockfile: `package-lock.json` present in `plugin/vibe/`.

## Frameworks

**Core:**
- MCP SDK (Model Context Protocol) - Standardized server integration for AI terminals.
- Zod - Schema validation for MCP tool inputs and outputs.

**Testing:**
- Custom E2E - Shell-based testing in `scripts/e2e_test.sh`.
- Node-based test runners for deterministic plugin manifests verification.

## Key Dependencies

**Critical:**
- `@modelcontextprotocol/sdk` - Implements the MCP server protocol.
- `zod` - Runtime type validation for JSON structures and MCP tool parameters.

**Infrastructure:**
- File-based state management (`.vibe/`) for tasks and locks.

## Configuration

**Environment:**
- Configuration files in `.vibe/`.

**Build:**
- `package.json` (Plugin and manifest definitions).

## Platform Requirements

**Development:**
- Node.js Environment.
- Compatible AI Terminals (Gemini CLI, Claude Code, Codex CLI).

**Production:**
- Cross-platform support via Node.js runtime.

---

*Stack analysis: Updated for MCP plugin-first architecture*