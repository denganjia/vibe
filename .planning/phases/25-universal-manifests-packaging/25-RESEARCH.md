# Phase 25: Universal Manifests & Packaging - Research

**Researched:** 2026-04-24
**Domain:** 跨平台 AI CLI plugin manifest 与 package skeleton
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Manifest 元数据统一
- **D-01:** `plugin/vibe/package.json` is the source of truth for shared plugin identity fields.
- **D-02:** Gemini, Claude, and Codex manifests should keep `name`, `version`, `description`, and display identity aligned with `package.json`.
- **D-03:** Platform manifests may add platform-specific required fields, but should not drift on core identity.

### package.json 依赖边界
- **D-04:** Phase 25 should create a real package skeleton, not a placeholder-only JSON file.
- **D-05:** `package.json` should include practical package metadata such as `name`, `version`, `description`, `type`, `scripts`, and `engines` where useful for the plugin runtime.
- **D-06:** Do not add MCP SDK dependencies in Phase 25. Actual MCP dependencies should be introduced in Phase 27 when the server is implemented.

### 跨平台发现方式
- **D-07:** Phase 25 should maintain both plugin-internal manifests and a repo-level local discovery example.
- **D-08:** Existing `.agents/plugins/marketplace.json` should remain aligned with `./plugin/vibe` for local Codex discovery and smoke testing.
- **D-09:** Do not turn this phase into external registry or marketplace publishing work. External distribution remains future scope.

### 验证强度
- **D-10:** Add or use a Node.js smoke test for manifest/package validation.
- **D-11:** The smoke test should verify required files exist, JSON parses, core identity fields are consistent with `package.json`, and local discovery points to `./plugin/vibe`.
- **D-12:** Do not require real Gemini/Claude/Codex CLI loading in this phase because it would make validation depend on user-local tooling.

### Claude's Discretion
- Exact platform-specific optional fields are left to research and planning, as long as required manifest schemas are satisfied.
- Exact smoke test file name/location is flexible, but it should live near existing plugin scripts/tests and be easy to run with Node.js.
- Exact `engines` version and script names are flexible, but should match existing pure Node.js runtime conventions.

