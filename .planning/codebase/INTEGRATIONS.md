# External Integrations

**Analysis Date:** 2025-02-12

## APIs & External Services

**Terminal Multiplexers:**
- WezTerm - Primary orchestration target. Integrated via `wezterm cli` commands.
  - SDK/Client: `std::process::Command` calls to `wezterm cli`.
  - Operations: `split-pane`, `send-text`, `activate-pane`, `list`.
- Tmux - Secondary orchestration target. Integrated via `tmux` CLI.
  - SDK/Client: `std::process::Command` calls to `tmux`.
  - Operations: `split-window`, `send-keys`, `select-pane`, `list-panes`.

**AI Agents (Targeted):**
- Claude/Gemini/Codex - CLI-based agents.
  - Integration: Spawned as child processes in terminal panes.
  - Auth/Persona: Securely injected via the `$VIBE_PERSONA` environment variable which passes role-specific instructions (`skills/vibe-operator/SKILL.md`).

## Data Storage

**Databases:**
- Local Filesystem (JSON)
  - Connection: File-based access.
  - Client: `serde_json` with custom `StateStore` in `crates/vibe-core/src/state/mod.rs`.
  - Concurrency: Atomic writes (`.tmp` rename) and file-based locking (`.lock` files).

**File Storage:**
- Local filesystem only (`.vibe/` directory).

**Caching:**
- None (State is persisted directly to disk).

## Authentication & Identity

**Auth Provider:**
- Custom (Local Only)
  - Implementation: Unique IDs generated via `uuid` and passed through environment variables (`VIBE_ID`).

## Monitoring & Observability

**Error Tracking:**
- None (Local logging only).

**Logs:**
- Standard output/error redirected to `/dev/null` for daemons, or displayed in the TUI dashboard.

## CI/CD & Deployment

**Hosting:**
- Local machine (CLI tool).

**CI Pipeline:**
- GitHub Actions - Automated releases for multiple targets (`.github/workflows/release.yml`).

## Environment Configuration

**Required env vars:**
- `VIBE_ID`: Unique identifier for the current pane.
- `VIBE_MASTER_ID`: Physical ID of the master/orchestrator pane for signaling.
- `WEZTERM_PANE` / `TMUX_PANE`: Provided by the terminal emulator for context detection.
- `VIBE_PERSONA`: Injected securely to pass role-specific instructions to spawned AI agents (`apps/vibe-cli/src/main.rs`).

**Secrets location:**
- Not applicable (No external secrets required).

## Webhooks & Callbacks

**Incoming:**
- None.

**Outgoing (Stateless Bus Signals):**
- FileBus IPC: Primary cross-pane communication method using reliable file-based transmission (`crates/vibe-core/src/ipc/bus.rs`).
- Terminal Text Injection: Fallback method utilizing `[vibe-signal:NAME] {payload}` markers injected into terminal buffers.

---

*Integration audit: 2025-02-12*
