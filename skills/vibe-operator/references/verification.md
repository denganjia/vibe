# 验证与交叉检查标准作业程序 (Verification & Cross-checking SOP)

## 1. 任务后逻辑审计 (Post-task Logic Audit)

Evaluator (审计者) 必须在任务完成后、标记为 SUCCESS 前执行逻辑审计。

- **审计流程**:
  1. **读取意图**: 从初始计划中提取该任务的原始意图，核实 Intent Locking 记录 (`vibe list`)。
  2. **代码/产出走查**: 检查生成的代码、配置或文档是否实现了所有要求。
  3. **环境核实**: 运行自动化测试脚本（如 `cargo test`）验证物理变更。
  4. **反馈提交**: 如果发现偏差，通过 `vibe report` 提交审计结果并通过 `.vibe/bus/` 发送信号通知 Conductor。

## 2. 意图对齐检查表 (Intent Alignment Checklist)

审计者在验证时应逐项核对以下内容：

- [ ] **意图完整性 (Intent Completeness)**: 是否所有子任务要求都已达成？
- [ ] **逻辑严密性 (Logic Integrity)**: 代码逻辑是否闭环？是否存在明显的竞态条件？
- [ ] **A-D-E-V 遵守情况 (A-D-E-V Compliance)**: Worker 是否在修改前声明了 Intent Locks？
- [ ] **状态一致性 (State Consistency)**: 变更是否已正确汇入最终的 `DELIVERY.md` 或全局状态中？

## 3. 模式化死锁检测 (Pattern-based Deadlock Detection)

为了防止 AI 陷入无效循环，Conductor 必须监控执行模式。

- **检测标准**:
  - **连续重试失败**: Worker 达到 A-D-E-V 规定的 3 次自愈重试上限并发出 `BLOCKED` 信号。
  - **静默挂起**: Worker 超过 120 秒未在 `vibe list` 的 summary 中更新进度。
- **触发逻辑**:
  - 一旦满足上述任一条件，自动触发 `recovery.md` 中定义的恢复流程。
