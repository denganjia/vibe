# Vibe Plugin

Vibe is the product name per D-18. `plugin/vibe/` is the plugin package root per D-19 and is the primary Phase 20 surface for plugin-first multi-model collaboration.

## Package Layout

`plugin/vibe/` is organized around model-readable and scriptable plugin surfaces:

- `.codex-plugin/plugin.json` declares the Codex-compatible plugin identity.
- `skills/` will teach the current model to act as the Conductor.
- `skills/` will expose workflow entry contracts.
- `references/` will hold collaboration, task, Agent, review, workspace, and migration contracts.
- `scripts/` holds thin deterministic helpers.
- `templates/` holds `.vibe` workspace scaffolds.
- `examples/` holds model-readable demonstrations of the contracts.

## Phase 20 Boundary

Phase 20 is plugin-first scaffold work. It establishes local discovery, package identity, documented plugin surfaces, and `.vibe` template boundaries so later phases can add concrete skills, command contracts, references, and scripts without moving the old workspace.

This phase does not implement the full runtime, publish to an external marketplace, or migrate all old CLI behavior. It keeps the scaffold explicit and local.

## Runtime Boundary

Scripts default to JS per D-14. They are deterministic helpers for initializing `.vibe`, writing task JSON, file locks, launching configured Agent subprocesses, capturing run artifacts, and drafting local release summaries.

Scripts do not own task priority, Agent assignment policy, review acceptance, recovery policy, or Conductor judgment. Those decisions belong in skills and references.

## Legacy Rust Workspace

`apps/` plus `crates/` remain compatibility reference in Phase 20 per D-15 and D-20. They preserve validated behavior for migration classification and comparison while the plugin package becomes the primary user-facing product surface.

The old Rust CLI is not the primary user entry point for Vibe's plugin-first direction.
