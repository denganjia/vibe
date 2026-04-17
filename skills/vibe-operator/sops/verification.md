# 验证与交叉检查标准作业程序 (Verification & Cross-checking SOP)

## 1. 任务后逻辑审计 (Post-task Logic Audit)

Evaluator (审计者) 必须在任务完成后、标记为 SUCCESS 前执行逻辑审计。

- **审计流程 (D-03, D-04)**:
  1. **读取意图**: 从 `vibe_logs` 或初始计划中提取该任务的原始意图。
  2. **代码/产出走查**: 检查生成的代码、配置或文档是否实现了所有要求。
  3. **环境核实**: 使用 `ls`, `grep`, `run_tests` 等工具验证物理变更。
  4. **反馈提交**: 如果发现偏差，通过 `vibe report` 提交审计结果并通知 Conductor。

## 2. 意图对齐检查表 (Intent Alignment Checklist)

审计者在验证时应逐项核对以下内容：

- [ ] **意图完整性 (Intent Completeness)**: 是否所有子任务要求都已达成？
- [ ] **逻辑严密性 (Logic Integrity)**: 代码逻辑是否闭环？是否存在明显的竞态条件或边界错误？
- [ ] **副作用审计 (Side-effect Audit)**: 是否对不相关的模块造成了非预期的破坏？
- [ ] **状态一致性 (State Consistency)**: 变更是否已正确记录在系统状态中（如 `STATE.md` 或数据库）？

## 3. 模式化死锁检测 (Pattern-based Deadlock Detection)

为了防止 AI 陷入无效循环，Conductor 必须监控执行模式 (D-06)。

- **检测标准 (M=3)**:
  - **重复因子 (M=3)**: 同一个 Worker 连续 3 次执行完全相同的命令且结果无变化。
  - **静默挂起 (120s)**: Worker 超过 120 秒未产出任何日志且无 CPU 活跃迹象。
- **触发逻辑**:
  - 一旦满足上述任一条件，Conductor 必须将该 Worker 标记为 `DEADLOCK`。
  - 自动触发 `recovery.md` 中定义的恢复流程。
