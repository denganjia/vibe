# Phase 26: Skill Standardization - Context

**Gathered:** 2026-04-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 26 unifies skill and command definitions across platforms using YAML frontmatter. It reorganizes legacy `commands/` and `roles/` into `skills/<skill-name>/SKILL.md` structures.

</domain>

<decisions>
## Implementation Decisions

### Migration Strategy
- **D-01:** Legacy commands (e.g. `run-task`, `plan`, `init`) will be mapped 1:1 to new individual skill folders (e.g. `plugin/vibe/skills/run-task/SKILL.md`) to keep execution discrete and atomic.
- **D-02:** Legacy roles (e.g. `Conductor.md`) will be mapped to their corresponding skill contexts or overarching skills.

### YAML Frontmatter Schema
- **D-03:** The YAML frontmatter for each `SKILL.md` will contain minimal required fields: `name`, `version`, and `description`.
- **D-04:** This aligns with the `plugin/vibe/package.json` identity and ensures cross-platform compatibility without introducing platform-specific clutter early on.

### Validation Strategy
- **D-05:** We will implement an automated Node.js test script (e.g. `skills.test.js`) to parse all `SKILL.md` files.
- **D-06:** The script will verify that frontmatter schemas are correct and assert that they live in the correct directory paths.

### Claude's Discretion
None explicitly captured, but exactly how the Node validation script integrates with existing tests is left to Claude's discretion.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and requirements
- `.planning/ROADMAP.md` — Phase 26 boundary, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — SKL-01 and SKL-02 traceability.

### Existing Context
- `.planning/phases/25-universal-manifests-packaging/25-CONTEXT.md` — Prior phase constraints on package structure and manifests.
- `plugin/vibe/package.json` — The source of truth for identity metadata that SKILL.md frontmatters should complement.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `plugin/vibe/skills/conductor/Conductor.md` — Existing skill pattern to adapt or follow.
- `plugin/vibe/scripts/manifests.test.js` — Existing pure Node.js test style that the validation script can mimic.

### Established Patterns
- Legacy commands live in `plugin/vibe/commands/*.md`.
- Legacy roles live in `plugin/vibe/roles/*.md`.
- Both need to be transitioned into `plugin/vibe/skills/<skill-name>/SKILL.md`.

### Integration Points
- Tests will need to be added to `npm test` scripts in `plugin/vibe/package.json`.

</code_context>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches based on decisions.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 26-skill-standardization*
*Context gathered: 2026-04-24*
