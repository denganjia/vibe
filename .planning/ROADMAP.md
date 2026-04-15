# ROADMAP

## Phases

- [x] **Phase 1: Terminal Orchestration Foundation** - Implement core abstraction for Wezterm/Tmux with cross-platform CLI detection.
- [x] **Phase 2: Monorepo Transition** - Refactor the project into a Rust Workspace with dedicated apps/ and crates/ directories.
- [x] **Phase 3: State Persistence & IPC Layer** - Build infrastructure for tracking tasks and cross-pane communication via unified UDS.
- [x] **Phase 4: Intent Injection & Human-in-the-Loop** - Enable Master to delegate tasks to Workers with safety controls.
- [ ] **Phase 5: Output Monitoring & Lifecycle Safety** - Capture output and ensure robust process cleanup (Windows Job Objects).
- [ ] **Phase 6: Status Dashboard & UX** - Provide visibility into the entire AI team's status via TUI.
- [ ] **Phase 7: AI Skill Integration & Release** - Expose vibe-cli capabilities as actionable AI tools and prepare for distribution.

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
**Goal**: The system intelligently captures task progress and ensures robust process cleanup to prevent zombie processes.
**Depends on**: Phase 4
**Requirements**: MON-01, MON-03, ORCH-02
**Success Criteria** (what must be TRUE):
  1. Worker stdout/stderr is captured and available to the Master as clean text (stripped of ANSI codes).
  2. Master receives a concise summary of long logs rather than raw data to save context window tokens.
  3. The active terminal focus automatically shifts to a Worker pane that completes its task or encounters an error.
  4. On Windows, all Worker processes are managed via Job Objects, ensuring they are automatically terminated if the Master process exits.
**Plans**: TBD

### Phase 6: Status Dashboard & UX
**Goal**: Users can monitor the high-level health and progress of all concurrent AI tasks.
**Depends on**: Phase 5
**Requirements**: MON-02
**Success Criteria** (what must be TRUE):
  1. User can run `vibe status` to see a live-updating table of all workers, their current task, and elapsed time.
  2. The dashboard clearly distinguishes between idling, working, and failed agents.
**Plans**: TBD
**UI hint**: yes

### Phase 7: AI Skill Integration & Release
**Goal**: Make vibe-cli natively discoverable and usable by AI models as a professional toolset.
**Depends on**: Phase 6
**Requirements**: SKILL-01, SKILL-02
**Success Criteria** (what must be TRUE):
  1. Deployment of an MCP (Model Context Protocol) server or JSON tool definition for vibe-cli.
  2. Documentation/System prompts provided for Claude/GPT to trigger vibe actions autonomously.
  3. Final binary packaging (v0.1.0) and installation script validation across platforms.
**Plans**: TBD

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Terminal Orchestration | 3/3 | Completed | 2026-04-14 |
| 2. Monorepo Transition | 1/1 | Completed | 2026-04-14 |
| 3. State Persistence & IPC Layer | 4/4 | Completed | 2026-04-14 |
| 4. Intent Injection & Human-in-the-Loop | 1/1 | Completed | 2026-04-14 |
| 5. Output Monitoring & Lifecycle Safety | 0/0 | Not started | - |
| 6. Status Dashboard & UX | 0/0 | Not started | - |
| 7. AI Skill Integration & Release | 0/0 | Not started | - |
