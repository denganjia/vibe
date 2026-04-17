# Codebase Concerns

**Analysis Date:** 2025-03-05

## Tech Debt

**State Management Concurrency:**
- Issue: `StateStore` loads and saves a single JSON file (`panes.json`) without file-level locking or cross-process synchronization.
- Files: `crates/vibe-core/src/state/mod.rs`, `apps/vibe-cli/src/main.rs`
- Impact: Concurrent CLI commands or multiple agents can overwrite each other's state, leading to data loss or an inconsistent view of the system. Each process has its own in-memory cache that is out of sync with others.
- Fix approach: Transition to SQLite or use file-level advisory locks (`flock`). Alternatively, centralize all state mutations through the `MasterServer` and mandate that all clients use IPC to update or query state.

**IPC Protocol Robustness:**
- Issue: Line-based JSON (NDJSON) over Unix sockets using `LinesCodec` without robust framing, checksums, or versioning.
- Files: `crates/vibe-core/src/ipc/protocol.rs`, `crates/vibe-core/src/ipc/server.rs`
- Impact: Difficult to evolve the protocol without breaking backward compatibility. Risk of partial message reads if lines are very long (hitting buffer limits).
- Fix approach: Implement length-prefixed framing and include a protocol version field in the handshake or message header.

**Terminal Adapter Fragility:**
- Issue: Adapters rely on parsing CLI output of external tools (`wezterm cli`, `tmux`).
- Files: `crates/vibe-core/src/adapter/wezterm.rs`, `crates/vibe-core/src/adapter/tmux.rs`
- Impact: Changes in output format across different versions of WezTerm or Tmux will break core functionality (splitting, focusing, listing).
- Fix approach: Use machine-readable formats where available (e.g., `wezterm cli list --format json`) and implement robust error handling/fallback for parsing.

## Known Bugs

**Silent State Corruption:**
- Symptoms: If `panes.json` is malformed, `StateStore::new()` silently defaults to an empty state. Subsequent saves will then overwrite the previous data with an empty or partial set.
- Files: `crates/vibe-core/src/state/mod.rs`
- Trigger: Manual edit of JSON, interrupted write, or filesystem issues.
- Workaround: None currently. The system assumes a well-formed file or no file.

**Zombie Workers:**
- Symptoms: Workers started via `vibe run` stay alive indefinitely even after their primary task is finished, consuming system resources.
- Files: `apps/vibe-cli/src/main.rs`
- Trigger: Normal use of `vibe run <command>`.
- Workaround: Manual cleanup using `vibe kill` or system-level process management.

## Security Considerations

**Insecure IPC Socket Permissions:**
- Risk: Unix sockets are created with default system permissions. On multi-user systems, other users could connect to the master socket and inject arbitrary commands into workers via `ExecuteIntent`.
- Files: `crates/vibe-core/src/ipc/server.rs`
- Current mitigation: None.
- Recommendations: Explicitly set socket permissions to `0600` (owner only) and verify the UID of connecting clients using `ucred` (on Linux) or equivalent.

**Arbitrary Code Execution via Inject:**
- Risk: The `vibe inject` command allows sending any shell command to a worker. If an attacker gains access to the IPC socket, they have full control over the worker's shell.
- Files: `apps/vibe-cli/src/main.rs`, `crates/vibe-core/src/ipc/server.rs`
- Current mitigation: Relies on OS-level socket protection.
- Recommendations: Implement a shared secret or token-based authentication for IPC connections.

## Performance Bottlenecks

**Monolithic State Persistence:**
- Problem: Every heartbeat or status update from any worker triggers a full read/write cycle of the `panes.json` file.
- Files: `crates/vibe-core/src/state/mod.rs`
- Cause: Simple JSON-based storage for all pane records.
- Improvement path: Use a database (SQLite) for incremental updates or an append-only log format.

**Broadcast Storms:**
- Problem: `MasterServer` broadcasts the entire state to all subscribers on every update.
- Files: `crates/vibe-core/src/ipc/server.rs` (broadcast_states)
- Cause: Naive synchronization strategy.
- Improvement path: Send incremental diffs or allow subscribers to filter which updates they care about.

## Fragile Areas

**JSON-based Persistence:**
- Files: `crates/vibe-core/src/state/mod.rs`
- Why fragile: Highly susceptible to race conditions and file corruption in a multi-process environment.
- Safe modification: Add file locking or move state ownership to a single persistent daemon.
- Test coverage: Gaps in multi-process concurrency testing.

**External Process Dependency:**
- Files: `crates/vibe-core/src/adapter/*.rs`
- Why fragile: Vibe's core logic is heavily dependent on the presence and specific behavior of `wezterm` or `tmux` binaries in the PATH.
- Safe modification: Implement a "capability check" on startup and provide clear error messages if dependencies are missing or outdated.

## Scaling Limits

**Single-Master Bottleneck:**
- Current capacity: Limited by the single-threaded async loop of the `MasterServer` and the `mpsc` channel capacities.
- Limit: As the number of agents (panes) scales to dozens or hundreds, the broadcast-on-every-update strategy will lead to high CPU and latency.
- Scaling path: Introduce a more decentralized "Agent Bus" where communication can happen peer-to-peer or via a more efficient message broker.

## Missing Critical Features

**Self-Healing State:**
- Problem: No mechanism to detect and remove stale entries from `panes.json` if a worker or master crashes ungracefully.
- Blocks: Long-term reliability of the orchestration environment.

**Agent Discovery & Negotiation:**
- Problem: Current "Autonomous Agent Bus" is still highly centralized around the `MasterServer`. Agents cannot directly discover or communicate with each other without going through the master.
- Blocks: Truly decentralized autonomous agent coordination.

## Test Coverage Gaps

**Concurrency & Race Conditions:**
- What's not tested: Multiple instances of `StateStore` or `MasterServer` interacting with the same files/sockets.
- Files: `crates/vibe-core/src/state/mod.rs`, `crates/vibe-core/src/ipc/server.rs`
- Risk: Data loss or corruption that only appears under high load or specific timing.
- Priority: High.

**Terminal Adapter Integration:**
- What's not tested: Interaction with actual `wezterm` or `tmux` binaries (current tests are mostly unit tests or mocked).
- Files: `crates/vibe-core/src/adapter/*.rs`
- Risk: Compatibility regressions on new terminal versions.
- Priority: Medium.

---

*Concerns audit: 2025-03-05*
