---
phase: 25-universal-manifests-packaging
reviewed: 2026-04-24T03:57:42Z
depth: standard
files_reviewed: 4
files_reviewed_list:
  - plugin/vibe/package.json
  - plugin/vibe/gemini-extension.json
  - plugin/vibe/.claude-plugin/plugin.json
  - plugin/vibe/scripts/manifests.test.js
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues_found
---

# Phase 25: Code Review Report

**Reviewed:** 2026-04-24T03:57:42Z
**Depth:** standard
**Files Reviewed:** 4
**Status:** issues_found

## Summary

本次 review 对照了 phase 25 的 PLAN / CONTEXT / RESEARCH / VALIDATION，以及 `.agents/plugins/marketplace.json` 与 `plugin/vibe/.codex-plugin/plugin.json` 的既有契约。当前四个目标文件本身都能通过 `node --test plugin/vibe/scripts/manifests.test.js` 和 `npm test`，manifest 当前快照也与计划要求基本一致。

问题集中在 `plugin/vibe/scripts/manifests.test.js` 的防回归强度不足：它证明了“现在是对的”，但还没有完整锁住 phase 25 明确要求的 package publish/dependency guardrails，以及 marketplace 必须保持 repo-local-only 的约束。这会让后续 drift 以绿测形式溜过。

## Warnings

### WR-01: smoke test 没有锁住 package publish/dependency guardrails

**File:** `plugin/vibe/scripts/manifests.test.js:47-53`
**Issue:** 这里目前只检查了 `mcpServers` 不存在，以及 `@modelcontextprotocol/sdk` 没有出现在 `dependencies` / `devDependencies` 中。但 phase 25 的 acceptance criteria 还要求 `plugin/vibe/package.json` 不得出现 `dependencies`、`devDependencies`、`bin`、`publishConfig`。按照当前测试，如果后续有人加入任意非 MCP 依赖，或补上 `publishConfig` / `bin`，测试仍会通过，形成 false positive，并削弱“repo-local、non-publishing、Phase 27 前零 runtime wiring”的 guardrail。
**Fix:**
```js
assert.ok(!('dependencies' in pkg), 'package.json should not declare dependencies in Phase 25');
assert.ok(!('devDependencies' in pkg), 'package.json should not declare devDependencies in Phase 25');
assert.ok(!('bin' in pkg), 'package.json should not expose a bin entry yet');
assert.ok(!('publishConfig' in pkg), 'package.json should not include publishConfig');
```

### WR-02: marketplace 测试只验证存在一个正确本地条目，没有验证文件整体仍是 local-only 合约

**File:** `plugin/vibe/scripts/manifests.test.js:83-94`
**Issue:** 这段测试只查找 `name === "vibe"` 的条目并验证该条目是 `local` 且路径正确，但没有约束 `marketplace.json` 整体仍是 phase 25 计划要求的 repo-local discovery example。后续如果有人再加一个 `remote` source 条目、额外 marketplace publication metadata，或者把 `plugins` 扩成混合来源列表，当前测试仍可能保持绿色，无法捕获 D-09 / Task 3 明确禁止的 drift。
**Fix:**
```js
assert.deepEqual(Object.keys(marketplace).sort(), ['interface', 'name', 'plugins']);
assert.equal(marketplace.plugins.length, 1, 'marketplace should stay a single local example in Phase 25');
assert.deepEqual(marketplace.plugins[0], {
  name: 'vibe',
  source: { source: 'local', path: './plugin/vibe' },
  policy: { installation: 'AVAILABLE', authentication: 'ON_INSTALL' },
  category: 'Productivity'
});
```

---

_Reviewed: 2026-04-24T03:57:42Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
