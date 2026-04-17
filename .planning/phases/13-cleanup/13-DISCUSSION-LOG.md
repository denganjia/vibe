# Phase 13: 架构清理与瘦身 (Cleanup) - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-17
**Phase:** 13-cleanup
**Areas discussed:** SQLite removal, MCP removal, Protocol simplification, CLI flags, Project structure

---

## Strategic Pivot Alignment

| Area | Decision | Consensus |
|------|----------|-----------|
| **Database** | Completely remove SQLite and DB Actors | ✓ Yes |
| **MCP Server** | Completely remove `mcp.rs` and related routes | ✓ Yes |
| **Approval Flow** | Remove all command-level approval gates and flags (`--yes`) | ✓ Yes |
| **Project Context** | Use `.vibe/` directory with automatic initialization | ✓ Yes |
| **Messaging** | Simplify IPC to support async `signal/wait` | ✓ Yes |

**Summary**: 
用户确认了“不需要 DB”和“不需要 MCP”的激进瘦身计划。这标志着 `vibe-cli` 从一个繁琐的管控工具向轻量级开发总线的正式转型。重点转向基于文件系统的持久化和基于 UDS 的信号同步。

---

## Protocol Refinement

- Removed: `ApprovalRequest`, `ApprovalResult`, `GateRequest`, `GateResponse`.
- Kept: `Register`, `Heartbeat`, `Report`, `ExecuteIntent`.
- Future (Phase 14): `Signal`, `Wait`.

## CLI Cleanup

- Commands to be removed: `vibe mcp`.
- Flags to be removed: `--yes` from `vibe run` and `vibe inject`.
- Implementation change: `vibe run` and `vibe split` should trigger `.vibe` folder creation if missing.

---

## Actionable Outcomes (Claude's Discretion)

- Remove `rusqlite` dependencies from `Cargo.toml`.
- Ensure `vibe status` still works by reading from memory/file instead of DB.
- Standardize the `.vibe/` directory structure.
