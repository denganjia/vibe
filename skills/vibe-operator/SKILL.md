---
name: vibe-operator
description: Orchestrate multiple AI agents using Vibe-CLI in a stateless terminal environment. Use when spawning sub-agents, synchronizing tasks via signal/wait, or managing terminal panes for complex multi-model development workflows.
---

# Vibe-Operator

This skill transforms the terminal into a physical orchestration room for autonomous AI agents using a **Stateless Bus** architecture.

## Tool Reference

Use these `vibe` shell commands to manage agents and synchronization:

- `vibe init [--force]`: Interactive wizard to initialize `.vibe/config.json` and role templates.
- `vibe check`: Verify terminal orchestration (WezTerm/Tmux) support.
- `vibe list`: List all active vibe agents, roles, status, and summaries. Automatically cleans up stale panes.
- `vibe spawn [--role <ROLE> | --stack <NAME>]`: Create a new pane/tab, start the agent with auto-approve flags, and securely inject persona via the `$VIBE_PERSONA` environment variable.
- `vibe signal <NAME> [PAYLOAD]`: Write a signal and optional JSON/file payload to the `.vibe/bus/` file bus.
- `vibe wait <NAME> [--timeout <SEC>]`: Block until a specific signal is received via the `.vibe/bus/` file bus.
- `vibe report --status <STATUS> --message <MSG>`: Update state store (`.vibe/state/panes.json`). Declare intent locking here.
- `vibe focus <ID>`: Switch terminal focus to a specific agent pane.
- `vibe inject <ID> <CMD>`: Inject text or commands into a running agent's physical pane.
- `vibe kill`: Terminate all active vibe panes and clear state.

## Core SOPs

For detailed procedural guidance, refer to these references:

- **Collaboration**: [references/collaboration.md](references/collaboration.md) - Task assignment & A-D-E-V cycle.
- **State & Bus**: [references/state.md](references/state.md) - How the file-based bus and smart cleanup work.
- **Approvals**: [references/approval.md](references/approval.md) - Manual gates via `vibe wait approved`.
- **Orchestration**: [references/orchestration.md](references/orchestration.md) - Stack spawning and project init flow.
- **Recovery**: [references/recovery.md](references/recovery.md) - Intervening in loops via `vibe inject`.
- **Roles**: [references/role.md](references/role.md) - Intelligence-First logic for Conductor and Worker.

## Workflow Templates

Use these assets to drive structured development cycles:

- **Software Design (SDD)**: Use templates in `assets/sdd/`.
- **Refactoring**: Use templates in `assets/refactoring/`.
- **Specification**: Use `assets/spec/01-define.md`.

## Critical Protocols

1. **Analyze-Declare-Execute-Verify Loop**: All autonomous tasks must follow this strict lifecycle.
2. **Intent Locking**: Workers MUST declare target files via `vibe report --status blocked --message "writing:path/to/file"` before modification to prevent race conditions.
3. **Verification & Retries**: Workers MUST run local tests (e.g., `cargo test`) after execution. If verification fails, automatically attempt to fix up to 3 times before signaling `BLOCKED`.
4. **File-based Signaling**: Never assume a worker is done. Use `vibe wait` to synchronize via the highly reliable `.vibe/bus/` file bus.
5. **Atomic Reports**: Use `vibe report` at every milestone so the global state remains accurate and stale panes can be pruned.
6. **Master Awareness**: Sub-agents MUST know their `VIBE_MASTER_ID` and `VIBE_ID` (provided via env vars by `vibe spawn`) to signal correctly.
7. **Native Persona**: The injected `$VIBE_PERSONA` contains the role-specific instructions and MUST be obeyed.
