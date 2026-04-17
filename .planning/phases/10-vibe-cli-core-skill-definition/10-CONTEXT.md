# Phase 10: Vibe-CLI Core Skill Definition - Context

**Gathered:** 2026-04-16
**Status:** Ready for planning

<domain>
## Phase Boundary

本阶段的目标是为 `vibe-cli` 构建一套结构化的 AI 技能定义包。这套技能包将作为 AI 模型的“操作手册”，使其能够理解如何利用 `vibe-cli` 的窗格管理、IPC 状态同步和审批流功能来执行复杂的、多模型的 SDD/SPEC 开发工作流。

</domain>

<decisions>
## Implementation Decisions

### 技能包结构 (Package Structure)
- **D-01: 混合架构**：采用 `YAML + Markdown` 的结构。YAML 用于定义元数据、路由索引和依赖；Markdown 用于描述详细的指令流和标准操作规程 (SOP)。
- **D-02: 模块化模板**：工作流模板（Workflow Templates）将采用模块化组织，每个模式（如 SDD, Hotfix）拥有独立的目录或文件，而非合并在一个大文档中。

### 角色管理 (Role Management)
- **D-03: role.md 机制**：技能包中包含 `role.md` 角色定义文档。
- **D-04: 交互式初始化**：仅在初次使用该技能包时，AI 需检查 `role.md`，并通过与用户的交互式问答（问卷/对话）来完善和个性化该文档。后续使用将严格遵循该文档定义的角色分工。
- **D-05: 协作模式**：采用“严格层级控制”（Strict Hierarchical Control），由 Master 模型负责全局指挥和任务指派。

### 标准操作规程 (SOP)
- **D-06: 抽象逻辑流 (Conceptual SOP)**：SOP 侧重于逻辑上的步骤顺序（例如：先 split 准备环境，再 run 执行任务，最后 report 状态），而非硬编码具体的命令字符串，以保持对不同终端环境的适配性。

### 交叉检查与审计 (Review & Audit)
- **D-07: AI 自主触发 (AI Discretion)**：交叉检查被定义为一个 Review 过程。在技能流程中告知 AI，当任务完成或达到关键里程碑时，由 AI 根据改动的影响范围自主决定是否需要发起 Review 请求。

### 工作流模式 (Workflow Patterns)
- **D-08: SDD/SPEC 导向**：模板需遵循“讨论 - 调研 - 计划 - 实现 - 审查/测试 - 结束”的标准开发生命周期。

### Claude's Discretion
- YAML 元数据的具体字段设计。
- `role.md` 中问答环节的具体问题设计。
- 具体的 Markdown 指令描述文案优化。

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Vibe-CLI 核心协议与命令
- `apps/vibe-cli/src/main.rs` — 现有的 CLI 命令集定义 (Commands enum)。
- `apps/vibe-cli/src/mcp.rs` — 现有的 MCP 工具列表及其参数 Schema。

### 开发规范
- `CLAUDE.md` — 项目现有的开发规范与指令参考。

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Commands` enum in `main.rs`: 提供了所有可调用的底层指令。
- `mcp::handle_request`: 展示了 AI 如何通过 JSON-RPC 调用这些指令。

### Integration Points
- 技能包应当放置在项目根目录或配置指定的 `skills/` 目录下。
- `vibe-cli` 的 MCP `initialize` 响应可以作为技能加载的触发点。

</code_context>

<deferred>
## Deferred Ideas

- 自动化技能包更新检查机制。
- 非 SDD 模式的其他垂直领域模板（如运维、数据分析）。

</deferred>

---

*Phase: 10-vibe-cli-core-skill-definition*
*Context gathered: 2026-04-16*
