# Phase 26: Skill Standardization - Research

**Researched:** 2026-04-24
**Domain:** Plugin structure and frontmatter validation
**Confidence:** HIGH

## Summary

The current project has a mix of legacy `commands/` and `roles/` that need to be migrated to individual `skills/<skill-name>/SKILL.md` files. We must add YAML frontmatter (`name`, `version`, `description`) to all `SKILL.md` files to align with the `package.json` identity. To enforce this, we will introduce a Node-based validation script (`skills.test.js`) leveraging `js-yaml` to assert the correctness of these files. The old `plugin/vibe/roles/Conductor.md` should be folded into `plugin/vibe/skills/conductor/SKILL.md`, and the duplicate flat file `plugin/vibe/skills/Conductor.md` should be removed.

**Primary recommendation:** Use `js-yaml` as a devDependency to robustly parse the `SKILL.md` frontmatter in the test script, preventing brittle regex failures, and map commands 1:1 to new skill directories.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Legacy commands (e.g. `run-task`, `plan`, `init`) will be mapped 1:1 to new individual skill folders (e.g. `plugin/vibe/skills/run-task/SKILL.md`) to keep execution discrete and atomic.
- **D-02:** Legacy roles (e.g. `Conductor.md`) will be mapped to their corresponding skill contexts or overarching skills.
- **D-03:** The YAML frontmatter for each `SKILL.md` will contain minimal required fields: `name`, `version`, and `description`.
- **D-04:** This aligns with the `plugin/vibe/package.json` identity and ensures cross-platform compatibility without introducing platform-specific clutter early on.
- **D-05:** We will implement an automated Node.js test script (e.g. `skills.test.js`) to parse all `SKILL.md` files.
- **D-06:** The script will verify that frontmatter schemas are correct and assert that they live in the correct directory paths.

### Claude's Discretion
None explicitly captured, but exactly how the Node validation script integrates with existing tests is left to Claude's discretion.

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SKL-01 | Standardize SKILL.md files to include cross-platform YAML frontmatter | Use `js-yaml` in tests to ensure valid frontmatter schema across all skills |
| SKL-02 | Reorganize and clean up legacy commands/ and roles/ to strictly follow skills/<skill-name>/SKILL.md structure | Confirmed the list of commands (`init`, `plan`, `release-summary`, `review-task`, `run-task`, `status`) and roles (`Conductor`) needing migration |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| js-yaml | 4.1.1 | Parses YAML frontmatter in Markdown | Robust handling of YAML block syntax over brittle regex. Native to JS ecosystem. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| node:test | built-in | Test runner | Running `skills.test.js` to match existing project conventions |
| node:assert | built-in | Assertions | Verifying schema correctness |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| js-yaml | regex matcher | Regex is prone to edge cases (e.g., multi-line strings, whitespace). `js-yaml` is a standard, battle-tested library. |

**Installation:**
```bash
npm install --save-dev js-yaml
```

**Version verification:** 
```bash
npm view js-yaml version
```
Verified as 4.1.1 on npm registry.

## Architecture Patterns

### Recommended Project Structure
```
plugin/vibe/
├── package.json
├── scripts/
│   ├── manifests.test.js
│   └── skills.test.js
├── skills/
│   ├── conductor/
│   │   └── SKILL.md
│   ├── init/
│   │   └── SKILL.md
│   ├── plan/
│   │   └── SKILL.md
│   ├── release-summary/
│   │   └── SKILL.md
│   ├── review-task/
│   │   └── SKILL.md
│   ├── run-task/
│   │   └── SKILL.md
│   └── status/
│       └── SKILL.md
```

### Pattern 1: Skill Frontmatter
**What:** Each `SKILL.md` must start with a YAML block containing `name`, `version`, and `description`.
**When to use:** In all `SKILL.md` files.
**Example:**
```markdown
---
name: vibe-init
version: 0.1.0
description: Initialize a project-local .vibe workspace.
---
# vibe init
...
```