### Deferred Ideas (OUT OF SCOPE)
- External marketplace or registry publication belongs to future requirement FUT-01, not Phase 25.
- Real provider CLI loading tests can be added later if the project decides to support environment-specific integration tests.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| MAN-01 | Initialize `package.json` in `plugin/vibe/` to manage MCP SDK dependencies | 先建立零依赖 `package.json` 骨架，承载统一 identity、`scripts` 与 `engines`，并明确 Phase 25 不引入 MCP SDK，Phase 27 再加真实依赖。[VERIFIED: .planning/REQUIREMENTS.md][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |
| MAN-02 | Create `gemini-extension.json` for Gemini CLI compatibility | 提供 Gemini 最小可识别 manifest，仅保留当前 phase 必需 identity 字段，暂不写 `mcpServers`/`contextFileName`/`plan` 等未来字段。[CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/writing-extensions.md] |
| MAN-03 | Create `.claude-plugin/plugin.json` for Claude Code compatibility | 提供 Claude manifest 并遵守 `.claude-plugin/` 只放 `plugin.json`、其余目录留在 plugin root 的规则；优先最小 identity manifest，避免 Phase 26 前过早绑定目录路径。[CITED: https://code.claude.com/docs/en/plugins][CITED: https://code.claude.com/docs/en/plugins-reference] |
| MAN-04 | Ensure `.codex-plugin/plugin.json` aligns with Codex CLI standards | 保留 Codex manifest 作为现有基线，校正为与 `package.json` 一致，并维持 `skills` 相对路径与 repo-level marketplace 示例的 `./plugin/vibe` 本地发现路径。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][VERIFIED: .agents/plugins/marketplace.json][CITED: https://developers.openai.com/codex/plugins/build] |
</phase_requirements>

## Project Constraints (from CLAUDE.md)

- 项目主技术栈仍是 Rust；Phase 25 的 Node.js 工作应局限于 `plugin/vibe/` 的 packaging 与测试辅助，不应扩散成新的主运行时替代品。[VERIFIED: CLAUDE.md]
- 目标环境是 MacOS/Linux；package scripts 不需要为 Windows 做第一优先适配。[VERIFIED: CLAUDE.md]
- 在这个仓库里改文件应通过 GSD 流程进行；本 research 文档本身就是该流程的 planning 产物之一。[VERIFIED: CLAUDE.md]

## Summary

这个 phase 的本质不是“做三个 provider 集成”，而是“把 `plugin/vibe/` 变成一个跨平台可识别的静态 package surface”。当前代码库已经有 `plugin/vibe/.codex-plugin/plugin.json` 与 `.agents/plugins/marketplace.json`，并且 `plugin/vibe/scripts/` 已经在使用纯 Node.js 测试与脚本模式；所以最稳妥的 Phase 25 策略是：让 `package.json` 成为统一 identity 源，分别补齐 Gemini 与 Claude manifest，修正 Codex manifest，然后用一个纯 Node.js smoke test 去约束文件存在性、JSON 可解析性、identity 一致性与本地发现路径，而不是在本 phase 引入真实 CLI 加载或 MCP server 启动。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][VERIFIED: .agents/plugins/marketplace.json][VERIFIED: plugin/vibe/scripts/release-summary.test.js][VERIFIED: plugin/vibe/scripts/init.test.js][CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md]

三家平台的共同约束很清楚：identity 要稳定，路径要相对且可移植，manifest 只声明当前真实存在的东西。Codex 明确要求 `.codex-plugin/plugin.json` 作为入口，并要求 marketplace 本地 `source.path` 以 `./` 开头且位于 marketplace root 内；Claude 明确要求 `.claude-plugin/` 里只放 `plugin.json`，路径字段必须以 `./` 开头；Gemini 明确要求 `gemini-extension.json` 在扩展根，并要求扩展名使用 lowercase/dashes，同时推荐用 `${extensionPath}` 做可移植路径。由于 Phase 25 锁定了“不要提前做 MCP”，所以这三个 manifest 都应避免声明 `mcpServers` 或其他会指向未来运行时的字段，直到 Phase 27 真的实现服务器。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

**Primary recommendation:** 用 `plugin/vibe/package.json` 持有共享 identity，新增 `gemini-extension.json` 与 `.claude-plugin/plugin.json`，修正 `.codex-plugin/plugin.json`，再用单个 `node:test` smoke test 强制三份 manifest 与 `.agents/plugins/marketplace.json` 对齐；不要在本 phase 引入 `.mcp.json`、`mcpServers`、provider CLI 安装/加载依赖或 manifest 生成器。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][VERIFIED: plugin/vibe/scripts/release-summary.test.js][CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md]

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| 统一 plugin identity | `plugin/vibe/package.json` | provider manifests | 共享 `name`/`version`/`description` 被锁定为由 `package.json` 持有；provider manifest 只是消费方，不应各自漂移。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |
| Codex 本地发现 | `.agents/plugins/marketplace.json` | `plugin/vibe/.codex-plugin/plugin.json` | Codex repo marketplace 位于 repo root，下游插件通过 `./plugin/vibe` 相对路径被发现；plugin manifest 只描述插件本身。[VERIFIED: .agents/plugins/marketplace.json][CITED: https://developers.openai.com/codex/plugins/build] |
| Claude / Gemini 平台识别 | 各自根级 manifest | `plugin/vibe/` 现有 `skills/`/`commands/` 目录 | 这两个平台先通过根 manifest 确认扩展身份，再从 plugin root 读取默认目录；Phase 25 不需要改技能内容本身。[VERIFIED: plugin/vibe][CITED: https://code.claude.com/docs/en/plugins][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md] |
| 跨 manifest 一致性验证 | `plugin/vibe/scripts/manifests.test.js` | `npm test` script | 锁定决策要求 Node.js smoke test；它是本 phase 的 deterministic gate，而不是 provider CLI 真实启动。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][VERIFIED: plugin/vibe/scripts/release-summary.test.js] |

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Node.js | `22.16.0` | 运行 package scripts 与 manifest smoke test | 本机已安装，现有 `plugin/vibe/scripts/*.test.js` 已采用纯 Node.js 风格，无需额外测试框架。[VERIFIED: node --version][VERIFIED: plugin/vibe/scripts/release-summary.test.js][VERIFIED: plugin/vibe/scripts/init.test.js] |
| npm | `10.9.2` | 承载 `package.json`、统一 script 入口与未来依赖管理 | MAN-01 明确要求在 `plugin/vibe/` 初始化 `package.json`；Phase 25 只需要 package surface，不需要第三方依赖安装。[VERIFIED: npm --version][VERIFIED: .planning/REQUIREMENTS.md][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |
| `node:test` + `node:assert` | bundled with Node `22.16.0` | 提供零依赖 smoke test | 当前仓库已存在 `node:test` 用法，继续沿用能保持 Phase 25 零依赖且输出稳定。[VERIFIED: plugin/vibe/scripts/release-summary.test.js][VERIFIED: node --version] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Codex CLI | `0.124.0` | 可选人工 spot-check manifest / marketplace 可见性 | 本机可用，但锁定决策明确本 phase 不把真实 CLI 加载当成验证前置；仅适合作为非阻塞人工检查。[VERIFIED: codex --version][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |
| Claude Code CLI | `2.1.92` | 可选人工运行 `plugin validate` | 本机可用，但不应进入 Phase 25 的必过验证链路。[VERIFIED: claude --version][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |
| Gemini CLI | `0.39.0` | 可选人工检查扩展安装/列出 | 本机可用，但不应成为本 phase 的自动化要求。[VERIFIED: gemini --version][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| 纯 Node.js smoke test | 直接调用 provider CLI 真实加载 | 真实加载更接近运行时，但会把 Phase 25 绑定到用户本机环境与 provider 安装状态，违反 D-12。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |
| 手工保持 manifest 一致 | 生成脚本从 `package.json` 自动产出三个 manifest | 生成器能进一步去重，但会引入额外 build surface；本 phase 只需静态 package surface，用 smoke test 约束一致性更低风险。[ASSUMED] |
| `node:test` | shell + `jq` 断言 JSON | `jq` 已安装但 shell 断言对跨文件对象比较更脆弱；Node.js 更适合做 JSON/path/array 断言，且已符合现有脚本风格。[VERIFIED: jq --version][VERIFIED: plugin/vibe/scripts/release-summary.test.js][ASSUMED] |

**Installation:**
```bash
# Phase 25 不应新增外部 npm dependencies 或 devDependencies
# 只创建 package.json，并把测试建立在 Node.js built-ins 上
```

**Version verification:** Phase 25 的推荐栈不包含新的第三方 npm package；因此无需执行 `npm view <package> version`。Node.js 与 npm 版本已直接在本机验证。[VERIFIED: node --version][VERIFIED: npm --version]

## Architecture Patterns

### System Architecture Diagram

```text
package.json (identity source)
  |-- name/version/description -------------------\
  |                                                \
  v                                                 v
Codex manifest (.codex-plugin/plugin.json)     Claude manifest (.claude-plugin/plugin.json)
  |                                               |
  |                                               |
  v                                               v
Codex repo marketplace ---------------------> plugin/vibe root <---------------- Gemini manifest (gemini-extension.json)
(.agents/plugins/marketplace.json)              |                                 |
  |                                             |                                 |
  \-------------------------- smoke test reads all JSON files -------------------/
                                |
                                v
                     deterministic assertions:
                     file existence, JSON parse, identity consistency,
                     relative path rules, local discovery path
```

这个 phase 的主数据流是“`package.json` identity -> provider manifests / marketplace -> smoke test gate”，而不是“manifest -> 真实 provider CLI 启动”。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md]

### Recommended Project Structure

```text
plugin/vibe/
├── package.json                   # CREATE: shared identity + scripts + engines
├── gemini-extension.json          # CREATE: Gemini manifest
├── .claude-plugin/
│   └── plugin.json                # CREATE: Claude manifest
├── .codex-plugin/
│   └── plugin.json                # UPDATE: align with package.json + Codex rules
├── skills/                        # KEEP: existing skill surface
├── commands/                      # KEEP: existing command surface; do not standardize in Phase 25
└── scripts/
    └── manifests.test.js          # CREATE: deterministic smoke test

.agents/plugins/marketplace.json   # VERIFY/UPDATE ONLY IF DRIFTED
```

上面的文件集与 success criteria 一一对应，并且把测试放在现有 `plugin/vibe/scripts/` 邻近位置，符合当前纯 Node.js 脚本布局。[VERIFIED: .planning/ROADMAP.md][VERIFIED: plugin/vibe][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

### Pattern 1: `package.json` 作为唯一共享 identity 源

**What:** 在 `plugin/vibe/package.json` 持有 `name`、`version`、`description`、`type`、`scripts`、`engines`，其余平台 manifest 复制最少的 identity 字段并由 smoke test 校验一致性。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

**When to use:** 这个 pattern 适用于当前“先建 package surface、后接 MCP runtime”的阶段，因为它避免在还没有真实构建产物时引入 manifest 生成步骤。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED]

**Example:**
```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces.",
  "private": true,
  "type": "commonjs",
  "scripts": {
    "test": "npm run test:manifests",
    "test:manifests": "node --test scripts/manifests.test.js"
  },
  "engines": {
    "node": ">=22.0.0"
  }
}
```

这个示例里的 `name`/`version`/`description` 来自现有 Codex manifest；`type: "commonjs"` 与现有 `require(...)` 脚本风格一致；`private: true` 与 `engines.node >=22` 是为 Phase 25 降低 accidental publish 与环境漂移风险的实施建议，不是 provider schema 强制要求。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][VERIFIED: plugin/vibe/scripts/init.test.js][VERIFIED: plugin/vibe/scripts/release-summary.test.js][ASSUMED]

### Pattern 2: 平台 manifest 只声明本 phase 真实存在的最小字段

**What:** Codex manifest 保留 `skills` 与 `interface`；Claude manifest 保持最小 identity；Gemini manifest 保持最小 identity；三个文件都暂不声明 `mcpServers`、`.mcp.json`、`contextFileName`、`plan` 等未来运行时字段。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

**When to use:** 当目录结构仍会在 Phase 26 标准化、MCP runtime 仍会在 Phase 27 落地时，最小 manifest 能减少未来返工。[VERIFIED: .planning/ROADMAP.md]

**Example:**
`plugin/vibe/.codex-plugin/plugin.json`

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

`plugin/vibe/.claude-plugin/plugin.json`

```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces."
}
```

`plugin/vibe/gemini-extension.json`

```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces."
}
```

Codex 需要用 manifest 显式指向 `skills`；Claude 如果指定自定义路径会替换默认扫描路径，因此在目录仍可能变化时保持 identity-only 更安全；Gemini 会自动发现 `skills/` 与 `commands/`，所以 Phase 25 不需要把这些目录再写进 manifest。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/writing-extensions.md]

### Pattern 3: 使用独立 smoke test 守住一致性，而不是引入生成器

**What:** 在 `plugin/vibe/scripts/manifests.test.js` 中读取 `package.json`、三个 provider manifest 与 `.agents/plugins/marketplace.json`，以断言替代 build-time codegen。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

**When to use:** 当目标是“确保静态 surface 一致”而不是“从源代码生成发行产物”时，这个模式足够而且更可维护。[ASSUMED]

**Example:**
```js
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

test('manifest identity stays aligned', () => {
  const root = path.resolve(__dirname, '..');
  const pkg = JSON.parse(fs.readFileSync(path.join(root, 'package.json'), 'utf8'));
  const codex = JSON.parse(fs.readFileSync(path.join(root, '.codex-plugin', 'plugin.json'), 'utf8'));
  const claude = JSON.parse(fs.readFileSync(path.join(root, '.claude-plugin', 'plugin.json'), 'utf8'));
  const gemini = JSON.parse(fs.readFileSync(path.join(root, 'gemini-extension.json'), 'utf8'));

  for (const manifest of [codex, claude, gemini]) {
    assert.equal(manifest.name, pkg.name);
    assert.equal(manifest.version, pkg.version);
    assert.equal(manifest.description, pkg.description);
  }
});
```

这个测试风格与现有 `release-summary.test.js` 的 `node:test` 用法一致，并且不依赖 provider CLI 是否安装或行为是否变更。[VERIFIED: plugin/vibe/scripts/release-summary.test.js][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

### Platform-by-Platform Field Guidance

| Platform | File | Recommended fields now | Deliberately omit in Phase 25 | Reason |
|----------|------|------------------------|-------------------------------|--------|
| Codex | `plugin/vibe/.codex-plugin/plugin.json` | `name`, `version`, `description`, `skills`, `interface.displayName`, `interface.shortDescription` | `mcpServers`, `apps`, visual assets, policy fields | `skills` 是当前唯一已存在且需要显式声明的 plugin component；policy/category 属于 marketplace entry，不属于 plugin manifest；MCP/app 仍未实现。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][CITED: https://developers.openai.com/codex/plugins/build][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |
| Claude Code | `plugin/vibe/.claude-plugin/plugin.json` | `name`, `version`, `description` | `skills`, `commands`, `agents`, `hooks`, `mcpServers`, `lspServers` | Claude manifest 若写自定义路径会替换默认扫描路径；Phase 26 还会整理技能/命令结构，因此现在保持 identity-only 最稳。[CITED: https://code.claude.com/docs/en/plugins-reference][VERIFIED: .planning/ROADMAP.md] |
| Gemini | `plugin/vibe/gemini-extension.json` | `name`, `version`, `description` | `mcpServers`, `contextFileName`, `excludeTools`, `migratedTo`, `plan`, `settings` | Gemini 已支持自动发现 `skills/` 与 `commands/`；当前没有 `GEMINI.md` 或 MCP server，提前声明只会制造漂移或坏链接。[CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/writing-extensions.md][VERIFIED: plugin/vibe] |

### Exact `package.json` Fields and Scripts

| Field | Recommendation | Why |
|-------|----------------|-----|
| `name` | `"vibe"` | 现有 Codex manifest 与目录名都已使用 `vibe`；Gemini 也要求 lowercase/dashes，`vibe` 合法且不会引入额外映射。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md] |
| `version` | `"0.1.0"` | 现有 Codex manifest 已是 `0.1.0`；先沿用可减少 Phase 25 纯 packaging 改动带来的无意义版本分歧。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][ASSUMED] |
| `description` | `"Plugin-first multi-model collaboration for Vibe workspaces."` | 现有 Codex manifest 已有这条描述，应上移为统一来源。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json] |
| `private` | `true` | 当前明确不做外部 registry/publish，`private` 可降低误发布风险。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED] |
| `type` | `"commonjs"` | 现有 `plugin/vibe/scripts/*.js` 通过 `require(...)` 组织，继续 CommonJS 可避免 package 初始化后意外切到 ESM 语义。[VERIFIED: plugin/vibe/scripts/init.test.js][VERIFIED: plugin/vibe/scripts/release-summary.test.js] |
| `scripts.test` | `"npm run test:manifests"` | 保持 `npm test` 作为最短入口，并把 scope 限定在本 phase 的 manifest smoke test。[ASSUMED] |
| `scripts.test:manifests` | `"node --test scripts/manifests.test.js"` | 使用内建 test runner，零依赖、输出稳定、容易在后续 phase 复用。[VERIFIED: plugin/vibe/scripts/release-summary.test.js][ASSUMED] |
| `engines.node` | `">=22.0.0"` | 本机是 Node `22.16.0`；以 Node 22 为底线能匹配当前开发环境并为后续 Node-side MCP 工作留出空间。[VERIFIED: node --version][ASSUMED] |

