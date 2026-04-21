# Milestone 5.0 Requirements: Interaction & Initialization

## 1. 双向交互闭环 (Bi-directional Interaction)

### 1.1 主会话输入增强 (Master Input Enhancement)
- **需求**: 支持主会话通过 `vibe inject` 向特定 Worker 发送补充指令，而无需重启 Worker。
- **验证**: 主会话执行 `vibe inject v-xxxx "new instruction"`，Worker 的交互式 CLI 能够即时接收并处理。

### 1.2 Worker 自动回复 (Worker Auto-reply)
- **需求**: Worker 任务完成后，必须通过 `vibe signal [name]` 返回结果。
- **验证**: Conductor 能够通过 `vibe wait [name]` 捕获到结果，并将数据流式传递给后续任务。

### 1.3 交互可靠性 (Interaction Reliability)
- **需求**: 解决 `\r` 注入在不同 AI CLI（Claude vs Gemini）中的行为不一致问题。
- **验证**: 确保所有主流 AI CLI 都能在不按回车的情况下开始工作。

## 2. CLI 初始化标准化 (Initialization Standardization)

### 2.1 项目根目录配置 (`.vibe/config.json`)
- **需求**: 支持在项目根目录通过配置文件定义所有角色的默认命令。
- **验证**: 运行 `vibe init` 或 `vibe check` 时，如果不存在配置，则自动生成标准化的 `.vibe/config.json`。

### 2.2 环境一键就绪
- **需求**: `vibe spawn --all` 能够根据配置自动启动全套智能体（Conductor + Worker）。
- **验证**: 一个命令即可打开所有必要的 Tab 并初始化好各自的 Persona。

## 3. 高自治工作流 (Autonomous Workflow)

### 3.1 减少人力介入
- **需求**: 通过增强 Persona 模板，让智能体具备“遇到错误自动尝试修复”和“完成后主动发信号”的本能。
- **成功标准**: 至少完成一个中等复杂度的任务（如“重构一个包含 3 个文件的模块”），且 Conductor 不需要用户输入任何物理命令。
