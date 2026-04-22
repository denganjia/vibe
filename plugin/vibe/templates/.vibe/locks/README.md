# .vibe/locks

## Owned By

Lock files are owned by deterministic scripts that coordinate writes to task-scoped files.

## File Naming

Use repo-root path normalization before deriving lock names so equivalent paths map to the same lock.

Path-scoped locks should be named from normalized repo-relative paths and created with atomic create semantics.

## Safety Notes

Locks prevent overlapping writes. Future runtime code should acquire locks before modifying owned files and release them when work completes or fails cleanly.