### Smoke Test Design: Exact Validation Assertions

`plugin/vibe/scripts/manifests.test.js` 应至少包含以下断言；这些断言就是 planner 可以直接拆成实现任务的最小验收面。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED]

1. 断言以下文件存在：
   `plugin/vibe/package.json`、`plugin/vibe/gemini-extension.json`、`plugin/vibe/.claude-plugin/plugin.json`、`plugin/vibe/.codex-plugin/plugin.json`、`.agents/plugins/marketplace.json`。[VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]
2. 断言上述每个 JSON 文件都能被 `JSON.parse` 成功解析。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]
3. 断言 `package.json` 的 `name`、`version`、`description` 是非空字符串，并且 `scripts.test` 与 `scripts["test:manifests"]` 存在。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED]
4. 断言 `package.json.type === "commonjs"`，避免现有 `require(...)` 脚本在 package 初始化后被隐式切换语义。[VERIFIED: plugin/vibe/scripts/init.test.js][VERIFIED: plugin/vibe/scripts/release-summary.test.js][ASSUMED]
5. 断言 `package.json.dependencies` 与 `devDependencies` 中都不存在 `@modelcontextprotocol/sdk`，也不存在以当前 phase 名义提前加入的 MCP server runtime 依赖。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED] 
6. 断言 Codex/Claude/Gemini 三份 manifest 的 `name`、`version`、`description` 与 `package.json` 完全一致。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]
7. 断言 Codex manifest 的 `skills === "./skills/"`，并且 `interface.displayName`、`interface.shortDescription` 为非空字符串。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][CITED: https://developers.openai.com/codex/plugins/build]
8. 断言 Claude manifest 若包含任何路径型字段，则每个值都以 `./` 开头；若只包含 identity 字段，也应被视为合法最小 manifest。[CITED: https://code.claude.com/docs/en/plugins-reference][ASSUMED]
9. 断言 Gemini manifest 的 `name` 满足 lowercase/dashes 约束，且当前 phase 不包含 `mcpServers`；若未来有人加了 `contextFileName`，测试必须顺带断言目标文件存在。[CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][ASSUMED]
10. 断言 `.agents/plugins/marketplace.json` 至少存在一个 `plugins[]` 条目满足：
    `name === package.json.name`、`source.source === "local"`、`source.path === "./plugin/vibe"`、`policy.installation` 存在、`policy.authentication` 存在、`category` 存在。[VERIFIED: .agents/plugins/marketplace.json][CITED: https://developers.openai.com/codex/plugins/build]

### Anti-Patterns to Avoid

- **把 Phase 25 做成半个 Phase 27：** 不要创建 `.mcp.json`、不要填 `mcpServers`、不要引入 MCP SDK、不要做伪 server 占位实现。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][VERIFIED: .planning/ROADMAP.md]
- **用生成脚本替代清晰的静态文件：** 这个 phase 要的是清晰可读的 package/manifests；生成器会制造额外维护面与调试面。[ASSUMED]
- **在 Claude manifest 里过早绑定 `skills`/`commands` 自定义路径：** Claude 文档明确自定义路径会替换默认扫描；Phase 26 还会调整目录结构，现在过度声明会放大返工面。[CITED: https://code.claude.com/docs/en/plugins-reference][VERIFIED: .planning/ROADMAP.md]
- **使用绝对路径或 `../`：** Codex 与 Claude 都要求相对路径且以 `./` 开头；Gemini 也推荐 `${extensionPath}` 做可移植定位。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md]
- **把自动化验证绑定到真实 provider CLI：** 这样会让 Phase 25 的红绿取决于用户本机工具与缓存状态，而不是仓库内文件本身。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| 跨 provider 识别验证 | 真实 provider 安装/启用/加载 harness | 单个 Node.js smoke test | 目标是验证静态 manifest surface，不是验证三家 CLI 在本机的完整运行时。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |
| MCP 占位集成 | 假的 `.mcp.json` 或空 server 脚本 | 先省略 MCP 相关字段，等 Phase 27 再落地 | 文档都把 MCP 视为可选组件；在还没有 server 时声明它只会让 manifest 指向不存在的 runtime。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md] |
| JSON/path 校验 | shell + `grep`/`sed`/`jq` 拼接式断言 | `node:test` + `JSON.parse` + `path` | 现有仓库已有 Node.js 测试样式，代码更容易表达对象级比较与路径规则。[VERIFIED: plugin/vibe/scripts/release-summary.test.js][ASSUMED] |

**Key insight:** 这个 phase 的复杂度不在 schema 数量，而在“多个静态文件共享同一 identity 时如何防漂移”。最便宜且最稳的答案是“一个 source of truth + 一个 smoke test gate”，而不是额外的代码生成或 provider-specific 运行时桥接。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED]

