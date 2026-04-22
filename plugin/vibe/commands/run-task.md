# vibe run-task

## Command Contract Only

This file documents the intended workflow contract and does not claim executable Codex command binding.

## Purpose

Run one planned Vibe task through a configured executor Agent while preserving
task, run, log, and review handoff artifacts for Conductor aggregation.

## Inputs

- Task id input naming a task JSON file under `.vibe/tasks/`.
- Optional executor Agent id, when the Conductor overrides the task default.
- Optional dry-run mode for validating task and Agent contracts without
  launching a subprocess.

## Reads

- `.vibe/tasks/<task-id>.json`
- `.vibe/Agents/`
- `.vibe/config.json`
- `plugin/vibe/references/task-contract.md`
- `plugin/vibe/references/agent-contract.md`

## Writes

- Run metadata and result artifacts under `.vibe/runs/`.
- Safe stdout and stderr log references under `.vibe/logs/`.
- Task status updates such as `running`, `review-needed`, `failed`, or
  `blocked`.

## Subprocess Boundary

The Phase 20 contract uses a non-interactive subprocess default. A future
script selects the Agent `model_command`, provides task context from
`references/task-contract.md` and `references/agent-contract.md`, captures
stdout/stderr/exit_code, and writes durable results under `.vibe/runs/`.

The command contract accepts structured task ids, not arbitrary shell text.
Secrets may cross the subprocess boundary only through allowlisted environment
variables defined by the Agent contract, and persisted artifacts must redact
secret values.

## Expected Output

- Run id and linked task id.
- Executor command description with secrets redacted.
- stdout/stderr/exit_code capture summary.
- Paths to `.vibe/runs/` and log artifacts.
- Next status: review-needed, blocked, failed, or completed only when review is
  not required.
