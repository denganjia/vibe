# .vibe/Agents

`.vibe/Agents` stores model-readable Agent definition files for planner, executor, reviewer, release, and other project-local roles.

Each Agent definition must include these required fields:

- `id`
- `role`
- `model_command`
- `prompt`
- `references`
- `allowed_tools`
- `expected_output`
- `review_policy`

The `model_command` field names the configured subprocess command, not a hard-coded provider. The `allowed_tools`, `expected_output`, and `review_policy` fields make execution boundaries reviewable before a task runs.

Old `.vibe/roles/*.md` files are migration input only. They are not the new Agent contract.
