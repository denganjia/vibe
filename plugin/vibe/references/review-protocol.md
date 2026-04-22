# Review Protocol

## When Review Runs

Review runs after executor verification and before the Conductor marks a task
as `completed`. The task contract's `reviewer_requirements` field decides
whether review is required, which reviewer Agent should run, and which evidence
must be inspected.

Review may also run after a fix, after a failed verification retry, or when the
Conductor sees risk in executor output.

## Finding Fields

Every reviewer finding is structured data with these fields:

- `id`: stable finding identifier.
- `severity`: impact level for the issue.
- `file`: repo-root-relative file path, when applicable.
- `line`: line number or null when the issue is not line-specific.
- `summary`: concise statement of the problem.
- `evidence`: concrete observation, command result, or artifact reference.
- `required_fix`: action required before the task can complete.

Reviewer reports should be written under `.vibe/reviews/` and linked from the
task or run artifact.

## Severity

Allowed `severity` values are:

- `critical`: data loss, security break, destructive behavior, or task result is
  unusable.
- `high`: core requirement missing, verification false positive, or likely user
  visible failure.
- `medium`: correctness or maintainability issue that should be fixed before
  completion.
- `low`: minor issue that the Conductor may accept explicitly.
- `info`: note that does not block completion.

Severity must be based on impact and evidence, not reviewer preference.

## Required Fixes

Findings with `critical`, `high`, or `medium` severity require a fix or an
explicit Conductor acceptance decision. The `required_fix` field must be
actionable enough for the executor to apply without rereading the whole review.

Low and info findings may still be fixed, but they do not block completion
unless `reviewer_requirements` says they are mandatory.

## Completion Gate

A task cannot be completed until reviewer findings are resolved or explicitly accepted by the Conductor. Resolution means the executor made the required
change, verification was re-run when applicable, and the reviewer or Conductor
recorded the outcome in `.vibe/reviews/`.

If the Conductor accepts an unresolved finding, the acceptance must be durable:
record the finding id, severity, reason, and accepted risk in the review or
task artifact.
