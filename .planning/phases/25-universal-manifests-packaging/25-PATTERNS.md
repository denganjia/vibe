# Phase 25: Universal Manifests & Packaging - Pattern Map

**Mapped:** 2026-04-24 11:22:46 CST
**Files analyzed:** 6
**Analogs found:** 6 / 6

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `plugin/vibe/package.json` | config | transform | `plugin/vibe/.codex-plugin/plugin.json` | partial |
| `plugin/vibe/gemini-extension.json` | config | transform | `plugin/vibe/.codex-plugin/plugin.json` | partial |
| `plugin/vibe/.claude-plugin/plugin.json` | config | transform | `plugin/vibe/.codex-plugin/plugin.json` | partial |
| `plugin/vibe/.codex-plugin/plugin.json` | config | transform | `plugin/vibe/.codex-plugin/plugin.json` | exact |
| `.agents/plugins/marketplace.json` | config | file-I/O | `.agents/plugins/marketplace.json` | exact |
| `plugin/vibe/scripts/manifests.test.js` | test | file-I/O | `plugin/vibe/scripts/release-summary.test.js` | role-match |

## Pattern Assignments

### `plugin/vibe/package.json` (config, transform)

**Primary analog:** `plugin/vibe/.codex-plugin/plugin.json`  
**Supporting analogs:** `plugin/vibe/README.md`, `plugin/vibe/scripts/README.md`, `plugin/vibe/scripts/release-summary.test.js`

**Shared identity seed** (`plugin/vibe/.codex-plugin/plugin.json:1-10`):
```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces.",
  "skills": "./skills/",
  "interface": {
    "displayName": "Vibe",
    "shortDescription": "Coordinate AI Agents through project-local tasks, reviews, and logs."
  }
}
```

**Plugin root layout pattern** (`plugin/vibe/README.md:7-15`):
```markdown
- `.codex-plugin/plugin.json` declares the Codex-compatible plugin identity.
- `skills/` will teach the current model to act as the Conductor.
- `commands/` will expose workflow entry contracts.
- `references/` will hold collaboration, task, Agent, review, workspace, and migration contracts.
- `scripts/` holds thin deterministic helpers.
- `templates/` holds `.vibe` workspace scaffolds.
- `examples/` holds model-readable demonstrations of the contracts.
```

**Runtime/script boundary to mirror in `scripts`/`type` fields** (`plugin/vibe/scripts/README.md:16-18`):
```markdown
All scripts are written in pure Node.js using native APIs to ensure maximum portability and minimal dependencies.
They follow the contracts defined in `plugin/vibe/references/`.
```

**Test entrypoint style to mirror in `scripts.test`** (`plugin/vibe/scripts/release-summary.test.js:1-4`):
```js
const test = require('node:test');
const assert = require('node:assert');
const fs = require('node:fs');
const path = require('node:path');
```

**How to apply**
- 把 `name`、`version`、`description` 直接从现有 Codex manifest 上移成 source of truth。
- `type` 应保持 `commonjs`，因为现有 `plugin/vibe/scripts/*.js` 全部是 `require(...)` 风格。
- `scripts` 应至少暴露 `test` 和 `test:manifests`，并把 `scripts/manifests.test.js` 作为唯一 smoke gate。
- 不要引入 `dependencies` / `devDependencies` 里的 MCP SDK；Phase 25 只建立 package surface。

---

### `plugin/vibe/gemini-extension.json` (config, transform)

**Primary analog:** `plugin/vibe/.codex-plugin/plugin.json`  
**Supporting analog:** `plugin/vibe/README.md`

**Minimal identity pattern** (`plugin/vibe/.codex-plugin/plugin.json:1-10`):
```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces.",
  "skills": "./skills/",
  "interface": {
    "displayName": "Vibe",
    "shortDescription": "Coordinate AI Agents through project-local tasks, reviews, and logs."
  }
}
```

