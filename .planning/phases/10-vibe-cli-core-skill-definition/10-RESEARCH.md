# Phase 10: Vibe-CLI Core Skill Definition - Research

**Researched:** 2026-04-16
**Domain:** AI Agent Skill Definitions, Multi-Agent Orchestration, MCP
**Confidence:** HIGH

## Summary

本研究旨在为 `vibe-cli` 设计一套标准化的 AI 技能包。根据 Phase 10 的决策，技能包将采用 `YAML + Markdown` 的混合结构，通过 `SKILL.md` 作为主入口，引导 AI Agent（特别是 Master Orchestrator）如何利用 `vibe-cli` 提供的工具链执行复杂的开发任务。

核心发现：
- 采用“渐进式披露”原则，将元数据、角色定义、SOP 和 模块化模板分层组织，以节省上下文 Token。
- `role.md` 应包含交互式初始化逻辑，确保 AI 在开始工作前与用户对齐环境和偏好。
- 技能定义应深度集成 `vibe-cli` 的 MCP 工具集，并提供抽象的逻辑流（Conceptual SOP）而非死板的命令列表。

**Primary recommendation:** 使用 `metadata.yaml` 定义元数据和路由，`role.md` 定义协作协议，并在 `templates/` 下按模式（SDD/SPEC）组织工作流。

<user_constraints>
## User Constraints (from 10-CONTEXT.md)

### Locked Decisions
- **D-01: 混合架构**：采用 `YAML + Markdown` 的结构。YAML 用于定义元数据、路由索引和依赖；Markdown 用于描述详细的指令流和标准操作规程 (SOP)。
- **D-02: 模块化模板**：工作流模板（Workflow Templates）将采用模块化组织，每个模式（如 SDD, Hotfix）拥有独立的目录或文件，而非合并在一个大文档中。
- **D-03: role.md 机制**：技能包中包含 `role.md` 角色定义文档。
- **D-04: 交互式初始化**：仅在初次使用该技能包时，AI 需检查 `role.md`，并通过与用户的交互式问答（问卷/对话）来完善和个性化该文档。后续使用将严格遵循该文档定义的角色分工。
- **D-05: 协作模式**：采用“严格层级控制”（Strict Hierarchical Control），由 Master 模型负责全局指挥和任务指派。
- **D-06: 抽象逻辑流 (Conceptual SOP)**：SOP 侧重于逻辑上的步骤顺序（例如：先 split 准备环境，再 run 执行任务，最后 report 状态），而非硬编码具体的命令字符串，以保持对不同终端环境性适配。
- **D-07: AI 自主触发 (AI Discretion)**：交叉检查被定义为一个 Review 过程。在技能流程中告知 AI，当任务完成或达到关键里程碑时，由 AI 根据改动的影响范围自主决定是否需要发起 Review 请求。
- **D-08: SDD/SPEC 导向**：模板需遵循“讨论 - 调研 - 计划 - 实现 - 审查/测试 - 结束”的标准开发生命周期。

### the agent's Discretion
- YAML 元数据的具体字段设计。
- `role.md` 中问答环节的具体问题设计。
- 具体的 Markdown 指令描述文案优化。

### Deferred Ideas (OUT OF SCOPE)
- 自动化技能包更新检查机制。
- 非 SDD 模式的其他垂直领域模板（如运维、数据分析）。
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SKILL-01 | Design YAML metadata schema | 基于 MCP 最佳实践设计的 `metadata.yaml` |
| SKILL-02 | Draft `role.md` and questions | 设计了 5 个核心交互问题及 Master/Worker 角色职责 |
| SKILL-03 | Define directory structure | 确定了 `skills/vibe-cli/` 的标准层级结构 |
| SKILL-04 | Outline Conceptual SOPs | 梳理了窗格管理、状态同步、审批流的逻辑步骤 |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `YAML` | 1.2 | Metadata definition | Industry standard for manifests. [VERIFIED: web search] |
| `Markdown` | GFM | Documentation & SOPs | Best for AI readability and human inspection. [VERIFIED: web search] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `JSON Schema` | Draft 7 | Parameter validation | Validating tool inputs in MCP. [VERIFIED: mcp.rs] |

## Architecture Patterns

### Recommended Project Structure
```
skills/vibe-cli/
├── SKILL.md          # 技能主入口 (Markdown + YAML Frontmatter)
├── metadata.yaml     # 详细元数据、路由与依赖 (可选，若不放在 SKILL.md 中)
├── role.md           # 角色协议与初始化问答
├── sops/             # 标准操作规程 (抽象逻辑流)
│   ├── pane-management.md
│   ├── state-sync.md
│   └── approval-flow.md
└── templates/        # 工作流模块化模板
    ├── sdd/          # Software Design Document 模式
    │   ├── 01-discuss.md
    │   ├── 02-research.md
    │   └── ...
    └── spec/         # Specification 模式
        └── ...
```

