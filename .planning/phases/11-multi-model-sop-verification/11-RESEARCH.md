# Phase 11: Multi-model SOP & Verification - Research

**Researched:** 2026-04-16
**Domain:** Multi-agent Collaboration, SOP Design, Automated Verification
**Confidence:** HIGH

## Summary

本阶段研究了多模型协作的标准操作规程 (SOP) 和基于 `vibe-cli` 状态的校验逻辑。研究核心围绕“Conductor-Worker”架构，通过结构化的提示词策略、标准化的状态报告格式以及智能的故障恢复机制，确保多模型在复杂软件工程任务中的协同效率和代码质量。

**Primary recommendation:** 采用“意图对齐 (Intent Alignment)”作为校验的核心指标，并利用 `vibe report` 作为轻量级上下文交换协议，减少大模型间的 Token 消耗，同时提升 Conductor 的监控效率。

<user_constraints>
## User Constraints (from 11-CONTEXT.md)

### Locked Decisions
- **D-01: 基于推理能力分工**：采用“强 Master (Conductor) + 轻量 Worker”的模式。
- **D-02: 基于摘要的上下文传递 (Summary-based)**：Worker 通过 `vibe report` 发送关键摘要。
- **D-03: 任务后校验 (Post-task Only)**：校验在子任务完成时触发。
- **D-04: 逻辑审计与意图对齐 (Logic Audit / Intent Alignment)**：审计侧重于检查代码实现是否完全对齐了规划阶段的“原始意图”。
- **D-05: 独立文档组织**：创建 `skills/vibe-operator/sops/verification.md`。
- **D-06: 基于模式的死锁判定 (Pattern-based)**：连续 M 次相同错误判定为死锁。
- **D-07: 外科手术式注入 (Surgical Inject)**：发现异常时优先使用 `vibe_inject` 注入诊断指令。

### the agent's Discretion
- 具体死锁判定的重复次数 (M) 和超时间隔。
- `vibe report` 摘要的推荐格式规范。
- 逻辑审计时的提示词 (Prompt) 结构设计。

### Deferred Ideas (OUT OF SCOPE)
- 基于领域专家（前端/后端）的模型自动路由。
- 实时状态对比流（执行前后的 ls/grep 差异分析）。
- 复杂的自动回滚逻辑。
</user_constraints>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Vibe-CLI | current | 基础架构 | 提供进程隔离与状态同步 |
| Claude 3.5 Sonnet | N/A | Conductor | 高推理能力，负责规划与决策 |
| Claude 3 Haiku / GPT-4o-mini | N/A | Worker | 高性价比，负责执行具体任务 |

## Architecture Patterns

### Pattern 1: Conductor-Worker Prompting
**What:** 分级提示词策略。Conductor 负责 Objective 和 Definition of Done (DoD)，Worker 负责 Execution 和 Status Reporting。
**When to use:** 所有多模型协作场景。

### Pattern 2: Summary-based Context (SCC)
**What:** 仅通过摘要同步状态，而非全文同步。
**When to use:** 当多个 Worker 同时运行，且 Conductor 需要保持低 Context 消耗时。

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| 进程状态监控 | 自研轮询逻辑 | `vibe_list` | `vibe-cli` 已内置持久化状态存储 |
| 远程命令执行 | `ssh` 或裸 `sh` | `vibe_inject` | 带有审计审计和确认闸门的安全通道 |

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| vibe-cli | 核心通信 | ✓ | 0.1.0 | — |
| WezTerm/Tmux | 窗格管理 | ✓ | — | `vibe run` 外部窗口 |

## Logic Audit / Intent Alignment Checklist

作为 Evaluator 角色，在任务完成后必须执行以下对齐检查：

1.  **意图完整性 (Intent Completeness)**: 实现是否覆盖了 `PLAN.md` 中定义的所有“原因”和“目的”？
2.  **逻辑严密性 (Logic Integrity)**:
    *   是否存在未处理的异常路径？
    *   并发场景下是否存在竞态条件？
3.  **副作用审计 (Side-effect Audit)**:
    *   是否误删了不相关的文件？
    *   是否引入了不必要的库依赖？
4.  **环境一致性 (State Consistency)**: `vibe_list` 报告的状态是否与文件系统的实际状态吻合？

## Standard `vibe report` Format

Worker 必须使用以下半结构化格式进行汇报，以便 Conductor 高效解析：

```text
[STATUS] <简短动作描述>
- Outcome: <任务结果总结>
- Files: <主要变动文件列表>
- Blocks: <遇到的阻碍或警告 (可选)>
- Next: <建议的下一步动作 (供 Conductor 参考)>
```

## Pattern-based Deadlock Detection

判定逻辑如下：

- **重复因子 (M)**: 默认设为 `3`。
- **判定规则**:
  - **Error Loop**: Worker 连续 3 次汇报相同的错误信息（通过 `vibe report` 或 `vibe_list` 中的 `summary` 字段判定）。
  - **Silence Hang**: Worker 状态为 `running`，但 `summary` 在 120 秒内（或 3 个轮询周期内）未发生任何变化。
- **触发动作**: Conductor 立即调用 `vibe_inject` 进入诊断模式。

## Surgical Inject Recovery Sequences

当检测到异常或死锁时，Conductor 应按以下顺序执行“外科手术式注入”：

1.  **感知注入 (Perception Inject)**:
    *   `ls -la <context_path>` (确认文件是否存在)
    *   `cat <config_file>` (检查配置是否正确)
2.  **干预注入 (Intervention Inject)**:
    *   `chmod +x <script>` (修复权限)
    *   `npm install <missing_package>` (补齐依赖)
3.  **验证注入 (Verification Inject)**:
    *   `./run_test.sh <specific_case>` (运行最小验证单元)
4.  **恢复指令 (Resume)**:
    *   指示 Worker 重新执行主任务或跳过已知坏点。

## Code Examples

### Conductor 分派任务示例 (Reasoning-based)
```markdown
# To: Worker-A (vibe-1)
## Objective
实现 `crates/vibe-core/src/state/db.rs` 中的 `UpdateReport` 方法。

## Reasoning
你需要负责数据持久化层的具体实现，因为它是一个高度确定性的任务，不需要全局上下文。

## DoD
- [ ] 支持更新 status 和 summary 字段。
- [ ] 通过 `test_actor_concurrency` 测试。

## Reporting
完成后请执行: `vibe report --status success --message "[DONE] UpdateReport implemented and tested."`
```

### Worker 汇报示例
```bash
vibe report --status failed --message "[FAILED] Database locked
- Outcome: SqliteError(5) while writing.
- Blocks: Concurrent access from master daemon.
- Next: Inspect lock status or retry with backoff."
```

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Worker 的 Listener 模式在执行命令时仍能接收 Intent | Surgical Inject | 注入指令可能无法实时到达，导致恢复失效 |
| A2 | LLM 可以稳定解析半结构化的 `vibe report` 文本 | Standard Format | Conductor 可能无法正确理解 Worker 的进度 |

## Sources

### Primary (HIGH confidence)
- `crates/vibe-core/src/ipc/protocol.rs` - `ReportInfo` 结构验证。
- `apps/vibe-cli/src/main.rs` - `Report` 和 `Inject` 命令实现验证。
- `skills/vibe-operator/role.md` - 角色定义验证。

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - 基于现有 codebase
- Architecture: HIGH - 符合多模型主流协作模式
- Pitfalls: MEDIUM - 死锁判定依赖于总结的质量

**Research date:** 2026-04-16
**Valid until:** 2026-05-16
