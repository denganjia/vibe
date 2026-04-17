# Requirements: vibe-cli (Milestone 4.0)

**Defined:** 2026-04-17
**Core Value:** Strategic Pivot to "Agent Collaboration Bus" - High autonomy, local context (.vibe), and simple signaling.

## v1 Requirements (Milestone 4.0: AI Agent Bus)

### Architecture Cleanup (Cleanup)
- [ ] **BUS-01**: 移除现有的 MCP 服务代码 (`mcp.rs`) 及相关依赖。
- [ ] **BUS-02**: 移除沉重的 SQLite 强一致性业务逻辑，转为极简的状态追踪。
- [ ] **BUS-03**: 简化 IPC 协议，移除命令拦截与人工审批字段。

### 信号总线 (Messaging Bus)
- [ ] **BUS-04**: 实现 `vibe signal <MESSAGE>`。允许子窗格向总线发送异步信号。
- [ ] **BUS-05**: 实现 `vibe wait [SIGNAL_TYPE]`。主会话进入阻塞监听，直至收到目标信号。
- [ ] **BUS-06**: 建立基于 UDS 的极简消息路由机制，支持跨窗格寻址。

### 自治编排 (Autonomous Spawner)
- [ ] **BUS-07**: 实现 `vibe spawn --role <ROLE>`。自动 split 窗格并启动预定义的 AI CLI 会话。
- [ ] **BUS-08**: 角色 Prompt 注入。从 `.vibe/roles/` 读取配置并通过管道注入新启动的进程。

### 项目本地上下文 (Local Context)
- [ ] **BUS-09**: 建立 `.vibe/` 目录规范。包含 `roles/`, `state/`, `config.yaml`。
- [ ] **BUS-10**: 任务追踪自治。主 Agent 负责维护 `.vibe/project.md` 进度，而非由 CLI 强制同步。

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| BUS-01 | Phase 13 | Pending |
| BUS-02 | Phase 13 | Pending |
| BUS-03 | Phase 13 | Pending |
| BUS-04 | Phase 14 | Pending |
| BUS-05 | Phase 14 | Pending |
| BUS-06 | Phase 14 | Pending |
| BUS-07 | Phase 15 | Pending |
| BUS-08 | Phase 15 | Pending |
| BUS-09 | Phase 15 | Pending |
| BUS-10 | Phase 16 | Pending |

**Coverage:**
- Milestone 4.0 requirements: 10 total
- Mapped to phases: 10
- Unmapped: 0 ✓

---
*Last updated: 2026-04-17 after Strategic Pivot*
