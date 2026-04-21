# Phase 19 Context: Full Autonomous Workflow (E2E)

## Implementation Decisions

### 1. Autonomous Collaboration Loop (SOP)
- **Protocol**: Vibe-Operator skill will be updated with an "Analyze-Declare-Execute-Verify" SOP.
- **Intent Locking**: Workers MUST declare their target files via `vibe report` before modification to prevent race conditions.
- **Verification Gate**: Workers MUST run local tests (e.g., `cargo test`) and send a SUCCESS signal only after tests pass.
- **Retry Logic**: SOP allows up to 3 automatic "Fix" cycles if verification fails before signaling BLOCKED to the Conductor.

### 2. Decision Logic: Intelligence-First
- **Mechanism**: Conductor uses its LLM reasoning to determine the next phase of the workflow.
- **Input**: Conductor monitors `.vibe/bus/` signals and `vibe list` summaries.
- **Routing**: Decisions are enacted via `vibe inject` (for running agents) or `vibe spawn` (for new sub-tasks).

### 3. Deliverables & State
- **Final Output**: System will generate a comprehensive `DELIVERY.md` at the end of the E2E task.
- **Reporting**: Conductor aggregates all Worker results and maps them against the initial `REQUIREMENTS.md`.

### 4. Stress Test Scenario
- **Target**: A real-world refactoring task (e.g., "Refactor the TUI module in apps/vibe-cli to be more modular").
- **Success Metric**: 100% completion of the task (code written, tests passed, delivery report generated) without a single manual keyboard input from the user.

## Locked Constraints
- **Zero Friction**: If an agent can fix a bug it introduced during refactoring within 3 retries, it must do so autonomously.
- **Single Source of Truth**: All inter-agent synchronization MUST use the File-based Bus (.vibe/bus/).

## Next Steps
- Implement the "Autonomous Loop" persona templates for Conductor and Worker.
- Update `vibe-operator` skill assets (SOPs).
- Run the E2E stress test and capture logs for v5.0 release validation.
