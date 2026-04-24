# Phase 28: Workflow & Documentation Alignment - Research

**Researched:** 2026-04-24
**Domain:** Workflow Standardization, Documentation, AI SOP
**Confidence:** HIGH

## Summary

Phase 28 致力于将 AI Conductor 和执行者的工作流与 Phase 27 引入的 MCP 工具集对齐。核心目标是更新角色 SOP（Standard Operating Procedures）和项目文档，使 AI Agent 能够识别并优先使用 MCP 工具而非原始 Shell 命令，从而实现更安全、更结构化的协作。

**Primary recommendation:** 
1. 更新 `.vibe/roles/Conductor.md` 和 `.vibe/roles/Worker.md`。
2. 更新全局 `README.md` 和 `.planning/codebase/ARCHITECTURE.md`。
3. 验证 AI Agent 在新 SOP 下的工具发现和使用能力。

## Workflow Mapping (Legacy vs. MCP)

| Legacy Command | MCP Tool | Role | Improvement |
|----------------|----------|------|-------------|
| `vibe spawn` | `vibe_create_task` | Conductor | 结构化输入，避免 shell 转义，自动记录 ID |
| `vibe list` | `vibe_list_tasks` | Both | 纯文本输出优化为结构化响应 |
| `vibe report` | `vibe_acquire_lock` | Worker | 明确意图锁定，防止物理冲突 |
| `vibe signal` | `vibe_skill_run_task` | Worker | 流程化状态流转 |
| `vibe wait` | `vibe_get_status` | Conductor | 轮询/阻塞优化为状态查询 |

## Documentation Inventory

### Target Documents
- `.vibe/roles/Conductor.md`: 核心编排指南。
- `.vibe/roles/Worker.md`: 核心执行指南。
- `README.md`: 项目入口，需体现 "Plugin-first" 和 "MCP" 特性。
- `.planning/codebase/ARCHITECTURE.md`: 需重写以移除 Rust 引用，加入 MCP 服务层。
- `.planning/codebase/STRUCTURE.md`: 需更新以反映 `plugin/vibe/` 的核心地位。

## Key Updates for SOPs

### Conductor.md Updates
- 指导 Conductor 使用 `vibe_create_task` 定义子任务。
- 使用 `vibe_list_tasks` 和 `vibe_get_status` 监控进度。
- 使用 `vibe_skill_plan` 和 `vibe_skill_release_summary` 进行阶段管理。

### Worker.md Updates
- 强调在修改文件前必须调用 `vibe_acquire_lock`。
- 完成任务后调用 `vibe_release_lock`。
- 使用 `vibe_skill_run_task` (如果适用) 或直接汇报状态。

## Architecture Realignment

### New Layer Model
1. **Host Layer**: AI CLI (Gemini, Claude, Codex).
2. **Interface Layer**: MCP Server (`mcp-server.js`) + Manifests.
3. **Logic Layer**: Node.js Scripts (`scripts/*.js`).
4. **Data Layer**: Durable `.vibe/` artifacts (JSON).

## Verification Strategy
- **SOP Validation**: 手动测试 Conductor 是否能根据新 SOP 正确选择工具。
- **Doc Integrity**: 确保所有文档中不再包含对 `vibe-core` (Rust) 的活跃引用（除非标注为 Legacy）。

## Metadata
**Confidence breakdown:**
- Workflow Mapping: HIGH - Based on Phase 27 implementation.
- Target Documents: HIGH - Standard project structure.
- Layer Model: HIGH - Standard MCP architecture.

**Research date:** 2026-04-24
