# ARCHITECTURE

**Domain:** Terminal AI Agent Orchestration (vibe-cli)
**Researched:** 2024-05-24

## Recommended Architecture

The new capabilities (Skills Definitions, Multi-model Workflows, Cross-checking) will build on top of the existing Master-Worker-TUI triangular topology and SQLite state management.

### Component Boundaries

| Component | Status | Responsibility | Communicates With |
|-----------|--------|----------------|-------------------|
| `SkillsRegistry` | **NEW** | Parses and loads declarative AI skill definitions (e.g., YAML) into executable tool schemas. | `MCP Server`, `Master Server` |
| `WorkflowEngine` | **NEW** | Executes directed graphs (DAGs) of tasks, handling state transitions, step passing, and dependencies. | `StateStore`, `Master Server` |
| `MCP Server` | *Modified* | Dynamically exposes tools from `SkillsRegistry` and endpoints to interact with `WorkflowEngine`. | `SkillsRegistry`, `LLM Clients` |
| `StateStore (SQLite)` | *Modified* | Schema updates to persist workflow definitions, step execution state, and cross-check results. | `WorkflowEngine`, `TUI`, `Master Server` |
| `TUI Dashboard` | *Modified* | New views to visualize workflow progress (DAG/steps) and cross-check validation statuses. | `Master Server (via UDS)` |
| `Worker Client` | *Modified* | Executes specific workflow steps; captures output for cross-checking evaluation. | `Master Server (via UDS)` |

### Data Flow

1. **Skill Loading**: On startup, `SkillsRegistry` loads `.yaml` definitions from a standard directory (`~/.local/share/vibe/skills`). `MCP Server` reads this registry to dynamically populate `tools/list`.
2. **Workflow Initiation**: A user or AI agent submits a workflow definition via a new MCP tool (e.g., `vibe_submit_workflow`). `WorkflowEngine` parses it, persists it in `StateStore`, and queues the first step.
3. **Execution & State Transitions**:
   - `Master Server` spawns/injects commands into a `Worker Client` for the current workflow step.
   - The worker executes the task and streams output.
   - Upon completion, `Master Server` updates step status in `StateStore` and triggers the `WorkflowEngine` to advance.
4. **Cross-Checking**:
   - If the next step is defined as a `check` node, `WorkflowEngine` pauses execution of the main task.
   - A secondary cross-checking agent (or specific LLM prompt) is invoked, providing the previous step's output as context.
   - Depending on the boolean result of the check, the `WorkflowEngine` either proceeds to the next step, marks the workflow as failed, or loops back for correction.

## Patterns to Follow

### Pattern 1: Dynamic Tool Exposure (Skill Registry)
**What:** Instead of hardcoding tools in `mcp.rs`, define them declaratively.
**When:** Adding new domain-specific capabilities without recompiling the CLI.
**Example:**
```rust
// vibe-core/src/skills/mod.rs
pub struct SkillRegistry {
    skills: HashMap<String, SkillDefinition>,
}

impl SkillRegistry {
    pub fn load_from_dir(path: &Path) -> Result<Self> { ... }
    pub fn to_mcp_tools(&self) -> Vec<serde_json::Value> { ... }
}
```

### Pattern 2: State-Machine Driven Workflows
**What:** Use SQLite to track the state machine of a multi-step workflow.
**When:** Orchestrating sequences of actions (Plan -> Execute -> Check) that might span multiple panes or models.
**Example:**
`WorkflowEngine` polls or receives events from `StateStore`. Steps have states: `Pending`, `Running`, `Checking`, `Completed`, `Failed`.

### Pattern 3: Decoupled Verification (Cross-checking)
**What:** Treat cross-checking as just another node/step in the workflow, but with routing logic.
**When:** High-stakes operations where LLM hallucinations could be destructive.
**Instead of:** Combining execution and verification in a single prompt.

## Anti-Patterns to Avoid

### Anti-Pattern 1: Blocking IPC during Workflows
**What:** Waiting for a multi-step workflow to complete in a single synchronous UDS call.
**Why bad:** Will block the Master Server, preventing TUI updates and other worker heartbeats.
**Instead:** Make workflow submission asynchronous. The MCP server immediately returns a `workflow_id`, and the agent can poll status or the TUI can observe it passively.

### Anti-Pattern 2: In-Memory Workflow State
**What:** Tracking workflow progress purely in Rust memory.
**Why bad:** `vibe-cli` is often invoked statelessly or might crash.
**Instead:** All workflow state transitions MUST be committed to SQLite (`StateStore`) before execution.

## Build Order (Integration Strategy)

To minimize disruption to existing validated capabilities, build in this order:

1. **Phase 1: Skills Definitions (Foundation)**
   - Implement `SkillRegistry` and YAML parsing.
   - Migrate existing hardcoded tools (in `mcp.rs`) to the new registry format.
   - *Dependency*: None.
2. **Phase 2: Database Evolution for Workflows**
   - Implement schema migrations (as planned in Wave 2).
   - Add tables: `workflows`, `workflow_steps`.
   - *Dependency*: SQLite State Management.
3. **Phase 3: Workflow Engine & Cross-checking**
   - Implement state machine logic for step transitions.
   - Introduce "Check" nodes that evaluate output.
   - Add MCP tools to submit and query workflows.
   - *Dependency*: Phase 2.
4. **Phase 4: TUI Observability**
   - Update `Ratatui` interface to display a Workflow tab.
   - *Dependency*: Phase 3.

## Sources

- `.planning/PROJECT.md` (Validated capabilities: SQLite, Master-Worker, TUI, MCP)
- `apps/vibe-cli/src/mcp.rs` (Current hardcoded tool implementation)
- `.planning/codebase/ARCHITECTURE.md` (Current Master-Worker-TUI topology)