## Common Pitfalls

### Pitfall 1: 路径写对了语义，写错了格式

**What goes wrong:** `skills`、`source.path` 或其他 manifest 路径写成绝对路径、缺少 `./`、或跳出 plugin/marketplace root，最终在某个平台可见、另一个平台失效。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference]

**Why it happens:** 三家平台都支持相对路径，但语法约束略有不同，容易凭经验乱写。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md]

**How to avoid:** smoke test 对所有路径型字段执行统一断言：Codex/Claude 必须以 `./` 开头；Gemini 若声明文件路径则优先使用 `${extensionPath}` 或根目录相对存在性检查。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][ASSUMED]

**Warning signs:** 需要在代码里写额外 path normalization 才能“跑起来”时，通常已经偏离官方约束。[ASSUMED]

### Pitfall 2: `package.json` 成了“名义真相源”，但没有 enforcement

**What goes wrong:** 三份 manifest 都复制了 `name`/`version`/`description`，但没人检查它们是否同步，几次改动后必然漂移。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

**Why it happens:** “source of truth” 只是约定；没有自动化检查就不会被真正执行。[ASSUMED]

**How to avoid:** 把 identity equality 作为 smoke test 的核心断言，并让 `npm test` 默认跑这组断言。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED]

**Warning signs:** 改完一个 manifest 需要靠人工记忆去同步另外两个文件时，这个 phase 的实现方式就已经不稳定了。[ASSUMED]

