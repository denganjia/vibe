# 故障恢复标准作业程序 (Recovery SOP)

## 1. 精准注入干预 (Surgical Intervention)

当 Worker 陷入循环、死锁或非预期行为时，Conductor 应使用 `vibe inject` 进行强制干预。

- **干预序列**:
  1. **状态探测**: `vibe focus <ID>` 观察当前输出，或 `vibe list` 检查最后一次汇报。
  2. **认知修正**: 使用 `vibe inject <ID> "<MSG>"` 发送提示词，纠正 Worker 的逻辑错误或路径偏差。
  3. **强制指令**: 使用 `vibe inject <ID> "<CMD>"` 直接在 Worker 窗格执行恢复命令（如 `rm -f .lock` 或 `git restore .`）。
  4. **验证恢复**: `vibe list` 检查 Worker 状态是否更新。

- **指令示例**:
  ```bash
  vibe inject 3 "Stop searching in /usr/local. The config file is in ./config/."
  vibe inject 3 "rm -f ./tmp/lockfile"
  ```

## 2. 升级协议 (Escalation Protocols)

如果文本注入无法恢复 Worker，应执行以下操作：

- **Level 1: 软重置 (Soft Reset)**:
  - 在该窗格内注入中断指令 (Ctrl+C 等) 并重新启动 Agent CLI。
- **Level 2: 硬重置 (Hard Reset)**:
  - 使用 `vibe focus <ID>` 后手动关闭该窗格，或者使用外部终端工具强制终止进程。
  - 使用 `vibe spawn --role <ROLE>` 重新创建一个干净的 Worker 环境。
- **Level 3: 人工介入 (Human Intervention)**:
  - 如果多次重置依然失败，上报 Conductor 并请求用户介入处理底层环境问题。

## 3. 常见故障场景

### 场景：Permission Denied 循环
- **故障**: Worker 不断重试无权限的写操作。
- **恢复**: `vibe inject <ID> "You don't have sudo access. Use the project-local ./tmp directory instead."`

### 场景：依赖缺失循环
- **故障**: Worker 不断尝试运行未安装的工具。
- **恢复**: `vibe inject <ID> "The tool 'jq' is missing. Use a python one-liner to parse the JSON instead."`
