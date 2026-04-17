# Phase 14: Signal Bus Implementation (Stateless Edition)

## Objective
Implement `vibe signal` and `vibe wait` based on the Injected Conversation model, removing dependency on a persistent background daemon.

## Requirements
- **BUS-04**: Implement `vibe signal <NAME> [PAYLOAD]`. Notify target via text injection.
- **BUS-05**: Implement `vibe wait <NAME>`. Block until signal marker arrives on stdin.
- **BUS-06**: Concurrent safe `panes.json` management via file locking.

## Key Decisions
- **No Master Daemon**: Communication via terminal text injection instead of UDS. (D-01)
- **ID Inheritance**: `VIBE_MASTER_ID` injected into children during `spawn/split`. (D-02)
- **Signal Marker**: Format `\n[vibe-signal:<NAME>] <JSON_PAYLOAD>\n`. (D-04)
- **Wait Mechanism**: `vibe wait` scans stdin, matching markers and parsing JSON. (D-05, D-06)
- **State Store**: `panes.json` uses `fd-lock` for cross-process concurrency safety. (D-08)

## Technical Details
- **Terminal Adapters**: Use `send-text` (WezTerm) or `send-keys` (Tmux) for injection.
- **StateStore**: Ensure `save()` and `load()` are wrapped in file locks.
- **Cleanup**: Remove `server.rs`, `client.rs`, and unused IPC protocols.

## Traceability
- D-01, D-02, D-08 -> Plan 01
- D-03, D-04, D-05, D-06, D-07, D-09 -> Plan 02
