# vibe init

## Command Contract Only

This file documents the intended workflow contract and does not claim executable Codex command binding.

## Purpose

Initialize a project-local `.vibe` workspace from `plugin/vibe/templates/.vibe`
so the Conductor, scripts, executor Agents, and reviewer Agents share visible
workspace state.

## Inputs

- Optional workspace root, defaulting to the current repository root.
- Optional explicit force overwrite option for replacing existing generated
  files.

## Reads

- `plugin/vibe/templates/.vibe`
- `plugin/vibe/references/workspace-layout.md`
- Existing `.vibe/` files, when present.

## Writes

- Missing `.vibe/Agents/`, `.vibe/tasks/`, `.vibe/runs/`, `.vibe/locks/`,
  `.vibe/reviews/`, and `.vibe/logs/` directories.
- Missing `.vibe/config.json` and starter model-readable documentation.

## Safety

Initialization is non-destructive by default. Existing user-edited `.vibe`
files must be left unchanged unless the user provides an explicit option for
force overwrite.

A force overwrite path must be explicit, auditable, and limited to known
workspace template files.

## Expected Output

- A concise list of created files and directories.
- A list of existing files that were preserved.
- Clear next-step guidance for reading or editing `.vibe/Agents/`.