**Manifest belongs at plugin root** (`plugin/vibe/README.md:5-15`):
```markdown
## Package Layout

`plugin/vibe/` is organized around model-readable and scriptable plugin surfaces:

- `.codex-plugin/plugin.json` declares the Codex-compatible plugin identity.
- `skills/` will teach the current model to act as the Conductor.
- `commands/` will expose workflow entry contracts.
- `references/` will hold collaboration, task, Agent, review, workspace, and migration contracts.
- `scripts/` holds thin deterministic helpers.
```

**How to apply**
- 直接复用 `name`、`version`、`description` 的值，不复制 Codex 专属的 `skills`、`interface` 字段。
- 这个文件是新增根级 manifest，不要放进隐藏目录。
- 维持最小静态 manifest；不要提前声明 Phase 27 才会出现的 MCP/runtime 字段。

---

### `plugin/vibe/.claude-plugin/plugin.json` (config, transform)

**Primary analog:** `plugin/vibe/.codex-plugin/plugin.json`  
**Supporting analogs:** `plugin/vibe/README.md`, `plugin/vibe/references/plugin-architecture.md`

**Minimal identity pattern** (`plugin/vibe/.codex-plugin/plugin.json:1-10`):
```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces.",
  "skills": "./skills/",
  "interface": {
    "displayName": "Vibe",
    "shortDescription": "Coordinate AI Agents through project-local tasks, reviews, and logs."
  }
}
```

**Plugin package remains the primary product surface** (`plugin/vibe/references/plugin-architecture.md:5-12`):
```markdown
Vibe is a plugin-first collaboration system. The installed plugin teaches the
current AI terminal model to act as the Conductor, gives it model-readable
protocol references, and exposes thin deterministic scripts for filesystem and
subprocess work.

The product surface is the plugin package under `plugin/vibe/`.
```

**Current package layout** (`plugin/vibe/README.md:7-13`):
```markdown
- `.codex-plugin/plugin.json` declares the Codex-compatible plugin identity.
- `skills/` will teach the current model to act as the Conductor.
- `commands/` will expose workflow entry contracts.
- `references/` will hold collaboration, task, Agent, review, workspace, and migration contracts.
- `scripts/` holds thin deterministic helpers.
```

**How to apply**
- 只复制共享 identity 字段，保持 `.claude-plugin/` 内只有 `plugin.json`。
- 不要在这个 phase 里绑定 `skills`、`commands` 等路径；目录标准化属于后续 phase。
- 路径约束如果未来要加，保持 plugin-root 内相对路径，不要使用绝对路径或 `../`。

---

### `plugin/vibe/.codex-plugin/plugin.json` (config, transform)

**Analog:** `plugin/vibe/.codex-plugin/plugin.json`

**Current JSON structure** (`plugin/vibe/.codex-plugin/plugin.json:1-10`):
```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces.",
  "skills": "./skills/",
  "interface": {
    "displayName": "Vibe",
    "shortDescription": "Coordinate AI Agents through project-local tasks, reviews, and logs."
  }
}
```

**Package layout context** (`plugin/vibe/README.md:7-13`):
```markdown
- `.codex-plugin/plugin.json` declares the Codex-compatible plugin identity.
- `skills/` will teach the current model to act as the Conductor.
- `commands/` will expose workflow entry contracts.
- `references/` will hold collaboration, task, Agent, review, workspace, and migration contracts.
- `scripts/` holds thin deterministic helpers.
```

**How to apply**
- 保留现有结构，不要重写成另一套 schema。
- 只做对齐性修改：`name`、`version`、`description` 与新的 `package.json` 完全一致。
- 继续使用 `skills: "./skills/"` 和 `interface.displayName` / `interface.shortDescription` 的现有写法。

---

### `.agents/plugins/marketplace.json` (config, file-I/O)

**Analog:** `.agents/plugins/marketplace.json`

