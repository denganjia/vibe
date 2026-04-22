---
name: vibe-conductor
description: Use when planning or running Vibe multi-agent workflows through plugin/vibe and .vibe workspace files.
---

# Vibe Conductor

## Role

The Conductor is the current model using the Vibe plugin. It turns a user
request into explicit decisions, task artifacts, executor runs, reviewer checks,
and a final aggregate summary by reading plugin references and `.vibe`
workspace files.

The Conductor owns collaboration judgment. It decides when to ask for
clarification, how to split task ownership, which Agent definition should run,
whether review findings block completion, and how to present the final result.

## Required Reads

Read these references progressively as the workflow needs them:

- [../../references/plugin-architecture.md](../../references/plugin-architecture.md)
- [../../references/collaboration-protocol.md](../../references/collaboration-protocol.md)
- [../../references/task-contract.md](../../references/task-contract.md)
- [../../references/agent-contract.md](../../references/agent-contract.md)
- [../../references/review-protocol.md](../../references/review-protocol.md)
- [../../references/workspace-layout.md](../../references/workspace-layout.md)

## Operating Loop

1. Clarify the request until the goal, constraints, owned files, expected
   output, and verification method are explicit.
2. Plan the work using plugin references and visible `.vibe` state instead of
   hidden assumptions.
3. Split the plan into task contracts with narrow repo-root-relative file
   scopes and reviewer requirements.
4. Execute tasks through configured Agent subprocesses or deterministic scripts
   when filesystem or subprocess work is needed.
5. Review executor output with structured findings before marking risky work
   complete.
6. Aggregate task, run, review, lock, and log artifacts from `.vibe` into the
   final user-facing result.

## Runtime Boundary

The Conductor keeps collaboration judgment in the model; scripts perform deterministic filesystem/subprocess actions. Scripts may initialize `.vibe`, write
task JSON, acquire locks, launch configured Agent subprocesses, capture stdout,
stderr, and exit codes, and generate local release summary drafts.

Scripts do not decide task priority, Agent assignment policy, review
acceptance, recovery policy, or user-facing tradeoffs. Those decisions remain
with the Conductor and the model-readable references.

## Completion Rules

- Do not mark a task complete until verification has passed and required review
  findings are resolved or explicitly accepted by the Conductor.
- Use `.vibe` artifacts as the durable source of task, run, review, log, and
  lock state.
- Keep terminal pane orchestration as compatibility behavior only; the default
  Agent path is a non-interactive subprocess described by `.vibe/Agents`.
- Do not persist secrets, raw environment dumps, or provider credentials in
  task, run, review, or log artifacts.
