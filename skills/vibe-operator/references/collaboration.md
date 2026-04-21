# 多模型协作标准作业程序 (Collaboration SOP)

## 1. 核心流: Analyze-Declare-Execute-Verify

所有智能体必须严格遵守以下自治循环，以实现高强度的零干预工作流：

### Step A: Analyze (分析)
- Conductor 分配任务后，Worker 须在新环境中读取目标范围内的代码和相关上下文文档。
- **产出**：清晰理解修改范围。

### Step B: Declare (声明 / Intent Locking)
- **强制约束**：在修改任何文件前，Worker 必须声明对该文件的修改意图，防止多代理同时修改造成冲突。
- **指令示例**：
  ```bash
  vibe report --status blocked --message "writing:src/auth.rs"
  ```
- *注意*：其他 Worker 在分析阶段应使用 `vibe list` 检查是否存在冲突声明。

### Step C: Execute (执行)
- 应用代码修改、配置更新等。

### Step D: Verify & Fix (验证与自愈)
- **强制门禁**：任务执行后，必须运行相关验证（如 `cargo test` 或 `npm run lint`）。
- **自适应重试**：
  - 若验证失败，Worker **必须尝试自动修复错误**（最多 3 次）。
  - 若 3 次重试后仍未通过，方可通过信号向 Conductor 上报 `BLOCKED`。
- **成功信号**：
  ```bash
  vibe report --status success --message "Refactoring complete, all tests passed."
  vibe signal task_done '{"status":"ok", "next_step":"ready"}'
  ```

## 2. 信号驱动的工作流 (Signal-driven Workflow)

利用基于文件总线的 `signal` 和 `wait` 实现代理间的异步协作，彻底消除终端输入延迟和丢包风险。

- **同步协议**:
  1. **Conductor**: `vibe spawn --role Worker`。
  2. **Conductor**: 执行 `vibe wait done` 进入等待。
  3. **Worker**: 在新窗格中完成 `A-D-E-V` 任务循环。
  4. **Worker**: 发出 `vibe signal done`，内容包含 JSON Envelope。
  5. **Conductor**: 捕获文件总线信号，触发下一步智能推理。

## 3. 标准化汇报 (Standard 'vibe report')

所有 Worker 必须提交结构化汇报以保持 `panes.json` 的最新状态。

- **汇报参数**:
  - `--status`: `success`, `failed`, `in_progress`, `blocked`。
  - `--message`: 简明扼要的进展或锁定意图（如 "writing:path"）。

## 4. 上下文传递与状态检查

- **Conductor**: 定期使用 `vibe list` 监控所有 Worker 的状态。
- **Intelligence-First 决策**: Conductor 获取信号载荷后，不再机械依赖状态码，应通过自身推理评估项目状况，并利用 `vibe inject` 向挂起的代理发新指令，或重新派生。
