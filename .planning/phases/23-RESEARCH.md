# Phase 23: Multi-model Execution and Review Loop - Research

**Researched:** 2026-04-23
**Domain:** Agent Orchestration & Workflow Automation
**Confidence:** HIGH

## Summary

This research defines the implementation path for a structured, multi-model agentic workflow in the Vibe-CLI ecosystem. It focuses on transforming the current terminal-pane-centric orchestration into a **Plugin-First**, artifact-driven architecture. 

The core of this phase is the **Conductor Operating Loop**: a repeatable cycle where the Conductor (LLM) clarifies user goals, plans a series of tasks with explicit dependencies, dispatches them to specialized Agents (Executor/Reviewer), and manages the lifecycle including review-fix loops and interruption recovery.

**Primary recommendation:** Use a "Checkpointing Pattern" where every status transition and dependency check is recorded in `.vibe` JSON artifacts, ensuring the Conductor can resume from any point without losing context or wasting tokens.

## User Constraints (from CONTEXT.md)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### 1. 澄清机制 (Clarification Strategy)
- **保守计划优先**：Conductor 必须确保 `goal`, `file_scope` 和 `verification` 明确后才写入 `task.json`。
- **持久化上下文**：重要的澄清背景应写入 `.vibe/planning_notes.md`，Executor 启动时需将其作为参考文档读取。

#### 2. 任务拆分与依赖管理 (Task Decomposition & Dependencies)
- **显式依赖字段**：在 `task.json` 中引入 `dependencies: string[]`（存储任务 ID）。
- **静态拓扑生成**：Conductor 倾向于在 `plan` 阶段一次性生成所有任务 JSON，以便用户和模型能预览完整流程。
- **阻塞逻辑**：运行时脚本（或 `run.js` 的前置检查）若发现依赖任务未达到 `completed` 状态，应拒绝执行并将当前任务标记为 `blocked`。

#### 3. 审查与修复循环 (Review & Fix Loop)
- **结构化反馈**：Reviewer 产出的 Findings 存放在 `.vibe/reviews/<task-id>_run_<run-id>.json`。
- **状态驱动修复**：
    - Reviewer 发现问题时将任务设为 `fix-needed`。
    - Conductor 负责将 Findings 汇总到 `task.json` 的 `context` 或 `constraints` 中，并重置状态为 `queued` 以触发重跑。
- **安全阈值**：单个任务的修复循环上限暂定为 **3 次**。达到上限后状态转为 `failed`，需人工干预。

#### 4. 中断恢复与计划管理 (Recovery & Plan Management)
- **Plan Manifest**：引入 `.vibe/plan.json` 记录当前 Plan 的元数据（ID, tasks[], goal）。
- **自愈逻辑**：Conductor 启动时需对比 `plan.json`、`tasks/*.json` 和 `locks/`。若发现任务状态为 `running` 但没有对应进程或锁，应将其标记为 `interrupted` 并提示恢复。

### User Constraints
- 严禁引入外部数据库或中心化 Daemon。
- 所有协作状态必须对齐 `task-contract.md` 和 `agent-contract.md`。

### the agent's Discretion
- [None explicitly listed in CONTEXT.md, but the following are researched as best practices]
- Implementation of `plan.json` schema.
- Logic for `review.js` findings aggregation.
- Topological execution order algorithm (simple greedy approach).

### Deferred Ideas (OUT OF SCOPE)
- 跨项目的任务依赖。
- 复杂的动态计划调整（执行中途大幅度增删任务）。
</user_constraints>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| COND-01 | Conductor Skill design (Clarify/Plan/Dispatch) | Defines the Operating Loop and artifact structure. |
| DEP-01 | Task dependency handling | Proposes `dependencies` field and `run.js` pre-check logic. |
| REV-01 | Review loop and safety limits | Defines `review.js` logic and 3-cycle retry counter. |
| RECOV-01 | Interruption recovery | Proposes `plan.json` manifest and sync logic. |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Node.js | v22.x | Runtime | Project's current script runtime [VERIFIED: local] |
| crypto | Built-in | ID Generation | Secure and lightweight for task/run IDs [VERIFIED: node docs] |
| fs/path | Built-in | File I/O | Handles all artifact persistence [VERIFIED: node docs] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|--------------|
| child_process | Built-in | Subprocess Execution | Running Agent model commands [VERIFIED: node docs] |

## Architecture Patterns

### Recommended Project Structure
```
.vibe/
├── plan.json           # New: Current plan manifest
├── planning_notes.md   # New: Clarification context
├── tasks/              # Existing: Task definitions (with dependencies)
├── runs/               # Existing: Run results
├── reviews/            # New: Structured reviewer findings
├── logs/               # Existing: Stdout/stderr logs
├── agents/             # Existing: Agent definitions
└── locks/              # Existing: File-scope locks
```

