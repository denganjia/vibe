# Project Research Summary

**Project:** vibe-cli
**Domain:** Terminal AI Agent Orchestration
**Researched:** 2024-05-24
**Confidence:** HIGH

## Executive Summary

vibe-cli 是一个基于终端的 AI Agent 编排工具，采用 Master-Worker-TUI 架构。本次研究重点在于引入声明式 AI 技能（Skills）、多模型工作流（Multi-model Workflows）以及 AI 交叉检查（Cross-checking）机制。此类系统的核心在于如何标准化 Agent 的能力边界，并确保在复杂的终端环境下执行的安全性与可靠性。

推荐的研究方案是采用 **Model Context Protocol (MCP)** 和 **JSON Schema** 作为技能定义的标准，利用现有的 **SQLite** 状态层构建轻量级的 **有向无环图 (DAG) 工作流引擎**。为了降低幻觉率，必须在架构设计上隔离 Agent 上下文，并在交叉检查阶段强制使用不同型号的模型（异构模型校验），以避免“同质化错误放大”。

关键风险包括多 Agent 协作中的上下文污染、同质化模型的盲目互信以及声明式技能在动态终端环境下的不可控执行。我们将通过严格的 JSON Schema 校验、HITL（人工介入）审批节点以及确定性的 dry-run 机制来规避这些风险。

## Key Findings

### Recommended Stack

研究推荐在现有 Rust 架构基础上，引入标准化的协议和轻量级执行引擎，以保持 CLI 的简洁与高效。

**Core technologies:**
- **Model Context Protocol (MCP) v1.0**: 技能接口 — 作为 LLM 与工具交互的标准，确保技能定义的通用性。
- **JSON Schema (2020-12)**: 参数校验 — 提供强类型的输入验证，降低 LLM 调用参数错误的概率。
- **Custom DAG Engine (SQLite-backed)**: 工作流执行 — 基于现有 SQLite 存储实现状态机，避免引入 Temporal 等重型中间件。
- **Rust Serde/Valico**: 高性能校验 — 在指令执行前进行严苛的模式验证。

### Expected Features

**Must have (table stakes):**
- **结构化技能定义** — 使用 YAML/JSON Schema 定义 Agent 能力，支持元数据与依赖描述。
- **状态同步 (IPC Sync)** — 跨窗格、跨 Worker 的环境变量与上下文同步。
- **受控编排 (Human-in-the-loop)** — 在高风险操作（如文件删除、部署）前强制人工确认。

**Should have (competitive):**
- **交叉验证流 (Cross-checking)** — 实现“执行-检查”闭环，显著降低幻觉导致的误操作。
- **模型路由 (Routing)** — 根据任务复杂度自动分配推理模型或执行模型。

**Defer (v2+):**
- **动态 Handoffs** — Agent 间完全自主的控制权移交，初期建议采用半自动编排。

### Architecture Approach

新架构将基于现有的三角形拓扑进行扩展，引入 `SkillsRegistry` 解析器和 `WorkflowEngine` 状态机。

**Major components:**
1. **SkillsRegistry**: 负责从本地目录加载 YAML 定义，并动态暴露给 MCP Server。
2. **WorkflowEngine**: 驱动 DAG 任务流，管理 Pending 到 Completed 的状态迁移。
3. **StateStore (SQLite)**: 持久化工作流定义与执行记录，确保 CLI 重启后的状态恢复。

### Critical Pitfalls

1. **共享内存污染** — 避免所有 Agent 共享同一个巨量上下文。应采用 Manager-Worker 模式，仅传递经过解析的结构化数据。
2. **同质化交叉检查** — 避免用同族模型校验自己。必须强制异构模型（如 Claude 校验 GPT）结合确定性逻辑（如 Exit Code）。
3. **黑盒盲目执行** — 严禁 Agent 直接执行高危操作。必须实现 Plan-Confirm-Execute 流程，并在 TUI 中可视化执行计划。

## Implications for Roadmap

基于研究结果，建议按以下阶段组织开发：

### Phase 1: Skills Framework (Foundation)
**Rationale:** 技能定义是所有后续自动化流程的基础。
**Delivers:** 声明式技能加载机制、YAML 解析器、动态 MCP Tool 暴露。
**Addresses:** 结构化技能定义。
**Avoids:** 技能描述模糊导致的“幻觉调用”。

### Phase 2: Workflow Engine & State Persistence
**Rationale:** 需要在 SQLite 中建立任务追踪模型，才能支持多步协作。
**Delivers:** SQLite Schema 升级、轻量级 DAG 执行逻辑、任务状态追踪。
**Uses:** Custom DAG Engine (SQLite-backed).
**Implements:** WorkflowEngine.

### Phase 3: Multi-model Orchestration & Verification
**Rationale:** 在有了执行引擎后，引入交叉检查节点以确保安全。
**Delivers:** 异构模型路由逻辑、Cross-checking 校验节点、HITL 确认机制。
**Avoids:** 同质化交叉检查、共享内存污染。

### Phase 4: TUI Observability & Advanced Routing
**Rationale:** 增强用户感知，让工作流进度可视化。
**Delivers:** TUI 工作流仪表盘、执行计划预览视图、动态模型选择。

### Phase Ordering Rationale

- **先定义，后执行**：先完成 Skills 定义标准，确保所有工具都有强类型约束。
- **状态先行**：在实现复杂逻辑前，先在 SQLite 层夯实状态持久化能力，避免内存状态丢失。
- **安全后置但核心**：将交叉检查放在工作流引擎之后，是因为校验本身就是一个特定的工作流节点。

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 3:** 涉及多模型 API 的并发调用与异常处理，需研究最优的提示词隔离策略。

Phases with standard patterns (skip research-phase):
- **Phase 1:** JSON Schema 与 YAML 解析在 Rust 生态中有成熟方案。
- **Phase 2:** 简单的状态机迁移属于标准工程实践。

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | MCP 与 JSON Schema 是行业标准，Rust 生态支持完善。 |
| Features | HIGH | 紧贴 `vibe-cli` 的本地开发场景，需求明确。 |
| Architecture | HIGH | 基于现有 Master-Worker 架构平滑演进。 |
| Pitfalls | MEDIUM | 跨 Agent 通信的复杂性仍有挑战，需在开发中动态调整。 |

**Overall confidence:** HIGH

### Gaps to Address

- **异构模型成本估算**: 多模型校验会增加 Token 开销，需在规划中加入成本/性能平衡策略。
- **并发冲突处理**: 多个 Worker 同时修改同一文件时的锁定机制需在 Phase 2 详细设计。

## Sources

### Primary (HIGH confidence)
- **MCP Specification (Anthropic)** — 技能定义与工具调用标准。
- **SQLite Workflow Patterns** — 借鉴了轻量级任务队列的实现。

### Secondary (MEDIUM confidence)
- **Google DeepMind/Galileo.ai** — 多 Agent 协作中的幻觉与错误放大研究。

---
*Research completed: 2024-05-24*
*Ready for roadmap: yes*
