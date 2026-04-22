# Vibe Plugin Architecture

## Product Shape

Vibe is a plugin-first collaboration system. The installed plugin teaches the
current AI terminal model to act as the Conductor, gives it model-readable
protocol references, and exposes thin deterministic scripts for filesystem and
subprocess work.

The product surface is the plugin package under `plugin/vibe/`. The historical
`vibe-cli` Rust workspace remains useful as a compatibility reference during the
migration, but it is no longer the primary user interface.

## Ownership Map

- `skills` own Conductor judgment: clarify the user request, decide whether the
  plan is ready, split work into tasks, choose executor and reviewer Agents, and
  aggregate results.
- `commands` own user-facing workflow entry contracts: init, plan, run task,
  review task, status, and release summary describe what the user can ask the
  plugin to do.
- `references` own durable model-readable protocol: collaboration rules, task
  JSON shape, Agent definition shape, review findings, workspace layout, and
  migration boundaries.
- `scripts` own deterministic filesystem/subprocess actions: initialize
  `.vibe`, write task JSON, acquire local locks, launch configured Agent
  commands, capture logs, and record run or review artifacts.
- `.vibe` owns project-local state: Agent definitions, tasks, runs, locks,
  reviews, logs, and configuration that the current model can inspect directly.
- `apps/` plus `crates/` are compatibility reference during Phase 20: their
  Rust behavior guides migration decisions without remaining the primary
  product surface.

## Phase 20 Boundary

Phase 20 defines the architecture and contract surface. It does not implement
the full scripts runtime, migrate every Rust command, or publish a plugin to a
marketplace.

Phase 20 outputs must make later phases executable by documenting:

- which plugin tier owns each responsibility;
- which `.vibe` files are the synchronization source;
- which task, Agent, and review fields are required;
- which legacy Rust CLI behavior is reference-only until classified.

## Compatibility Boundary

Subprocess execution is the default Agent path for the plugin-first system.
Terminal pane orchestration, old `vibe spawn` flows, and Rust state files can be
used as migration evidence or optional compatibility behavior, but new contracts
must not require them for the normal path.

The compatibility layer must preserve useful lessons from the Rust codebase:
project-local state, simple JSON artifacts, atomic writes where needed, and
explicit role/model commands. It must not recreate a standalone heavy CLI,
daemon, database, or terminal-only control plane inside `scripts`.