### Pattern 1: Checkpointed State Machine
**What:** Every transition in the task lifecycle (queued -> running -> review-needed -> ...) is persisted to the `.vibe/tasks/<id>.json` file.
**When to use:** All task executions.
**Example:**
```json
// .vibe/tasks/task1.json
{
  "id": "task1",
  "status": "queued",
  "dependencies": ["init_task"],
  "run_count": 0,
  "context": ["Ref: planning_notes.md"]
}
```

### Pattern 2: Conservative Planning (Conductor Skill)
**What:** The Conductor must satisfy a checklist before creating tasks:
1. Is the `goal` binary (pass/fail)?
2. Is the `file_scope` explicitly listed?
3. Is the `verification` command runnable in this environment?
**If NO to any:** Emit a clarification question instead of `plan.json`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Centralized Scheduler | Background Daemon | Simple `run.js` loop | Project constraint against daemons. |
| Complex DB | SQLite/MongoDB | Filesystem JSON | Maintain portability and "vibe" of local-first development. |

## Runtime State Inventory

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | `tasks/*.json`, `plan.json` | `status.js` and Conductor must keep these in sync. |
| Live service config | None | N/A |
| OS-registered state | `.vibe/locks/*.lock` | Sync logic must check lock existence to detect stale `running` tasks. |
| Secrets/env vars | Agent allowlisted env vars | `run.js` must continue to redact these from logs. |
| Build artifacts | None | N/A |

## Common Pitfalls

### Pitfall 1: Deadlocks in Dependencies
**What goes wrong:** Task A depends on B, and B depends on A (Circular dependency).
**Prevention strategy:** Conductor MUST validate acyclicity during the `plan` phase. A simple DFS check during `plan.json` generation is sufficient.

### Pitfall 2: Stale Locks on Crash
**What goes wrong:** An Agent process crashes, leaving a `.lock` file. Other tasks remain `blocked` forever.
**Prevention strategy:** `run.js` or a new `sync.js` should check if the PID associated with a lock (optionally added to lock file) is still active.

## Code Examples

### 1. Dependency Check in `run.js`
```javascript
// Source: Proposed implementation based on task-contract.md
function canRun(task, workspaceRoot) {
  if (!task.dependencies || task.dependencies.length === 0) return true;
  
  for (const depId of task.dependencies) {
    const depPath = path.join(workspaceRoot, '.vibe', 'tasks', `${depId}.json`);
    if (!fs.existsSync(depPath)) return false; // Missing dependency
    const dep = JSON.parse(fs.readFileSync(depPath, 'utf8'));
    if (dep.status !== 'completed') return false;
  }
  return true;
}
```

### 2. Review Loop Safety Limit
```javascript
// Source: Derived from Phase 23 CONTEXT.md
function handleReviewResult(taskId, findings, workspaceRoot) {
  const task = getTask(taskId);
  const currentRetries = task.run_count || 0;
  
  if (findings.some(f => f.severity === 'high' || f.severity === 'critical')) {
    if (currentRetries >= 3) {
      updateTask(taskId, { status: 'failed', error: 'Max review cycles reached' });
    } else {
      updateTask(taskId, { 
        status: 'fix-needed', 
        run_count: currentRetries + 1,
        context: [...task.context, `Findings: ${JSON.stringify(findings)}`]
      });
    }
  } else {
    updateTask(taskId, { status: 'completed' });
  }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Terminal Panes | Subprocess Agents | Phase 20 | Higher reliability, easier log capture, no UI dependency. |
| Implicit Plan | `plan.json` Manifest | Phase 23 | Enables interruption recovery and cross-model handoff. |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Tasks are idempotent enough to resume from `queued` | Interruption Recovery | Redundant work or state corruption if side effects aren't handled. |
| A2 | Conductor can reliably parse structured review findings | Review Loop | Conductor might ignore critical issues if the parser fails. |

## Open Questions (RESOLVED)

1. **How to handle partial file edits?** 
   - If an Agent crashes mid-file-edit, the file might be corrupted. 
   - **RESOLVED:** 运行时脚本（`run.js`）不负责原子写保护，因为这会显著增加复杂度。Conductor 在恢复（`sync`）过程中，如果发现任务状态异常，应提示用户检查 `file_scope` 中的文件完整性。同时，建议用户在生产环境中使用 Git 作为底层“撤销键”。

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | Script execution | ✓ | v22.16.0 | — |
| vibe-cli | Orchestration | ✓ | current | — |

## Sources

### Primary (HIGH confidence)
- `plugin/vibe/references/task-contract.md` - Core task schema.
- `plugin/vibe/references/agent-contract.md` - Agent subprocess execution.
- `plugin/vibe/references/review-protocol.md` - Structured findings definition.

### Secondary (MEDIUM confidence)
- Google Search: "agent review loop safety limits" - Verified standard retry patterns.
- Google Search: "file-based workflow state machine" - Verified checkpointing patterns.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Using built-in Node.js.
- Architecture: HIGH - Aligns with existing "Plugin-First" docs.
- Pitfalls: MEDIUM - Based on general distributed systems experience.

**Research date:** 2026-04-23
**Valid until:** 2026-05-23
