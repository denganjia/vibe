# Task Contract

## Required JSON Fields

Every task artifact under `.vibe/tasks/` is JSON and must include these fields:

- `id`: stable task identifier, unique inside the workspace.
- `goal`: concise statement of the user-visible outcome.
- `context`: references, decisions, and source files the executor must read.
- `file_scope`: list of repo-root-relative paths the executor may modify.
- `constraints`: limits such as ownership, style, security, or compatibility.
- `expected_output`: artifacts or behavior the executor must produce.
- `verification`: commands, content checks, or manual checks that prove success.
- `reviewer_requirements`: review depth, reviewer role, and required checks.
- `status`: current lifecycle state.
- `created_at`: ISO-8601 creation timestamp.
- `updated_at`: ISO-8601 timestamp for the latest task state change.

Optional fields may be added by later phases, but executors and reviewers must
not depend on hidden data outside the task artifact.

## File Scope Rules

`file_scope` is the executor's write boundary. Entries must be
repo-root-relative paths, not absolute paths and not paths relative to an
executor's temporary working directory.

The contract rejects path traversal using `..`. Scripts and Conductor checks
must treat any `file_scope` entry containing `..`, absolute path roots, or home
directory expansion as invalid before an executor starts.

The executor may read additional context when needed, but it may only modify
paths inside `file_scope`. If a required fix needs a new path outside the
declared scope, the task must be updated by the Conductor or blocked for a
decision.

Concurrent executors must respect file scope locks. A task should not run while
another active task owns the same file or a parent/child path that would create
ambiguous ownership.

## Verification

`verification` records the checks the executor must run or satisfy before
requesting review. Verification can include shell commands, file existence
checks, content searches, or manual review instructions.

Verification results should be written to the run artifact, including command,
exit code, and a concise result summary. A task with skipped verification must
explain why in the run artifact and is not automatically complete.

## Status Values

Allowed `status` values are:

- `queued`: task exists but execution has not started.
- `running`: executor has accepted the task.
- `blocked`: executor cannot continue without Conductor decision or external
  input.
- `review-needed`: executor verification passed and reviewer work is required.
- `fix-needed`: reviewer found issues that require executor changes.
- `failed`: execution or verification failed and no automatic fix remains.
- `completed`: verification passed and review findings are resolved or accepted.

Scripts may add timestamps and result references around these status changes,
but they must preserve the plain JSON contract.
