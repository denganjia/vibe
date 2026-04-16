# Phase 09: Interactive Workflow Orchestration - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-16
**Phase:** 09-Interactive Workflow Orchestration
**Areas discussed:** 审批交互位置 (Approval UX), 阻塞机制 (Blocking Mechanism), 计划展示方式 (Plan Presentation), 拒绝处理 (Rejection Handling)

---

## 审批交互位置 (Approval UX)

| Option | Description | Selected |
|--------|-------------|----------|
| Worker 终端窗格内 (推荐) | 利用现有的 stdin 接收用户输入，适合显示详细内容并允许输入长文本的修改意见。 | ✓ |
| TUI 面板快捷键 | 通过快捷键快速响应，但难以输入长文本。 | |
| 由你决定 (You decide) | | |

**User's choice:** Worker 终端窗格内 (推荐)
**Notes:** -

---

## 阻塞机制 (Blocking Mechanism)

| Option | Description | Selected |
|--------|-------------|----------|
| 立即返回并轮询状态 (推荐) | AI 工具立即返回 'pending' 状态，主动轮询获取结果。 | ✓ |
| 保持连接长阻塞 | MCP 接收请求后长连接阻塞。 | |
| 由你决定 (You decide) | | |

**User's choice:** 立即返回并轮询状态 (推荐)
**Notes:** -

---

## 计划展示方式 (Plan Presentation)

| Option | Description | Selected |
|--------|-------------|----------|
| 保存为本地 Markdown 文件 (推荐) | 存入临时文件，终端展示路径。 | ✓ |
| 直接输出到终端缓冲区 | 直接打印，不利于长计划的查看。 | |
| 由你决定 (You decide) | | |

**User's choice:** 保存为本地 Markdown 文件 (推荐)
**Notes:** -

---

## 拒绝处理 (Rejection Handling)

| Option | Description | Selected |
|--------|-------------|----------|
| 允许输入附加原因 (推荐) | 终端输入修改意见，AI 轮询获取该反馈。 | ✓ |
| 仅返回拒绝状态 | 仅输入 N。 | |
| 由你决定 (You decide) | | |

**User's choice:** 允许输入附加原因 (推荐)
**Notes:** -

---

## Claude's Discretion

None

## Deferred Ideas

None