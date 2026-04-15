# REQUIREMENTS.md

## v1 Requirements (MVP)

### 窗格编排 (Orchestration)
- [ ] **ORCH-01**: Wezterm/Tmux 窗格的基本生命周期管理。支持水平/垂直切分、调整大小以及关闭指定窗格。
- [ ] **ORCH-02**: 智能聚焦 (Smart Focus)。Master 能够根据任务状态，自动聚焦或将特定窗格置于前台。

### 命令与意图注入 (Injection)
- [ ] **INJ-01**: 基本意图注入 (Basic Intent Injection)。支持 Master AI 通过 CLI 向新开启的 Worker 窗格发送初始任务 Spec。
- [ ] **INJ-02**: 环境变量与路径自动传播。启动新窗格时，自动继承当前 shell 的 `PATH` 和关键环境变量。

### 监控与反馈 (Monitoring)
- [ ] **MON-01**: 基础日志捕获 (Basic Log Capture)。支持从 Worker 窗格捕获退出状态码及最后 100 行原始输出日志。
- [ ] **MON-02**: 状态仪表盘 (Status Dashboard)。提供 `vibe status` 命令，以简单的 TUI 或表格形式显示所有 Worker 的运行状态。
- [ ] **MON-03**: 智能摘要与 ANSI 过滤。在回传给 Master AI 之前，自动过滤 ANSI 转义序列并进行信息摘要，以节省上下文 Token。

### 状态与上下文管理 (State)
- [ ] **STAT-01**: 持久化状态 (Persistent State)。使用 SQLite 数据库存储“任务 ID <-> 角色 <-> 窗格 ID”的映射关系，支持会话恢复。
- [ ] **STAT-02**: 实现基于 Unix Domain Sockets (UDS) 的实时状态同步机制。

### 安全与人工干预 (Safety)
- [ ] **SAFE-01**: 确认网关 (Confirmation Gate)。默认要求用户手动确认 (`y/N`) Agent 生成的命令执行。
- [ ] **SAFE-02**: 一键清理 (Kill Switch)。提供命令一键安全终止所有 Worker 进程并关闭对应窗格。

### AI Skill 集成 (AI Integration)
- [ ] **SKILL-01**: 构建 AI Skill/Tool 定义。支持以 MCP (Model Context Protocol) 或 JSON Tool 定义的形式，向 AI 模型暴露 `vibe-cli` 的核心调度能力。
- [ ] **SKILL-02**: 引导提示词 (System Prompting)。提供针对不同模型的引导词建议，确保模型知道何时以及如何正确调用 `vibe` 技能。

## v2 Requirements (Deferred)

- [ ] **AUTO-01**: 复杂的自主闭环。Master 根据 Worker 产出自动触发迭代修正，无需用户每步干预。
- [ ] **INJ-03**: 隐式上下文注入。自动将 Git Diff 和最新的错误日志作为元数据注入启动指令。
- [ ] **SYNC-01**: 跨 Shell 的深度状态同步（如 `zsh` alias 或未持久化的函数）。

## Out of Scope

- **GUI-01**: 开发专用的原生终端图形界面。
- **NET-01**: 跨主机的分布式 AI 调度。

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| ORCH-01     | Phase 1 | Done |
| ORCH-02     | Phase 5 | Pending |
| INJ-01      | Phase 4 | Done |
| INJ-02      | Phase 4 | Done |
| MON-01      | Phase 5 | Pending |
| MON-02      | Phase 6 | Pending |
| MON-03      | Phase 5 | Pending |
| STAT-01     | Phase 3 | Done |
| STAT-02     | Phase 3 | Done |
| SAFE-01     | Phase 4 | Done |

| SAFE-02     | Phase 1 | Done |
| SKILL-01    | Phase 7 | Pending |
| SKILL-02    | Phase 7 | Pending |

---
*Last updated: 2026-04-14 after Roadmap Revision (Monorepo Transition)*