**Current discovery entry** (`.agents/plugins/marketplace.json:1-20`):
```json
{
  "name": "vibe-local",
  "interface": {
    "displayName": "Vibe Local Plugins"
  },
  "plugins": [
    {
      "name": "vibe",
      "source": {
        "source": "local",
        "path": "./plugin/vibe"
      },
      "policy": {
        "installation": "AVAILABLE",
        "authentication": "ON_INSTALL"
      },
      "category": "Productivity"
    }
  ]
}
```

**Plugin root that the local path must continue to target** (`plugin/vibe/references/plugin-architecture.md:10-12`):
```markdown
The product surface is the plugin package under `plugin/vibe/`. The historical
`vibe-cli` Rust workspace remains useful as a compatibility reference during the
migration, but it is no longer the primary user interface.
```

**How to apply**
- 如果当前文件不漂移，优先保持现状。
- 如果需要改，只允许做与 `plugin/vibe/package.json` / `.codex-plugin/plugin.json` 的 identity 对齐。
- `source.path` 必须继续指向 `./plugin/vibe`；这是 smoke test 的核心断言之一。

---

### `plugin/vibe/scripts/manifests.test.js` (test, file-I/O)

**Primary analog:** `plugin/vibe/scripts/release-summary.test.js`  
**Supporting analogs:** `plugin/vibe/scripts/test-runtime.js`, `plugin/vibe/scripts/init.test.js`

**Node.js test harness imports** (`plugin/vibe/scripts/release-summary.test.js:1-9`):
```js
const test = require('node:test');
const assert = require('node:assert');
const fs = require('node:fs');
const path = require('node:path');

// We will implement these in release-summary.js
const releaseSummary = require('./release-summary.js');
```

**Subtest structure pattern** (`plugin/vibe/scripts/release-summary.test.js:31-66`):
```js
test('Task association logic', async (t) => {
  const mockVibeDir = path.join(process.cwd(), '.vibe', 'tasks');
  if (!fs.existsSync(mockVibeDir)) {
    fs.mkdirSync(mockVibeDir, { recursive: true });
  }

  const taskId = 'TEST-123';
  const taskFile = path.join(mockVibeDir, `${taskId}.json`);
  const taskData = { id: taskId, title: 'Test Task', status: 'done' };
  
  fs.writeFileSync(taskFile, JSON.stringify(taskData));

  await t.test('should extract and load task info when file exists', () => {
    const info = releaseSummary.getTaskInfo(`completed (task: ${taskId})`);
    assert.ok(info);
    assert.strictEqual(info.id, taskId);
  });

  if (fs.existsSync(taskFile)) {
    fs.unlinkSync(taskFile);
  }
});
```

**Filesystem assertion pattern** (`plugin/vibe/scripts/test-runtime.js:38-90`):
```js
try {
  setup();
  console.log('1. Initializing workspace...');
  runScript('init', ['.']);
  assert.ok(fs.existsSync(path.join(TEST_DIR, '.vibe', 'config.json')));

  console.log('2. Creating a task...');
  const taskOutput = runScript('task', ['create', JSON.stringify({ goal: taskGoal, file_scope: ['test.txt'] })]);
  const task = JSON.parse(taskOutput);
  assert.strictEqual(task.status, 'queued');

  const runResult = JSON.parse(fs.readFileSync(path.join(runsDir, runFiles[0])));
  assert.strictEqual(runResult.exit_code, 0);
} catch (error) {
  console.error('\nRuntime verification failed:');
  console.error(error.message);
  process.exit(1);
} finally {
  cleanup();
}
```

**JSON existence / parse assertion pattern** (`plugin/vibe/scripts/init.test.js:30-48`):
```js
const configPath = path.join(TEST_DIR, '.vibe', 'config.json');
assert.ok(fs.existsSync(configPath), 'config.json should exist');

const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
assert.strictEqual(config.default_model, 'claude', 'config.json should have default_model');
assert.ok(config.lock_policy, 'config.json should have lock_policy');

const plannerPath = path.join(TEST_DIR, '.vibe', 'agents', 'planner.json');
assert.ok(fs.existsSync(plannerPath), 'planner.json should exist');
```

