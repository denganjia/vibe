# ROADMAP

## Phases

- [x] **Phase 25: Universal Manifests & Packaging** - Set up plugin structure and manifest files for major AI CLIs (completed 2026-04-24)
- [ ] **Phase 26: Skill Standardization** - Unify skill metadata and folder structures across platforms
- [ ] **Phase 27: MCP Server Integration** - Replace raw shell scripts with a standardized MCP server and tools
- [ ] **Phase 28: Workflow & Documentation Alignment** - Update AI instructions and project docs to leverage MCP tools

## Phase Details

### Phase 25: Universal Manifests & Packaging
**Goal**: Make the plugin recognizable by major AI CLIs and set up package management
**Depends on**: Nothing
**Requirements**: MAN-01, MAN-02, MAN-03, MAN-04
**Success Criteria** (what must be TRUE):
  1. `package.json` exists in `plugin/vibe/` for dependency management.
  2. `gemini-extension.json` exists for Gemini CLI integration.
  3. `.claude-plugin/plugin.json` exists for Claude Code integration.
  4. `.codex-plugin/plugin.json` is correctly structured for Codex CLI standards.
**Plans**: 1 plan
Plans:
- [x] `25-01-PLAN.md` — Establish `plugin/vibe/package.json` as the source of truth, add Gemini/Claude manifests, align the Codex manifest, and lock local discovery with a deterministic Node.js smoke test.

### Phase 26: Skill Standardization
**Goal**: Unify skill and command definitions across platforms using YAML frontmatter
**Depends on**: Phase 25
**Requirements**: SKL-01, SKL-02
**Success Criteria** (what must be TRUE):
  1. All SKILL.md files include standardized cross-platform YAML frontmatter.
  2. Legacy `commands/` and `roles/` directories are removed or fully migrated into `skills/<skill-name>/SKILL.md` structures.
**Plans**: TBD

### Phase 27: MCP Server Integration
**Goal**: Replace raw shell script executions with standard, safe Model Context Protocol (MCP) tool calls
**Depends on**: Phase 26
**Requirements**: MCP-01, MCP-02, MCP-03, MCP-04, MCP-05
**Success Criteria** (what must be TRUE):
  1. A local lightweight Node.js MCP server is running and accessible within the workspace.
  2. File locking, task creation, and release summary operations are exposed as MCP tools rather than independent shell scripts.
  3. The MCP server is correctly registered in the manifests for Gemini, Claude, and Codex extensions.
**Plans**: TBD

### Phase 28: Workflow & Documentation Alignment
**Goal**: Ensure the AI Conductor uses the new MCP tools and all documentation reflects the new architecture
**Depends on**: Phase 27
**Requirements**: WF-01, WF-02
**Success Criteria** (what must be TRUE):
  1. Conductor SOP explicitly instructs the model to use MCP tools for workspace operations.
  2. Project `README.md` and `.planning/` architecture docs accurately describe the MCP-based plugin-first model.
**Plans**: TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 25. Universal Manifests & Packaging | 1/1 | Complete    | 2026-04-24 |
| 26. Skill Standardization | 0/0 | Not started | - |
| 27. MCP Server Integration | 0/0 | Not started | - |
| 28. Workflow & Documentation Alignment | 0/0 | Not started | - |
