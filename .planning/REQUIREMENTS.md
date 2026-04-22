# Milestone 6.0 Requirements: Task Flow Automation

**Defined:** 2026-04-22
**Core Value:** 打破 AI 与本地开发环境之间的“次元壁”，将终端从单纯的字符输入框升级为分布式 AI 协作的物理调度室。

## v6.0 Requirements

### Task Assignment

- [ ] **ASSIGN-01**: Conductor can create a structured task request with goal, file scope, constraints, expected output, and verification command.
- [ ] **ASSIGN-02**: System can classify tasks by type, required skills, affected files, dependency risk, and expected execution cost.
- [ ] **ASSIGN-03**: System can match a task to an available Worker role using declared capabilities, current load, and conflict rules.
- [ ] **ASSIGN-04**: System can split a multi-step objective into ordered or parallelizable task units with explicit handoff contracts.
- [ ] **ASSIGN-05**: System can detect assignment conflicts, including overlapping file ownership, dependency ordering, and duplicate task intent.

### `.vibe` Configuration

- [ ] **CONF-01**: `.vibe/config.json` has a versioned schema covering roles, commands, stacks, capabilities, task templates, state paths, and release settings.
- [ ] **CONF-02**: `vibe check` validates `.vibe/config.json` and reports actionable errors for missing fields, invalid role references, and unsupported schema versions.
- [ ] **CONF-03**: `vibe init` can create or migrate `.vibe/config.json` without overwriting user customizations.
- [ ] **CONF-04**: Runtime commands load configuration through one shared config path with deterministic defaults and project-level overrides.
- [ ] **CONF-05**: Role templates and configured capabilities stay consistent, so task assignment uses the same role names and capabilities that spawn uses.

### Filesystem State

- [ ] **STATE-01**: `.vibe/state` records tasks, workers, assignments, leases, locks, heartbeats, results, failures, and completed task history as inspectable files.
- [ ] **STATE-02**: State writes are atomic enough to avoid partial JSON reads during concurrent Worker and Conductor activity.
- [ ] **STATE-03**: File ownership and task locks prevent two Workers from writing the same owned path unless the task explicitly allows shared ownership.
- [ ] **STATE-04**: Stale leases and dead Worker heartbeats can be detected and recovered without deleting valid task results.
- [ ] **STATE-05**: Users can inspect current task flow state through CLI output that distinguishes queued, assigned, running, blocked, complete, and failed work.

### Task Flow Automation

- [ ] **FLOW-01**: Conductor can enqueue tasks, dispatch them to Workers, wait for completion signals, and aggregate results without manual terminal interaction.
- [ ] **FLOW-02**: Worker lifecycle reports include accepted, writing, verifying, blocked, failed, and completed states with structured payloads.
- [ ] **FLOW-03**: Failed tasks can be retried according to policy with retry count, last error, and changed assignment context preserved.
- [ ] **FLOW-04**: Parallel tasks only run when their file scopes and dependency graph allow safe concurrent execution.
- [ ] **FLOW-05**: A task flow can resume after interruption by reading filesystem state and continuing from the last durable checkpoint.

### GitHub Release Summaries

- [ ] **REL-01**: Release command can determine a commit range from the latest tag or an explicit `--from/--to` range.
- [ ] **REL-02**: Release summary groups commits into features, fixes, docs, tests, refactors, and internal changes using deterministic rules.
- [ ] **REL-03**: Release summary includes changed files and phase references when commit messages or planning artifacts provide them.
- [ ] **REL-04**: Release command can write a GitHub release notes draft without requiring network access.
- [ ] **REL-05**: Release workflow validates that the changelog source range is non-empty and warns about uncommitted changes before drafting.

## Future Requirements

### Assignment Intelligence

- **ASSIGN-F01**: System learns Worker success rates over time and uses historical quality as assignment input.
- **ASSIGN-F02**: System supports pluggable assignment strategies for different project types.

### Release Automation

- **REL-F01**: Release command can publish directly to GitHub when `gh` is authenticated.
- **REL-F02**: Release summaries can include issue and PR metadata from GitHub APIs.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Network-required GitHub publishing | v6.0 focuses on deterministic local release summaries; publishing can build on the draft output later. |
| Central daemon or database | The product direction remains project-local filesystem state, not a long-running service. |
| ML-based assignment scoring | Accurate deterministic assignment comes first; learning from history is deferred. |
| Cross-repository task orchestration | v6.0 scopes state and assignment to one project root. |

## Traceability

Milestone 6.0 需求已映射到 Phase 20-24。每条 v6.0 requirement 只归属一个 phase。

| Requirement | Phase | Status |
|-------------|-------|--------|
| ASSIGN-01 | Phase 20 | Pending |
| ASSIGN-02 | Phase 20 | Pending |
| ASSIGN-03 | Phase 20 | Pending |
| ASSIGN-04 | Phase 20 | Pending |
| ASSIGN-05 | Phase 20 | Pending |
| CONF-01 | Phase 21 | Pending |
| CONF-02 | Phase 21 | Pending |
| CONF-03 | Phase 21 | Pending |
| CONF-04 | Phase 21 | Pending |
| CONF-05 | Phase 21 | Pending |
| STATE-01 | Phase 22 | Pending |
| STATE-02 | Phase 22 | Pending |
| STATE-03 | Phase 22 | Pending |
| STATE-04 | Phase 22 | Pending |
| STATE-05 | Phase 22 | Pending |
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
*Last updated: 2026-04-22 after roadmap creation*
