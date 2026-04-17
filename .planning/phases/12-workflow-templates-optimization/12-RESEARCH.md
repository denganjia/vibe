# Phase 12: Workflow Templates & Optimization - Research

**Researched:** 2026-04-16
**Domain:** Prompt Engineering, Workflow Automation, Token Optimization
**Confidence:** HIGH

## Summary

本阶段的研究重点在于提升 `vibe-operator` 技能包在复杂任务（特别是代码重构）中的可靠性与效率。通过引入标准化的 `refactoring/` 模板组，我们可以引导模型遵循“分析-实现-测试-评审”的严谨流程。
同时，通过对 `SKILL.md` 进行 Token 审计和压缩，预计可减少约 30-40% 的上下文开销。引入 `$[VARIABLE]` 注入协议和可靠性基准测试将进一步确保模型在边缘情况下的鲁棒性。

**Primary recommendation:** 采用符号化与结构化压缩技术重写 `SKILL.md`，并建立基于 Mock 环境的可靠性验证集。

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01: 聚焦代码重构 (Refactoring Focus)**：目前仅引入专门的 `refactoring/` 工作流模板。
- **D-02: 标准重构流 (Standard SOP)**：重构路径定义为：分析 (Analyze) -> 实现重构 (Refactor) -> 测试验证 (Test) -> 最终审计 (Review)。不强制要求自动化快照/备份节点，保持流程轻量级。
- **D-03: Prompt 变量注入 (Prompt Injection)**：采用 `$[VARIABLE_NAME]` 格式作为占位符。AI 模型在解析模板时，应根据当前会话语义动态注入变量内容，而非简单的字符串替换。
- **D-04: 显式上下文标注 (Explicit Referencing)**：在模板中强制使用 `[See <FILE>]`（如 `[See RESEARCH.md]`, `[See CONTEXT.md]`）作为引用前序结论的标准方式，提升多阶段协作的一致性。
- **D-05: 指令精简 (Token Optimization)**：优化 `SKILL.md` 中的工具描述和参数 Schema，移除冗余描述，使用更紧凑的 Prompt 表达，以降低各轮次 Token 消耗。
- **D-06: 可靠性验证集 (Reliability Benchmark)**：建立一组边缘案例 (Edge Cases) 测试集，用于验证 AI 生成的 Vibe 指令是否 95%+ 符合规范（如检测无效 ID, 处理冲突的方向等）。
- **D-07: 快速开始 (Quick Start)**：在 `SKILL.md` 中增加“常用命令序列” Cheat Sheet，使模型在不读取复杂模板的情况下也能执行简单的窗格管理。

### the agent's Discretion
- `refactoring/` 模板目录的结构设计。
- `SKILL.md` 中 Token 压缩的具体文案调整。
- 变量注入时的默认后备逻辑。

### Deferred Ideas (OUT OF SCOPE)
- 自动化测试驱动开发 (TDD) 专项模板。
- 环境深度诊断与网络排查模板。
- 动态变量的运行时校验逻辑。
</user_constraints>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| vibe-cli | 0.1.0+ | 核心编排工具 | 项目自研，提供物理窗格管理 |
| handlebars | - | (Potential) 模板引擎 | 若需服务器端注入时使用，但本阶段优先由 LLM 解析 |

## Architecture Patterns

### Recommended Project Structure
```
skills/vibe-operator/
├── templates/
│   └── refactoring/       # New: Refactoring specific templates
│       ├── 01-analyze.md
│       ├── 02-refactor.md
│       ├── 03-test.md
│       └── 04-review.md
├── SKILL.md               # Optimized: Compressed version
└── benchmarks/            # New: Reliability test cases
    └── edge_cases.json
```

### Pattern 1: Modular Refactoring Flow
**What:** 将重构拆分为 4 个离散阶段，强制要求阶段间通过文件（RESEARCH.md, PLAN.md）传递上下文。
**When to use:** 任何非琐碎的代码改动（如跨文件修改、接口重定义）。

### Pattern 2: Dynamic Variable Injection (`$[VAR]`)
**What:** LLM 在读取模板时，将 `$[VARIABLE]` 替换为当前上下文中的已知信息。
**Logic:**
1. **Source 1: History** - 从最近 5-10 轮对话中提取目标（Target Files/Modules）。
2. **Source 2: Local Docs** - 从 `CONTEXT.md` 或 `STATE.md` 中读取当前阶段目标。
3. **Fallback** - 若无法确定，模型必须暂停并询问用户，或在指令中明确标注 `[MISSING: VAR_NAME]`。

