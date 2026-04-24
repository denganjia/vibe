---
phase: 27-mcp-integration
reviewed: 2025-03-24T16:20:00Z
depth: standard
files_reviewed: 10
files_reviewed_list:
  - plugin/vibe/package.json
  - plugin/vibe/mcp-server.js
  - plugin/vibe/gemini-extension.json
  - plugin/vibe/.claude-plugin/plugin.json
  - plugin/vibe/.codex-plugin/plugin.json
  - plugin/vibe/scripts/test-mcp.js
  - plugin/vibe/scripts/init.js
  - plugin/vibe/scripts/plan.js
  - plugin/vibe/scripts/review-task.js
  - plugin/vibe/scripts/review.js
findings:
  critical: 2
  warning: 3
  info: 3
  total: 8
status: issues_found
---

# Phase 27: Code Review Report (MCP Server Integration)

**Reviewed:** 2025-03-24
**Depth:** standard
**Files Reviewed:** 10
**Status:** issues_found

## Summary

The MCP server implementation successfully provides a standardized interface for Vibe workspace operations. It correctly uses the `@modelcontextprotocol/sdk` and implements dynamic skill loading based on `SKILL.md` metadata. The integration with existing scripts via `runSkill` exports is well-structured.

However, several **Critical** security issues related to path traversal were identified in the helper scripts. These must be addressed before deployment, as an LLM could be manipulated into reading or writing arbitrary files on the host system. Additionally, the tool output capturing mechanism in `mcp-server.js` is not thread-safe and may cause issues under concurrent execution.

## Critical Issues

### CR-01: Path Traversal via Task ID and Run ID

**File:** `plugin/vibe/scripts/plan.js:46`, `plugin/vibe/scripts/review.js:41`, `plugin/vibe/scripts/review-task.js:38`
**Issue:** User-provided `taskId` and `runId` are used directly in `path.join` to construct file paths. An attacker (or misbehaving LLM) could provide values like `../../etc/passwd` to write or read files outside the intended `.vibe` directory.
**Fix:**
Sanitize `taskId` and `runId` to ensure they only contain safe characters.
```javascript
function sanitizeId(id) {
  return id.replace(/[^a-zA-Z0-9_-]/g, '');
}

// In plan.js
const safeId = sanitizeId(task.id);
fs.writeFileSync(path.join(tasksDir, `${safeId}.json`), ...);
```

### CR-02: Path Traversal via `targetDir` in `initWorkspace`

**File:** `plugin/vibe/scripts/init.js:28`
**Issue:** `initWorkspace` resolves `targetDir` (from user input) against `workspaceRoot`. Since `targetDir` is not validated, it can be an absolute path or contain `..`, allowing initialization of a `.vibe` directory anywhere on the system.
**Fix:**
Verify that the resolved target is within the intended root.
```javascript
function initWorkspace(targetDir, force, workspaceRoot = process.cwd()) {
  const resolvedTarget = path.resolve(workspaceRoot, targetDir);
  if (!resolvedTarget.startsWith(workspaceRoot)) {
    throw new Error("Target directory must be within the workspace root.");
  }
  // ...
}
```

## Warnings

### WR-01: Race Condition in Tool Output Capture

**File:** `plugin/vibe/mcp-server.js:231-236`
**Issue:** The `vibe_list_tasks` tool globally monkey-patches `console.error` to capture output from `listTasks()`. Since MCP tools can be executed concurrently by the server, multiple overlapping calls will result in interleaved or corrupted output.
**Fix:**
Refactor `listTasks` to return a string or accept a custom output stream.
```javascript
// In status.js
function listTasks(workspaceRoot, logger = console.log) {
  // Use logger() instead of console.log()
}

// In mcp-server.js
let output = "";
listTasks(process.cwd(), (msg) => { output += msg + "\n"; });
```

### WR-02: Inconsistent Tool Name in Test Script

**File:** `plugin/vibe/scripts/test-mcp.js:189`
**Issue:** The test script calls `vibe_skill_vibe_init`, but the server's registration logic (stripping `vibe-` prefix) results in `vibe_skill_init` for the `vibe-init` skill. This will cause the test to fail.
**Fix:**
Update `test-mcp.js` to call the correct tool name:
```javascript
// Change from:
const initRes = await callTool("vibe_skill_vibe_init", ...);
// To:
const initRes = await callTool("vibe_skill_init", ...);
```

### WR-03: Incomplete Path Restriction Check

**File:** `plugin/vibe/mcp-server.js:77-83`
**Issue:** `root.startsWith(cwd)` is a weak check that can be bypassed by case-insensitive filesystems (Windows) or symlinks.
**Fix:**
Use `path.relative` to check if the path is outside the base directory.
```javascript
const relative = path.relative(cwd, root);
const isOutside = relative.startsWith('..') || path.isAbsolute(relative);
if (isOutside) {
  // Error...
}
```

## Info

### IN-01: Redundant `require.cache` Deletion

**File:** `plugin/vibe/mcp-server.js:86`
**Issue:** `delete require.cache[...]` is called on every dynamic tool execution. While useful for development to pick up script changes, it adds overhead in a production-like environment. Consider enabling this only in a development mode.

### IN-02: Weak Schema for Skill Parameters

**File:** `plugin/vibe/mcp-server.js:63`
**Issue:** Dynamic tools use `z.any()` for parameters. This deprives the LLM of structured documentation about the expected inputs for each skill.
**Fix:** In a future version, consider parsing the `Inputs` section of `SKILL.md` to generate more specific Zod schemas.

### IN-03: `review.js` Not Registered as Tool

**File:** `plugin/vibe/scripts/review.js`
**Issue:** Unlike `plan.js` or `init.js`, `review.js` has no corresponding folder in `skills/`, so it isn't registered as a standalone MCP tool. It is correctly utilized as a library by `review-task.js`. This appears intentional but is worth noting for consistency.

---

_Reviewed: 2025-03-24T16:20:00Z_
_Reviewer: gsd-code-reviewer_
_Depth: standard_
