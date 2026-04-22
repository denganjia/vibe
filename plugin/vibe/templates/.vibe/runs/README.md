# .vibe/runs

## Owned By

Run records are owned by scripts that launch configured Agent subprocesses and capture their results.

## File Naming

Use a task-derived run identifier, such as `task-001-run-001.json`, so runs can be traced back to source tasks.

Run artifacts should record `stdout`, `stderr`, `exit_code`, `started_at`, `finished_at`, and `artifacts`.

## Safety Notes

Run records are evidence for review and recovery. They should capture process results without becoming a hidden scheduler state.
