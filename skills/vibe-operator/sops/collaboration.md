# 多模型协作标准作业程序 (Collaboration SOP)

## 1. 基于推理的任务分配 (Reasoning-based Task Assignment)

Vibe-Conductor (指挥官) 应基于模型的逻辑推理能力和任务复杂度分配子任务。

- **分配原则 (D-01)**:
  - **复杂逻辑/重构**: 分配给具备强推理能力的模型（如 GPT-4, Claude 3 Opus）。
  - **原子化执行/脚本编写**: 分配给快速、低延迟的 Worker 模型。
  - **审计/验证**: 分配给 Evaluator 角色，负责交叉检查。

- **任务分派指令示例**:
  ```bash
  vibe_run "Worker-A" --command "implement the auth logic in auth.rs" --reasoning "This task requires high precision and strict type safety."
  ```

## 2. 标准化报告格式 (Standard 'vibe report' Format)

所有 Worker 必须通过 `vibe report` 提交结构化报告，以保持状态一致性 (基于 D-02)。

- **标准格式**:
  - **[STATUS]**: 当前任务状态 (SUCCESS, IN_PROGRESS, FAILED, BLOCKED)。
  - **Summary**: 核心工作简述（20-30字）。
  - **Outcome**: 具体的执行产出（文件、URL、哈希等）。
  - **Blocks**: 遇到的阻塞项。
  - **Next**: 建议的后续步骤。

- **示例**:
  ```bash
  vibe report "[SUCCESS] Auth implementation complete. 
  Summary: Added JWT login and session refresh. 
  Outcome: Modified src/auth.rs, added tests/auth_test.rs. 
  Blocks: None. 
  Next: Proceed to Task 5 (Database integration)."
  ```

## 3. 上下文传递策略 (Context Passing Strategy)

为了避免大型日志读取导致的上下文拥塞，应采用基于摘要的传递方式 (基于 D-02)。

- **检索原则**:
  - 指挥官 (Conductor) 应通过 `vibe_list --summaries` 获取所有 Worker 的最新 `vibe report` 摘要。
  - 只有在摘要不明确或发生错误时，才调用 `vibe_logs` 获取完整详细信息。
  - 严禁将非关键的调试信息存入 `vibe report` 的 Summary 字段。

## 4. 协作流程 (Collaboration Workflow)

1. **Conductor**: `vibe_check` 环境 → `vibe_submit_plan` 计划。
2. **Conductor**: `vibe_run` 启动多个 Workers。
3. **Workers**: 执行任务 → `vibe report` 定时或完成后汇报。
4. **Evaluator**: `vibe_list` 获取摘要 → `vibe_verify` 执行审计。
5. **Conductor**: `vibe_summary` 整合所有状态并决定下一步。
