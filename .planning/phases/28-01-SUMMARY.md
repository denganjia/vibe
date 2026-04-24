---
phase: 28-workflow-alignment
plan: 01
subsystem: documentation
tags:
  - mcp
  - sops
  - architecture
  - workflow
requires:
  - 27-mcp-integration
provides:
  - "Updated orchestration guidelines using MCP"
  - "New system architecture definition without Rust CLI"
affects:
  - ".vibe/roles/Conductor.md"
  - ".vibe/roles/Worker.md"
  - "README.md"
  - ".planning/codebase/ARCHITECTURE.md"
  - ".planning/codebase/STRUCTURE.md"
  - ".planning/ROADMAP.md"
  - ".planning/REQUIREMENTS.md"
tech-stack:
  added: []
  patterns:
    - "MCP Tooling for Workflow"
key-files:
  created: []
  modified:
    - ".vibe/roles/Conductor.md"
    - ".vibe/roles/Worker.md"
    - "README.md"
    - ".planning/codebase/ARCHITECTURE.md"
    - ".planning/codebase/STRUCTURE.md"
    - ".planning/ROADMAP.md"
    - ".planning/REQUIREMENTS.md"
key-decisions:
  - "Replaced legacy `vibe` shell command mentions in Conductor and Worker SOPs with their MCP equivalents."
  - "Completely removed legacy Rust `vibe-core` / `vibe-cli` references from architecture and README.md, shifting entirely to Node.js `plugin/vibe/` based architecture."
metrics:
  duration: "5m"
  completed_date: "2024-10-24"
---

# Phase 28 Plan 01: Workflow & Documentation Alignment Summary

Updated the AI Conductor and Worker SOPs and architecture documents to fully support the Phase 27 MCP toolset.

## Completed Tasks

1. **Task 1: 更新角色 SOP (Conductor & Worker)**
   - Replaced legacy shell commands with `vibe_create_task`, `vibe_get_status`, `vibe_list_tasks`, `vibe_acquire_lock`, and `vibe_release_lock` in `.vibe/roles/Conductor.md` and `.vibe/roles/Worker.md`.
   - Commit: 7ec0202

2. **Task 2: 更新架构与结构文档**
   - Refactored `ARCHITECTURE.md`, `STRUCTURE.md`, and `README.md` to establish the Plugin-First (MCP) architecture.
   - Rust crates and apps have been explicitly archived and marked as legacy.
   - Commit: 8219424

3. **Task 3: 最终验证与清理**
   - Updated `.planning/ROADMAP.md` and `.planning/REQUIREMENTS.md` to mark Phase 28 and WF-01, WF-02 as completed.
   - Commit: 0c35582

## Deviations from Plan

None - plan executed exactly as written.

## Threat Flags

None - changes are purely documentation aligning with existing functionality.

## Self-Check: PASSED
- `Conductor.md` and `Worker.md` are updated to use MCP tools.
- Architecture docs accurately describe the plugin-first Node.js approach.
- State is correctly tracked and plan goals are met.