### Pitfall 3: 提前声明未来字段

**What goes wrong:** 在 Gemini/Claude/Codex manifest 里加入 `mcpServers`、`contextFileName`、`plan`、`.mcp.json` 等字段，但目标文件或 runtime 还不存在，导致 manifest 看起来“更完整”，实际却更脆弱。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md]

**Why it happens:** 容易把 provider 文档里的“完整示例”误读成“当前 phase 必选字段”。[ASSUMED]

**How to avoid:** 只声明当前 phase 已经真实存在的 surface；未来 phase 再增量扩展 manifest。[VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

**Warning signs:** PR 里开始出现不存在的 `server.js`、`.mcp.json`、`GEMINI.md` 或空目录占位时，基本就是 phase boundary 正在滑坡。[ASSUMED]

### Pitfall 4: 把 Claude manifest 当成路径注册表

**What goes wrong:** 为了“显式一点”把 `skills`、`commands`、`agents` 都写进 `.claude-plugin/plugin.json`，结果后续目录标准化时需要同步修改 manifest，甚至因为 custom path 替换默认扫描而漏掉目录。[CITED: https://code.claude.com/docs/en/plugins-reference]

**Why it happens:** Claude 的 manifest 是可选的，很多字段只是为了覆盖默认位置；不是每个存在的目录都必须声明。[CITED: https://code.claude.com/docs/en/plugins-reference]

**How to avoid:** Phase 25 仅用 Claude manifest 表达 identity；把目录标准化留给 Phase 26。[VERIFIED: .planning/ROADMAP.md][ASSUMED]

**Warning signs:** 任何为了让 Claude 识别而写进去、但其实与默认目录完全相同的 path 字段，都值得重新审视。[ASSUMED]

### Pitfall 5: 把 `npm test` 绑到仓库所有现有测试

**What goes wrong:** 新建 package.json 后直接把 `npm test` 指到 `scripts/*.test.js` 全量运行，导致 Phase 25 被与 manifest 无关的旧测试状态阻塞。[VERIFIED: plugin/vibe/scripts/init.test.js][VERIFIED: plugin/vibe/scripts/release-summary.test.js][ASSUMED]

**Why it happens:** package 初始化时容易顺手把“所有测试都接进去”，但当前 phase 的目标是 packaging smoke gate，不是统一整套测试基建。[VERIFIED: .planning/ROADMAP.md][ASSUMED]

**How to avoid:** `npm test` 先只接 `scripts/manifests.test.js`，现有其它测试保留原状或单独脚本名，等后续 phase 再统一测试矩阵。[ASSUMED]

**Warning signs:** 只改 manifest 却需要 debug 初始化脚本或 release summary 逻辑时，测试 scope 已经越界。[ASSUMED]

## Code Examples

Verified patterns from official sources and current codebase:

### Codex Manifest with Local Skill Path
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
Source basis: Codex docs require `.codex-plugin/plugin.json`, use relative `./` paths, and show `skills` + `interface` as standard plugin fields; current repo already has this exact surface in simplified form.[CITED: https://developers.openai.com/codex/plugins/build][VERIFIED: plugin/vibe/.codex-plugin/plugin.json]

### Claude Minimal Identity Manifest
```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces."
}
```
Source basis: Claude manifest is optional, `name` is the only required field if manifest exists, and all non-manifest directories stay at plugin root. Keeping only identity fields minimizes drift before Phase 26 path standardization.[CITED: https://code.claude.com/docs/en/plugins][CITED: https://code.claude.com/docs/en/plugins-reference]

### Gemini Minimal Extension Manifest
```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces."
}
```
Source basis: Gemini requires `gemini-extension.json` at extension root and documents `name`/`version`/`description` as core fields; `skills/` and `commands/` are discovered from root directories, so they do not need to be duplicated into the manifest for Phase 25.[CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/writing-extensions.md]

### Repo-Level Codex Marketplace Entry
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
Source basis: Codex docs require local marketplace entries to include `policy.installation`, `policy.authentication`, `category`, and a `./`-prefixed `source.path`; the existing repo example already matches this pattern.[CITED: https://developers.openai.com/codex/plugins/build][VERIFIED: .agents/plugins/marketplace.json]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| 只维持单个平台 manifest（当前 repo 仅有 Codex manifest） | 在 plugin root 内并存 Codex / Claude / Gemini 三份最小 manifest，由 `package.json` 统一 identity | v7.0 / Phase 25 boundary 定义于 2026-04-23 至 2026-04-24。[VERIFIED: .planning/REQUIREMENTS.md][VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] | 为 Phase 27 的真正 MCP 接入预留稳定 packaging surface，而不提前引入 runtime 复杂度。[VERIFIED: .planning/ROADMAP.md] |
| 用 provider CLI 真实加载来证明插件“可用” | 先用 deterministic JSON smoke test 证明 package surface 正确 | 当前 phase 锁定于 2026-04-24。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] | 计划与实现不会被本机 provider 安装状态绑死。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md] |
| 示例文档直接带 `mcpServers`/`contextFileName` 等完整字段 | 仅声明当前 phase 已存在的字段，其余延后到相应 phase | 这是本次 research 针对当前 phase boundary 的实施结论。[VERIFIED: .planning/ROADMAP.md][ASSUMED] | 避免坏链接、坏路径与假集成。[ASSUMED] |

**Deprecated/outdated:**
- “先放空的 `mcpServers` 占位，后面再补实现” 不是当前 phase 的好策略；Phase 25 已明确将真实 MCP 集成延后到 Phase 27。[VERIFIED: .planning/ROADMAP.md][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

## Assumptions Log

> List all claims tagged `[ASSUMED]` in this research. The planner and discuss-phase use this
> section to identify decisions that need user confirmation before execution.

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `package.json` should set `"private": true` to avoid accidental publish. | Architecture Patterns / package.json fields | 低；即使不设也不影响 provider schema，但会增加误发布风险。 |
| A2 | `engines.node` should be `>=22.0.0` rather than a lower LTS floor. | Architecture Patterns / package.json fields | 中；如果团队要兼容更低 Node 版本，需要同步调整 script 与验证矩阵。 |
| A3 | `npm test` should initially run only `scripts/manifests.test.js` instead of all existing plugin tests. | Common Pitfalls / Validation Architecture | 中；如果团队希望一步统一测试入口，计划需要扩大 scope 并先处理现有测试稳定性。 |
| A4 | `plugin/vibe/scripts/manifests.test.js` is the best file name and location for the smoke test. | Recommended Project Structure | 低；可替换为附近等价位置，但 planner 需要保持与 `package.json` scripts 一致。 |
| A5 | 用 smoke test 约束多份 manifest 一致性，比引入生成器更适合当前 phase。 | Alternatives / Key insight | 中；如果团队偏好 codegen，需要为生成流程额外设计可读性与验证面。 |

## Open Questions (RESOLVED)

1. **`engines.node` 是否要锁到 Node 22？**
   - What we know: 本机 Node 版本是 `22.16.0`，现有脚本与测试都是纯 Node.js / CommonJS；research 的 standard stack 也以 Node 22 为 Phase 25 执行基线。[VERIFIED: node --version][VERIFIED: plugin/vibe/scripts/init.test.js][VERIFIED: plugin/vibe/scripts/release-summary.test.js]
   - Resolution: 是。Phase 25 将 `engines.node` 明确收敛为 `>=22.0.0`，与当前 plugin scripts/tests 和已选 standard stack 保持一致，避免在仅做 manifest/package skeleton 的阶段额外扩展兼容面。
   - Plan alignment: `25-01-PLAN.md` Task 1 已使用 `engines.node: ">=22.0.0"` 作为最终计划值。

2. **是否要把 package name 固定为未加 scope 的 `vibe`？**
   - What we know: 现有 Codex manifest 名称是 `vibe`，Gemini 要求名称合法且最好匹配目录名，当前目录也叫 `vibe`；同时 D-09 明确 Phase 25 不做外部 registry/marketplace 发布工作。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]
   - Resolution: 是。Phase 25 保持未加 scope 的本地插件标识 `vibe`，用于本地 manifest identity 与 repo-level discovery；任何 npm scope 或外部发布命名策略都延后到未来外部发布工作。
   - Plan alignment: `25-01-PLAN.md` Task 1/Task 2 已以 `name: "vibe"` 作为最终计划值。

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | `package.json` scripts, smoke test | ✓ | `22.16.0` | — |
| npm | `package.json`, `npm test` | ✓ | `10.9.2` | — |
| Codex CLI | 非阻塞人工检查 | ✓ | `codex-cli 0.124.0` | 不需要；自动化以 smoke test 为准 |
| Claude Code CLI | 非阻塞人工检查 | ✓ | `2.1.92 (Claude Code)` | 不需要；自动化以 smoke test 为准 |
| Gemini CLI | 非阻塞人工检查 | ✓ | `0.39.0` | 不需要；自动化以 smoke test 为准 |

**Missing dependencies with no fallback:**
- None — 这个 phase 的必需依赖只有 Node.js 与 npm，且都已可用。[VERIFIED: node --version][VERIFIED: npm --version]

**Missing dependencies with fallback:**
- None — provider CLI 虽然可用，但按锁定决策本就不是阻塞前置。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Node.js built-in `node:test` + `node:assert` on Node `22.16.0`.[VERIFIED: plugin/vibe/scripts/release-summary.test.js][VERIFIED: node --version] |
| Config file | none — 直接使用 Node built-ins，无额外 test config。[VERIFIED: plugin/vibe/scripts/release-summary.test.js] |
| Quick run command | `npm test` -> `npm run test:manifests` -> `node --test scripts/manifests.test.js`.[ASSUMED] |
| Full suite command | `node --test plugin/vibe/scripts/manifests.test.js` for Phase 25 gate; 其它现有脚本测试不应默认并入本 phase gate。[ASSUMED] |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| MAN-01 | `plugin/vibe/package.json` 存在且包含最小 package skeleton | smoke | `node --test plugin/vibe/scripts/manifests.test.js` | ❌ Wave 0 |
| MAN-02 | `gemini-extension.json` 存在、可解析、identity 与 `package.json` 一致 | smoke | `node --test plugin/vibe/scripts/manifests.test.js` | ❌ Wave 0 |
| MAN-03 | `.claude-plugin/plugin.json` 存在、可解析、路径规则合法 | smoke | `node --test plugin/vibe/scripts/manifests.test.js` | ❌ Wave 0 |
| MAN-04 | `.codex-plugin/plugin.json` 与 `.agents/plugins/marketplace.json` 对齐且符合本地发现规则 | smoke | `node --test plugin/vibe/scripts/manifests.test.js` | ❌ Wave 0 |

### Sampling Rate

- **Per task commit:** `node --test plugin/vibe/scripts/manifests.test.js`。[ASSUMED]
- **Per wave merge:** `npm test`。[ASSUMED]
- **Phase gate:** `npm test` green，且文件树与 success criteria 一致后再进入 `/gsd-verify-work`。[VERIFIED: .planning/ROADMAP.md][ASSUMED]

### Wave 0 Gaps

- [ ] `plugin/vibe/scripts/manifests.test.js` — 覆盖 MAN-01 / MAN-02 / MAN-03 / MAN-04。[VERIFIED: .planning/REQUIREMENTS.md]
- [ ] `plugin/vibe/package.json` — 提供统一 identity 与测试入口。[VERIFIED: .planning/REQUIREMENTS.md]
- [ ] `plugin/vibe/gemini-extension.json` — 提供 Gemini 最小识别 surface。[VERIFIED: .planning/REQUIREMENTS.md]
- [ ] `plugin/vibe/.claude-plugin/plugin.json` — 提供 Claude 最小识别 surface。[VERIFIED: .planning/REQUIREMENTS.md]
- [ ] `plugin/vibe/.codex-plugin/plugin.json` 对齐修正 — 现有文件存在，但需要与 `package.json` 成为一致关系。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][ASSUMED]

## Suggested Plan Decomposition

1. **建立 identity 基线**
   - 新建 `plugin/vibe/package.json`，把现有 Codex manifest 的 `name`/`version`/`description` 上移为统一来源，并补 `type`、`scripts`、`engines`。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]
2. **补齐平台 manifest**
   - 新建 `plugin/vibe/gemini-extension.json` 与 `plugin/vibe/.claude-plugin/plugin.json`，用最小 identity manifest 达成平台可识别性。[CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][CITED: https://code.claude.com/docs/en/plugins-reference]
3. **修正 Codex surface**
   - 更新 `plugin/vibe/.codex-plugin/plugin.json`，确保与 `package.json` 对齐，并确认 `.agents/plugins/marketplace.json` 仍指向 `./plugin/vibe` 且条目字段完整。[VERIFIED: plugin/vibe/.codex-plugin/plugin.json][VERIFIED: .agents/plugins/marketplace.json][CITED: https://developers.openai.com/codex/plugins/build]
4. **补 deterministic smoke test**
   - 新建 `plugin/vibe/scripts/manifests.test.js`，覆盖存在性、JSON parse、identity equality、路径规则与本地发现路径。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md]
5. **接入最小测试入口**
   - 让 `npm test` 只跑 manifest smoke test，不扩大到其它 plugin 脚本测试。[ASSUMED]

这个顺序的依赖关系是单向的：`package.json` 必须先出现，后续 manifest 才能以它为真相源；smoke test 必须最后写，才能同时校验四类文件。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | 本 phase 不处理用户登录、token 流或 provider auth 流程。[VERIFIED: .planning/ROADMAP.md] |
| V3 Session Management | no | 本 phase 只处理静态 package/manifests，不处理 session 生命周期。[VERIFIED: .planning/ROADMAP.md] |
| V4 Access Control | no | 不引入新的授权边界；但本地路径边界仍需要 smoke test 保护。[VERIFIED: .planning/ROADMAP.md][ASSUMED] |
| V5 Input Validation | yes | 用 `JSON.parse`、显式字段断言与相对路径约束验证 manifest 输入面。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED] |
| V6 Cryptography | no | 本 phase 不做 secret storage 或加密逻辑。[VERIFIED: .planning/ROADMAP.md] |

### Known Threat Patterns for this stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| 本地 marketplace / manifest 路径越界 | Tampering | 断言所有 Codex / Claude 路径以 `./` 开头，Codex marketplace path 固定为 `./plugin/vibe`。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference] |
| 通过提前声明 `mcpServers` 把不存在的执行命令引入后续 phase | Elevation of Privilege | Phase 25 明确不声明 MCP runtime 字段，等 Phase 27 有真实实现再接入。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][VERIFIED: .planning/ROADMAP.md] |
| identity drift 导致用户安装了“同名不同版本/描述”的 plugin surface | Spoofing | 用 `package.json` 做统一 identity 源，并在 smoke test 中逐文件比较完全相等。[VERIFIED: .planning/phases/25-universal-manifests-packaging/25-CONTEXT.md][ASSUMED] |

## Sources

### Primary (HIGH confidence)

- `https://developers.openai.com/codex/plugins/build` — 检查了 Codex `.codex-plugin/plugin.json` 入口、manifest 字段、path rules，以及 repo-level `.agents/plugins/marketplace.json` / `source.path` / `policy.*` / `category` 规则。[CITED: https://developers.openai.com/codex/plugins/build]
- `https://code.claude.com/docs/en/plugins` — 检查了 `.claude-plugin/` 目录约束与 plugin root 布局规则。[CITED: https://code.claude.com/docs/en/plugins]
- `https://code.claude.com/docs/en/plugins-reference` — 检查了 Claude manifest schema、`name` required、path behavior rules、custom path replacement semantics。[CITED: https://code.claude.com/docs/en/plugins-reference]
- `https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md` — 检查了 `gemini-extension.json` 根路径、`name` 命名规则、`mcpServers`/`contextFileName`/`plan` 等字段语义、`skills/` 与 `commands/` 自动发现规则。[CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md]
- `https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/writing-extensions.md` — 检查了 Gemini package skeleton、`${extensionPath}` 用法、`skills/` 自动发现、示例 `package.json` 存在性。[CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/writing-extensions.md]
- 本地代码与规划文档：`CLAUDE.md`、`.planning/config.json`、`.planning/REQUIREMENTS.md`、`.planning/ROADMAP.md`、`.planning/STATE.md`、`.planning/phases/25-universal-manifests-packaging/25-CONTEXT.md`、`plugin/vibe/README.md`、`plugin/vibe/references/plugin-architecture.md`、`plugin/vibe/references/workspace-layout.md`、`plugin/vibe/.codex-plugin/plugin.json`、`.agents/plugins/marketplace.json`、`plugin/vibe/scripts/init.test.js`、`plugin/vibe/scripts/release-summary.test.js`。[VERIFIED: codebase grep]

### Secondary (MEDIUM confidence)

- None.

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - 只依赖本机已验证的 Node.js/npm 与现有代码库测试模式，没有额外第三方 package 变量。[VERIFIED: node --version][VERIFIED: npm --version][VERIFIED: plugin/vibe/scripts/release-summary.test.js]
- Architecture: HIGH - 三个平台的 manifest 规则与 phase boundary 都有当前官方文档和本地 planning 文档双重支撑。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][VERIFIED: .planning/ROADMAP.md]
- Pitfalls: MEDIUM - 大部分来自官方 path/schema 规则与 phase boundary，少量“如何最少返工”的策略判断仍是工程经验型建议。[CITED: https://developers.openai.com/codex/plugins/build][CITED: https://code.claude.com/docs/en/plugins-reference][CITED: https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md][ASSUMED]

**Research date:** 2026-04-24
**Valid until:** 2026-05-08 — provider plugin schemas 与 docs 仍在快速演进，建议两周内视为新鲜，之后若再 planning 应复核官方文档。[ASSUMED]
