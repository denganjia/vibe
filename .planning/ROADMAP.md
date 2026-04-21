# Project Roadmap: Vibe-CLI

## Milestone 5.0: Interaction & Initialization (Current)

### Phase 17: Bi-directional Flow & Reliability (Completed)
- ✓ **Task 17.1**: 增强 `vibe inject` 的回车模拟，确保跨平台/跨 CLI 的指令接收可靠性。
- ✓ **Task 17.2**: 完善 `vibe wait` 的 Payload 解析逻辑，支持从信号中提取更复杂的结构化数据。
- ✓ **Task 17.3**: 验证 Worker 自动回复链路，确保主会话能基于回复自动推进下一步。

**Plans:** 3 plans
- [x] 17-01-PLAN.md — 增强 TTY 注入可靠性（限流与回车优化）
- [x] 17-02-PLAN.md — 实现基于文件的信号总线（.vibe/bus）
- [x] 17-03-PLAN.md — CLI 命令迁移、回退机制与 E2E 验证

### Phase 18: Standardization & Init
- [ ] **Task 18.1**: 实现 `vibe init` 命令，自动扫描环境并生成 `.vibe/config.json` 和角色模板。
- [ ] **Task 18.2**: 支持通过配置文件批量 `spawn` 智能体（`vibe spawn --stack <NAME>`）。
- [ ] **Task 18.3**: 优化 `.vibe/state` 的自动清理逻辑，防止残留的 Vibe ID 干扰新任务。

**Plans:** 3 plans
- [ ] 18-01-PLAN.md — 核心模型与终端适配器增强（Stacks & 物理状态列表）
- [ ] 18-02-PLAN.md — 实现 vibe init 与状态自动清理集成
- [ ] 18-03-PLAN.md — 实现批量派生部署（--stack）与全链路 E2E 验证

### Phase 19: Full Autonomous Workflow (E2E)
- [ ] **Task 19.1**: 更新 Vibe-Operator 技能，引入“自治循环”SOP（分析-执行-自检-信号）。
- [ ] **Task 19.2**: 进行端到端自动化压力测试：完成一个真实的模块重构任务，无需人工介入。
- [ ] **Task 19.3**: 发布 v5.0 正式版本。

**Plans:** 3 plans
- [ ] 19-01-PLAN.md — 角色模板与技能 SOP 更新
- [ ] 19-02-PLAN.md — 端到端全自动重构压力测试
- [ ] 19-03-PLAN.md — v5.0 版本发布与交付

---

## Past Milestones (Completed)
- ✓ **v1.0 - v3.0**: 基础窗格管理、MCP 集成、早期状态管理。
- ✓ **v4.0**: 无 stateless 总线架构重构、UUID 身份注入、WezTerm Tab 支持。