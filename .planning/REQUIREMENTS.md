# Milestone 6.0 Requirements: Plugin-first Multi-model Collaboration

**Defined:** 2026-04-22
**Core Value:** 打破 AI 与本地开发环境之间的“次元壁”，将终端从单纯的字符输入框升级为分布式 AI 协作的物理调度室。

## v6.0 Requirements

### Plugin Architecture

- [ ] **PLUG-01**: Project defines a plugin package layout that can inject skills, commands, references, and executable scripts into supported AI terminal environments.
- [ ] **PLUG-02**: Plugin references define the collaboration protocol, task contract, Agent contract, review protocol, and `.vibe` workspace layout in model-readable documents.
- [ ] **PLUG-03**: Plugin skills teach the current model to act as Conductor: clarify the user request, produce a plan, split tasks, launch executors, request reviews, and aggregate results.
- [ ] **PLUG-04**: Plugin commands expose the main workflow actions, including init, plan, run task, review task, status, and release summary.
- [ ] **PLUG-05**: The old standalone CLI responsibilities are classified into migrate-to-script, keep-as-compatibility, or remove categories.

### `.vibe` Workspace

- [x] **VIBE-01**: Plugin initialization creates a project-local `.vibe` workspace with `Agents/`, `tasks/`, `runs/`, `locks/`, `reviews/`, `logs/`, and config files.
- [x] **VIBE-02**: `.vibe/Agents` can define planner, executor, reviewer, and release roles, including model command, prompt/reference files, allowed tools, and expected outputs.
- [x] **VIBE-03**: `.vibe/config.json` records default models, Agent definitions, concurrency limits, task paths, lock policy, review policy, and release summary settings.
- [x] **VIBE-04**: Initialization is non-destructive: existing `.vibe` files and user-edited Agent definitions are never overwritten unless explicitly forced.
- [x] **VIBE-05**: The workspace format is simple enough that the current model can inspect and reason about it without relying on hidden daemon state.

### Scripts Runtime

- [ ] **RUN-01**: Plugin scripts can create task JSON files with goal, context, file scope, constraints, expected output, verification command, and reviewer requirements.
- [ ] **RUN-02**: Plugin scripts can acquire and release file locks for task-owned paths using project-local lock files.
- [ ] **RUN-03**: Plugin scripts can launch claude, gemini, codex, or another configured Agent command as a subprocess with task file and Agent definition injected.
- [ ] **RUN-04**: Plugin scripts capture stdout, stderr, exit code, timestamps, and result artifacts into `.vibe/runs` and `.vibe/logs`.
- [ ] **RUN-05**: Plugin scripts are implemented as small JS or Python modules with no standalone server, database, or long-running daemon requirement.

### Multi-model Workflow

- [ ] **FLOW-01**: Current model can conduct multi-round clarification with the user before task execution and persist the resulting plan into `.vibe/tasks`.
- [ ] **FLOW-02**: Current model can split a plan into independent or ordered tasks, respecting file scopes and dependency order.
- [ ] **FLOW-03**: Current model can choose executor Agents based on role definition, model command, task type, and file ownership constraints.
- [ ] **FLOW-04**: Reviewer Agents can inspect executor outputs, produce structured findings, and request fixes before a task is marked complete.
- [ ] **FLOW-05**: The workflow can resume from `.vibe` files after interruption, showing queued, running, blocked, review-needed, failed, and completed tasks.

### Release and Migration

- [ ] **REL-01**: Plugin release command can determine a commit range from the latest tag or explicit `--from/--to` parameters.
- [ ] **REL-02**: Release summary groups commits into features, fixes, docs, tests, refactors, and internal changes using deterministic rules.
- [ ] **REL-03**: Release summary can include changed files, phase references, and task/review artifacts when available.
- [ ] **REL-04**: Release summary writes a local GitHub release notes draft without requiring network access.
- [ ] **REL-05**: Rust CLI code is slimmed or archived according to the migration classification, with any retained functionality callable from plugin scripts only when justified.

## Future Requirements

### Plugin Distribution

- **PLUG-F01**: Plugin can be published to multiple plugin marketplaces or registries with compatibility metadata.
- **PLUG-F02**: Plugin can generate provider-specific command wrappers for Claude, Gemini, Codex, and future AI terminals.

### Runtime Intelligence

- **RUN-F01**: Runtime tracks Agent success history and uses it for assignment scoring.
- **RUN-F02**: Runtime supports remote or containerized executor Agents.

### Release Automation

- **REL-F01**: Release command can publish directly to GitHub when `gh` is authenticated.
- **REL-F02**: Release summaries can include issue and PR metadata from GitHub APIs.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Standalone heavy CLI as the primary interface | v6.0 product direction is plugin-first; scripts are runtime helpers, not the main UX. |
| Central daemon or database | The system remains project-local, inspectable, and file-based. |
| Terminal pane orchestration as the only execution path | Subprocess Agent launch from plugin scripts is the default; terminal adapters are compatibility only. |
| Network-required GitHub publishing | v6.0 focuses on deterministic local release notes drafts. |
| Marketplace publishing | v6.0 builds the plugin package locally first; distribution hardening comes later. |

## Traceability

Milestone 6.0 requirements are mapped to Phase 20-24. Each v6.0 requirement belongs to exactly one phase.

| Requirement | Phase | Status |
|-------------|-------|--------|
| PLUG-01 | Phase 20 | Pending |
| PLUG-02 | Phase 20 | Pending |
| PLUG-03 | Phase 20 | Pending |
| PLUG-04 | Phase 20 | Pending |
| PLUG-05 | Phase 20 | Pending |
| VIBE-01 | Phase 21 | Complete |
| VIBE-02 | Phase 21 | Complete |
| VIBE-03 | Phase 21 | Complete |
| VIBE-04 | Phase 21 | Complete |
| VIBE-05 | Phase 21 | Complete |
| RUN-01 | Phase 22 | Pending |
| RUN-02 | Phase 22 | Pending |
| RUN-03 | Phase 22 | Pending |
| RUN-04 | Phase 22 | Pending |
| RUN-05 | Phase 22 | Pending |
| FLOW-01 | Phase 23 | Pending |
| FLOW-02 | Phase 23 | Pending |
| FLOW-03 | Phase 23 | Pending |
| FLOW-04 | Phase 23 | Pending |
| FLOW-05 | Phase 23 | Pending |
| REL-01 | Phase 24 | Pending |
| REL-02 | Phase 24 | Pending |
| REL-03 | Phase 24 | Pending |
| REL-04 | Phase 24 | Pending |
| REL-05 | Phase 24 | Pending |

**Coverage:**
- v6.0 requirements: 25 total
- Mapped to phases: 25
- Unmapped: 0

---
*Requirements defined: 2026-04-22*
*Last updated: 2026-04-22 after plugin-first pivot*
