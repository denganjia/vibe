# Feature Landscape: AI Skills & Multi-model Workflows

## Table Stakes (必选功能)
| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| 结构化技能定义 | 用户需要标准方式定义 Agent 能力（SKILL.md/YAML）。 | Low | 支持元数据、依赖描述和分步指令。 |
| 模型路由 (Routing) | 不同任务需要指派最适合的模型（推理 vs 执行）。 | Medium | 允许在技能定义中指定 preferred_model。 |
| 状态同步 (IPC Sync) | 跨窗格协作时，必须保证上下文（环境变量、临时变量）同步。 | Medium | 复用现有的 SQLite + IPC 机制。 |

## Differentiators (差异化竞争力)
| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| 交叉验证流 (Cross-checking) | 显著降低代码生成或系统操作的幻觉率。 | Medium | 实现“执行-检查”闭环。 |
| 动态 Handoffs | Agent 间可自主移交控制权，实现更复杂的流水线。 | High | 类似于终端中的“接力赛”。 |
| 受控编排 (Human-in-the-loop) | 在高风险节点（如删除、部署）强制介入。 | Medium | 对应 PROJECT.md 中的 Active 需求。 |

## Anti-Features (应避免)
| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| 全局上下文广播 | 导致 Token 膨胀和模型性能下降。 | 采用按需注入（Selective Context）。 |
| 硬编码的工作流 | 缺乏灵活性。 | 采用声明式的技能文件驱动。 |

## Feature Dependencies
`Skill Definition` -> `Agent Handoffs` (移交需要标准接口)
`State Sync` -> `Cross-checking` (校验需要读取前序状态)

### Roadmap Implications

- **Phase 1: Skills Framework** - 定义 `.vibe/skills/` 规范，实现技能解析与注入。
- **Phase 2: Multi-model Orchestration** - 增强 Master 节点的调度能力，支持 Handoff 逻辑。
- **Phase 3: Verification Layer** - 引入交叉检查 Agent 类型，实现 Evaluator 模式。
