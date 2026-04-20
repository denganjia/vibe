# 多模型协作标准作业程序 (Collaboration SOP)

## 1. 基于角色的自治启动 (Role-based Spawning)

Vibe-Conductor (指挥官) 应使用 `vibe spawn` 根据任务需求启动具备特定角色的 Worker。

- **分配原则**:
  - **复杂重构/架构设计**: 启动具备 `Conductor` 或高级 `Worker` 角色的模型。
  - **执行/单元测试**: 启动标准 `Worker` 角色。
  - **审计/验证**: 启动 `Evaluator` 角色。

- **启动指令示例**:
  ```bash
  vibe spawn --role Worker
  ```
  *注：`vibe spawn` 会自动从 `.vibe/roles/Worker.md` 读取 Persona 并注入子进程。*

## 2. 信号驱动的工作流 (Signal-driven Workflow)

利用 `signal` 和 `wait` 实现代理间的异步协作。

- **同步协议**:
  1. **Conductor**: `vibe spawn --role Worker`。
  2. **Conductor**: 执行 `vibe wait done` 进入等待。
  3. **Worker**: 在新窗格中完成任务。
  4. **Worker**: `vibe report --status success --message "Refactoring complete"`。
  5. **Worker**: `vibe signal done`。
  6. **Conductor**: 捕获信号，恢复执行，通过 `vibe list` 检查结果。

## 3. 标准化汇报 (Standard 'vibe report')

所有 Worker 必须提交结构化汇报以保持状态一致性。

- **汇报参数**:
  - `--status`: `success`, `failed`, `in_progress`, `blocked`。
  - `--message`: 简明扼要的进展总结。

- **示例**:
  ```bash
  vibe report --status success --message "Added JWT login logic and passing unit tests."
  ```

## 4. 上下文传递与状态检查

- **Conductor**: 定期使用 `vibe list` 监控所有 Worker 的状态和 Summary。
- **状态同步**: 关键决策和项目进度应反映在 `.vibe/state/` 目录下的 JSON 文件或项目顶层的 `STATE.md` 中。
