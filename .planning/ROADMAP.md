# ROADMAP

## Phases

- [x] **Phase 1: Terminal Orchestration Foundation** - Implement core abstraction for Wezterm/Tmux with cross-platform CLI detection.
- [x] **Phase 2: Monorepo Transition** - Refactor the project into a Rust Workspace with dedicated apps/ and crates/ directories.
- [x] **Phase 3: State Persistence & IPC Layer** - Build infrastructure for tracking tasks and cross-pane communication via unified UDS.
- [x] **Phase 4: Intent Injection & Human-in-the-Loop** - Enable Master to delegate tasks to Workers with safety controls.
- [x] **Phase 5: Output Monitoring & Lifecycle Safety** - Capture output and ensure robust process cleanup (Windows Job Objects).
- [x] **Phase 6: Status Dashboard & UX** - Provide visibility into the entire AI team's status via TUI.
- [x] **Phase 7: AI Skill Integration & Release** - Expose vibe-cli capabilities as actionable AI tools and prepare for distribution.
- [x] **Phase 8: Production Infrastructure & State Evolution** - Implement DB migrations and automated packaging/install scripts.
- [x] **Phase 9: Interactive Workflow Orchestration** - Implement "Plan-Review-Execute" flow with human-in-the-loop gates.
- [ ] **Phase 10: Vibe-CLI Core Skill Definition** - Create the foundational skill definition (SKILL.md) for AI agents to understand vibe-cli.
- [ ] **Phase 11: Multi-model SOP & Verification** - Define collaboration patterns (SOP) and cross-checking paths using vibe state.
- [x] **Phase 12: Workflow Templates & Optimization** - Provide ready-to-use workflow templates and optimize skill for model reliability. (completed 2026-04-17)

## Phase Details

### Phase 1: Terminal Orchestration Foundation
**Status**: Completed

### Phase 2: Monorepo Transition
**Status**: Completed

### Phase 3: State Persistence & IPC Layer
**Status**: Completed

### Phase 4: Intent Injection & Human-in-the-Loop
**Status**: Completed

### Phase 5: Output Monitoring & Lifecycle Safety
**Status**: Completed

### Phase 6: Status Dashboard & UX
**Status**: Completed

### Phase 7: AI Skill Integration & Release
**Status**: Completed

### Phase 8: Production Infrastructure & State Evolution
**Status**: Completed

### Phase 9: Interactive Workflow Orchestration
**Goal**: Enable complex AI task sequences that require human validation at critical boundaries.
**Status**: Completed
**Depends on**: Phase 8
**Requirements**: SCO-01
**Plans**: 5 plans
- [x] 09-01-PLAN.md — Define core IPC protocol and database schema updates for approval tracking.
- [x] 09-02-PLAN.md — Implement MCP tools (submit/query) and local Markdown plan storage.
- [x] 09-03-PLAN.md — Update TUI dashboard to display "WAITING" for panes pending approval.
- [x] 09-04-PLAN.md — Implement human-in-the-loop prompt in the Worker client terminal.
- [x] 09-05-PLAN.md — Implement master routing and final end-to-end verification of the workflow.
**Success Criteria**:
  1. AI can submit a "Plan" via MCP tool.
  2. System blocks execution and notifies human (via TUI/Worker).
  3. Human approval triggers the next phase of the task.

### Phase 10: Vibe-CLI Core Skill Definition
**Goal**: Create the foundational skill definition for AI agents to understand vibe-cli.
**Status**: Completed
**Depends on**: Phase 9
**Requirements**: SKL-01
**Success Criteria** (what must be TRUE):
  1. `SKILL.md` contains a comprehensive definition of all core `vibe-cli` commands and parameters.
  2. The skill definition clearly explains pane management workflows (split, focus, close).
  3. The IPC state flow (how to use UDS/SQLite) is described such that a model can successfully read/write state.
**Plans**: 4 plans
- [x] 10-01-PLAN.md — Initialize core skill package, metadata, and role protocols.
- [x] 10-02-PLAN.md — Define Standard Operating Procedures (SOPs) for orchestration, state, and approvals.
- [x] 10-03-PLAN.md — Implement modular SDD/SPEC workflow templates and finalize documentation.
- [x] 10-04-PLAN.md — Fix verification gaps (parameters and IPC details).

### Phase 11: Multi-model SOP & Verification
**Goal**: Define collaboration patterns and cross-checking paths using vibe state.
**Status**: Completed
**Depends on**: Phase 10
**Requirements**: SKL-02, SKL-03
**Success Criteria** (what must be TRUE):
  1. Standard Operating Procedures (SOP) for Master/Worker/Evaluator patterns are clearly defined within the skill.
  2. A concrete verification path using `vibe-cli` status for cross-checking is established.
  3. The skill includes guidance for models on how to utilize `vibe` state to resolve conflicts or verify execution success.
**Plans**: 3 plans
- [x] 11-01-PLAN.md — Update core role definitions and create the Collaboration SOP.
- [x] 11-02-PLAN.md — Define the verification and recovery protocols (audit, deadlock recovery).
- [x] 11-03-PLAN.md — Integrate SOPs into SKILL.md and verify collaboration logic via dry-run.

### Phase 12: Workflow Templates & Optimization
**Goal**: Provide ready-to-use workflow templates and optimize skill for model reliability.
**Depends on**: Phase 11
**Requirements**: SKL-04, SKL-05
**Success Criteria** (what must be TRUE):
  1. The skill provides tested templates for common scenarios like code refactoring and automated testing.
  2. In testing, AI models generate valid `vibe-cli` instructions that comply with defined specifications 95%+ of the time.
  3. The skill definition is optimized for token efficiency without sacrificing clarity or command reliability.
**Plans**: 3 plans
- [x] 12-01-PLAN.md — Implement specialized refactoring templates and update skill routing configuration.
- [x] 12-02-PLAN.md — Optimize skill definition for token efficiency and document the dynamic variable injection protocol.
- [x] 12-03-PLAN.md — Establish reliability benchmarks and verification standards for the Vibe-Operator skill.

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Terminal Orchestration | 3/3 | Completed | 2026-04-14 |
| 2. Monorepo Transition | 1/1 | Completed | 2026-04-14 |
| 3. State Persistence & IPC Layer | 4/4 | Completed | 2026-04-14 |
| 4. Intent Injection & Human-in-the-Loop | 1/1 | Completed | 2026-04-14 |
| 5. Output Monitoring & Lifecycle Safety | 1/1 | Completed | 2026-04-14 |
| 6. Status Dashboard & UX | 1/1 | Completed | 2026-04-15 |
| 7. AI Skill Integration & Release | 1/1 | Completed | 2026-04-15 |
| 8. Production Infrastructure | 1/1 | Completed | 2026-04-15 |
| 9. Workflow Orchestration | 5/5 | Completed | 2026-04-15 |
| 10. Vibe-CLI Core Skill | 0/3 | Not started | - |
| 11. Multi-model SOP | 0/3 | Not started | - |
| 12. Workflow Templates | 3/3 | Complete   | 2026-04-17 |
