# ARCHITECTURE

## System Overview
Vibe CLI uses a **Master-Worker-TUI** triangular topology built on Unix Domain Sockets (UDS). It breaks the "dimensional wall" between AI and the local environment by orchestrating physical terminal panes.

## Core Components

### 1. Master Server (`vibe-core/src/ipc/server.rs`)
The central orchestrator that:
- Manages a persistent SQLite state store of all active panes.
- Routes intents (commands) from users or AI to specific Workers.
- Acts as a **Broadcast Station**, pushing real-time state updates to all subscribed TUIs.
- Handles bidirectional IPC: listening for Worker heartbeats/reports while simultaneously queueing outgoing intents.

### 2. Worker Client (`vibe-core/src/ipc/client.rs`)
The execution agent residing in each physical pane:
- Registers with the Master on startup.
- Maintains a 5s heartbeat loop to signal health and status.
- Implements a **Confirmation Gate (HITL)** for sensitive commands.
- Captures and strips ANSI codes from task output before logging to local files.

### 3. TUI Dashboard (`apps/vibe-cli/src/tui.rs`)
The monitoring center ("Command Tower"):
- Subscribes to the Master's broadcast stream via UDS.
- Provides real-time visual feedback of all agent states (Running, Failed, Exited).
- Implemented using **Ratatui** for a high-performance terminal UI.
- Enables physical orchestration (focusing/killing panes) via hotkeys.

## Communication Protocol
- **Transport**: Unix Domain Sockets (UDS) with `LinesCodec`.
- **Payload**: NDJSON (Newline Delimited JSON) using the `Message` enum in `vibe-core/src/ipc/protocol.rs`.
- **Flow**:
  - `Register`: Worker -> Master (Initial handshake)
  - `Heartbeat`: Worker -> Master (Periodic health check)
  - `Subscribe`: TUI -> Master (Subscription for state updates)
  - `Broadcast`: Master -> TUI (Real-time global state push)
  - `ExecuteIntent`: Master -> Worker (Command injection)

## Design Patterns
- **Serialized Actor**: The database is managed by a `DbActor` that processes requests via an mpsc channel, preventing SQLite concurrency issues.
- **Hybrid Injection**: Commands are injected via UDS structured messages when possible, with fallback to raw terminal keys if the Worker is unresponsive.
- **Passive Capture**: ANSI stripping is done at the source (Worker) to ensure logs remain clean and searchable.
