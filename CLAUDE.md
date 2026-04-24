<!-- GSD:project-start source:PROJECT.md -->
## Project

**vibe-cli**

`vibe-cli` 是一个 plugin-first 的多模型协作协议与轻量运行时，基于 Node.js 和 MCP (Model Context Protocol) 构建。它将 AI Agent 转化为能够自主操控任务协作、共享上下文并实现任务闭环的“终端虚拟操作员”，让开发者通过指挥 AI 团队在真实的窗口与文件系统中“并联作业”。

**Core Value:** 打破 AI 与本地开发环境之间的“次元壁”，将当前 AI 终端模型升级为分布式 AI 协作的调度室，通过标准 MCP 工具开箱即用。

### Constraints

- **Tech Stack**: Node.js & MCP SDK — 确保作为标准化 MCP 插件的跨端兼容性和生态集成。
- **Architecture**: Plugin-first & MCP — 摒弃直接的 shell 注入，所有工作区交互均通过标准化的 MCP Tools 暴露。
- **Environment**: Cross-platform — 支持运行 Node.js 的所有主流操作系统。
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- JavaScript / Node.js - Core logic, MCP server implementation
- None detected
## Runtime
- Node.js
- npm
- Lockfile: package-lock.json in plugin/vibe/
## Frameworks
- MCP SDK (Model Context Protocol)
- Zod
- Node-based test runners
## Key Dependencies
- @modelcontextprotocol/sdk
- zod
## Configuration
- .vibe/ directory
- package.json
## Platform Requirements
- Node.js Environment
- Compatible AI Terminals
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Patterns
- camelCase or kebab-case for file names.
- camelCase for function and variable names.
- PascalCase for classes and Zod schemas.
## Code Style
- Prettier/ESLint for formatting and linting.
## Import Organization
- ES Modules (`import`/`export`).
## Error Handling
- try/catch blocks with custom error classes.
## Logging
- `console.log`/`console.error` for output, MCP logging utilities.
## Comments
- JSDoc for complex logic.
## Function Design
- Small, focused tools exposed via MCP.
## Module Design
- Encapsulated logic per MCP tool.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- MCP Plugin Architecture
- Tool-based action exposure
## Layers
- Purpose: MCP Server for AI Terminals
- Location: `plugin/vibe/mcp-server.js`
- Depends on: `@modelcontextprotocol/sdk`
- Used by: AI Terminals (Gemini, Claude, Codex)
## Data Flow
- Standard MCP protocol (JSON-RPC over stdio)
## Key Abstractions
- Tools for workspace operations
## Entry Points
- Location: `plugin/vibe/mcp-server.js`
- Triggers: Started by host AI CLI
## Error Handling
- MCP standardized error responses
## Cross-Cutting Concerns
- Authentication and authorization via host CLI
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->
## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, or `.github/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
