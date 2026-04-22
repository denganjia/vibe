# .vibe Workspace Layout

## Required Directories

A Vibe project workspace lives at the repository root in `.vibe/`. The required
entries are:

- `Agents/` for role and model command definitions.
- `tasks/` for task JSON contracts created by the Conductor or scripts.
- `runs/` for executor run metadata, exit status, and result artifacts.
- `locks/` for project-local file-scope locks.
- `reviews/` for reviewer findings and completion decisions.
- `logs/` for stdout, stderr, and runtime diagnostics that are safe to persist.
- `config.json` for workspace-level defaults and policy.

These names are stable contract names. Scripts may add files below them, but
the top-level workspace should remain small enough for the current model to
read and reason about.

## Config Contract

`.vibe/config.json` records defaults that deterministic scripts need before
launching work. It should stay human-readable JSON and avoid hidden state.

Expected categories include:

- default Agent command or model command fallback;
- Agent definition locations under `Agents/`;
- concurrency limits and lock policy;
- task, run, review, and log directory paths;
- review policy defaults;
- release summary settings.

Provider-specific command lines belong in Agent definitions or config values,
not in the Conductor skill text and not hard-coded into scripts.

## Non-Destructive Initialization

Generated initialization must not overwrite user-edited `.vibe` files unless an
explicit force option is provided. The default behavior is create-if-missing:
create required directories, write missing starter files, and leave existing
files unchanged.

When a future initializer detects an existing file that differs from the
template, it should report the difference and continue without replacing it.
If a force mode is later added, it must be explicit in the command contract and
must preserve a clear audit trail in `logs/` or another durable artifact.

## Model Readability

The workspace is a synchronization surface for humans, the Conductor, executor
Agents, reviewer Agents, and scripts. Files should be plain Markdown or JSON
unless a later phase documents a stronger reason to add another format.

The current model must be able to answer these questions by reading `.vibe`
files directly:

- Which Agents exist and which `model_command` each one uses?
- Which tasks are queued, running, blocked, review-needed, failed, or completed?
- Which file scopes are locked and by which task?
- Which reviews found blocking issues?
- Which run artifacts and logs explain the latest outcome?
