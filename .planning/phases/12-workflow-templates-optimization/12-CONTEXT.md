# Phase 12: Workflow Templates & Optimization - Context

**Gathered:** 2026-04-16
**Status:** Ready for planning

<domain>
## Phase Boundary

本阶段的目标是为 `Vibe-Operator` 技能包交付高可靠性的工作流模板。重点在于实现专门的 **代码重构 (Code Refactoring)** 模板，引入 **Prompt 变量注入协议**，并通过显式标注提升 AI 的上下文感知能力。同时，优化 `SKILL.md` 的 Token 消耗并建立可靠性验证基准。

</domain>

<decisions>
## Implementation Decisions

### 场景化模板 (Scenario Templates)
- **D-01: 聚焦代码重构 (Refactoring Focus)**：目前仅引入专门的 `refactoring/` 工作流模板。
- **D-02: 标准重构流 (Standard SOP)**：重构路径定义为：分析 (Analyze) -> 实现重构 (Refactor) -> 测试验证 (Test) -> 最终审计 (Review)。不强制要求自动化快照/备份节点，保持流程轻量级。

### 变量注入与上下文 (Variables & Context)
- **D-03: Prompt 变量注入 (Prompt Injection)**：采用 `$[VARIABLE_NAME]` 格式作为占位符。AI 模型在解析模板时，应根据当前会话语义动态注入变量内容，而非简单的字符串替换。
- **D-04: 显式上下文标注 (Explicit Referencing)**：在模板中强制使用 `[See <FILE>]`（如 `[See RESEARCH.md]`, `[See CONTEXT.md]`）作为引用前序结论的标准方式，提升多阶段协作的一致性。

### 优化与交付 (Optimization & Delivery)
- **D-05: 指令精简 (Token Optimization)**：优化 `SKILL.md` 中的工具描述和参数 Schema，移除冗余描述，使用更紧凑的 Prompt 表达，以降低各轮次 Token 消耗。
- **D-06: 可靠性验证集 (Reliability Benchmark)**：建立一组边缘案例 (Edge Cases) 测试集，用于验证 AI 生成的 Vibe 指令是否 95%+ 符合规范（如检测无效 ID, 处理冲突的方向等）。
- **D-07: 快速开始 (Quick Start)**：在 `SKILL.md` 中增加“常用命令序列” Cheat Sheet，使模型在不读取复杂模板的情况下也能执行简单的窗格管理。

### Claude's Discretion
- `refactoring/` 模板目录的结构设计。
- `SKILL.md` 中 Token 压缩的具体文案调整。
- 变量注入时的默认后备逻辑。

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### 现有工作流定义
- `skills/vibe-operator/templates/sdd/` — 现有的 SDD 工作流作为重构模板的参考。
- `skills/vibe-operator/sops/` — 所有的 SOP 必须在模板中被正确引用。

### 技术规范
- `skills/vibe-operator/SKILL.yaml` — 定义了模板路由的基础配置。

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `sops/orchestration.md`: 提供重构时窗格切分的逻辑参考。
- `sops/verification.md`: 提供重构后 Logic Audit 的执行标准。

### Integration Points
- 所有的新模板应当放置在 `skills/vibe-operator/templates/refactoring/` 目录下。
- `SKILL.yaml` 需要更新以包含新的重构路由模式。

</code_context>

<deferred>
## Deferred Ideas

- 自动化测试驱动开发 (TDD) 专项模板。
- 环境深度诊断与网络排查模板。
- 动态变量的运行时校验逻辑。

</deferred>

---

*Phase: 12-workflow-templates-optimization*
*Context gathered: 2026-04-16*