### Anti-Patterns to Avoid
- **Legacy Directories:** Do not leave `commands/` or `roles/` folders in the `plugin/vibe/` directory. They must be deleted after migration.
- **Duplicate Skills:** The old `plugin/vibe/skills/Conductor.md` (capital C, at root of skills folder) must be removed. All skills must live in subdirectories `skills/<name>/SKILL.md`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| YAML parsing | Custom Regex extraction | `js-yaml` | YAML has complex rules for arrays, multi-line strings, and escaping. Custom regex extractors fail on edge cases. |

**Key insight:** Using a proven parser prevents false positives in test scripts, ensuring the cross-platform manifest files stay compliant with future parsers (like Claude and Gemini loaders).

## Runtime State Inventory

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | None — verified by grep | none |
| Live service config | None — verified by grep | none |
| OS-registered state | None — verified by grep | none |
| Secrets/env vars | None — verified by grep | none |
| Build artifacts | `plugin/vibe/README.md` and `references/` docs referencing legacy `commands/` or `roles/` | Code edit: Update documentation paths in these files |

## Common Pitfalls

### Pitfall 1: Frontmatter Boundaries
**What goes wrong:** The test script fails to extract the YAML block properly or reads the entire file.
**Why it happens:** The split delimiter `---` might be reused later in the markdown file.
**How to avoid:** Use a robust extraction method that only grabs the first occurrence of `---...---` at the very beginning of the file `^\s*---\n([\s\S]*?)\n---`.

### Pitfall 2: Broken Markdown Links
**What goes wrong:** Documentation links in `plugin/vibe/README.md` or `plugin/vibe/references/*.md` still point to `commands/plan.md`.
**Why it happens:** Moving files changes their relative paths.
**How to avoid:** Run a grep to update internal markdown links pointing to `plugin/vibe/commands/` or `plugin/vibe/roles/`.

## Code Examples

### Test Validation Script
```javascript
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const yaml = require('js-yaml');

test('SKILL.md files have valid frontmatter', () => {
  const content = fs.readFileSync(skillPath, 'utf8');
  const match = content.match(/^---\n([\s\S]*?)\n---/);
  assert.ok(match, 'Must have YAML frontmatter');
  
  const frontmatter = yaml.load(match[1]);
  assert.ok(frontmatter.name, 'Missing name in frontmatter');
  assert.ok(frontmatter.version, 'Missing version in frontmatter');
  assert.ok(frontmatter.description, 'Missing description in frontmatter');
});
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Flat `commands/*.md` | Namespaced `skills/*/SKILL.md` | Phase 26 | Universal plugin standard compatible with major platforms. |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | No assumptions | All | None |

## Open Questions
None.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | test scripts | ✓ | >=22.0.0 | — |
| npm | package management | ✓ | 10.9.x | — |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | node:test |
| Config file | `package.json` scripts |
| Quick run command | `npm run test:skills` |
| Full suite command | `npm test` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SKL-01 | Frontmatter schema validation | unit | `node --test plugin/vibe/scripts/skills.test.js` | ❌ Wave 0 |
| SKL-02 | Folder structure assertion | unit | `node --test plugin/vibe/scripts/skills.test.js` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `npm run test`
- **Per wave merge:** `npm run test`
- **Phase gate:** Full suite green before `/gsd-verify-work`

### Wave 0 Gaps
- [ ] `plugin/vibe/scripts/skills.test.js` — covers SKL-01, SKL-02
- [ ] Add `test:skills` to `plugin/vibe/package.json` and combine it with the `test` script

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V5 Input Validation | yes | `js-yaml` safeLoad (default) for YAML parsing |

### Known Threat Patterns for Node.js YAML parsing

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Code Execution via YAML | Elevation of Privilege | Use `js-yaml` which removes unsafe features by default (since v4) |

## Sources

### Primary (HIGH confidence)
- `.planning/phases/26-skill-standardization/26-CONTEXT.md` - Phase constraints and requirements.
- `plugin/vibe/package.json` - Current Node.js version and test scripts.
- npm registry (`npm view js-yaml version`) - Verified package version.

### Secondary (MEDIUM confidence)
- `plugin/vibe/scripts/manifests.test.js` - Used to infer test structure.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Built-in Node libraries and standard js-yaml.
- Architecture: HIGH - Matches D-01 through D-06 explicitly.
- Pitfalls: HIGH - Common migration errors.

**Research date:** 2026-04-24
**Valid until:** 2026-05-24