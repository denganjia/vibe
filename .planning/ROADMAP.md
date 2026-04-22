# Roadmap: vibe-cli

## Overview

Milestone 6.0 将 `vibe-cli` 从可运行的多 Agent 协作总线推进到可规划、可分配、可恢复、可发布的自动化任务流系统。本里程碑先解决高效且准确的任务拆分与 Worker 分配，再固化 `.vibe` 配置合同、文件系统状态机制、自动任务流和本地 GitHub release 总结。

## Past Milestones

- ✓ **v1.0 - v3.0**: 基础窗格管理、早期状态管理和 MCP 方向探索。
- ✓ **v4.0**: 转向 stateless 总线架构，完成 UUID 身份注入、`vibe spawn`、`vibe signal/wait` 和 WezTerm/Tmux 协作基础。
- ✓ **v5.0**: 完成双向通信闭环、CLI 初始化增强、全自动流水线验证和高可靠性交互修复。

## Milestone 6.0: 智能任务流与配置化状态系统

**Milestone Goal:** 让 Conductor 可以基于项目配置和文件状态，准确拆解、分配、推进和恢复任务，并在发布时从 git commit 历史生成可信的 release notes 草稿。

## Phases

- [ ] **Phase 20: 智能任务分配与拆解** - Conductor 可以把目标拆成结构化任务，并按能力、负载和冲突规则准确派发给 Worker。
- [ ] **Phase 21: `.vibe` 配置系统** - `.vibe/config.json` 成为角色、能力、任务模板、状态路径和发布设置的版本化配置合同。
- [ ] **Phase 22: 文件系统状态机制** - `.vibe/state` 可靠记录任务、Worker、锁、租约、心跳、结果、失败和恢复点。
- [ ] **Phase 23: 任务流自动化** - Conductor 可以自动推进任务从入队、分配、执行、等待、聚合、重试到恢复。
- [ ] **Phase 24: GitHub Release Commit 总结** - release 命令可以从本地 commit 区间生成结构化 changelog 和 GitHub release notes 草稿。

## Phase Details

### Phase 20: 智能任务分配与拆解
**Goal**: Conductor 可以高效且准确地把目标转化为可执行任务，并把任务分配给合适的 Worker。
**Depends on**: Phase 19
**Requirements**: ASSIGN-01, ASSIGN-02, ASSIGN-03, ASSIGN-04, ASSIGN-05
**Success Criteria** (what must be TRUE):
  1. Conductor 可以创建包含目标、文件范围、约束、预期产物和验证命令的结构化任务请求。
  2. 系统可以按任务类型、所需能力、影响文件、依赖风险和预估成本对任务分类。
  3. 系统可以根据角色能力、当前负载和冲突规则，把任务匹配给可用 Worker。
  4. Conductor 可以把多步骤目标拆成有顺序或可并行的任务单元，并为每个单元生成明确交接合同。
  5. 系统可以在派发前发现文件所有权重叠、依赖顺序错误和重复任务意图。
**Plans**: TBD

### Phase 21: `.vibe` 配置系统
**Goal**: `.vibe/config.json` 提供可校验、可迁移、可覆盖的项目级配置合同，并被运行时命令一致使用。
**Depends on**: Phase 20
**Requirements**: CONF-01, CONF-02, CONF-03, CONF-04, CONF-05
**Success Criteria** (what must be TRUE):
  1. 用户可以在版本化 schema 中声明角色、命令、stacks、能力、任务模板、状态路径和 release 设置。
  2. 用户运行 `vibe check` 时，可以看到缺失字段、无效角色引用和不支持 schema 版本的可操作错误。
  3. 用户运行 `vibe init` 时，可以创建或迁移 `.vibe/config.json`，且已有自定义配置不会被覆盖。
  4. 运行时命令通过同一配置加载路径获得确定性默认值和项目级覆盖。
  5. 角色模板、配置能力和 `spawn` 使用的角色名称保持一致，任务分配不会引用不存在的能力或角色。
**Plans**: TBD

### Phase 22: 文件系统状态机制
**Goal**: `.vibe/state` 成为可观察、可并发写入、可恢复的任务流状态源。
**Depends on**: Phase 21
**Requirements**: STATE-01, STATE-02, STATE-03, STATE-04, STATE-05
**Success Criteria** (what must be TRUE):
  1. 用户可以在 `.vibe/state` 中检查任务、Worker、分配、租约、锁、心跳、结果、失败和完成历史文件。
  2. 并发的 Worker 与 Conductor 活动不会让读者看到半写入 JSON。
  3. 文件所有权和任务锁可以阻止两个 Worker 写入同一受保护路径，除非任务显式允许共享所有权。
  4. 系统可以识别过期租约和死亡 Worker 心跳，并恢复任务而不删除有效结果。
  5. 用户可以通过 CLI 区分 queued、assigned、running、blocked、complete 和 failed 的当前任务状态。
**Plans**: TBD

### Phase 23: 任务流自动化
**Goal**: Conductor 可以基于分配引擎和文件状态自动推进完整任务生命周期。
**Depends on**: Phase 20, Phase 22
**Requirements**: FLOW-01, FLOW-02, FLOW-03, FLOW-04, FLOW-05
**Success Criteria** (what must be TRUE):
  1. Conductor 可以自动入队任务、派发给 Worker、等待完成信号并聚合结果，不需要手动终端交互。
  2. Worker 生命周期报告可以表达 accepted、writing、verifying、blocked、failed 和 completed，并携带结构化 payload。
  3. 失败任务可以按策略重试，且保留重试次数、最后错误和变化后的分配上下文。
  4. 并行任务只会在文件范围和依赖图允许安全并发时同时运行。
  5. 任务流中断后可以读取文件系统状态，并从最后一个持久检查点继续。
**Plans**: TBD

### Phase 24: GitHub Release Commit 总结
**Goal**: release 命令可以从本地 git commit 历史生成确定、可检查、可发布的 GitHub release notes 草稿。
**Depends on**: Phase 21
**Requirements**: REL-01, REL-02, REL-03, REL-04, REL-05
**Success Criteria** (what must be TRUE):
  1. 用户可以让 release 命令从最新 tag 或显式 `--from/--to` 参数确定 commit 区间。
  2. release 总结可以用确定性规则把 commits 分组为 features、fixes、docs、tests、refactors 和 internal changes。
  3. release 总结可以在 commit message 或规划产物提供信息时包含 changed files 和 phase references。
  4. 用户可以在无网络环境下生成 GitHub release notes 草稿文件。
  5. release workflow 会在 changelog 来源区间为空时失败，并在工作区有未提交改动时给出警告。
**Plans**: TBD

## Progress

**Execution Order:** Phase 20 → Phase 21 → Phase 22 → Phase 23 → Phase 24

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 20. 智能任务分配与拆解 | v6.0 | 0/TBD | Not started | - |
| 21. `.vibe` 配置系统 | v6.0 | 0/TBD | Not started | - |
| 22. 文件系统状态机制 | v6.0 | 0/TBD | Not started | - |
| 23. 任务流自动化 | v6.0 | 0/TBD | Not started | - |
| 24. GitHub Release Commit 总结 | v6.0 | 0/TBD | Not started | - |
