# Technology Stack (AI Agent Bus)

**Project:** vibe-cli
**Researched:** 2024-10-27

## Recommended Stack

### Core Communication (The Bus)
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Rust / Tokio | 1.x | Async Runtime | Industry standard for safe, concurrent, and high-performance system tools. Excellent UDS & Process support. |
| NDJSON | - | Protocol Format | Human-readable, streamable, and avoids complexity of binary protocols like Protobuf. Consistent with existing `vibe-core`. |
| Unix Domain Sockets (UDS) | - | IPC Layer | Extremely fast, supports file-permission security, and maps naturally to "Project-Local" bus via path-based socket. |

### Process Management (The Spawner)
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| tokio::process | - | Child Processes | Async process spawning with piped stdin/stdout handling. |
| nix / libc | - | TTY / Signal Handling | Low-level terminal control and signal forwarding (Unix-specific). |

### Persistence (The Brain)
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Markdown / TOML | - | Context Storage | Human & AI readable. Git-friendly for versioning `.vibe` directory context. |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| IPC Protocol | NDJSON | gRPC / MCP | Too heavy for simple CLI-to-CLI signaling. Requires complex code generation and runtime overhead. |
| Database | .vibe (Files) | SQLite | Pivoed in 4.0 to prioritize git-friendliness and simplicity over structured relational data. |
| IPC Layer | UDS | TCP (Localhost) | TCP has higher overhead and is harder to secure (port collision/scanning). UDS is file-based and faster. |

## Sources

- [Tokio: Async I/O for Rust](https://tokio.rs/)
- [NDJSON Specification](http://ndjson.org/)
- Strategic Pivot (Milestone 4.0)
