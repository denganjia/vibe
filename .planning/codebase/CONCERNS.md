# CONCERNS

## Technical Debt & Risks

### 1. IPC Robustness (Critical Lessons from Wave 1)
- **Stale Sockets**: A common "deadlock" occurs when a `.sock` file exists but its Master process is dead. `vibe-cli` now detects this via `connect()` and auto-deletes the stale file to force a Master restart.
- **Empty Line Parsing**: In TUI message streams, `LinesCodec` can sometimes return empty lines (especially with duplicate `\n` in the source). TUI clients **must** skip whitespace-only lines before attempting JSON deserialization.
- **Bidirectional Async Loop**: The Master's connection handler must use `tokio::select!` to simultaneously listen for incoming Worker messages (heartbeats) and outgoing Master intents. Blocking on either will stall the Agent's health reporting.

### 2. State & Database
- **Schema Evolution**: SQLite schema changes (e.g., adding `role` or `summary`) without automated migrations currently require manual deletion of `state.db`. This needs to be addressed before distributing the tool.
- **State Dir Resolution**: Path resolution for `vibe/` config and logs is platform-specific (`Library/Application Support/` on macOS vs `%APPDATA%` on Windows). Manual cleanup of these directories is occasionally necessary during development.

### 3. Performance & Resource Usage
- **ANSI Stripping Overhead**: Regex-based ANSI stripping occurs for every line of Worker output. This could be a bottleneck for extremely high-volume tasks.
- **Log Management**: Currently, the TUI reads the entire log file to slice the last 20 lines. This will not scale for long-running processes with large logs. A seek-from-end strategy is required.

### 4. Cross-Platform Consistency
- **Terminal Adapters**: `WezTerm` and `Tmux` have different metadata capabilities. Ensuring a unified `VibeID` to physical pane mapping across both is ongoing work.
- **Windows Jobs**: The process group cleanup on Windows depends on `JobObjects`. Ensuring these jobs are correctly released on Master shutdown is a critical stability point.

## Monitoring
- Use `vibe status` (TUI) as the primary way to detect "stuck" agents.
- Check `~/.local/share/vibe/logs/*.log` for raw task output.
