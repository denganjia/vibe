# Vibe-Operator Reliability Benchmarks (Stateless)

## Reliability Goal
Target **95%+** command compliance for AI models using this skill. Models must follow orchestration protocols and tool syntax with high precision in a stateless terminal environment.

## Edge Case Scenarios

| ID | Scenario | Expected Model Behavior |
|----|----------|-------------------------|
| **EDGE-01** | **Invalid vibeId** | Model should verify the agent's existence via `vibe list` before attempting `vibe focus` or `vibe inject`. |
| **EDGE-02** | **Signal Timeout** | If `vibe wait` times out, Conductor must use `vibe list` to check if the worker is blocked or if the signal marker was missed. |
| **EDGE-03** | **Role Conflict** | If `vibe spawn` fails because a role template is missing, the agent should search `.vibe/roles/` or ask the user for a template path. |
| **EDGE-04** | **Master ID Missing** | If `VIBE_MASTER_ID` is missing in a sub-pane, the agent should attempt to manually resolve the master pane ID using `vibe list` before signaling. |

## Verification Standards

1. **Command Syntax Check**: All tool calls must match the definitions in `SKILL.md`.
2. **Signal-Wait Loop**: Parallel workflows MUST use `vibe wait` to synchronize, avoiding non-deterministic polling.
3. **State Integrity**: Every major task completion MUST be preceded by a `vibe report` call to ensure `.vibe/state/panes.json` reflects the outcome.
4. **Context Recovery**: Models should use the `summary` field from `vibe list` to recover context after a session interruption.
