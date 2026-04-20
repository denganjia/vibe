# CONCERNS.md

## Technical Debt
- **Legacy Orchestration**: The transition from a "Master Daemon / UDS" architecture to a "Stateless Bus" is mostly complete, but some legacy logic or assumptions might remain in the codebase.
- **Error Handling Robustness**: While structured, some error cases (especially around terminal interaction failures) might benefit from more granular retry logic or user feedback.

## Known Issues
- **Terminal Dependency**: The system heavily relies on `wezterm` or `tmux` being present and properly configured. Unsupported terminals will fail to orchestrate.
- **Race Conditions**: Although file locking is used for `panes.json`, rapid concurrent spawning and signaling could still potentially lead to synchronization issues if not carefully managed.

## Security Concerns
- **Text Injection**: The "Stateless Bus" relies on injecting text into terminal buffers. If an agent prints malicious content that mimics a signal marker, it could potentially spoof signals.
- **Local Filesystem Permissions**: `.vibe/state/panes.json` contains session state. Insecure permissions on the project directory could lead to state tampering by other local users.

## Performance
- **Stdin Polling**: `vibe wait` relies on reading stdin. If the input buffer is extremely large, scanning for signal markers could introduce minor latency.
- **JSON Overhead**: Frequent reads/writes to `panes.json` for every heartbeat/report might become a bottleneck for very large agent swarms (>100 agents).
