# Phase 27: MCP Server Integration - Research

**Researched:** 2025-03-24
**Domain:** Model Context Protocol (MCP), Node.js, Workspace Security
**Confidence:** HIGH

## Summary

Phase 27 旨在将当前通过 `run_shell_command` 调用的原始 Node.js 脚本转换为标准的 Model Context Protocol (MCP) 工具调用。通过实现一个轻量级的 MCP 服务器，我们可以为 Agent 提供更安全、更具结构化且跨平台兼容的工具接口，同时减少 Agent 直接操作 Shell 的安全风险。

**Primary recommendation:** 在 `plugin/vibe/` 目录下实现一个 `mcp-server.js`，集成现有的任务管理、锁定和发布总结逻辑，并在各平台清单文件中声明该 MCP 服务器。

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `@modelcontextprotocol/sdk` | ^1.29.0 | MCP 服务器核心 SDK | Anthropic 发布的官方标准实现 [VERIFIED: npm registry] |
| `zod` | ^3.24.0 | Schema 验证和类型定义 | MCP SDK 强依赖 Zod 进行工具输入校验 [VERIFIED: npm registry] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|--------------|
| `js-yaml` | ^4.1.1 | 解析 YAML 格式的 SKILL 定义 | 当需要从现有 SKILL.md 动态生成工具描述时 |

**Installation:**
```bash
cd plugin/vibe
npm install @modelcontextprotocol/sdk zod
```

## Architecture Patterns

### Recommended Project Structure
```
plugin/vibe/
├── mcp-server.js         # MCP 服务器入口 (Stdio 传输)
├── scripts/              # 逻辑实现层 (现有脚本)
│   ├── lock.js           # 锁定逻辑
│   ├── task.js           # 任务逻辑
│   └── ...
├── skills/               # 声明层 (提供工具描述)
└── package.json          # 依赖管理
```

### Pattern 1: Tool Encapsulation
将现有脚本中的函数封装为 MCP Tool。
```javascript
// Source: [CITED: modelcontextprotocol.io]
const { McpServer } = require("@modelcontextprotocol/sdk/server/mcp.js");
const { StdioServerTransport } = require("@modelcontextprotocol/sdk/server/stdio.js");
const { z } = require("zod");
const { createTask } = require("./scripts/task");

const server = new McpServer({ name: "vibe", version: "0.1.0" });

server.tool(
  "vibe_create_task",
  {
    goal: z.string().describe("Task goal"),
    file_scope: z.array(z.string()).optional()
  },
  async (args) => {
    const task = createTask(process.cwd(), args);
    return { content: [{ type: "text", text: JSON.stringify(task) }] };
  }
);
```

### Anti-Patterns to Avoid
- **Mixed Output**: 在 MCP 服务器中向 `stdout` 打印调试信息。这会破坏 Stdio 协议。所有日志必须使用 `console.error`。
- **Unbounded Paths**: 允许工具访问工作区以外的路径。必须在工具入口进行路径验证。

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Tool Schema | Manual JSON Schema | `zod` | 类型安全且与 SDK 深度集成 |
| Protocol Handoff | Manual JSON-RPC | `@modelcontextprotocol/sdk` | 确保与 Claude Desktop 等客户端的兼容性 |

## Runtime State Inventory

*本阶段为重构/集成阶段，涉及运行时的变更。*

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | None | 现有 `.vibe/` 数据格式保持不变 |
| Live service config | Agent manifest files | 需要更新以声明 MCP 服务器 |
| OS-registered state | None | - |
| Secrets/env vars | None | - |
| Build artifacts | `plugin/vibe/node_modules` | 需要安装新依赖 |

## Common Pitfalls

### Pitfall 1: Stdio Stream Corruption
**What goes wrong:** `console.log` 在非工具返回中使用。
**Why it happens:** 习惯性使用 log 进行调试，但 MCP 使用 stdout 传输 JSON-RPC。
**How to avoid:** 全局替换 `console.log` 为 `console.error` 或专门的日志库。

### Pitfall 2: Async Initialization
**What goes wrong:** 服务器在连接传输层前就收到请求。
**Why it happens:** 未正确 `await server.connect(transport)`。
**How to avoid:** 在 `main` 函数中严格控制启动顺序。

## Code Examples

### Tool Mapping (Scripts to MCP)
| Existing Script | MCP Tool Name | Arguments |
|-----------------|---------------|-----------|
| `lock.js` | `vibe_acquire_lock` | `taskId: string`, `paths: string[]` |
| `lock.js` | `vibe_release_lock` | `taskId: string` |
| `task.js` | `vibe_create_task` | `goal: string`, `file_scope?: string[]` |
| `status.js` | `vibe_get_status` | `taskId?: string` |
| `release-summary.js` | `vibe_gen_release_summary` | `from?: string`, `to?: string` |

### Manifest Registration (Gemini Extension)
```json
// plugin/vibe/gemini-extension.json
{
  "mcpServers": {
    "vibe": {
      "command": "node",
      "args": ["${extensionPath}/mcp-server.js"]
    }
  }
}
```

### Manifest Registration (Claude Plugin)
```json
// plugin/vibe/.claude-plugin/plugin.json
{
  "mcpServers": {
    "vibe": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/mcp-server.js"]
    }
  }
}
```

## Security Domain

### Workspace Restriction Pattern
所有工具必须强制校验 `workspaceRoot`：
```javascript
function validatePath(workspaceRoot, targetPath) {
  const absolute = path.resolve(workspaceRoot, targetPath);
  if (!absolute.startsWith(path.resolve(workspaceRoot))) {
    throw new Error("Access denied: path outside workspace");
  }
  return absolute;
}
```

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | Runtime | ✓ | 22.12.0 | - |
| npm | Dependencies | ✓ | 10.9.1 | - |

## Sources

### Primary (HIGH confidence)
- `@modelcontextprotocol/sdk` docs - Tool registration patterns.
- Gemini Extension Specification - `mcpServers` field in `gemini-extension.json`.
- Claude Code Plugin Docs - `.claude-plugin/plugin.json` structure.

### Secondary (MEDIUM confidence)
- Codex Plugin samples - `capabilities.mcpServers` format.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Official SDK used.
- Architecture: HIGH - Follows standard MCP patterns.
- Pitfalls: HIGH - Common Stdio issues well documented.

**Research date:** 2025-03-24
**Valid until:** 2025-06-24
