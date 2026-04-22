# Phase 20: plugin-first-architecture - Research

**Researched:** 2026-04-22  
**Domain:** Codex plugin package architecture, Agent Skills, `.vibe` migration boundary  
**Confidence:** HIGH for Codex plugin/skill shape and local code inventory; MEDIUM for future cross-provider command compatibility

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

## Implementation Decisions

### 产品形态
- **D-01:** v6.0 采用彻底 plugin-first，而不是 CLI-first 或 plugin-first + 独立 CLI runtime。
- **D-02:** 用户入口是当前 AI 终端可安装的 plugin。Plugin 注入 skills、commands、references 和 scripts，让当前主模型成为 Conductor。
- **D-03:** 不再把 `vibe-cli` 作为主产品形态；旧 CLI 能力需要被分类为迁入 plugin/scripts、保留兼容或移除。

### Runtime 边界
- **D-04:** 仍然需要 runtime，但 runtime 是 plugin 内的轻量 JS/Python scripts，不是独立重型 CLI、daemon 或数据库。
- **D-05:** scripts 只负责最低必要的确定性动作：初始化 `.vibe`、写 task JSON、文件锁、启动 Agent subprocess、记录 logs/results、生成 release summary。
- **D-06:** 协作智能留在模型和 skill 中，scripts 不实现复杂调度大脑，避免在脚本层重建重型系统。

### `.vibe` 工作区
- **D-07:** `.vibe` 是项目级可观察工作区，默认包含 `Agents/`、`tasks/`、`runs/`、`locks/`、`reviews/`、`logs/` 和配置文件。
- **D-08:** `.vibe/Agents` 是角色与模型定义的核心位置，可定义 planner、executor、reviewer、release 等 Agent 以及使用 claude/gemini/codex 的命令。
- **D-09:** `.vibe` 文件格式必须足够简单，让当前主模型能直接读取并推理，不依赖隐藏状态。

### 多模型协作协议
- **D-10:** 主模型负责与用户多轮问答完善计划细节，然后按计划大小拆分为多个 task。
- **D-11:** executor Agent 执行 task 后，reviewer Agent 审查执行结果并要求修复，主模型聚合最终结果。
- **D-12:** 子 Agent 通过 shell/subprocess 启动 claude、gemini、codex 等 CLI；不要求 terminal pane 编排作为默认路径。

### Phase 20 规划取舍
- **D-13:** Plugin 包结构采用通用 plugin 目录设计，但 Phase 20 先适配 Codex plugin 格式，避免过早绑定单一平台。
- **D-14:** Scripts runtime 默认使用 JS 实现；Python 可作为未来可选 runtime，但 Phase 20 不做双实现。
- **D-15:** 旧 Rust CLI 暂时保留为 compatibility reference，用于迁移分类和行为对照，不在 Phase 20 直接大规模删除。
- **D-16:** 子 Agent 默认通过 subprocess 非交互执行；terminal pane 编排保留为可选兼容模式，不作为默认执行路径。
- **D-17:** Phase 20 不只产出架构文档和迁移表，也要 scaffold 最小 plugin 目录，让后续 Phase 21/22 能直接接着扩展。
- **D-18:** 产品名定为 **Vibe**。`vibe-cli` 只表示旧 Rust CLI 实现或兼容层，不再作为主产品名。
- **D-19:** Plugin package root 使用 `plugin/vibe/`，内部放置 `.codex-plugin/plugin.json`、`skills/`、`commands/`、`references/`、`scripts/`、`templates/` 和 `examples/`。
- **D-20:** 旧 Rust workspace 暂时保留在 `apps/` 与 `crates/`，Phase 20 只建立迁移分类与 plugin scaffold，不做大规模目录搬迁。

### Claude's Discretion

- Plugin 具体文件夹命名、JS 或 Python 的默认选择、task JSON 字段命名和 release notes 模板格式可由 planner 在 Phase 20 plan 中提出具体方案。
- 只要不违背 plugin-first 与 scripts-thin-runtime 的边界，planner 可以保留少量兼容旧 Rust CLI 的过渡方案。

### Deferred Ideas (OUT OF SCOPE)