**How to apply**
- 用 `node:test` 作为主框架，保持 `require('node:test')` + `require('node:assert')` + `fs/path` 的纯内建组合。
- 测试体应直接读取 `package.json`、三个 plugin manifest 和 `.agents/plugins/marketplace.json`，做存在性、`JSON.parse`、identity 一致性与路径断言。
- 不要把真实 `gemini` / `claude` / `codex` CLI 加载放进自动化测试。
- 如果需要临时辅助函数，保持在测试文件内，风格靠近现有脚本测试，不引入第三方测试框架。

## Shared Patterns

### Plugin Root Layout
**Sources:** `plugin/vibe/README.md:5-15`, `plugin/vibe/references/plugin-architecture.md:10-12`
**Apply to:** `plugin/vibe/package.json`, `plugin/vibe/gemini-extension.json`, `plugin/vibe/.claude-plugin/plugin.json`, `plugin/vibe/.codex-plugin/plugin.json`

```markdown
`plugin/vibe/` is organized around model-readable and scriptable plugin surfaces:

- `.codex-plugin/plugin.json` declares the Codex-compatible plugin identity.
- `skills/` will teach the current model to act as the Conductor.
- `commands/` will expose workflow entry contracts.
- `references/` will hold collaboration, task, Agent, review, workspace, and migration contracts.
- `scripts/` holds thin deterministic helpers.
```

### Pure Node.js / Minimal Dependency Pattern
**Sources:** `plugin/vibe/scripts/README.md:3-18`, `plugin/vibe/references/plugin-architecture.md:25-27`
**Apply to:** `plugin/vibe/package.json`, `plugin/vibe/scripts/manifests.test.js`

```markdown
Vibe scripts are thin deterministic helpers for project-local `.vibe` workspace operations.

All scripts are written in pure Node.js using native APIs to ensure maximum portability and minimal dependencies.

- `scripts` own deterministic filesystem/subprocess actions
```

### JSON-Readable Contract Pattern
**Sources:** `plugin/vibe/references/workspace-layout.md:22-24`, `plugin/vibe/references/workspace-layout.md:49-53`
**Apply to:** 所有新的或更新的 JSON manifest / package 文件

```markdown
`.vibe/config.json` records defaults that deterministic scripts need before
launching work. It should stay human-readable JSON and avoid hidden state.

Files should be plain Markdown or JSON unless a later phase documents a
stronger reason to add another format.
```

### Local Discovery Path Pattern
**Sources:** `.agents/plugins/marketplace.json:7-18`, `plugin/vibe/.codex-plugin/plugin.json:5-8`
**Apply to:** `.agents/plugins/marketplace.json`, `plugin/vibe/.codex-plugin/plugin.json`, `plugin/vibe/scripts/manifests.test.js`

```json
{
  "name": "vibe",
  "source": {
    "source": "local",
    "path": "./plugin/vibe"
  },
  "policy": {
    "installation": "AVAILABLE",
    "authentication": "ON_INSTALL"
  },
  "category": "Productivity"
}
```

```json
{
  "skills": "./skills/",
  "interface": {
    "displayName": "Vibe",
    "shortDescription": "Coordinate AI Agents through project-local tasks, reviews, and logs."
  }
}
```

## No Analog Found

Files with no close same-role/same-format analog in the codebase:

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `plugin/vibe/package.json` | config | transform | 仓库里没有任何现成 `package.json`；只能组合现有 Codex manifest identity 与 Node.js 脚本约定。 |
| `plugin/vibe/gemini-extension.json` | config | transform | 仓库里没有任何 Gemini extension manifest。 |
| `plugin/vibe/.claude-plugin/plugin.json` | config | transform | 仓库里没有任何 Claude plugin manifest。 |

## Metadata

**Analog search scope:** `plugin/vibe/`, `.agents/plugins/`, `.planning/phases/25-universal-manifests-packaging/`, `CLAUDE.md`  
**Files scanned:** 34 in scope, 12 key files read  
**Pattern extraction date:** 2026-04-24
