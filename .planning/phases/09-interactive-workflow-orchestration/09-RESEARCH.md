# Phase 09: Interactive Workflow Orchestration - Research

**Researched:** 2026-04-16
**Domain:** Rust, MCP, TUI (Ratatui), Async IPC (Tokio)
**Confidence:** HIGH

## Summary

Phase 09 introduces the Interactive Workflow Orchestration feature, allowing AI agents to submit multi-step plans that pause execution until human approval is granted. Based on the decisions made, this system relies on an asynchronous polling pattern for the MCP Server to prevent LLM timeouts, local Markdown files for plan persistence, and Worker-pane standard input for user interaction.

**Primary recommendation:** Implement `vibe_submit_plan` and `vibe_query_approval` MCP tools using immediate-return pending states, and extend the IPC protocol to broadcast `ApprovalRequest` and `ApprovalResult` events between the Master and Worker panes.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** 在 Worker 终端窗格内直接进行审批交互 (利用现有的 stdin 接收用户输入，方便显示长计划路径及文本)。
- **D-02:** MCP 工具采用“立即返回并轮询状态”的设计。AI 提交计划后工具立即返回 pending，AI 需要调用查状态工具轮询结果，避免 MCP Server 或大模型连接超时。
- **D-03:** 提交的详细计划内容保存为本地 Markdown 文件，TUI 和 Worker 终端中仅提示路径供用户自行打开审阅。
- **D-04:** 允许用户在拒绝计划时输入附加原因文本，此原因将在 AI 下一次轮询时作为反馈返回给模型，构成闭环。

### Claude's Discretion
无。所有讨论项已明确决策。

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SCO-01 | Controlled Workflow (Multi-node task flow, human confirmation, MCP extension, TUI feedback) | Addressed via asynchronous MCP tools (`vibe_submit_plan`, `vibe_query_approval`), stdin-based human validation in the worker pane, and dashboard broadcasting in `ratatui`. |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `tokio` | workspace | Async runtime & IO | Standard async framework in the Rust ecosystem |
| `serde_json` | workspace | JSON-RPC 2.0 payloads | Essential for standardizing MCP tool IO parsing |
| `ratatui` | workspace | TUI Rendering | Modern standard for Rust TUI |
| `tempfile` | workspace | Plan file creation | Safe temporary file creation for markdown plans |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `dialoguer` | workspace | Terminal Prompting | Useful for handling the yes/no/reason stdin logic gracefully in the Worker pane |

## Architecture Patterns

### Recommended Project Structure
```text
crates/vibe-core/src/ipc/
├── protocol.rs        # Add ApprovalRequest, ApprovalResponse to Event enum
apps/vibe-cli/src/
├── mcp.rs             # Add vibe_submit_plan & vibe_query_approval routes
├── tui.rs             # Add "Waiting for Approval" visual state
```

### Pattern 1: Asynchronous MCP Tool Execution
**What:** Returning a `pending` status immediately instead of keeping the JSON-RPC request open indefinitely.
**When to use:** For any MCP tool that requires human intervention or takes longer than the standard LLM timeout (usually >30s).
**Example:**
```rust
// [CITED: MCP best practices for human-in-the-loop]
pub fn handle_submit_plan(params: SubmitPlanParams) -> Result<ToolResponse> {
    let plan_path = save_plan_to_markdown(&params.plan_text)?;
    let request_id = generate_approval_request_id();
    
    // Broadcast to Master -> Worker to prompt user
    broadcast_approval_request(request_id, plan_path.clone());
    
    Ok(ToolResponse::json(json!({
        "status": "pending",
        "request_id": request_id,
        "message": format!("Plan saved to {}. Please poll vibe_query_approval with request_id.", plan_path)
    })))
}
```

### Anti-Patterns to Avoid
- **Blocking the MCP Server Thread:** Never use standard synchronous channels (`std::sync::mpsc::Receiver::recv`) in the MCP async task to wait for user approval, which will block the async executor or timeout the client connection.
- **Large Payloads in State:** Do not save the entire plan content in the SQLite state database. Stick to the D-03 decision: store it in a local Markdown file and only pass around the file path.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Polling/Timeout handling | Blocking JSON-RPC responses | Immediate return + `vibe_query_approval` tool | LLM clients will drop connections on long stalls |
| Terminal Input Prompts | Raw byte parsing on `stdin` | `dialoguer` (already in workspace) | Handles cross-platform TTY nuances, line editing, and fallback logic cleanly |

**Key insight:** The user interaction in a spawned worker pane needs careful management of stdin hijacking. If the worker is actively running a shell, you must inject the prompt into the PTY or manage standard input carefully to avoid input collision.

## Common Pitfalls

### Pitfall 1: Worker TTY Input Collision
**What goes wrong:** The AI agent might have launched a background process in the pane, and prompting the user for Approval via `stdin` conflicts with the shell or running process.
**Why it happens:** Standard input is shared.
**How to avoid:** Ensure the worker agent loop pauses command execution while in the `PendingApproval` state.

### Pitfall 2: MCP Connection Timeout
**What goes wrong:** The AI client drops the connection or errors out.
**Why it happens:** The MCP server holds the HTTP/stdio request open waiting for human approval.
**How to avoid:** strictly adhere to Decision D-02: return instantly and make the AI poll.

## Code Examples

### State Broadcast for TUI
```rust
// In vibe-core/src/ipc/protocol.rs
#[derive(Serialize, Deserialize, Debug)]
pub enum IpcMessage {
    // ... existing variants
    ApprovalRequest {
        pane_id: String,
        plan_path: String,
    },
    ApprovalResult {
        pane_id: String,
        approved: bool,
        reason: Option<String>,
    }
}
```

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust Toolchain | Compilation | ✗ (Path issue) | — | Document path fixes in CI |
| WezTerm CLI | Terminal Orchestration | ✓ | 20240203 | Tmux |
| Tmux | Terminal Orchestration | ✗ | — | Wezterm |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` |
| Config file | `Cargo.toml` |
| Quick run command | `cargo test -p vibe-cli --lib mcp` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SCO-01 | MCP tool returns pending instantly | unit | `cargo test test_mcp_submit_plan` | ❌ Wave 0 |
| SCO-01 | State evolves to PendingApproval | unit | `cargo test test_state_approval_transition` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p vibe-cli`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps
- [ ] `apps/vibe-cli/src/mcp.rs` tests for the new JSON-RPC routes
- [ ] `crates/vibe-core/src/ipc/protocol.rs` serialization tests for new events

## Sources

### Primary (HIGH confidence)
- `crates/vibe-core/Cargo.toml` - Verified existing libraries like `dialoguer`, `tokio`, `serde`
- `.planning/phases/09-interactive-workflow-orchestration/09-CONTEXT.md` - Explicit user directives and codebase integration points
- `apps/vibe-cli/Cargo.toml` - Ratatui availability for UI implementation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Read directly from `Cargo.toml`
- Architecture: HIGH - Derived strictly from `09-CONTEXT.md` decisions
- Pitfalls: HIGH - TTY input and async blocking are standard Rust/CLI constraints

**Research date:** 2026-04-16
**Valid until:** Stable
