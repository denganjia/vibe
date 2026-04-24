# Milestone 7.0 Requirements: Universal Plugin & MCP Integration

**Defined:** 2026-04-23
**Core Value:** 打破 AI 与本地开发环境之间的“次元壁”，将终端从单纯的字符输入框升级为分布式 AI 协作的物理调度室。

## v7.0 Requirements

### Manifests

- [x] **MAN-01**: Initialize package.json in plugin/vibe/ to manage MCP SDK dependencies
- [x] **MAN-02**: Create gemini-extension.json for Gemini CLI compatibility
- [x] **MAN-03**: Create .claude-plugin/plugin.json for Claude Code compatibility
- [x] **MAN-04**: Ensure .codex-plugin/plugin.json aligns with Codex CLI standards

### Skills

- [x] **SKL-01**: Standardize SKILL.md files to include cross-platform YAML frontmatter
- [x] **SKL-02**: Reorganize and clean up legacy commands/ and roles/ to strictly follow skills/<skill-name>/SKILL.md structure

### MCP Server

- [ ] **MCP-01**: Implement a lightweight Node.js MCP server inside plugin/vibe/
- [x] **MCP-02**: Convert file locking logic to an MCP tool (e.g. vibe_acquire_lock, vibe_release_lock)
- [x] **MCP-03**: Convert task creation logic to an MCP tool (e.g. vibe_write_task)
- [x] **MCP-04**: Convert release summary generation to an MCP tool (e.g. vibe_generate_release)
- [ ] **MCP-05**: Register the MCP server in all three plugin manifest files

### Workflow

- [x] **WF-01**: Update Conductor SOP to utilize MCP tools instead of raw shell commands
- [x] **WF-02**: Update project README.md and architecture documentation to reflect MCP transition

## Future Requirements

- [ ] **FUT-01**: Publish plugin to external marketplace or registry
- [ ] **FUT-02**: Build a more comprehensive MCP tool suite covering detailed file diffs

## Out of Scope

| Feature | Reason |
|---------|--------|
| Rust Daemon | Completely removed in v7.0 transition. |
| Global Sync Database | Abandoned for local file persistence inside `.vibe`. |

## Traceability

Milestone 7.0 requirements are mapped to phases below. Each v7.0 requirement belongs to exactly one phase.

| Requirement | Phase | Status |
|-------------|-------|--------|
| MAN-01 | Phase 25 | Complete |
| MAN-02 | Phase 25 | Complete |
| MAN-03 | Phase 25 | Complete |
| MAN-04 | Phase 25 | Complete |
| SKL-01 | Phase 26 | Complete |
| SKL-02 | Phase 26 | Complete |
| MCP-01 | Phase 27 | Pending |
| MCP-02 | Phase 27 | Complete |
| MCP-03 | Phase 27 | Complete |
| MCP-04 | Phase 27 | Complete |
| MCP-05 | Phase 27 | Pending |
| WF-01 | Phase 28 | Complete |
| WF-02 | Phase 28 | Complete |

**Coverage:**
- v7.0 requirements: 13 total
- Mapped to phases: 13
- Unmapped: 0

---
*Requirements defined: 2026-04-23*
*Last updated: 2026-04-23 after v7.0 MCP pivot*