### Pattern 1: Progressive Disclosure (渐进式披露)
**What:** Agent 首先加载基础元数据（Name/Description），只有当意图匹配时才深入读取 `SKILL.md` 和 `role.md`，执行具体步骤时再按需读取 `sops/` 和 `templates/`。
**When to use:** 始终使用，以最小化 Token 消耗并保持上下文清晰。

### Anti-Patterns to Avoid
- **Hardcoding Commands:** 在 SOP 中直接写 `vibe split` 而不是逻辑描述。这会导致在不同环境（如 Windows vs Linux）中失效。应使用工具名（如 `vibe_split`）和逻辑步骤。
- **Monolithic Template:** 将所有模式放在一个文件中，导致 AI 难以快速定位正确的指令。

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Tool Schema | Custom JSON parser | MCP tool definitions | MCP 提供标准的类型安全协议。 [CITED: mcp.rs] |
| State Tracking | Custom log parser | `vibe_list` / DB | `vibe-cli` 已内置 SQLite 状态存储。 [CITED: StateStore] |

## Common Pitfalls

### Pitfall 1: Loop Synchronization
**What goes wrong:** Master 和 Worker 都在等待对方上报状态，导致死锁。
**How to avoid:** Master 必须主动通过 `vibe_list` 轮询状态，Worker 必须在关键节点通过 `vibe_report` 强制同步。

### Pitfall 2: Environment Mismatch
**What goes wrong:** AI 假设当前终端支持 `split` 但实际上是在 VSCode Terminal 中。
**How to avoid:** 初始化阶段必须调用 `vibe_check` 并根据返回的 `recommendation` 调整行为。

## Code Examples

### Metadata Schema (metadata.yaml)
```yaml
# Source: Designed based on MCP best practices
name: vibe-cli-core
version: 0.1.0
description: "Core skill for multi-agent orchestration and development using Vibe-CLI."
routing:
  default_mode: sdd
  patterns:
    - { pattern: "sdd", template: "templates/sdd/" }
dependencies:
  - mcp: vibe-cli
```

### role.md Interactive Questions
```markdown
## Initialization Questions
1. "Stack Detection: I've detected [Tech Stack], is this correct?"
2. "Terminal: Prefer WezTerm or Tmux for pane management?"
3. "Safety: Should I request approval for every command? (Strict/Moderate/Relaxed)"
4. "Focus: Should I auto-switch terminal focus to new panes?"
5. "Role: Should I (Master) manage all tasks, or will you assign them manually?"
```

## State of the Art

| Old Approach | Current Approach | Impact |
|--------------|------------------|--------|
| Manual Pane Mgmt | `vibe_split` + `vibe_focus` | AI can automate workspace setup. |
| Blind Execution | `vibe_submit_plan` + Approval | Improved safety and alignment. |
| Console Logs | `vibe_run` + StateStore | Centralized visibility of all parallel agents. |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | AI models can effectively parse multi-file skill structures. | Summary | Medium - might need to flatten if token limits are tight. |
| A2 | Users prefer interactive setup over static config files. | role.md | Low - can always provide a default config. |

## Open Questions

1. **How to handle skill updates?** 目前被推迟，但可能需要简单的版本检查。
2. **Template dynamic injection?** 是否支持在模板中使用变量（如 `{{vibe_id}}`）？建议在 Implementation Phase 确定。

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo` | Building vibe-cli | ✓ | 1.84.0 | Use pre-built binary |
| `wezterm` | Local orchestration | ✓ | 20240203 | Use `vibe_run` (external) |
| `tmux` | Local orchestration | ✗ | — | Fallback to WezTerm/External |
| `node` | Running MCP client | ✓ | v22.16.0 | — |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` |
| Config file | `Cargo.toml` |
| Quick run command | `cargo test --lib` |
| Full suite command | `cargo test` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| SKILL-01 | YAML schema validity | unit | `jsonschema -i metadata.yaml schema.json` | ❌ Wave 0 |
| SKILL-04 | SOP logical consistency | smoke | AI-driven walkthrough test | ❌ Wave 0 |

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V4 Access Control | yes | `vibe_submit_plan` for critical ops. |
| V5 Input Validation | yes | MCP JSON Schema validation. |
| V10 Malicious Code | yes | No `eval`-like execution of AI strings without `--yes` or user gate. |

### Known Threat Patterns for Vibe-CLI

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Command Injection | Tampering | Use `vibe_run` with shell-escaped arguments; mandatory approval for high-risk cmds. |
| Log Secret Leak | Information Disclosure | `vibe_run` should mask known env var patterns (e.g. `*_KEY`, `*_SECRET`). |

## Sources

### Primary (HIGH confidence)
- `apps/vibe-cli/src/main.rs` - CLI commands
- `apps/vibe-cli/src/mcp.rs` - MCP tool definitions
- `crates/vibe-core/src/env.rs` - Environment & Path logic

### Secondary (MEDIUM confidence)
- Anthropic/MCP best practices docs for tool naming and descriptions.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH
- Architecture: HIGH
- Pitfalls: MEDIUM (based on early multi-agent experiments)

**Research date:** 2026-04-16
**Valid until:** 2026-05-16
