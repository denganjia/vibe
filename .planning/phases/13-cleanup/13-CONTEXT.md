# Phase 13: 架构清理与瘦身 (Cleanup) - Context

**Gathered:** 2026-04-17
**Status:** Ready for planning

<domain>
## Phase Boundary

本阶段的目标是将 `vibe-cli` 从现有的复杂架构中解脱出来。我们将彻底移除 SQLite 数据库依赖、MCP 服务模块以及所有与“人工审批命令注入”相关的拦截逻辑。这为接下来的“信号总线”架构扫清障碍，并确立基于 `.vibe` 目录的轻量级自治工作模式。

</domain>

<decisions>
## Implementation Decisions

### 1. 彻底移除持久化数据库 (DB Removal)
- **D-01: 移除 SQLite**: 删除 `vibe-core` 中所有关于 `rusqlite` 和 `rusqlite_migration` 的代码。
- **D-02: 移除 DB Actor**: 删除 `DbActor` 和 `DbHandle` 等异步数据库处理层。
- **D-03: 文件系统持久化**: 原本由数据库承担的“窗格追踪”和“状态维护”职能，将暂时由内存中的 Hashmap（运行时）或 `.vibe/state/`（持久化）接管。

### 2. 移除 AI 协议冗余 (MCP Removal)
- **D-04: 物理删除 mcp.rs**: 彻底删除 `apps/vibe-cli/src/mcp.rs`。
- **D-05: 移除 MCP 命令**: 在 `main.rs` 中删除 `vibe mcp` 指令路由。

### 3. IPC 协议精简 (Protocol Slimdown)
- **D-06: 裁剪消息类型**: 从 `Message` 枚举中删除 `ApprovalRequest`, `ApprovalResult`, `GateRequest`, `GateResponse`。
- **D-07: 简化 WorkerState**: 移除 `approval_status`, `plan_path`, `rejection_reason` 等与审批流相关的字段。
- **D-08: 引入总线占位**: 为 Phase 14 预留 `Signal(SignalInfo)` 和 `Wait(WaitInfo)` 消息枚举项。

### 4. 自治模式切换 (Autonomy)
- **D-09: 移除信任标志**: 移除 `run` 和 `inject` 指令中的 `--yes` 参数。系统默认信任所有代理间的指令传递。
- **D-10: 简化注入逻辑**: `inject` 不再经过 Master 的审批检查，直接透传指令。

### 5. 项目初始化逻辑 (Auto-Init)
- **D-11: .vibe 自动创建**: 在执行任何 `vibe` 关键命令（如 `spawn` 或 `run`）时，若当前目录不存在 `.vibe`，则自动创建包含 `roles/` 和 `state/` 的标准结构。

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### 待清理的核心模块
- `apps/vibe-cli/src/main.rs` — 包含指令路由与命令行参数。
- `crates/vibe-core/src/ipc/protocol.rs` — 定义了消息契约。
- `crates/vibe-core/src/state/mod.rs` — 现有的 SQLite 逻辑所在地。
- `crates/vibe-core/Cargo.toml` — 需清理数据库相关的依赖。

</canonical_refs>

<code_context>
## Existing Code Insights

### Items to be Deleted
- `apps/vibe-cli/src/mcp.rs`
- `crates/vibe-core/src/state/db.rs`
- `crates/vibe-core/src/state/plans.rs` (或重构为纯文件操作)

### Integration Points
- `vibe-core/src/env.rs` 已经有 `resolve_state_dir`。我们需要新增一个 `resolve_project_vibe_dir` 来处理本地项目路径。

</code_context>

<deferred>
## Deferred Ideas

- 跨主机的信号传递（目前仅限本地 UDS）。
- `.vibe` 目录的版本冲突自动合并。

</deferred>

---

*Phase: 13-cleanup*
*Context gathered: 2026-04-17*
