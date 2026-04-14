# Technology Stack: vibe-cli

**Project:** vibe-cli
**Researched:** 2024-05-22 (Current Era: 2024-2025)
**Status:** Recommended for Greenfield Implementation

## Recommended Stack

### Core Frameworks
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **Rust** | 1.75+ | Programming Language | For performance, safety, and excellent terminal/PTY ecosystem. |
| **Tokio** | 1.x | Async Runtime | Essential for managing concurrent monitoring of multiple terminal panes and IPC. |
| **Clap** | 4.x | CLI Parser | Industry standard for modern Rust CLIs with excellent `derive` support. |
| **Serde** | 1.x | Serialization | For handling WezTerm's JSON output and state persistence. |

### Terminal Orchestration
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **tmux_interface** | 0.4.x | Tmux Control | Most comprehensive Rust wrapper for Tmux CLI commands. |
| **tokio::process** | Standard | WezTerm Control | Best way to call `wezterm cli` as a subprocess for MVP simplicity. |
| **portable-pty** | 0.8+ | PTY Management | (Optional) If internal PTY wrapping is needed for fine-grained worker control. |

### Data & State
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **rusqlite** | 0.31+ | Local Database | For `.vibe/state.db`. Simple, reliable, and single-file. |
| **tokio::net** | Standard | IPC (Unix Socket) | For real-time "Intent Injection" and status updates between panes. |

### Monitoring & Observability
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| **sysinfo** | 0.30+ | Process Tracking | For "Progress Monitoring" (checking if workers are busy/idle via process tree). |
| **tracing** | 0.1 | Structured Logging | Standard for Rust observability; works well with `tokio`. |
| **ratatui** | 0.26+ | TUI Dashboard | (Optional) For the "Orchestration Master" view/dashboard. |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| **Multiplexer** | WezTerm/Tmux | Zellij | Project requirement focus is WezTerm/Tmux; Zellij is a full replacement. |
| **Database** | SQLite (rusqlite) | JSON Files | SQLite provides better concurrency and query capabilities for task status. |
| **Tmux Lib** | `tmux_interface` | `tmux-rs` | `tmux-rs` is a rewrite of Tmux, not a control interface. |

## Installation

```bash
# Core Dependencies
cargo add tokio --features full
cargo add clap --features derive
cargo add serde --features derive
cargo add serde_json
cargo add rusqlite --features bundled
cargo add tmux_interface
cargo add tracing tracing-subscriber
cargo add sysinfo
cargo add ratatui crossterm # If TUI dashboard is planned
```

## Implementation Notes

### WezTerm Integration Strategy
Since no high-level SDK exists for WezTerm, the recommended pattern is:
1. Call `wezterm cli list --format json`.
2. Parse into Rust structs using `serde_json`.
3. Map `pane_id` (integer) and `window_id` to internal state.

### Tmux Integration Strategy
Use `tmux_interface` structs for type-safety. Wrap it in an abstraction layer (e.g., `TerminalBackend` trait) to handle both Tmux and WezTerm with the same logic.

## Sources

- [WezTerm CLI Documentation](https://wezfurlong.org/wezterm/cli/index.html)
- [tmux_interface (Crates.io)](https://crates.io/crates/tmux_interface)
- [Ratatui Documentation](https://ratatui.rs/)
- [Agent Hand (AI session manager pattern)](https://github.com/hitesh-mehta/agent-hand)
