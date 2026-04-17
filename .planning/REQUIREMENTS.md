# Requirements: vibe-cli

**Defined:** 2026-04-16
**Core Value:** Break the "dimensional wall" between AI and the local dev environment by turning the terminal into a physical orchestration room.

## v1 Requirements (Milestone v3.0)

### Skills Framework

- [ ] **SKL-01**: 定义结构化技能包规范 (SKILL.md/YAML)，包含元数据与指令流。
- [ ] **SKL-02**: 实现技能自动加载与 MCP 动态工具暴露。
- [ ] **SKL-03**: 支持技能依赖描述与版本管理。

### Workflow Engine & State

- [ ] **WFK-01**: 实现基于 SQLite 的多步任务状态持久化。
- [ ] **WFK-02**: 支持任务节点流 (DAG) 定义与执行。
- [ ] **WFK-03**: 实现任务失败重试机制。
- [ ] **WFK-04**: 支持任务状态断点续传（恢复执行）。

### Multi-model Orchestration

- [ ] **MOD-01**: 实现基于任务类型的不同模型路由 (Routing)。
- [ ] **MOD-02**: 实现 Evaluator-Optimizer 交叉检查机制。
- [ ] **MOD-03**: 支持 Agent 间的动态 Handoffs（控制权移交）。
- [ ] **MOD-04**: 实现异构模型结果自动对比分析。

### Human-in-the-Loop & Safety

- [ ] **SAF-01**: 在高风险节点（如系统修改）强制人工二次确认。
- [ ] **SAF-02**: 提供执行计划的详细预览 UI。
- [ ] **SAF-03**: 实现技能执行的 Dry-run 模式。
- [ ] **SAF-04**: 实现任务失败后的自动回滚策略（尽可能）。

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SKL-01 | Phase 10 | Pending |
| SKL-02 | Phase 10 | Pending |
| SKL-03 | Phase 10 | Pending |
| WFK-01 | Phase 11 | Pending |
| WFK-02 | Phase 11 | Pending |
| WFK-03 | Phase 11 | Pending |
| WFK-04 | Phase 11 | Pending |
| MOD-01 | Phase 12 | Pending |
| MOD-02 | Phase 12 | Pending |
| MOD-03 | Phase 12 | Pending |
| MOD-04 | Phase 12 | Pending |
| SAF-01 | Phase 13 | Pending |
| SAF-02 | Phase 13 | Pending |
| SAF-03 | Phase 13 | Pending |
| SAF-04 | Phase 13 | Pending |

**Coverage:**
- v1 requirements: 15 total
- Mapped to phases: 15
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-16*
*Last updated: 2026-04-16 after initial definition*
