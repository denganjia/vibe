# Milestone 7.0 Requirements: Universal Plugin & MCP Integration

**Defined:** 2026-04-23
**Core Value:** 打破 AI 与本地开发环境之间的“次元壁”，将终端从单纯的字符输入框升级为分布式 AI 协作的物理调度室。

## v7.0 Requirements

### Manifests

- [ ] **MAN-01**: Initialize package.json in plugin/vibe/ to manage MCP SDK dependencies
- [ ] **MAN-02**: Create gemini-extension.json for Gemini CLI compatibility
- [ ] **MAN-03**: Create .claude-plugin/plugin.json for Claude Code compatibility
- [ ] **MAN-04**: Ensure .codex-plugin/plugin.json aligns with Codex CLI standards

### Skills

- [ ] **SKL-01**: Standardize SKILL.md files to include cross-platform YAML frontmatter
- [ ] **SKL-02**: Reorganize and clean up legacy commands/ and roles/ to strictly follow skills/<skill-name>/SKILL.md structure

### MCP Server

- [ ] **MCP-01**: Implement a lightweight Node.js MCP server inside plugin/vibe/
- [ ] **MCP-02**: Convert file locking logic to an MCP tool (e.g. vibe_acquire_lock, vibe_release_lock)
- [ ] **MCP-03**: Convert task creation logic to an MCP tool (e.g. vibe_write_task)
- [ ] **MCP-04**: Convert release summary generation to an MCP tool (e.g. vibe_generate_release)
- [ ] **MCP-05**: Register the MCP server in all three plugin manifest files

### Workflow

- [ ] **WF-01**: Update Conductor SOP to utilize MCP tools instead of raw shell commands
- [ ] **WF-02**: Update project README.md and architecture documentation to reflect MCP transition

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
| MAN-01 | TBD | Pending |
| MAN-02 | TBD | Pending |
| MAN-03 | TBD | Pending |
| MAN-04 | TBD | Pending |
| SKL-01 | TBD | Pending |
| SKL-02 | TBD | Pending |
| MCP-01 | TBD | Pending |
| MCP-02 | TBD | Pending |
| MCP-03 | TBD | Pending |
| MCP-04 | TBD | Pending |
| MCP-05 | TBD | Pending |
| WF-01 | TBD | Pending |
| WF-02 | TBD | Pending |

**Coverage:**
- v7.0 requirements: 13 total
- Mapped to phases: 0
- Unmapped: 13

---
*Requirements defined: 2026-04-23*
*Last updated: 2026-04-23 after v7.0 MCP pivot*
