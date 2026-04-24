# Phase 26: Skill Standardization - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-24
**Phase:** 26-skill-standardization
**Areas discussed:** Migration grouping, YAML Frontmatter schema, Validation strategy

---

## Migration grouping

| Option | Description | Selected |
|--------|-------------|----------|
| 1:1 mapping | Map each legacy command (e.g. run-task) to an individual skill folder (e.g. `skills/run-task/SKILL.md`) to keep execution atomic and discrete. | ✓ |
| Domain grouping | Group related commands into a broader domain skill (e.g. a `task-management` skill containing run-task, review-task, etc.). | |

**User's choice:** 1:1 mapping

---

## YAML Frontmatter schema

| Option | Description | Selected |
|--------|-------------|----------|
| Minimal Required Fields | Only name, version, and description (Recommended to align with `plugin/vibe/package.json` identity and ensure cross-platform compatibility). | ✓ |
| Extensive Metadata | Name, version, description, platform compatibility flags, and execution context. | |

**User's choice:** Minimal Required Fields

---

## Validation strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Automated node script | Add a node test script (e.g. `skills.test.js`) to parse all SKILL.md files, verify frontmatter schemas, and assert correct directory paths. | ✓ |
| Manual checklist | Write a markdown validation checklist to be done manually during execution/PR review. | |

**User's choice:** Automated node script

---
