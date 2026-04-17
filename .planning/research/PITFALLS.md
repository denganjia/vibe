# Domain Pitfalls (AI Agent Bus)

**Domain:** AI Agent Coordination
**Researched:** 2024-10-27

## Critical Pitfalls

### Pitfall 1: UDS Path length limit (108 chars)
**What goes wrong:** `bind()` calls fail with `ENAMETOOLONG` if project path is deep (e.g., `/Users/name/work/clients/very-long-project-name-v2/.vibe/state/bus.sock`).
**Prevention:** Store socket in `/tmp/vibe-<project-hash>.sock`.

### Pitfall 2: Stdin/TTY Conflicts
**What goes wrong:** Using `Stdio::piped()` for `stdin` disables the TTY features of a child process (like `gh ssh` or `vim`).
**Consequences:** AI agents might hang waiting for user input that can't be typed, or output looks mangled.
**Prevention:** For truly interactive sessions, use terminal-specific commands (like `wezterm cli send-text`) after the window is created, instead of piping stdin directly.

### Pitfall 3: Zombie Waiters
**What goes wrong:** If a client crashes after sending `WAIT`, the server's list of waiters grows indefinitely.
**Prevention:** 
1. Server should detect broken TCP/UDS connections and remove associated waiters.
2. Implement a mandatory timeout for all `WAIT` commands.

## Moderate Pitfalls

### Pitfall 4: NDJSON Delimiter Ambiguity
**What goes wrong:** If the JSON payload itself contains newlines (though escaped), or if the client sends two messages in one packet, the parser might fail.
**Prevention:** Use a robust line-buffered reader like `tokio::io::BufReader`.

### Pitfall 5: .vibe/state Git Tracking
**What goes wrong:** Runtime state files (like `bus_info.json`) are committed to git by mistake.
**Prevention:** Ensure the `vibe init` command automatically creates/updates `.gitignore`.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Phase 14 (Bus) | Deadlocks on Signal/Wait | Use `oneshot` channels and `timeout` on `wait`. |
| Phase 15 (Spawner) | Role Injection Race Condition | Wait for child to stabilize or use a "Role Ready" signal from the agent itself if possible. |

## Sources

- [Linux man pages for unix(7)](https://man7.org/linux/man-pages/man7/unix.7.html)
- [Personal Experience with Terminal Multiplexer automation]
