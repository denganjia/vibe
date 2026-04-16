# ROADMAP

## Phases

- [x] **Phase 1: Terminal Orchestration Foundation** - Implement core abstraction for Wezterm/Tmux with cross-platform CLI detection.
- [x] **Phase 2: Monorepo Transition** - Refactor the project into a Rust Workspace with dedicated apps/ and crates/ directories.
- [x] **Phase 3: State Persistence & IPC Layer** - Build infrastructure for tracking tasks and cross-pane communication via unified UDS.
- [x] **Phase 4: Intent Injection & Human-in-the-Loop** - Enable Master to delegate tasks to Workers with safety controls.
- [x] **Phase 5: Output Monitoring & Lifecycle Safety** - Capture output and ensure robust process cleanup (Windows Job Objects).
- [x] **Phase 6: Status Dashboard & UX** - Provide visibility into the entire AI team's status via TUI.
- [/] **Phase 7: AI Skill Integration & Release** - Expose vibe-cli capabilities as actionable AI tools and prepare for distribution.

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
- [x] **Phase 7: AI Skill Integration & Release** - Expose vibe-cli capabilities as actionable AI tools and prepare for distribution.
- [ ] **Phase 8: Production Infrastructure & State Evolution** - Implement DB migrations and automated packaging/install scripts.
- [ ] **Phase 9: Interactive Workflow Orchestration** - Implement "Plan-Review-Execute" flow with human-in-the-loop gates.

## Phase Details
...
### Phase 7: AI Skill Integration & Release
**Status**: Completed

### Phase 8: Production Infrastructure & State Evolution
**Goal**: Transition from experimental tool to a stable, distributable product.
**Depends on**: Phase 7
**Requirements**: INF-01, OPS-01, OPS-02
**Success Criteria**:
  1. Database automatically upgrades when new fields are added (no more manual rm state.db).
  2. A single script can install vibe across macOS/Linux/Windows.
  3. Pre-compiled binaries available for major platforms.

### Phase 9: Interactive Workflow Orchestration
**Goal**: Enable complex AI task sequences that require human validation at critical boundaries.
**Depends on**: Phase 8
**Requirements**: SCO-01
**Success Criteria**:
  1. AI can submit a "Plan" via MCP tool.
  2. System blocks execution and notifies human (via TUI/Worker).
  3. Human approval triggers the next phase of the task.

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
| 8. Production Infrastructure | 0/0 | Not started | - |
| 9. Workflow Orchestration | 0/0 | Not started | - |

