# Agent Contract

## Agent Definition Fields

Each Agent definition under `.vibe/Agents/` describes one role that the
Conductor may select for execution or review. Agent definitions must include:

- `id`: stable Agent identifier.
- `role`: planner, executor, reviewer, release, or another documented role.
- `model_command`: command line used to launch the Agent as a subprocess.
- `prompt`: role instructions or path to a prompt document.
- `references`: model-readable files the Agent should read before work.
- `allowed_tools`: filesystem, shell, network, or provider tools the Agent may
  use.
- `expected_output`: artifacts, report shape, or status update the Agent must
  return.
- `review_policy`: whether this Agent requires review and how findings are
  handled.

Agent files should be Markdown or JSON that the current model can inspect
without executing code.

## Subprocess Boundary

The plugin-first default is subprocess execution. Scripts launch the configured
`model_command` and provide task context through files, arguments, stdin, or
allowlisted environment variables.

The subprocess boundary must be explicit:

- the Agent command comes from `.vibe/Agents/` or `.vibe/config.json`;
- task context comes from `.vibe/tasks/`;
- outputs are written to `.vibe/runs/`, `.vibe/reviews/`, and `.vibe/logs/`;
- terminal pane orchestration is compatibility behavior, not the normal path.

Scripts are responsible for exit code capture, timeout handling, log paths, and
artifact paths. The Conductor remains responsible for deciding which Agent to
run and what to do with the result.

## Secret Handling

Environment variables are allowlisted. Agent definitions or scripts must name
the exact variables that may cross into a subprocess.

Secrets must never be dumped into task, run, review, or log artifacts. This
includes raw environment dumps, shell traces that include token values, provider
API keys, session cookies, and OAuth refresh tokens.

If an Agent needs a secret, the runtime should pass only the named variable to
the subprocess and redact the value in any persisted command description.

## Compatibility Notes

Existing `.vibe/roles/` files and Rust role configuration are migration
references. The plugin-first target is `.vibe/Agents/` with explicit fields for
role, `model_command`, prompt, references, tools, output, and review policy.

Existing terminal adapter behavior may inform optional compatibility modes, but
new Agent definitions must be usable by a non-interactive subprocess launcher.
