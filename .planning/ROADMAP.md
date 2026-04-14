# ROADMAP

## Phases

- [x] **Phase 1: Terminal Orchestration Foundation** - Implement core abstraction for Wezterm/Tmux with cross-platform CLI detection.
- [x] **Phase 2: Monorepo Transition** - Refactor the project into a Rust Workspace with dedicated apps/ and crates/ directories.
- [ ] **Phase 3: State Persistence & IPC Layer** - Build infrastructure for tracking tasks and cross-pane communication via unified UDS.
- [ ] **Phase 4: Intent Injection & Human-in-the-Loop** - Enable Master to delegate tasks to Workers with safety controls.
- [ ] **Phase 5: Output Monitoring & Lifecycle Safety** - Capture output and ensure robust process cleanup (Windows Job Objects).
- [ ] **Phase 6: Status Dashboard & UX** - Provide visibility into the entire AI team's status via TUI.
- [ ] **Phase 7: AI Skill Integration & Release** - Expose vibe-cli capabilities as actionable AI tools and prepare for distribution.

## Phase Details

### Phase 1: Terminal Orchestration Foundation
**Goal**: Users can programmatically control terminal layouts across Wezterm and Tmux with cross-platform support.
**Requirements**: ORCH-01, SAFE-02
**Status**: Completed

### Phase 2: Monorepo Transition
**Goal**: Refactor the project into a Rust Workspace to support future modularity.
**Requirements**: Structural refactoring.
**Status**: Completed

### Phase 3: State Persistence & IPC Layer
**Goal**: The system maintains a reliable record of the AI team's state and enables real-time coordination via a unified IPC layer.
**Depends on**: Phase 2
**Requirements**: STAT-01, STAT-02
**Success Criteria** (what must be TRUE):
  1. After a terminal restart, `vibe` can re-attach to existing panes and resume tracking by querying the local SQLite database.
  2. Worker processes can send "heartbeats" and status updates to the Master via Unix Domain Sockets (UDS).
  3. Master server successfully binds to UDS on both Windows (AF_UNIX) and Unix, with automatic cleanup of stale socket files on Windows.
**Plans**: 4 plans
- [ ] 03-01-PLAN.md — Define the messaging protocol (NDJSON) and centralized database actor.
- [ ] 03-02-PLAN.md — Implement cross-platform daemonization and the Master UDS server.
- [ ] 03-03-PLAN.md — Implement the Worker client and the vibe run subcommand.
- [ ] 03-04-PLAN.md — Validate the system with multi-worker concurrency and master recovery tests.

### Phase 4: Intent Injection & Human-in-the-Loop
**Goal**: Master AI can safely delegate tasks to Workers with full context.
**Depends on**: Phase 3
**Requirements**: INJ-01, INJ-02, SAFE-01
**Success Criteria** (what must be TRUE):
  1. Master can launch a command in a new window/pane that inherits the current shell's `PATH` and environment variables.
  2. Worker panes block execution of Agent-generated commands until the user provides manual confirmation via a `y/N` prompt.
**Plans**: TBD
**UI hint**: yes

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
| 3. State Persistence & IPC Layer | 0/4 | Not started | - |
| 4. Intent Injection & Human-in-the-Loop | 0/0 | Not started | - |
| 5. Output Monitoring & Lifecycle Safety | 0/0 | Not started | - |
| 6. Status Dashboard & UX | 0/0 | Not started | - |
| 7. AI Skill Integration & Release | 0/0 | Not started | - |
