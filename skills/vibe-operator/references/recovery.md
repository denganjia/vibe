# 故障恢复标准作业程序 (Recovery SOP)

## 1. A-D-E-V 自愈 (Autonomous Self-Healing)

在进入人工或 Conductor 干预前，Worker **必须**首先尝试自治愈：
- 若运行验证（如 `cargo test`）失败，Worker 需自主读取报错日志并修改代码修复。
- Worker 被允许最多连续尝试 **3 次** 自动修复。
- 只有在 3 次修复彻底失败后，Worker 才可以发出 `vibe signal task_failed '{"status":"blocked"}'` 向上级求援。

## 2. 精准注入干预 (Surgical Intervention)

当 Worker 陷入循环、死锁或发出 `BLOCKED` 信号求援时，Conductor 应使用 `vibe inject` 进行强制干预。

- **干预序列**:
  1. **状态探测**: `vibe list` 检查最后一次汇报或 `vibe focus <ID>` 观察当前物理输出。
  2. **认知修正**: 使用 `vibe inject <ID> "<MSG>"` 发送提示词，纠正 Worker 的逻辑错误或路径偏差。
  3. **强制指令**: 使用 `vibe inject <ID> "<CMD>"` 直接在 Worker 窗格执行恢复命令（如 `rm -f .lock` 或 `git restore .`）。
  4. **验证恢复**: `vibe list` 检查 Worker 状态是否更新。

## 3. 升级协议 (Escalation Protocols)

如果文本注入无法恢复 Worker，应执行以下操作：

- **Level 1: 软重置 (Soft Reset)**:
  - 在该窗格内注入中断指令 (Ctrl+C 等) 并重新启动 Agent CLI。
- **Level 2: 硬重置 (Hard Reset)**:
  - 销毁当前窗格。
  - 使用 `vibe spawn --role <ROLE>` 重新创建一个干净的 Worker 环境。
- **Level 3: 人工介入 (Human Intervention)**:
  - 如果多次重置依然失败，上报并请求人类介入处理底层环境或权限问题。