## Token Audit & Optimization

### Current SKILL.md Audit (Estimated)
- Total Tokens: ~850
- Redundancy Areas:
    - Over-verbose tool descriptions (e.g., "Check if the current terminal environment supports...").
    - Recursive headers in Protocols section.
    - Repetitive role responsibility lists.

### Proposed Compression Strategy [VERIFIED: web search]
- **Symbolic Replacement**: Use `->` for transitions, `[]` for parameters.
- **Header Flattening**: Merge sub-sections to reduce Markdown nesting.
- **Instruction Pruning**: Remove "You are an AI..." and politeness.
- **Tool Schema Shrinking**: Use compact single-line descriptions.

**Target Token Count:** ~500 (40% reduction).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| String Template Engine | Custom Regex | LLM Inference | D-03 明确要求基于语义注入，而非简单替换。 |
| Task Queue | Custom scheduler | Vibe-CLI Panes | 物理窗格本身即是状态容器。 |

## Runtime State Inventory

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | SQLite `panes` table (vibe_id, role, status) | None - 架构保持兼容。 |
| Live service config | `SKILL.yaml` routing patterns | 更新路由以支持 `refactoring` 模式。 |
| OS-registered state | Terminal panes (WezTerm/Tmux) | None. |
| Secrets/env vars | None | N/A |
| Build artifacts | `target/debug/vibe-cli` | 确保最新版本已构建。 |

## Common Pitfalls

### Pitfall 1: Injection Hallucination
**What goes wrong:** 模型为 `$[VARIABLE]` 编造了一个不存在的路径。
**How to avoid:** 模板应包含“Verify file existence before use”指令。

### Pitfall 2: Context Overflow in Workers
**What goes wrong:** Worker 模型被分配了过多的上下文，导致丢失核心指令。
**How to avoid:** 遵循 Collaboration SOP 中的“摘要传递策略”。

## Reliability Benchmarks (Edge Cases)

| Case ID | Scenario | Expected Model Behavior |
|---------|----------|-------------------------|
| EDGE-01 | **Invalid vibeId** | 模型应先通过 `vibe_list` 核对，若不存在则报错。 |
| EDGE-02 | **Conflicting Splits** | 在已经深度拆分的窗口再次执行 `vibe_split`，模型应判断空间是否足够。 |
| EDGE-03 | **Missing Approval** | 未通过 `vibe_submit_plan` 就执行高危操作，Evaluator 应拦截。 |
| EDGE-04 | **Variable Ambiguity** | `$[TARGET]` 对应多个模糊目标，模型应先进行澄清。 |

## Code Examples

### Refactoring Template Snippet (01-analyze.md)
```markdown
# Refactoring Phase 1: Analyze
Goal: Understand $[REFACTOR_TARGET] and prepare for change.
[See CONTEXT.md] for high-level requirements.

Instructions:
1. Run `ls $[REFACTOR_TARGET]` to verify target exists.
2. Call `vibe_split` for a research pane if needed.
3. Document logic in RESEARCH.md.
```

### Quick Start / Cheat Sheet
```markdown
## Quick Start
- **INIT**: `vibe_check` -> `vibe_split` -> `vibe_run worker`
- **PLAN**: `vibe_submit_plan` -> `vibe_query_approval`
- **EXEC**: `vibe_inject [ID] "[CMD]"`
- **SYNC**: `vibe_list` -> `vibe_focus [ID]`
```

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| vibe-cli | Execution | ✓ | target/debug/vibe-cli | Build from source |
| node | Scripting | ✓ | v22.16.0 | — |
| sqlite3 | State | ✓ | - | — |

## Sources

### Primary (HIGH confidence)
- `skills/vibe-operator/SKILL.md` - Existing skill definition.
- `skills/vibe-operator/SOPs/` - Collaboration and Verification protocols.
- `target/debug/vibe-cli` - Local binary verification.

### Secondary (MEDIUM confidence)
- [WebSearch] "AI model prompt compression token audit techniques"
- [WebSearch] "reliability benchmarks for AI agent CLI tool use edge cases"

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH
- Architecture: HIGH
- Pitfalls: MEDIUM (Requires execution data)

**Research date:** 2026-04-16
**Valid until:** 2026-05-16
