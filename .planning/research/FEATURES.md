# Feature Landscape (AI Agent Bus)

**Domain:** AI Agent Coordination
**Researched:** 2024-10-27

## Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **vibe spawn** | Ability to launch agents with predefined roles. | Medium | Requires stdin injection and terminal management. |
| **vibe signal** | Sending status updates/events to the bus. | Low | Simple NDJSON push over UDS. |
| **vibe wait** | Blocking execution until a signal is received. | Medium | Requires server-side connection parking. |
| **.vibe roles** | Markdown-based role templates. | Low | File I/O + String templating. |

## Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Zero-Friction Inbound** | Injecting role prompts into existing interactive sessions. | High | Hard to do without breaking PTY/TTY. |
| **Cross-Pane Signaling** | Agent in Pane A triggers reaction in Pane B via bus. | Medium | Core value of the Bus architecture. |
| **Project-Local Memory** | Automatic sync of `active_context.md` between agents. | Medium | Requires file watching or bus-driven updates. |

## Anti-Features

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Global Registry** | Avoid central servers or global state. | Use project-local `.vibe` directory. |
| **Strict Workflow Enforcement** | Avoid locking agents into rigid DAGs. | Use flexible Signal/Wait for loose coupling. |

## Feature Dependencies

```
vibe daemon -> vibe signal/wait
vibe spawn -> .vibe roles
```

## MVP Recommendation

Prioritize:
1. `vibe signal/wait` (Core communication)
2. `vibe spawn --role` (Basic agent launching)
3. `.vibe/roles` (Template management)

## Sources

- Strategic Pivot (Milestone 4.0)
- AGENT_BUS_IMPLEMENTATION.md
