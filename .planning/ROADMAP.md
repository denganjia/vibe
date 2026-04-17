# ROADMAP (Milestone 4.0 - AI Agent Bus)

## Strategic Pivot 4.0
从“受控编排”转向“代理协同总线”。移除沉重中间层，建立基于信号的极简自治流。

## Phases

- [x] **Phase 1-9: Foundation & Controlled Workflow** - 已完成的基础设施建设。
- [x] **Phase 10-12: Vibe-Operator Skills** - 已定义的 AI 操作规程与模板。
- [ ] **Phase 13: 架构清理与瘦身 (Cleanup)** - 移除 MCP、审批拦截和沉重的 DB 逻辑。
- [ ] **Phase 14: 信号总线实现 (Bus Core)** - 实现 `vibe signal` 与 `vibe wait` 异步通信机制。
- [ ] **Phase 15: 自治代理启动器 (Autonomous Spawner)** - 实现 `vibe spawn` 及从 `.vibe/roles/` 注入角色。
- [ ] **Phase 16: 全流程集成测试 (E2E Integration)** - 验证多代理自治闭环流。

## Phase Details

### Phase 13: 架构清理与瘦身 (Cleanup)
**Goal**: 净化代码库，移除不再需要的 MCP 和复杂审批逻辑。
**Requirements**: BUS-01, BUS-02, BUS-03
**Success Criteria**:
1. `mcp.rs` 被移除，二进制文件体积缩小。
2. IPC 协议中不再包含命令拦截和 approval 字段。
3. `vibe-core` 状态管理被极简化。

### Phase 14: 信号总线实现 (Bus Core)
**Goal**: 实现代理间的通信基础设施。
**Requirements**: BUS-04, BUS-05, BUS-06
**Success Criteria**:
1. 子 Agent 运行 `vibe signal` 成功发送消息。
2. 主 Agent 运行 `vibe wait` 能正确阻塞并接收信号。
3. 消息路由在跨窗格环境下稳定。

### Phase 15: 自治代理启动器 (Autonomous Spawner)
**Goal**: 提供一键启动并指派角色的能力。
**Requirements**: BUS-07, BUS-08, BUS-09
**Success Criteria**:
1. `vibe spawn --role Scanner` 自动开窗。
2. 子窗格中自动启动 AI CLI 并注入 `.vibe/roles/Scanner.md`。
3. `.vibe/` 目录规范被正确初始化。

### Phase 16: 全流程集成测试 (E2E Integration)
**Goal**: 验证完整的自治开发工作流。
**Requirements**: BUS-10
**Success Criteria**:
1. 主 Agent 启动 Scanner。
2. Scanner 完成扫描后发送信号。
3. 主 Agent 接收信号并继续下一步，全程无需人类干预。

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1-9. Foundation | 100% | Completed | 2026-04-15 |
| 10. Vibe-Operator Skill | 4/4 | Completed | 2026-04-17 |
| 11. Multi-model SOP | 3/3 | Completed | 2026-04-17 |
| 12. Workflow Templates | 3/3 | Completed | 2026-04-17 |
| 13. Cleanup | 0/TBD | Not started | - |
| 14. Signal Bus | 0/TBD | Not started | - |
| 15. Spawner | 0/TBD | Not started | - |
| 16. E2E Integration | 0/TBD | Not started | - |
