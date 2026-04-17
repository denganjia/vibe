# Phase 11: Multi-model SOP & Verification - Context

**Gathered:** 2026-04-16
**Status:** Ready for planning

<domain>
## Phase Boundary

本阶段的目标是定义多模型协作的标准操作规程 (SOP) 以及利用 `vibe-cli` 状态进行交叉校验的具体路径。这不仅包括模型间的任务分配逻辑，还涵盖了如何通过 `vibe report` 传递上下文、如何触发逻辑审计以及如何从死锁中恢复。

</domain>

<decisions>
## Implementation Decisions

### 多模型协作 (Collaboration)
- **D-01: 基于推理能力分工**：采用“强 Master (Conductor) + 轻量 Worker”的模式。Conductor 负责复杂的规划和冲突解决，Worker 负责具体的、确定性的执行任务。
- **D-02: 基于摘要的上下文传递 (Summary-based)**：Worker 执行任务后，通过 `vibe report` 发送关键摘要。Conductor 通过 `vibe_list` 获取这些摘要以更新全局状态。只有当摘要信息不足或出现异常时，Conductor 才会考虑读取完整的日志文件。

### 校验逻辑 (Verification)
- **D-03: 任务后校验 (Post-task Only)**：校验（Audit）主要在子任务完成时触发，用于确认阶段性目标的达成。
- **D-04: 逻辑审计与意图对齐 (Logic Audit / Intent Alignment)**：审计过程侧重于检查代码实现是否完全对齐了规划阶段的“原始意图”（Intent Alignment），而不仅仅是事实性检查。
- **D-05: 独立文档组织**：创建一个独立的 `skills/vibe-operator/sops/verification.md` 文档，专门描述所有的校验逻辑和审计流。

### 冲突与恢复 (Conflict & Recovery)
- **D-06: 基于模式的死锁判定 (Pattern-based)**：当 Worker 连续 M 次报告相同的错误，或者输出进入了明显的循环模式时，Conductor 判定为死锁或异常。
- **D-07: 外科手术式注入 (Surgical Inject)**：发现异常时，Conductor 优先使用 `vibe_inject` 注入诊断指令（如调试命令或现状检查）尝试恢复，而非直接关闭窗格。

### Claude's Discretion
- 具体死锁判定的重复次数 (M) 和超时间隔。
- `vibe report` 摘要的推荐格式规范。
- 逻辑审计时的提示词 (Prompt) 结构设计。

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Vibe-CLI 协议与状态定义
- `crates/vibe-core/src/ipc/protocol.rs` — 定义了 `Message::Report` 和 `WorkerState` 结构，这是摘要传递的基础。
- `crates/vibe-core/src/state/mod.rs` — 定义了 `update_report` 方法，这是状态持久化的核心。

### 现有技能定义
- `skills/vibe-operator/SKILL.md` — 提供了工具参考和参数定义。
- `skills/vibe-operator/role.md` — 定义了 Conductor 与 Worker 的初始分工。

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ReportInfo` struct in `protocol.rs`: 已经包含了 `vibe_id`, `status`, `summary` 字段，可直接用于 D-02。
- `StateStore::list_active_panes`: 提供了一次性获取所有 Worker 摘要的能力。

### Integration Points
- 校验逻辑应当作为 `Vibe-Conductor` 角色的核心 SOP 指导。
- 所有的死锁判定逻辑应当在 Conductor 轮询 `vibe_list` 的循环中实现。

</code_context>

<deferred>
## Deferred Ideas

- 基于领域专家（前端/后端）的模型自动路由。
- 实时状态对比流（执行前后的 ls/grep 差异分析）。
- 复杂的自动回滚逻辑。

</deferred>

---

*Phase: 11-multi-model-sop-verification*
*Context gathered: 2026-04-16*
