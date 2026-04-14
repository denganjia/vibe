# ROADMAP

## Phases

- [ ] **Phase 1: Terminal Orchestration Foundation** - Implement core abstraction for Wezterm/Tmux with cross-platform CLI detection.
- [ ] **Phase 2: State Persistence & IPC Layer** - Build infrastructure for tracking tasks and cross-pane communication via unified UDS.
- [ ] **Phase 3: Intent Injection & Human-in-the-Loop** - Enable Master to delegate tasks to Workers with safety controls.
- [ ] **Phase 4: Output Monitoring & Lifecycle Safety** - Capture output and ensure robust process cleanup (Windows Job Objects).
- [ ] **Phase 5: Status Dashboard & UX** - Provide visibility into the entire AI team's status via TUI.
- [ ] **Phase 6: AI Skill Integration & Release** - Expose vibe-cli capabilities as actionable AI tools and prepare for distribution.

## Phase Details

### Phase 1: Terminal Orchestration Foundation
**Goal**: Users can programmatically control terminal layouts across Wezterm and Tmux with cross-platform support.
**Depends on**: Nothing
**Requirements**: ORCH-01, SAFE-02
**Success Criteria** (what must be TRUE):
  1. User can create vertical and horizontal splits using `vibe split`.
  2. User can resize and close specific panes via unique IDs.
  3. User can instantly terminate all active `vibe` panes using a "kill switch" command.
  4. System correctly detects and calls `wezterm.exe` on Windows and `wezterm` on Unix.
  5. Configuration and state paths are resolved correctly using platform-specific standards (e.g., `AppData` vs `~/.local/share`).
**Plans**: 3 plans
- [ ] 01-01-PLAN.md — 初始化项目结构与终端适配器抽象
- [ ] 01-02-PLAN.md — 实现 WezTerm 与 Tmux 的具体适配器及 Windows 进程安全机制
- [ ] 01-03-PLAN.md — 实现 SQLite 持久化层与核心 CLI 指令

### Phase 2: State Persistence & IPC Layer
**Goal**: The system maintains a reliable record of the AI team's state and enables real-time coordination via a unified IPC layer.
**Depends on**: Phase 1
**Requirements**: STAT-01, STAT-02
**Success Criteria** (what must be TRUE):
  1. After a terminal restart, `vibe` can re-attach to existing panes and resume tracking by querying the local SQLite database.
  2. Worker processes can send "heartbeats" and status updates to the Master via Unix Domain Sockets (UDS).
  3. Master server successfully binds to UDS on both Windows (AF_UNIX) and Unix, with automatic cleanup of stale socket files on Windows.
**Plans**: TBD

### Phase 3: Intent Injection & Human-in-the-Loop
**Goal**: Master AI can safely delegate tasks to Workers with full context.
**Depends on**: Phase 2
**Requirements**: INJ-01, INJ-02, SAFE-01
**Success Criteria** (what must be TRUE):
  1. Master can launch a command in a new window/pane that inherits the current shell's `PATH` and environment variables.
  2. Worker panes block execution of Agent-generated commands until the user provides manual confirmation via a `y/N` prompt.
**Plans**: TBD
**UI hint**: yes

### Phase 4: Output Monitoring & Lifecycle Safety
**Goal**: The system intelligently captures task progress and ensures robust process cleanup to prevent zombie processes.
**Depends on**: Phase 3
**Requirements**: MON-01, MON-03, ORCH-02
**Success Criteria** (what must be TRUE):
  1. Worker stdout/stderr is captured and available to the Master as clean text (stripped of ANSI codes).
  2. Master receives a concise summary of long logs rather than raw data to save context window tokens.
  3. The active terminal focus automatically shifts to a Worker pane that completes its task or encounters an error.
  4. On Windows, all Worker processes are managed via Job Objects, ensuring they are automatically terminated if the Master process exits.
**Plans**: TBD

### Phase 5: Status Dashboard & UX
**Goal**: Users can monitor the high-level health and progress of all concurrent AI tasks.
**Depends on**: Phase 4
**Requirements**: MON-02
**Success Criteria** (what must be TRUE):
  1. User can run `vibe status` to see a live-updating table of all workers, their current task, and elapsed time.
  2. The dashboard clearly distinguishes between idling, working, and failed agents.
**Plans**: TBD
**UI hint**: yes

### Phase 6: AI Skill Integration & Release
**Goal**: Make vibe-cli natively discoverable and usable by AI models as a professional toolset.
**Depends on**: Phase 5
**Requirements**: SKILL-01, SKILL-02
**Success Criteria** (what must be TRUE):
  1. Deployment of an MCP (Model Context Protocol) server or JSON tool definition for vibe-cli.
  2. Documentation/System prompts provided for Claude/GPT to trigger vibe actions autonomously.
  3. Final binary packaging (v0.1.0) and installation script validation across platforms.
**Plans**: TBD

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Terminal Orchestration Foundation | 0/3 | Not started | - |
| 2. State Persistence & IPC Layer | 0/0 | Not started | - |
| 3. Intent Injection & Human-in-the-Loop | 0/0 | Not started | - |
| 4. Output Monitoring & Lifecycle Safety | 0/0 | Not started | - |
| 5. Status Dashboard & UX | 0/0 | Not started | - |
| 6. AI Skill Integration & Release | 0/0 | Not started | - |
