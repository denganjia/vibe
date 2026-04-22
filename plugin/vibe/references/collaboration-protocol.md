# Collaboration Protocol

## Clarify

The current model is the Conductor. It starts by clarifying the user request
until the goal, constraints, file ownership, expected output, and verification
method are explicit enough to write tasks.

Clarification should produce model-readable decisions, not hidden state. If a
decision affects later execution, the Conductor records it in the task context
or another `.vibe` artifact.

## Plan

The Conductor turns the clarified request into a plan that separates reasoning
from deterministic work:

- skills and references guide judgment;
- scripts perform filesystem and subprocess actions;
- `.vibe` artifacts hold the durable state.

The plan should identify which tasks can run independently, which tasks require
ordering, and which reviewer requirements apply before completion.

## Split Tasks

Tasks are the handoff between Conductor and executor. Each task must follow the
shape defined in [task-contract.md](task-contract.md), including `file_scope`,
constraints, verification, and `reviewer_requirements`.

The Conductor must keep task file scopes narrow enough that parallel executors
can avoid conflicts. When ownership overlaps, tasks should be serialized or
blocked until the relevant lock is released.

## Execute

The default execution path is non-interactive subprocess Agents per D-12/D-16.
The Conductor or script runtime launches configured Agent commands, such as
claude, gemini, codex, or another project-defined command, with task and Agent
context injected through documented files or allowlisted environment variables.

For the default protocol, terminal panes are compatibility only. They may remain
useful for old Rust CLI behavior or optional adapters, but the plugin-first
protocol must not require a terminal pane control plane for normal task
execution.

Executor Agents follow an Analyze-Declare-Execute-Verify loop:

1. Analyze task context, references, and owned files.
2. Declare intended file ownership through the task and lock artifacts.
3. Execute only inside the approved file scope.
4. Verify using the task's verification commands or checks.
5. Write result artifacts and logs for Conductor aggregation.

## Review

Reviewer Agents inspect executor output before the Conductor marks a task done.
Review requirements come from the task's `reviewer_requirements` field and from
the review protocol in [review-protocol.md](review-protocol.md).

Reviewer output must be structured. Blocking findings require fixes, a
re-review, or explicit Conductor acceptance before the task can be completed.

## Aggregate

The Conductor aggregates from `.vibe`, not from transient chat memory. Task,
run, review, lock, and log artifacts under `.vibe` are the synchronization
source for executor status, review outcomes, and final summaries.

Scripts do deterministic work only: write JSON, acquire locks, launch
subprocesses, capture stdout or stderr, and persist artifacts. They do not own
collaboration judgment or task prioritization.
