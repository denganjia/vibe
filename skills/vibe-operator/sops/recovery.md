# 故障恢复标准作业程序 (Recovery SOP)

## 1. 精准注入序列 (Surgical Inject Sequences)

当 Worker 陷入循环或死锁时，Conductor 应按照以下序列通过 `vibe_inject` 进行干预 (D-07)。

1. **认知注入 (Perception Inject)**:
   - **目的**: 强制纠正 Worker 的错误认知。
   - **指令**: `vibe_inject <worker_id> --message "YOU ARE IN A LOOP. You have tried 'ls' 3 times. STOP and re-read the directory content using 'ls -a'."`
2. **干预注入 (Intervention Inject)**:
   - **目的**: 绕过故障点，直接执行特定命令。
   - **指令**: `vibe_inject <worker_id> --command "rm -f .lock" --message "Force clearing the lock file to proceed."`
3. **验证注入 (Verification Inject)**:
   - **目的**: 验证注入是否生效。
   - **指令**: `vibe_inject <worker_id> --command "check_health" --message "Verify system state after recovery."`
4. **恢复执行 (Resume)**:
   - **目的**: 让 Worker 回到正常轨道。
   - **指令**: `vibe_inject <worker_id> --message "State cleared. Resume from Task 3."`

## 2. 升级协议 (Escalation Protocols)

如果注入序列无法恢复 Worker，应执行升级操作：

- **Level 1: 软重启 (Soft Reset)**:
  - 停止 Worker 进程，并在同一窗格内重新初始化上下文。
- **Level 2: 硬重置 (Hard Reset)**:
  - 销毁当前 Worker (`vibe_stop`)，并创建一个具有全新环境的 Worker 进行替换。
- **Level 3: 人工介入 (Human Intervention)**:
  - 如果硬重置依然失败，标记任务为 FAILED 并请求用户手动干预。

## 3. 常见循环恢复示例

### 场景：权限拒绝循环
- **故障**: Worker 连续尝试 `cat /etc/shadow` 失败。
- **恢复**:
  ```bash
  vibe_inject "Worker-B" --message "Access denied is expected. Use 'ls -l' to check permissions first, then stop trying to read shadow."
  ```

### 场景：找不到文件循环
- **故障**: Worker 认为文件存在但路径错误，不断重试。
- **恢复**:
  ```bash
  vibe_inject "Worker-C" --command "find . -name 'config.yaml'" --message "The file path you are using is wrong. Here is the correct path."
  ```
