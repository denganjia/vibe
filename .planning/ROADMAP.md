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

## Phase Details
...
### Phase 7: AI Skill Integration & Release
**Goal**: Make vibe-cli natively discoverable and usable by AI models as a professional toolset.
**Depends on**: Phase 6
**Requirements**: SKILL-01, SKILL-02
**Success Criteria** (what must be TRUE):
  1. Deployment of an MCP (Model Context Protocol) server or JSON tool definition for vibe-cli.
  2. Documentation/System prompts provided for Claude/GPT to trigger vibe actions autonomously.
  3. Final binary packaging (v0.1.0) and installation script validation across platforms.
**Status**: Completed

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

