## Assumptions

### Agent Definition Format
- **Assumption:** Agent files will be written in JSON format (or Markdown with strictly parsable YAML frontmatter) rather than plain unstructured Markdown.
  - **Why this way:** `plugin/vibe/references/agent-contract.md` states "Agent files should be Markdown or JSON that the current model can inspect without executing code" but also mandates specific key-value fields (`id`, `model_command`, `allowed_tools`, etc.).
  - **Alternatives:** 
    1. Pure JSON: Easiest for scripts (Phase 22) to parse reliably.
    2. Markdown with YAML frontmatter: Better human readability but requires a YAML parser in the scripts.
  - **If wrong:** The lightweight scripts introduced in Phase 22 will fail to extract the exact `model_command` or list of `allowed_tools` reliably via regex, breaking the subprocess boundary.
  - **Confidence:** Likely

### Non-Destructive Workspace Initialization
- **Assumption:** The new initialization logic will be a plugin-level script (as defined in `plugin/vibe/commands/init.md`) that creates the `.vibe/` structure but strictly skips existing user files (like `.vibe/roles/`) instead of attempting an automatic inline migration.
  - **Why this way:** `plugin/vibe/commands/init.md` explicitly states "Initialization is non-destructive by default. Existing user-edited `.vibe` files must be left unchanged" and `plugin/vibe/references/agent-contract.md` states "Old `.vibe/roles/*.md` files are migration input only."
  - **If wrong:** Users upgrading from previous versions might unexpectedly lose their custom prompts, breaking their existing legacy workflows during the v6.0 transition.
  - **Confidence:** Confident

### Configuration Structure Overhaul
- **Assumption:** The `.vibe/config.json` schema is fundamentally changing from a flat `roles`/`stacks` mapping to a nested structural schema (`concurrency.max_parallel_tasks`, `runtime.scripts`), meaning old configs cannot be used natively by the new system.
  - **Why this way:** The template at `plugin/vibe/templates/.vibe/config.json` uses this new nested structure and drops the old `roles` mapping entirely, which is still present in the current project root `.vibe/config.json`.
  - **Alternatives:**
    1. Enforce the new schema strictly and ignore legacy files (requiring manual migration).
    2. Have the init script automatically map old `roles` into new `.vibe/Agents/` files on first run.
  - **If wrong:** The Phase 22 runtime scripts will attempt to read legacy config fields instead of the newly structured parameters, leading to missing concurrency limits or broken default agents.
  - **Confidence:** Likely

### Subprocess Execution Dependency
- **Assumption:** Agent definitions (`.vibe/Agents/`) must explicitly define `model_command` as an executable shell command (e.g., `gemini -y`) because the plugin architecture natively depends on subprocess execution.
  - **Why this way:** `plugin/vibe/references/agent-contract.md` outlines "The plugin-first default is subprocess execution. Scripts launch the configured `model_command`". `plugin/vibe/templates/.vibe/Agents/README.md` strictly requires the `model_command` field.
  - **If wrong:** If new Agents rely on the Conductor's internal Rust API client instead of an explicit CLI command, the plugin architecture will fail to decouple the execution runtime, violating the Milestone 6.0 goal.
  - **Confidence:** Confident
