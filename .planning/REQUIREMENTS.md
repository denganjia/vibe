# Requirements: vibe-cli (Milestone 4.0 - AI Agent Bus)

**Defined:** 2026-04-17
**Core Value:** Strategic Pivot to "Agent Collaboration Bus" - High autonomy, local context (.vibe), and simple signaling.

## v1 Requirements (Milestone 4.0: AI Agent Bus)

### 1. 架构净化 (Architecture Cleanup)
- [ ] **BUS-01**: 移除现有的 MCP 代码 (`mcp.rs`) 与复杂的人工审批逻辑。
- [ ] **BUS-02**: 移除沉重的 SQLite 强业务逻辑，仅保留内存中的轻量级 Session 表。
- [ ] **BUS-03**: 简化 IPC 协议，支持 `Signal` 和 `Wait` (基于连接挂起模式) 消息类型。

### 2. 信号总线 (The Bus)
- [ ] **BUS-04**: 实现 `vibe signal <MSG>`。通过总线通知所有关注该消息的订阅者。
- [ ] **BUS-05**: 实现 `vibe wait [SIGNAL]`。进入阻塞状态，监听 UDS 消息直至信号到达。
- [ ] **BUS-06**: 实现多项目隔离的消息路由，基于项目哈希生成的 UDS 路径 (`/tmp/vibe-<hash>.sock`)。

### 3. 自治代理启动 (Autonomous Spawner)
- [x] **BUS-07**: 实现 `vibe spawn --role <ROLE>`。
    - 自动通过终端适配器 (Wezterm/Tmux) 拆分窗格。
    - 读取 `.vibe/roles/<ROLE>.md` 作为 Persona。
- [x] **BUS-08**: 角色注入协议。在子进程启动时通过 `stdin` 管道注入 Persona 提示词。
- [x] **BUS-09**: 实现交互交接。注入完成后，自动将主 `stdin` 桥接到子进程。

### 4. 项目本地上下文 (Local Context)
- [x] **BUS-10**: 建立 `.vibe/` 规范目录结构。
- [ ] **BUS-11**: 实现跨 Agent 上下文共享。子 Agent 任务完成后自动更新 `.vibe/state/`。

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| BUS-01 | Phase 13 | Pending |
| BUS-02 | Phase 13 | Pending |
| BUS-03 | Phase 13 | Pending |
| BUS-04 | Phase 14 | Pending |
| BUS-05 | Phase 14 | Pending |
| BUS-06 | Phase 14 | Pending |
| BUS-07 | Phase 15 | Complete |
| BUS-08 | Phase 15 | Complete |
| BUS-09 | Phase 15 | Complete |
| BUS-10 | Phase 15 | Complete |
| BUS-11 | Phase 16 | Pending |

**Coverage:**
- Milestone 4.0 requirements: 11 total
- Mapped to phases: 11
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-17*
*Last updated: 2026-04-17 after Implementation Research*
