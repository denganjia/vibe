---
name: vibe-operator
description: Orchestrate multiple AI agents using Vibe-CLI in a stateless terminal environment. Use when spawning sub-agents, synchronizing tasks via signal/wait, or managing terminal panes for complex multi-model development workflows.
---

# Vibe-Operator

This skill transforms the terminal into a physical orchestration room for autonomous AI agents using a **Stateless Bus** architecture.

## Tool Reference

Use these `vibe` shell commands to manage agents and synchronization:

- `vibe check`: Verify terminal orchestration (WezTerm/Tmux) support.
- `vibe list`: List all active vibe agents, roles, status, and summaries.
- `vibe spawn --role <ROLE>`: Create a new pane, inject persona from `.vibe/roles/`, and launch agent.
- `vibe signal <NAME>`: Inject a signal marker `[vibe-signal:NAME]` into the master bus.
- `vibe wait <NAME> [--timeout <SEC>]`: Block until a specific signal is received via stdin.
- `vibe report --status <STATUS> --message <MSG>`: Update state store (`.vibe/state/panes.json`).
- `vibe focus <ID>`: Switch terminal focus to a specific agent pane.
- `vibe inject <ID> <CMD>`: Inject text or commands into a running agent's pane.
- `vibe kill`: Terminate all active vibe panes and clear state.

## Core SOPs

For detailed procedural guidance, refer to these references:

- **Collaboration**: [references/collaboration.md](references/collaboration.md) - Task assignment & signaling protocols.
- **State & Bus**: [references/state.md](references/state.md) - How the stateless bus and file-locked state work.
- **Approvals**: [references/approval.md](references/approval.md) - Manual gates via `vibe wait approved`.
- **Orchestration**: [references/orchestration.md](references/orchestration.md) - Layout management and spawning flow.
- **Recovery**: [references/recovery.md](references/recovery.md) - Intervening in loops via `vibe inject`.
- **Roles**: [references/role.md](references/role.md) - Definitions for Conductor, Worker, and Evaluator.

## Workflow Templates

Use these assets to drive structured development cycles:

- **Software Design (SDD)**: Use templates in `assets/sdd/`.
- **Refactoring**: Use templates in `assets/refactoring/`.
- **Specification**: Use `assets/spec/01-define.md`.

## Critical Protocols

1. **Always Signal**: Never assume a worker is done. Use `vibe wait` to synchronize.
2. **Atomic Reports**: Use `vibe report` at every milestone so the global state remains accurate.
3. **Master Awareness**: Sub-agents MUST know their `VIBE_MASTER_ID` (provided by `vibe spawn`) to signal correctly.
4. **Persona Injection**: Trust the persona injected by `vibe spawn`. It contains the role-specific instructions.
