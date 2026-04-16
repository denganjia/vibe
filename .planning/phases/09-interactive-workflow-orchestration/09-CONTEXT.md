# Phase 09: Interactive Workflow Orchestration - Context

**Gathered:** 2026-04-16
**Status:** Ready for planning

<domain>
## Phase Boundary

实现关键节点的计划提交与人工二次确认机制 (Interactive Workflow Orchestration)。确保 AI 提交复杂多步任务计划时，流程能被挂起并在人工确认后继续。
</domain>

<decisions>
## Implementation Decisions

### 审批交互位置 (Approval UX)
- **D-01:** 在 Worker 终端窗格内直接进行审批交互 (利用现有的 stdin 接收用户输入，方便显示长计划路径及文本)。

### 阻塞机制 (Blocking Mechanism)
- **D-02:** MCP 工具采用“立即返回并轮询状态”的设计。AI 提交计划后工具立即返回 pending，AI 需要调用查状态工具轮询结果，避免 MCP Server 或大模型连接超时。

### 计划展示方式 (Plan Presentation)
- **D-03:** 提交的详细计划内容保存为本地 Markdown 文件，TUI 和 Worker 终端中仅提示路径供用户自行打开审阅。

### 拒绝处理 (Rejection Handling)
- **D-04:** 允许用户在拒绝计划时输入附加原因文本，此原因将在 AI 下一次轮询时作为反馈返回给模型，构成闭环。

### Claude's Discretion
无。所有讨论项已明确决策。

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Core Requirements
- `.planning/REQUIREMENTS.md` — SCO-01 (Controlled Workflow)

No external specs — requirements fully captured in decisions above.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `vibe-core/src/ipc/client.rs`: 现有的 Worker 5秒心跳机制，可用于同步 Pending/Approval 状态。
- `apps/vibe-cli/src/tui.rs`: 仪表盘订阅广播，可直接在表格中标记 "Waiting for Approval" 状态字样。
- `apps/vibe-cli/src/mcp.rs`: 已具备 JSON-RPC 2.0 基础支持，可直接增加 `vibe_submit_plan` 与 `vibe_query_approval`。

### Established Patterns
- State 变动通过 `DbActor` 串行处理保证 SQLite 并发安全。
- 采用 UDS `LinesCodec` 与 NDJSON 进行跨进程通讯。

### Integration Points
- Master Server 需识别 Pending 状态的 Pane，并将该状态广播给 TUI。
- MCP 处理路由需要增加处理新请求类型的逻辑。
</code_context>

<specifics>
## Specific Ideas

无特殊偏好，按上述决策标准实现。
</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.
</deferred>

---

*Phase: 09-interactive-workflow-orchestration*
*Context gathered: 2026-04-16*