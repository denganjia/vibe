# Vibe-Operator Reliability Benchmarks (Stateless)

## Reliability Goal
Target **99%+** command compliance and signaling reliability for AI models using this skill. Models must follow orchestration protocols and tool syntax with high precision in a stateless terminal environment.

## Edge Case Scenarios

| ID | Scenario | Expected Model Behavior |
|----|----------|-------------------------|
| **EDGE-01** | **Invalid vibeId** | Model should verify the agent's existence via `vibe list` before attempting `vibe focus` or `vibe inject`. |
| **EDGE-02** | **Signal Missed/Timeout** | With the `.vibe/bus/` file bus, signals are highly reliable. However, if a timeout occurs, Conductor must use `vibe list` to check if the worker is blocked or dead. |
| **EDGE-03** | **Role Conflict** | If `vibe spawn` fails because a role template is missing, the agent should recommend running `vibe init --force` or search `.vibe/roles/`. |
| **EDGE-04** | **Master ID Missing** | If `VIBE_MASTER_ID` is missing in a sub-pane, the agent should attempt to manually resolve the master pane ID using `vibe list` before signaling. |
| **EDGE-05** | **Race Conditions** | Model MUST declare Intent Locks (`vibe report --status blocked --message "writing:path"`) to prevent multiple workers from modifying the same file concurrently. |

## Verification Standards

1. **Command Syntax Check**: All tool calls must match the definitions in `SKILL.md`.
2. **Signal-Wait Loop**: Parallel workflows MUST use `vibe wait` on the file bus to synchronize, avoiding non-deterministic polling.
3. **State Integrity**: Every major task completion MUST be preceded by a `vibe report` call to ensure `.vibe/state/panes.json` reflects the outcome.
4. **Autonomous Loops**: Workers MUST attempt up to 3 automatic fixes for test failures before escalating to the Conductor.
