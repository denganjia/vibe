# .vibe/logs

## Owned By

Logs are owned by deterministic scripts that record command execution and workflow events.

## File Naming

Use append-only command logs with date or task prefixes, such as `2026-04-22.ndjson` or `task-001.ndjson`.

## Safety Notes

Logs must contain no secrets. Scripts should avoid copying environment variables, tokens, credentials, or private keys into log output.
