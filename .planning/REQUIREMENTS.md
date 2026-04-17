# Requirements: vibe-cli

**Defined:** 2026-04-16
**Core Value:** Break the "dimensional wall" between AI and the local dev environment by turning the terminal into a physical orchestration room.

## v1 Requirements (Milestone v3.0: Vibe-CLI Skill Definition)

The focus of this milestone is creating the "vibe-cli skill" (AI-consumable definition) so that AI models understand how to use vibe-cli and coordinate multi-model workflows. It is NOT a general framework for users to define new skills.

- [ ] **SKL-01**: 编写 vibe-cli 核心技能定义 (SKILL.md)，涵盖命令集、窗格管理与 IPC 状态流。
- [ ] **SKL-02**: 在技能中定义多模型协作模式 (如 Master/Worker/Evaluator) 的标准操作规程 (SOP)。
- [ ] **SKL-03**: 定义交叉检查 (Cross-checking) 的具体实现路径（如何利用 vibe 状态进行校验）。
- [ ] **SKL-04**: 提供不同场景下的 Workflow 模版（如代码重构流、自动化测试流）。
- [ ] **SKL-05**: 验证并优化 Skill 定义，确保模型生成的指令符合 vibe-cli 规范。

## Deferred to v4.0+ (General Framework)

These requirements were originally planned for v3.0 but have been deferred to focus on the core vibe-cli skill definition.

- **WFK-01**: 实现基于 SQLite 的多步任务状态持久化。
- **WFK-02**: 支持任务节点流 (DAG) 定义与执行。
- **WFK-03**: 实现任务失败重试机制。
- **WFK-04**: 支持任务状态断点续传（恢复执行）。
- **MOD-01**: 实现基于任务类型的不同模型路由 (Routing)。
- **MOD-02**: 实现 Evaluator-Optimizer 交叉检查机制。
- **MOD-03**: 支持 Agent 间的动态 Handoffs（控制权移交）。
- **MOD-04**: 实现异构模型结果自动对比分析。
- **SAF-01**: 在高风险节点（如系统修改）强制人工二次确认。
- **SAF-02**: 提供执行计划的详细预览 UI。
- **SAF-03**: 实现技能执行的 Dry-run 模式。
- **SAF-04**: 实现任务失败后的自动回滚策略（尽可能）。

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SKL-01 | Phase 10 | Pending |
| SKL-02 | Phase 11 | Pending |
| SKL-03 | Phase 11 | Pending |
| SKL-04 | Phase 12 | Pending |
| SKL-05 | Phase 12 | Pending |

**Coverage:**
- v1 requirements: 5 total
- Mapped to phases: 5
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-16*
*Last updated: 2026-04-16 after Milestone v3.0 Scope Pivot*
