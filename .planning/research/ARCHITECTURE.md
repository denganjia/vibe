# Architecture Patterns (AI Agent Bus)

**Domain:** AI Agent Coordination
**Researched:** 2024-10-27

## Recommended Architecture: Decentralized Bus

Instead of a master orchestrator, `vibe-cli` uses a **Lightweight Daemon (The Bus Server)** and **Ephemeral Clients (The Agents)**.

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| **Vibe Bus (Daemon)** | Message routing, Signal/Wait parking. | All Vibe Clients. |
| **Vibe Client (CLI)** | `spawn`, `signal`, `wait` commands. | Bus Server via UDS. |
| **Agent (Child Process)** | Executing AI tasks (e.g., Cursor, shell). | Vibe Client (via stdin/pipes). |
| **Local Brain (.vibe)** | Storing context & role templates. | Vibe Client & Agent. |

### Data Flow (Signal/Wait)

1. Client A sends `WAIT signal-1` to Daemon.
2. Daemon parks Client A's connection.
3. Client B sends `SIGNAL signal-1 { data: "ready" }` to Daemon.
4. Daemon resumes Client A, sends data, and closes connections.

## Patterns to Follow

### Pattern 1: Stdin "Warming" (Role Injection)
**What:** Writing Persona prompts to a child process's stdin immediately after spawn.
**When:** Using `vibe spawn --role`.
**Example:**
```rust
let mut child = Command::new("bash").stdin(Stdio::piped()).spawn()?;
let mut stdin = child.stdin.take().unwrap();
stdin.write_all(ROLE_PROMPT).await?;
tokio::io::copy(&mut tokio::io::stdin(), &mut stdin).await?;
```

### Pattern 2: Project-Hashed UDS Paths
**What:** Generating a unique socket path based on the project root hash in `/tmp`.
**Why:** Avoiding UDS 108-character limit while keeping bus project-local.

## Anti-Patterns to Avoid

### Anti-Pattern 1: "Master" Thinking
**What:** Creating a central orchestrator that manages all agent state.
**Why bad:** High complexity, fragile to restarts, hard to version control.
**Instead:** Use the Bus + `.vibe` directory to store shared state.

## Scalability Considerations

| Concern | 1 Agent | 10 Agents | 100 Agents |
|---------|---------|-----------|------------|
| **Signal Latency** | < 1ms | < 1ms | ~1-5ms (UDS is extremely efficient) |
| **State Sync** | File reads | File reads | May need file watcher |

## Sources

- [Tokio Documentation](https://tokio.rs/)
- [UDS Best Practices](https://man7.org/linux/man-pages/man7/unix.7.html)
