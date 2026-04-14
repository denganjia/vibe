# STATE

## Project Reference

**Core Value**: Break the "dimensional wall" between AI and the local dev environment by turning the terminal into a physical orchestration room.
**Current Focus**: Initial setup and basic terminal abstraction for WezTerm and Tmux with cross-platform support.

## Current Position

**Phase**: 1 - Terminal Orchestration Foundation
**Plan**: None
**Status**: Not started
**Progress**: [░░░░░░░░░░░░░░░░░░░░] 0%

## Performance Metrics

| Metric | Start | Current | Target |
|--------|-------|---------|--------|
| Task Autonomy | 0% | 0% | 80% |
| Pane Sync Latency | - | - | < 100ms |
| Log Compression Ratio | - | - | 5:1 |

## Accumulated Context

### Decisions
- Using Rust for performance and terminal ecosystem support.
- Initial MVP will rely on CLI wrappers (`wezterm cli`, `tmux`) to accelerate development.
- Adopting a Master-Worker architecture.
- Support Windows (10+) and Unix via unified IPC (UDS) and platform-specific process management (Job Objects vs Process Groups).

### Todos
- [ ] Initialize Rust project structure.
- [ ] Implement `TerminalAdapter` trait with cross-platform support.
- [ ] Implement `WezTermAdapter` (handling `wezterm.exe` detection).
- [ ] Implement `TmuxAdapter`.
- [ ] Implement platform-specific path resolution (using `dirs` crate).

### Blockers
- None.

## Session Continuity

### Current Intent
Starting Phase 1: Terminal Orchestration Foundation.

### Next Steps
1. Create a detailed plan for Phase 1.
2. Define the core traits for terminal orchestration.