- Publishing plugin to external marketplaces — future milestone after local plugin package stabilizes.
- Direct GitHub release publishing through network APIs — future milestone after local release notes generation works.
- Agent historical success scoring — future enhancement after deterministic task/review loop exists.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PLUG-01 | Project defines a plugin package layout that can inject skills, commands, references, and executable scripts into supported AI terminal environments. | Use `plugin/vibe/` with required `.codex-plugin/plugin.json`, `skills/`, `commands/`, `references/`, `scripts/`, `templates/`, and `examples/`; Codex docs verify plugin manifest and marketplace mechanics. [VERIFIED: .planning/REQUIREMENTS.md] [CITED: https://developers.openai.com/codex/plugins/build] |
| PLUG-02 | Plugin references define the collaboration protocol, task contract, Agent contract, review protocol, and `.vibe` workspace layout in model-readable documents. | Keep protocol as Markdown under `plugin/vibe/references/`; existing `skills/vibe-operator/references/*.md` are reusable source material for collaboration, state, orchestration, and verification. [VERIFIED: skills/vibe-operator/references] |
| PLUG-03 | Plugin skills teach the current model to act as Conductor. | Create at least one conductor skill under `plugin/vibe/skills/<skill>/SKILL.md`; Codex Skills docs define `SKILL.md`, optional `scripts/`, `references/`, and progressive disclosure behavior. [CITED: https://developers.openai.com/codex/skills] |
| PLUG-04 | Plugin commands expose init, plan, run task, review task, status, and release summary. | Scaffold `commands/` as stable command entry docs/wrappers, with deterministic work delegated to scripts and judgment delegated to skills/references. [VERIFIED: 20-CONTEXT.md] [ASSUMED] |
| PLUG-05 | Old standalone CLI responsibilities are classified into migrate-to-script, keep-as-compatibility, or remove categories. | Classify every current `Commands` enum member and supporting state/bus behavior using `apps/vibe-cli/src/main.rs`, `state/mod.rs`, and `ipc/bus.rs`. [VERIFIED: apps/vibe-cli/src/main.rs] [VERIFIED: crates/vibe-core/src/state/mod.rs] [VERIFIED: crates/vibe-core/src/ipc/bus.rs] |
</phase_requirements>

## Summary

Phase 20 should plan a documentation-and-scaffold change, not a full runtime rewrite: create the Codex-compatible `plugin/vibe/` package, put Conductor behavior in skills, put stable contracts in references, reserve scripts for deterministic filesystem/subprocess actions, and produce a migration matrix for old Rust CLI commands. [VERIFIED: 20-CONTEXT.md] [CITED: https://developers.openai.com/codex/plugins/build] [CITED: https://developers.openai.com/codex/skills]

The key architecture boundary is that `.vibe` remains project-local and inspectable, while plugin files teach or invoke the workflow. Old Rust assets are useful evidence: `FileBus` already demonstrates atomic file write and consume semantics, `ProjectConfig` shows simple JSON config and role-command mapping, and `spawn_role` shows CLI subprocess/persona lessons; however pane orchestration should become compatibility, not the default product path. [VERIFIED: crates/vibe-core/src/ipc/bus.rs] [VERIFIED: crates/vibe-core/src/state/mod.rs] [VERIFIED: apps/vibe-cli/src/main.rs]

**Primary recommendation:** Plan Phase 20 as four deliverables: `plugin/vibe/` scaffold, model-readable reference contracts, Conductor command/skill stubs, and a reviewed CLI migration classification table. [VERIFIED: .planning/ROADMAP.md] [VERIFIED: 20-CONTEXT.md]

## Project Constraints (from CLAUDE.md)

- The project has historically been a Rust physical scheduling layer for terminal AI Agents on WezTerm/Tmux. [VERIFIED: CLAUDE.md]
- Existing conventions prefer Rust 2024, Cargo, `rustfmt`, `clippy`, standard `Result`/`Option`, and `cargo test`. [VERIFIED: CLAUDE.md] [VERIFIED: Cargo.toml]
- GSD workflow guidance says file-changing work should go through GSD entry points unless explicitly bypassed. [VERIFIED: CLAUDE.md]
- `AGENTS.md` was not present in the repo root during research; `CLAUDE.md` was present and used. [VERIFIED: ls AGENTS.md CLAUDE.md]
- No `.claude/skills/` or `.agents/skills/` project skill directories were present; repo-local legacy `skills/vibe-operator/` exists and is a migration source, not an installed Codex project skill location. [VERIFIED: find .claude .agents -maxdepth 3 -name SKILL.md] [VERIFIED: skills/vibe-operator/SKILL.md]

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|--------------|----------------|-----------|
| Plugin installation/package discovery | Codex plugin layer | Marketplace catalog | Codex requires plugin manifests and can use repo or personal marketplace JSON to expose installable plugins. [CITED: https://developers.openai.com/codex/plugins/build] |
| Conductor reasoning and workflow decisions | Current model via skill | References | Skills package instructions/resources/scripts and are activated by explicit or implicit invocation. [CITED: https://developers.openai.com/codex/skills] |
| Collaboration protocol, task/Agent/review contracts | References | `.vibe` examples | Markdown references are model-readable and map directly from existing Vibe SOP documents. [VERIFIED: skills/vibe-operator/references] |
| Deterministic filesystem/runtime actions | Plugin scripts | `.vibe` workspace | Locked scope says scripts initialize `.vibe`, write tasks, lock files, launch subprocesses, and record logs/results. [VERIFIED: 20-CONTEXT.md] |
| Durable project state | `.vibe` workspace | Plugin templates | Requirements assign workspace directories and config to `.vibe`, not to hidden daemon/database state. [VERIFIED: .planning/REQUIREMENTS.md] |
| Legacy pane orchestration | Compatibility Rust CLI | Optional terminal adapter | Current CLI implements WezTerm/Tmux pane split, focus, inject, kill, and status; Phase 20 keeps it as reference/compatibility. [VERIFIED: apps/vibe-cli/src/main.rs] |

## Standard Stack

### Core

| Library / Surface | Version | Purpose | Why Standard |
|-------------------|---------|---------|--------------|
| Codex plugin manifest | Schema is manifest-driven; no npm package version | Required package identity at `plugin/vibe/.codex-plugin/plugin.json`. [CITED: https://developers.openai.com/codex/plugins/build] | Official Codex docs define this as the manual plugin starting point and `$plugin-creator` scaffolds it. [CITED: https://developers.openai.com/codex/plugins/build] [VERIFIED: /Users/anjia/.codex/skills/.system/plugin-creator/references/plugin-json-spec.md] |
| Codex Agent Skills | Current Codex docs as of 2026-04-22 | Conductor behavior and reusable workflow instructions. [CITED: https://developers.openai.com/codex/skills] | Skills are the authoring format; plugins are the installable distribution unit. [CITED: https://developers.openai.com/codex/skills] |
| Markdown references | N/A | Collaboration protocol, task contract, Agent contract, review protocol, workspace layout. [VERIFIED: .planning/REQUIREMENTS.md] | Existing `skills/vibe-operator/references/` already use Markdown SOPs that models can read. [VERIFIED: skills/vibe-operator/references] |
| Node.js runtime | `v22.16.0` locally | Default JS scripts runtime for future deterministic actions. [VERIFIED: node --version] | Locked decision says JS is default and Python is future optional. [VERIFIED: 20-CONTEXT.md] |
| JSON workspace contracts | N/A | `plugin.json`, marketplace JSON, `.vibe/config.json`, task/run/review files. [CITED: https://developers.openai.com/codex/plugins/build] [VERIFIED: .vibe/config.json] | Existing Rust config and bus already use JSON and atomic filesystem writes. [VERIFIED: crates/vibe-core/src/state/mod.rs] [VERIFIED: crates/vibe-core/src/ipc/bus.rs] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| Rust workspace | `5.0.0`, Rust 2024 | Compatibility reference for old CLI behavior. [VERIFIED: Cargo.toml] | Keep during Phase 20 for migration classification and tests; do not make it the new primary product. [VERIFIED: 20-CONTEXT.md] |
| Cargo test | `cargo 1.94.1` locally | Existing Rust validation. [VERIFIED: cargo --version] | Use when touching legacy Rust docs/tests or verifying no accidental Rust breakage. [VERIFIED: CLAUDE.md] |
| Codex CLI | `codex-cli 0.122.0` locally | One possible subprocess Agent target. [VERIFIED: codex --version] | Use in examples as configurable Agent command, not hard-coded default. [VERIFIED: .vibe/config.json] |
| Claude CLI | `2.1.92` locally | One possible subprocess Agent target. [VERIFIED: claude --version] | Use in `.vibe/Agents` examples as configured command. [VERIFIED: apps/vibe-cli/src/main.rs] |
| Gemini CLI | `0.38.2` locally | One possible subprocess Agent target. [VERIFIED: gemini --version] | Current `.vibe/config.json` sets Gemini for roles. [VERIFIED: .vibe/config.json] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Codex-first plugin scaffold | Generic provider-neutral package only | Locked decision says generic design is desired, but Phase 20 first adapts Codex plugin format. [VERIFIED: 20-CONTEXT.md] |
| Thin JS scripts | Rust CLI runtime | Rust code already exists, but locked decision rejects standalone heavy CLI as primary product. [VERIFIED: 20-CONTEXT.md] [VERIFIED: apps/vibe-cli/src/main.rs] |
| Subprocess Agent launch | Terminal pane orchestration | Subprocess is default; pane orchestration remains optional compatibility. [VERIFIED: 20-CONTEXT.md] |

**Installation:**

```bash
# Phase 20 should not add runtime dependencies.
# Scaffold files under plugin/vibe/ and keep scripts dependency-free until Phase 22.
```

**Version verification:** No new npm package is required for Phase 20; local tool versions verified were Node `v22.16.0`, npm `10.9.2`, Cargo `1.94.1`, Rust `1.94.1`, Codex CLI `0.122.0`, Claude `2.1.92`, and Gemini `0.38.2`. [VERIFIED: node --version] [VERIFIED: npm --version] [VERIFIED: cargo --version] [VERIFIED: rustc --version] [VERIFIED: codex --version] [VERIFIED: claude --version] [VERIFIED: gemini --version]

## Architecture Patterns

### System Architecture Diagram

```text
User request
  |
  v
AI terminal loads Vibe plugin
  |
  +--> plugin/vibe/.codex-plugin/plugin.json
  |       |
  |       v
  |    exposes skills + command docs + references + scripts
  |
  v
Conductor skill in current model
  |
  +--> reads references: collaboration protocol, task contract, Agent contract, review protocol, workspace layout
  |
  +--> decision: clarify more?
  |       | yes -> ask user, update plan
  |       | no
  v
Command entry: init / plan / run-task / review-task / status / release-summary
  |
  +--> deterministic script needed?
          | yes -> JS script writes/reads .vibe files, launches subprocess Agent, records run/log/review artifacts
          | no  -> model performs reasoning from references and .vibe state
  |
  v
.vibe workspace: Agents / tasks / runs / locks / reviews / logs / config.json
  |
  v
Executor/reviewer subprocess CLIs: claude / gemini / codex / configured command
  |
  v
Structured artifacts returned to Conductor for aggregation
```

### Recommended Project Structure

```text
plugin/
└── vibe/
    ├── .codex-plugin/
    │   └── plugin.json
    ├── skills/
    │   └── conductor/
    │       └── SKILL.md
    ├── commands/
    │   ├── init.md
    │   ├── plan.md
    │   ├── run-task.md
    │   ├── review-task.md
    │   ├── status.md
    │   └── release-summary.md
    ├── references/
    │   ├── collaboration-protocol.md
    │   ├── task-contract.md
    │   ├── agent-contract.md
    │   ├── review-protocol.md
    │   ├── workspace-layout.md
    │   └── migration-classification.md
    ├── scripts/
    │   └── README.md
    ├── templates/
    │   └── .vibe/
    │       ├── Agents/
    │       ├── tasks/
    │       ├── runs/
    │       ├── locks/
    │       ├── reviews/
    │       ├── logs/
    │       └── config.json
    └── examples/
```

This structure follows the locked `plugin/vibe/` root and Codex's required `.codex-plugin/plugin.json`; `commands/` is kept as a Vibe-defined surface because Codex plugin examples mention plugin-level `commands/`, but detailed command file semantics are not fully specified in the official build page. [VERIFIED: 20-CONTEXT.md] [CITED: https://github.com/openai/plugins] [CITED: https://developers.openai.com/codex/plugins/build] [ASSUMED]

### Pattern 1: Manifest First, Everything Else Relative

**What:** Put plugin identity in `plugin/vibe/.codex-plugin/plugin.json`, and point optional surfaces with relative `./` paths. [CITED: https://developers.openai.com/codex/plugins/build] [VERIFIED: /Users/anjia/.codex/skills/.system/plugin-creator/references/plugin-json-spec.md]

**When to use:** Always in Phase 20 scaffold. [VERIFIED: 20-CONTEXT.md]

**Example:**

```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for project-local Agent workflows.",
  "skills": "./skills/",
  "interface": {
    "displayName": "Vibe",
    "shortDescription": "Coordinate AI Agents through project-local tasks, reviews, and logs."
  }
}
```

Source: Codex build plugins minimal manifest plus local plugin-creator schema. [CITED: https://developers.openai.com/codex/plugins/build] [VERIFIED: /Users/anjia/.codex/skills/.system/plugin-creator/references/plugin-json-spec.md]

### Pattern 2: Skills Own Judgment; Scripts Own Determinism

**What:** A Conductor skill describes clarification, planning, task splitting, Agent selection, review routing, and aggregation; scripts only perform filesystem/subprocess operations. [VERIFIED: 20-CONTEXT.md] [CITED: https://developers.openai.com/codex/skills]

**When to use:** For every PLUG-03/PLUG-04 task. [VERIFIED: .planning/REQUIREMENTS.md]

**Example:**

```markdown
---
name: vibe-conductor
description: Use when coordinating Vibe multi-agent tasks through .vibe workspace files.
---

1. Read references/workspace-layout.md and references/task-contract.md.
2. Clarify the user request until task boundaries and verification are explicit.
3. Write or request task JSON through the init/plan command surface.
4. Launch executor/reviewer only through configured Agent commands or scripts.
5. Aggregate artifacts from .vibe/runs, .vibe/reviews, and .vibe/logs.
```

Source: Codex Skills required frontmatter and Vibe locked Conductor behavior. [CITED: https://developers.openai.com/codex/skills] [VERIFIED: 20-CONTEXT.md]

### Pattern 3: File-Based Atomic State

**What:** Preserve the old `FileBus` lesson: write to temp file, rename to final JSON, consume only after reading a matching signal/artifact. [VERIFIED: crates/vibe-core/src/ipc/bus.rs]

**When to use:** In Phase 20 docs as a reference pattern; implement actual JS utilities in Phase 22. [VERIFIED: .planning/ROADMAP.md]

**Example:**

```javascript
import { writeFile, rename, mkdir } from "node:fs/promises";
import { randomUUID } from "node:crypto";
import path from "node:path";

export async function writeJsonAtomic(dir, prefix, payload) {
  await mkdir(dir, { recursive: true });
  const finalPath = path.join(dir, `${Date.now()}-${prefix}-${randomUUID().slice(0, 8)}.json`);
  const tmpPath = `${finalPath}.tmp`;
  await writeFile(tmpPath, JSON.stringify(payload, null, 2));
  await rename(tmpPath, finalPath);
  return finalPath;
}
```

Source: JS adaptation of local Rust `FileBus::send` atomic temp-write/rename pattern. [VERIFIED: crates/vibe-core/src/ipc/bus.rs]

### Anti-Patterns to Avoid

- **Rebuilding the old CLI as JS:** Phase 20 should document/script only the thin deterministic primitives, not recreate pane management, TUI, or smart scheduling in scripts. [VERIFIED: 20-CONTEXT.md]
- **Hiding state outside `.vibe`:** Requirements require model-readable state and no daemon/database dependency. [VERIFIED: .planning/REQUIREMENTS.md]
- **Treating terminal panes as the default Agent runtime:** Terminal pane orchestration is optional compatibility; subprocess is default. [VERIFIED: 20-CONTEXT.md]
- **Overwriting user workspace files:** Phase 21 requires non-destructive initialization; Phase 20 templates should already encode that expectation. [VERIFIED: .planning/REQUIREMENTS.md]
- **Letting commands contain policy that belongs in references:** Commands should be entry points; contracts and review rules belong in references so every skill/script can cite the same source. [VERIFIED: .planning/REQUIREMENTS.md] [ASSUMED]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Plugin package discovery | Custom installer or bespoke plugin registry | Codex `.codex-plugin/plugin.json` plus optional marketplace JSON | Official docs already define local repo/personal marketplace mechanics. [CITED: https://developers.openai.com/codex/plugins/build] |
| Skill loading and context management | Custom prompt concatenation runtime | Codex Agent Skills with `SKILL.md`, references, and progressive disclosure | Codex loads skill metadata first and full instructions when used. [CITED: https://developers.openai.com/codex/skills] |
| Agent execution intelligence | Script scheduler brain | Conductor skill plus model reasoning | Locked decisions keep collaboration intelligence in model/skill, not scripts. [VERIFIED: 20-CONTEXT.md] |
| File atomicity | Ad hoc direct JSON writes | Temp-file then rename pattern | Existing `FileBus` and `StateStore` already use atomic filesystem writes. [VERIFIED: crates/vibe-core/src/ipc/bus.rs] [VERIFIED: crates/vibe-core/src/state/mod.rs] |
| Cross-Agent state | Hidden database/daemon | `.vibe` JSON files | Requirements explicitly keep `.vibe` inspectable and daemon-free. [VERIFIED: .planning/REQUIREMENTS.md] |
| Terminal pane lifecycle | New JS pane manager | Compatibility Rust CLI only when justified | Old pane lifecycle is tightly coupled to WezTerm/Tmux adapters and should not be the default path. [VERIFIED: apps/vibe-cli/src/main.rs] [VERIFIED: 20-CONTEXT.md] |

**Key insight:** Phase 20's safest plan is to standardize contracts and boundaries before implementing runtime; the hard problem is architectural ownership, not code volume. [VERIFIED: .planning/ROADMAP.md] [VERIFIED: 20-CONTEXT.md]

## Legacy CLI Migration Classification

| Old CLI capability | Category | Phase 20 classification rationale |
|--------------------|----------|-----------------------------------|
| `init --force` wizard/config/role creation | Migrate-to-script, redesigned | Initialization belongs to plugin scripts/templates; old interactive wizard and `.vibe/roles` should become non-destructive `.vibe/Agents` generation later. [VERIFIED: apps/vibe-cli/src/main.rs] [VERIFIED: crates/vibe-core/src/state/mod.rs] [VERIFIED: .planning/REQUIREMENTS.md] |
| `run <command>` with `VIBE_MASTER_ID` | Migrate-to-script | Subprocess launch is a locked default, but future scripts should capture stdout/stderr/exit/timestamps into `.vibe/runs` and `.vibe/logs`. [VERIFIED: apps/vibe-cli/src/main.rs] [VERIFIED: .planning/REQUIREMENTS.md] |
| `signal` / `wait` file bus semantics | Migrate-to-script/reference | Atomic file bus is valuable, but should become a general artifact/event pattern under `.vibe`, not a required pane protocol. [VERIFIED: crates/vibe-core/src/ipc/bus.rs] [VERIFIED: 20-CONTEXT.md] |
| `report --status --message` | Migrate-to-script, redesigned | Status/reporting maps to task/run/review artifacts, not `panes.json` as the main state store. [VERIFIED: apps/vibe-cli/src/main.rs] [VERIFIED: .planning/REQUIREMENTS.md] |
| `spawn --role/--stack` Agent command construction | Migrate-to-script/reference | Command selection, persona injection, and CLI flags are useful lessons; pane/tab creation is not default. [VERIFIED: apps/vibe-cli/src/main.rs] [VERIFIED: 20-CONTEXT.md] |
| `split`, `focus`, `inject`, `kill`, pane-backed `list`, `check` | Compatibility | These are terminal orchestration features tied to WezTerm/Tmux and remain optional compatibility. [VERIFIED: apps/vibe-cli/src/main.rs] [VERIFIED: 20-CONTEXT.md] |
| `status` TUI | Compatibility or remove | TUI reads pane state, while plugin-first status should read `.vibe/tasks`, `.vibe/runs`, reviews, and logs. [VERIFIED: apps/vibe-cli/src/main.rs] [VERIFIED: .planning/REQUIREMENTS.md] |
| `.vibe/roles/*.md` | Migrate-to-`.vibe/Agents` | Current roles are Markdown personas; new workspace requires `.vibe/Agents` definitions with command, references, tools, and outputs. [VERIFIED: .vibe/roles/Conductor.md] [VERIFIED: .planning/REQUIREMENTS.md] |
| `.vibe/state/panes.json` | Compatibility | Pane records are legacy terminal state; future resumability is task/run/review state. [VERIFIED: crates/vibe-core/src/state/mod.rs] [VERIFIED: .planning/REQUIREMENTS.md] |

## Runtime State Inventory

| Category | Items Found | Action Required |
|----------|-------------|-----------------|
| Stored data | `.vibe/config.json` stores `roles`, `default_command`, and `stacks`; `.vibe/state/panes.json` exists; `.vibe/roles/*.md` exists. [VERIFIED: .vibe/config.json] [VERIFIED: find .vibe -maxdepth 3 -type f -print] | Phase 20 should document migration mapping only; Phase 21 should create non-destructive `.vibe/Agents` and config migration rules. [VERIFIED: .planning/ROADMAP.md] |
| Live service config | None found in repo or local inspection; no external service UI/database was identified for Phase 20. [VERIFIED: rg results] | No data migration task in Phase 20; revisit if future marketplace publishing or GitHub release APIs are introduced. [VERIFIED: .planning/STATE.md] |
| OS-registered state | No launchd/systemd/pm2/task scheduler registrations found in scope. [VERIFIED: rg results] | None for Phase 20. |
| Secrets/env vars | Runtime env vars appear in code as `VIBE_MASTER_ID`, `VIBE_ID`, and `VIBE_PERSONA`; no `.env` or secret file was found in the researched file list. [VERIFIED: apps/vibe-cli/src/main.rs] [VERIFIED: rg results] | Document as legacy compatibility variables; new scripts should prefer explicit task/Agent files and only pass env vars as subprocess context. [VERIFIED: 20-CONTEXT.md] |
| Build artifacts | `target/` contains Rust build/test artifacts; old Rust workspace remains in `apps/` and `crates/`. [VERIFIED: rg/find output] [VERIFIED: 20-CONTEXT.md] | Do not remove in Phase 20; later CLI slimming belongs to Phase 24. [VERIFIED: .planning/ROADMAP.md] |

## Common Pitfalls

### Pitfall 1: Plugin Scaffold Without Install Path

**What goes wrong:** The repo has `plugin/vibe/`, but Codex cannot see it because no marketplace entry or manual install path exists. [CITED: https://developers.openai.com/codex/plugins/build]

**Why it happens:** Codex reads plugins through marketplace/catalog paths and installed cache, not arbitrary folders. [CITED: https://developers.openai.com/codex/plugins/build]

**How to avoid:** Phase 20 should include either a repo-local `.agents/plugins/marketplace.json` example or explicit "manual install later" note; locked root remains `plugin/vibe/`, so the marketplace path must point there if used. [CITED: https://developers.openai.com/codex/plugins/build] [VERIFIED: 20-CONTEXT.md]

**Warning signs:** `plugin/vibe/.codex-plugin/plugin.json` exists but no install/marketplace verification step exists. [ASSUMED]

### Pitfall 2: Scripts Become a Scheduler

**What goes wrong:** JS scripts start deciding task priority, Agent choice, review loops, or recovery policy. [VERIFIED: 20-CONTEXT.md]

**Why it happens:** The old CLI mixed orchestration, pane lifecycle, status, signaling, and role behavior in one surface. [VERIFIED: apps/vibe-cli/src/main.rs]

**How to avoid:** Put policy in skills/references; scripts only execute deterministic actions and record artifacts. [VERIFIED: 20-CONTEXT.md]

**Warning signs:** A script needs model-like branching, retry heuristics, or hidden mutable state beyond `.vibe` files. [ASSUMED]

### Pitfall 3: Copying `.vibe/roles` Instead of Designing `.vibe/Agents`

**What goes wrong:** The new workspace keeps old role Markdown but lacks command, prompt/reference, allowed tools, expected output, or review metadata. [VERIFIED: .vibe/roles/Conductor.md] [VERIFIED: .planning/REQUIREMENTS.md]

**Why it happens:** Existing role files are useful personas, but the v6.0 requirements ask for richer Agent definitions. [VERIFIED: .planning/REQUIREMENTS.md]

**How to avoid:** Use old roles as input examples only; define `.vibe/Agents` contract in references and templates. [VERIFIED: 20-CONTEXT.md]

**Warning signs:** `templates/.vibe/Agents/` is empty while `.vibe/roles/` content is copied verbatim. [ASSUMED]

### Pitfall 4: Over-Specifying Future Runtime in Phase 20

**What goes wrong:** The plan implements task JSON, locks, subprocess capture, and release summaries instead of only documenting/scaffolding boundaries. [VERIFIED: .planning/ROADMAP.md]

**Why it happens:** Requirements for RUN/FLOW/REL phases are adjacent and tempting to pull forward. [VERIFIED: .planning/REQUIREMENTS.md]

**How to avoid:** Keep Phase 20 artifacts as contracts/stubs; schedule implementation to Phase 21-24. [VERIFIED: .planning/ROADMAP.md]

**Warning signs:** Phase 20 tasks include working lock acquisition or executor launch beyond placeholder scripts. [ASSUMED]

## Code Examples

### Minimal Plugin Manifest

```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for Vibe workspaces.",
  "skills": "./skills/"
}
```

Source: Codex build plugins manual example. [CITED: https://developers.openai.com/codex/plugins/build]

### Repo Marketplace Entry Pointing to Locked Root

```json
{
  "name": "vibe-local",
  "interface": {
    "displayName": "Vibe Local Plugins"
  },
  "plugins": [
    {
      "name": "vibe",
      "source": {
        "source": "local",
        "path": "./plugin/vibe"
      },
      "policy": {
        "installation": "AVAILABLE",
        "authentication": "ON_INSTALL"
      },
      "category": "Productivity"
    }
  ]
}
```

Source: Codex marketplace metadata allows local `source.path` relative to marketplace root and requires policy/category fields; root path adapted to locked `plugin/vibe/`. [CITED: https://developers.openai.com/codex/plugins/build] [VERIFIED: 20-CONTEXT.md]

### Conductor Skill Skeleton

```markdown
---
name: vibe-conductor
description: Use when planning or running Vibe multi-agent workflows through plugin/vibe and .vibe workspace files.
---

Follow `references/collaboration-protocol.md`.
Clarify before planning, persist plans as task contracts, launch configured Agents through command/script entry points, require reviewer output, and aggregate artifacts for the user.
```

Source: Codex skill structure and Vibe Conductor decisions. [CITED: https://developers.openai.com/codex/skills] [VERIFIED: 20-CONTEXT.md]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Standalone `vibe-cli` as primary UX | Installable Codex plugin package with skills/references/scripts | Milestone 6.0 pivot on 2026-04-22 | CLI becomes compatibility reference, not product center. [VERIFIED: .planning/STATE.md] |
| `.vibe/roles` Markdown personas | `.vibe/Agents` role/model/tool/output definitions | Planned for Phase 21 | Phase 20 must define contracts/templates without fully migrating runtime state. [VERIFIED: .planning/ROADMAP.md] |
| Pane-based Agent orchestration | Non-interactive subprocess Agent launch by default | Phase 20 locked decision | WezTerm/Tmux stays optional compatibility. [VERIFIED: 20-CONTEXT.md] |
| Hidden or terminal-specific status | Inspectable `.vibe/tasks`, `runs`, `reviews`, `logs`, `locks` | Milestone 6.0 requirements | Status must become file/artifact based. [VERIFIED: .planning/REQUIREMENTS.md] |

**Deprecated/outdated:**

- `vibe-cli` as main product name is outdated; product name is Vibe and `vibe-cli` refers to legacy Rust CLI or compatibility layer. [VERIFIED: 20-CONTEXT.md]
- `.vibe/state/panes.json` as main progress source is outdated for plugin-first workflows; future status should read task/run/review artifacts. [VERIFIED: crates/vibe-core/src/state/mod.rs] [VERIFIED: .planning/REQUIREMENTS.md]
- Terminal pane orchestration as default execution path is outdated; subprocess launch is default. [VERIFIED: 20-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `plugin/vibe/commands/*.md` is a safe scaffold shape for Vibe command docs/stubs even though Codex official build docs do not fully specify plugin command file semantics. | Architecture Patterns, Phase Requirements | Planner may need to adjust file names once command loading semantics are verified in Codex app/CLI. |
| A2 | Commands should be entry-point docs/wrappers and policy belongs in references. | Anti-Patterns | Planner might split command and reference content differently if Codex command docs impose stricter format. |
| A3 | Warning signs listed under pitfalls are predictive heuristics. | Common Pitfalls | Low implementation risk; planner should treat them as review checklist items, not hard requirements. |

## Open Questions (RESOLVED)

1. **Should Phase 20 create `.agents/plugins/marketplace.json` now?**
   - What we know: Codex supports repo marketplace files and local plugin paths. [CITED: https://developers.openai.com/codex/plugins/build]
   - What's unclear: The locked root is `plugin/vibe/`, while official examples commonly use `plugins/<name>/`; docs allow local paths, but the team may prefer deferring install wiring. [CITED: https://developers.openai.com/codex/plugins/build] [VERIFIED: 20-CONTEXT.md]
   - Resolution: Phase 20 should create a repo-local `.agents/plugins/marketplace.json` example that points to `./plugin/vibe`, because PLUG-01 requires a concrete loading/discovery path, not just a package directory. This remains a local Codex discovery scaffold, not external marketplace publishing. [RESOLVED]
   - Planning impact: Plan 20-01 must include `.agents/plugins/marketplace.json` in `files_modified`, a task action that creates it, and verification that the file references `./plugin/vibe`. [RESOLVED]

2. **What exact Codex command file semantics should Vibe use?**
   - What we know: OpenAI plugin examples mention plugin-level `commands/`. [CITED: https://github.com/openai/plugins]
   - What's unclear: The official build page emphasizes manifest/skills/marketplaces more than command file schema. [CITED: https://developers.openai.com/codex/plugins/build]
   - Resolution: Phase 20 `plugin/vibe/commands/*.md` files are documented command contracts and stubs only. They must not claim executable command binding until a later implementation phase verifies provider-specific command semantics. [RESOLVED]
   - Planning impact: Plan 20-03 must require command files to contain explicit wording such as `Command contract only` and must avoid language implying that Codex already loads these `.md` files as executable commands. [RESOLVED]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Node.js | Future JS scripts and Phase 20 scaffold examples | yes | `v22.16.0` | Keep scripts as docs/stubs if Node unavailable. [VERIFIED: node --version] |
| npm | Optional package checks/tooling | yes | `10.9.2` | No npm packages required in Phase 20. [VERIFIED: npm --version] |
| Cargo | Existing Rust validation | yes | `1.94.1` | Skip Rust test execution if no Rust files change. [VERIFIED: cargo --version] |
| Rust | Existing Rust workspace | yes | `1.94.1` | Not needed for plugin-only docs/scaffold except validation. [VERIFIED: rustc --version] |
| Git | Release/migration references and later release summary | yes | `2.49.0` | Manual changelog examples until Phase 24. [VERIFIED: git --version] |
| Codex CLI | Agent subprocess example | yes | `codex-cli 0.122.0` | Keep as configurable Agent command. [VERIFIED: codex --version] |
| Claude CLI | Agent subprocess example | yes | `2.1.92` | Keep as configurable Agent command. [VERIFIED: claude --version] |
| Gemini CLI | Current config default Agent command | yes | `0.38.2` | Keep as configurable Agent command. [VERIFIED: gemini --version] |

**Missing dependencies with no fallback:** None for Phase 20 research/scaffold. [VERIFIED: environment probes]

**Missing dependencies with fallback:** None found. [VERIFIED: environment probes]

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Cargo built-in tests for Rust workspace; no JS test framework detected. [VERIFIED: Cargo.toml] [VERIFIED: rg test config scan] |
| Config file | `Cargo.toml`; no `package.json`, Jest, Vitest, or pytest config found in source scope. [VERIFIED: Cargo.toml] [VERIFIED: find test config scan] |
| Quick run command | `cargo test -p vibe-core state::tests::test_vibe_initialization` [VERIFIED: crates/vibe-core/src/state/mod.rs] |
| Full suite command | `cargo test` [VERIFIED: CLAUDE.md] |

### Phase Requirements -> Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|--------------|
| PLUG-01 | Plugin package layout exists with required Codex manifest and Vibe surfaces. | filesystem smoke | `test -f plugin/vibe/.codex-plugin/plugin.json` | No, Wave 0. [VERIFIED: rg --files] |
| PLUG-02 | References include collaboration, task, Agent, review, workspace layout docs. | filesystem smoke | `test -f plugin/vibe/references/collaboration-protocol.md` | No, Wave 0. [VERIFIED: rg --files] |
| PLUG-03 | Conductor skill exists with Codex `SKILL.md` frontmatter. | filesystem/content smoke | `test -f plugin/vibe/skills/conductor/SKILL.md` | No, Wave 0. [VERIFIED: rg --files] |
| PLUG-04 | Command stubs for init, plan, run task, review task, status, release summary exist. | filesystem smoke | `test -f plugin/vibe/commands/init.md` | No, Wave 0. [VERIFIED: rg --files] |
| PLUG-05 | Migration classification document covers old CLI responsibilities. | content smoke | `rg "Migrate-to-script|Compatibility|Remove" plugin/vibe/references/migration-classification.md` | No, Wave 0. [VERIFIED: rg --files] |

### Sampling Rate

- **Per task commit:** `cargo test -p vibe-core state::tests::test_vibe_initialization` only if Rust files or `.vibe` initialization docs are touched; otherwise run filesystem smoke checks for scaffold files. [VERIFIED: crates/vibe-core/src/state/mod.rs] [ASSUMED]
- **Per wave merge:** `cargo test` plus scaffold smoke checks. [VERIFIED: CLAUDE.md] [ASSUMED]
- **Phase gate:** `cargo test` and all PLUG filesystem/content smoke checks green before `/gsd-verify-work`. [VERIFIED: .planning/config.json] [ASSUMED]

### Wave 0 Gaps

- [ ] `plugin/vibe/.codex-plugin/plugin.json` - covers PLUG-01. [VERIFIED: rg --files]
- [ ] `plugin/vibe/skills/conductor/SKILL.md` - covers PLUG-03. [VERIFIED: rg --files]
- [ ] `plugin/vibe/references/*.md` - covers PLUG-02 and PLUG-05. [VERIFIED: rg --files]
- [ ] `plugin/vibe/commands/*.md` - covers PLUG-04. [VERIFIED: rg --files]
- [ ] `plugin/vibe/templates/.vibe/` scaffold - supports Phase 21 planning boundary. [VERIFIED: .planning/ROADMAP.md]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | No for Phase 20 local scaffold | No auth implementation in Phase 20; marketplace auth policy is metadata only. [CITED: https://developers.openai.com/codex/plugins/build] |
| V3 Session Management | No | No sessions/daemon/web service in Phase 20. [VERIFIED: 20-CONTEXT.md] |
| V4 Access Control | Yes, limited | Scripts should honor configured file scope and locks in later phases; Phase 20 should document ownership boundaries. [VERIFIED: .planning/REQUIREMENTS.md] |
| V5 Input Validation | Yes | JSON contracts for tasks/Agents/reviews should define required fields and reject hidden state assumptions. [VERIFIED: .planning/REQUIREMENTS.md] |
| V6 Cryptography | No | No cryptography requirement found for Phase 20. [VERIFIED: .planning/REQUIREMENTS.md] |

### Known Threat Patterns for plugin-first local scripts

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Prompt/script boundary confusion | Tampering | Keep model policy in references/skills and deterministic actions in scripts; scripts should not interpret arbitrary model text as shell without structured task files. [VERIFIED: 20-CONTEXT.md] [ASSUMED] |
| Path traversal in task file scopes | Tampering | Phase 22 scripts should resolve paths under repo root and document file ownership/locks in Phase 20 contracts. [VERIFIED: .planning/REQUIREMENTS.md] [ASSUMED] |
| Accidental destructive init | Tampering | Non-destructive workspace initialization is a Phase 21 requirement and should be stated in Phase 20 templates. [VERIFIED: .planning/REQUIREMENTS.md] |
| Secret leakage to subprocess Agents | Information Disclosure | Agent contracts should define which env vars/context files can be passed; old `VIBE_PERSONA` pattern should not become unbounded env injection. [VERIFIED: apps/vibe-cli/src/main.rs] [ASSUMED] |
| Race conditions between executors | Tampering | Use project-local lock files and explicit task-owned paths. [VERIFIED: .planning/REQUIREMENTS.md] |

## Sources

### Primary (HIGH confidence)

- `20-CONTEXT.md` - locked decisions, scope, deferred ideas, canonical refs. [VERIFIED: .planning/phases/20-plugin-first-architecture/20-CONTEXT.md]
- `.planning/REQUIREMENTS.md` - PLUG requirements and future phase boundaries. [VERIFIED: .planning/REQUIREMENTS.md]
- `.planning/ROADMAP.md` - Phase 20 success criteria and Phase 21-24 split. [VERIFIED: .planning/ROADMAP.md]
- `apps/vibe-cli/src/main.rs` - old CLI command behavior and `spawn_role`. [VERIFIED: apps/vibe-cli/src/main.rs]
- `crates/vibe-core/src/state/mod.rs` - `.vibe` config, roles, state store, atomic save, initialization. [VERIFIED: crates/vibe-core/src/state/mod.rs]
- `crates/vibe-core/src/ipc/bus.rs` - atomic file bus send/receive pattern. [VERIFIED: crates/vibe-core/src/ipc/bus.rs]
- OpenAI Codex build plugins docs - manifest, marketplace, install mechanics. [CITED: https://developers.openai.com/codex/plugins/build]
- OpenAI Codex skills docs - skill format and progressive disclosure. [CITED: https://developers.openai.com/codex/skills]
- OpenAI Codex non-interactive docs - `codex exec` automation semantics. [CITED: https://developers.openai.com/codex/noninteractive]
- OpenAI Codex subagents docs - subagent/custom agent availability and constraints. [CITED: https://developers.openai.com/codex/subagents]
- OpenAI `openai/plugins` examples repo README - plugin example surfaces. [CITED: https://github.com/openai/plugins]
- Local Codex `plugin-creator` skill and `plugin-json-spec.md` - scaffold schema and marketplace examples. [VERIFIED: /Users/anjia/.codex/skills/.system/plugin-creator/SKILL.md] [VERIFIED: /Users/anjia/.codex/skills/.system/plugin-creator/references/plugin-json-spec.md]

### Secondary (MEDIUM confidence)

- `skills/vibe-operator/` - legacy Vibe skill/reference content to migrate into plugin references. [VERIFIED: skills/vibe-operator/SKILL.md] [VERIFIED: skills/vibe-operator/references]
- `.vibe/config.json` and `.vibe/roles/*.md` - current workspace state shape. [VERIFIED: .vibe/config.json] [VERIFIED: .vibe/roles]

### Tertiary (LOW confidence)

- Command file exact semantics for `plugin/vibe/commands/*.md` remain partially assumed because official docs found in this session confirm command existence/examples but not a full plugin command schema. [ASSUMED]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Codex plugin/skill docs, local plugin-creator spec, and repo constraints agree. [CITED: https://developers.openai.com/codex/plugins/build] [CITED: https://developers.openai.com/codex/skills] [VERIFIED: 20-CONTEXT.md]
- Architecture: HIGH - Phase decisions, roadmap, and old code inventory strongly support plugin-first plus thin scripts. [VERIFIED: 20-CONTEXT.md] [VERIFIED: .planning/ROADMAP.md] [VERIFIED: apps/vibe-cli/src/main.rs]
- Pitfalls: MEDIUM - Most are derived from locked decisions and local code, while command semantics warnings include assumptions. [VERIFIED: 20-CONTEXT.md] [ASSUMED]

**Research date:** 2026-04-22  
**Valid until:** 2026-05-22 for local architecture and old CLI inventory; 2026-04-29 for Codex plugin command semantics because Codex plugin docs are active and evolving. [ASSUMED]
