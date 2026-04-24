# Phase 25: Universal Manifests & Packaging - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-24
**Phase:** 25-Universal Manifests & Packaging
**Areas discussed:** Manifest 元数据统一, package.json 依赖边界, 跨平台发现方式, 验证强度

---

## Manifest 元数据统一

| Option | Description | Selected |
|--------|-------------|----------|
| 统一核心身份，平台字段各自扩展 | `name/version/description/displayName` 以 `package.json` 为准，各平台只增加自己的必需字段。 | yes |
| 完全按平台分别优化 | Gemini、Claude、Codex 可以有不同描述和展示名。 | |
| 最小占位即可 | 只填平台能识别的最小字段，不做一致性约束。 | |

**User's choice:** 统一核心身份，平台字段各自扩展。
**Notes:** `package.json` is the identity source of truth.

---

## package.json 依赖边界

| Option | Description | Selected |
|--------|-------------|----------|
| 创建真实包骨架并预留 MCP 依赖位 | 设置 `name/version/type/scripts/engines`，但不安装 MCP SDK；phase 27 再引入实际依赖。 | yes |
| 现在就加入 MCP SDK 依赖 | 直接把 MCP SDK 放进 dependencies。 | |
| 只创建最小 package.json | 只有 `name/version/description`。 | |

**User's choice:** 创建真实包骨架并预留 MCP 依赖位；MCP SDK 依赖留到 Phase 27。
**Notes:** Avoid unused dependencies in Phase 25.

---

## 跨平台发现方式

| Option | Description | Selected |
|--------|-------------|----------|
| 插件内 manifest + repo 级本地 discovery 示例 | 保留/校验 `.agents/plugins/marketplace.json` 指向 `./plugin/vibe`，同时新增 Gemini/Claude/Codex 各自 manifest。 | yes |
| 只做插件内 manifest | 不碰 repo 级 discovery。 | |
| 做成发布级 registry/marketplace 配置 | 面向外部分发更完整，但接近 FUT-01。 | |

**User's choice:** 插件内 manifest + repo 级本地 discovery 示例。
**Notes:** External publishing remains out of scope.

---

## 验证强度

| Option | Description | Selected |
|--------|-------------|----------|
| 结构 + 一致性验证脚本 | Node.js smoke test checks JSON parsing, required files, identity consistency, and discovery path. | yes |
| 只做文件存在和 JSON parse | Simpler, but cannot prevent manifest field drift. | |
| 平台 CLI 级真实加载验证 | Strongest, but depends on locally installed provider CLIs. | |

**User's choice:** 结构 + 一致性验证脚本。
**Notes:** Provider CLI loading is intentionally not required in this phase.

---

## the agent's Discretion

- Exact platform-specific optional fields.
- Exact smoke test file name/location.
- Exact `engines` version and script names in `package.json`.

## Deferred Ideas

- External marketplace or registry publication.
- Real provider CLI loading tests.
