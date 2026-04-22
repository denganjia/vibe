# .vibe/tasks

## Owned By

Task files are owned by the Conductor workflow and deterministic scripts that create or update task JSON.

## File Naming

Use stable task identifiers in JSON filenames, such as `task-001.json` or `2026-04-22-plan-001.json`.

Task JSON must include `goal`, `context`, `file_scope`, `constraints`, `expected_output`, `verification`, and `reviewer_requirements`.

## Safety Notes

Task files describe intended work. They are not shell scripts and must not be executed as commands.
