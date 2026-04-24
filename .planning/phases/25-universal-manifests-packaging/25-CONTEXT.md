# Phase 25: Universal Manifests & Packaging - Context

**Gathered:** 2026-04-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 25 makes `plugin/vibe/` recognizable by major AI CLIs and sets up package management for later MCP work. It delivers `plugin/vibe/package.json`, `plugin/vibe/gemini-extension.json`, `plugin/vibe/.claude-plugin/plugin.json`, and a correctly structured `plugin/vibe/.codex-plugin/plugin.json`.

MCP server implementation is out of scope for this phase and belongs to Phase 27. Skill/command directory standardization belongs to Phase 26. Workflow and product documentation alignment belongs to Phase 28.

</domain>

<decisions>
## Implementation Decisions

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

### the agent's Discretion
- Exact platform-specific optional fields are left to research and planning, as long as required manifest schemas are satisfied.
- Exact smoke test file name/location is flexible, but it should live near existing plugin scripts/tests and be easy to run with Node.js.
- Exact `engines` version and script names are flexible, but should match existing pure Node.js runtime conventions.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements
- `.planning/ROADMAP.md` — Phase 25 boundary, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — MAN-01 through MAN-04 and traceability for v7.0.
- `.planning/PROJECT.md` — v7.0 plugin/MCP pivot and product-level decisions.
- `.planning/MILESTONE_v7.0-DRAFT.md` — Draft milestone action list naming the target manifest/package files.

### Existing plugin architecture
- `plugin/vibe/README.md` — Current plugin package layout and runtime boundary.
- `plugin/vibe/references/plugin-architecture.md` — Ownership split between skills, commands, references, scripts, and `.vibe`.
- `plugin/vibe/references/workspace-layout.md` — Project-local `.vibe` workspace constraints relevant to plugin packaging.

### Existing manifests and discovery
- `plugin/vibe/.codex-plugin/plugin.json` — Existing Codex manifest that must be reviewed and aligned.
- `.agents/plugins/marketplace.json` — Existing local Codex discovery example pointing to `./plugin/vibe`.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `plugin/vibe/.codex-plugin/plugin.json`: Existing Codex manifest with `name`, `version`, `description`, `skills`, and `interface` fields.
- `.agents/plugins/marketplace.json`: Existing local discovery scaffold for Codex-style plugin installation.
- `plugin/vibe/scripts/*.test.js`: Existing pure Node.js test style that can guide a manifest smoke test.

### Established Patterns
- Plugin package root is `plugin/vibe/`, established by Phase 20 decisions and current files.
- Plugin runtime uses deterministic pure Node.js scripts with native APIs and minimal dependencies.
- Existing plugin files favor model-readable Markdown/JSON over hidden state.

### Integration Points
- `plugin/vibe/package.json` becomes the identity and package-management anchor for all plugin manifests.
- `plugin/vibe/gemini-extension.json` and `plugin/vibe/.claude-plugin/plugin.json` should be added under the plugin root.
- `plugin/vibe/.codex-plugin/plugin.json` should be checked and updated for consistency rather than replaced blindly.
- A Node.js smoke test should validate package/manifest/discovery consistency during planning and execution.

</code_context>

<specifics>
## Specific Ideas

- Keep the phase practical: establish package and manifest surfaces now, then let Phase 27 add MCP SDK dependencies when there is an actual MCP server entry point.
- Prefer local, deterministic verification over requiring provider CLI tools to be installed.
- Use `package.json` as the single place to prevent Gemini, Claude, and Codex metadata from drifting.

</specifics>

<deferred>
## Deferred Ideas

- External marketplace or registry publication belongs to future requirement FUT-01, not Phase 25.
- Real provider CLI loading tests can be added later if the project decides to support environment-specific integration tests.

</deferred>

---

*Phase: 25-universal-manifests-packaging*
*Context gathered: 2026-04-24*
