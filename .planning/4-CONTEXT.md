# Phase 4 CONTEXT: Intent Injection & Human-in-the-Loop

## 1. 核心目标与范围 (Goal & Scope)

本阶段旨在实现从 Master 到 Worker 的受控任务委派机制。必须确保指令注入的准确性、环境变量的安全性，以及关键操作的人工审核流程。

## 2. 关键架构决策 (Architectural Decisions)

### 2.1 指令注入与反馈 (Injection & Response)
- **混合注入模式 (Hybrid Injection)**:
  - 优先通过 UDS 协议向活跃的 Worker 发送结构化消息 (`ExecuteIntent`)。
  - 对于非 `vibe` 托管的物理窗格，回退至 `TerminalAdapter::send_keys` 原始文本注入。
- **自动执行**: 注入指令默认包含 `\n`，在经过网关确认后自动触发执行。

### 2.2 人工干预机制 (Human-in-the-Loop)
- **本地网关 (Local Confirmation Gate)**:
  - 确认提示必须显示在执行命令的 **Worker 窗格** 内部。
  - 提示符格式: `[VIBE GATE]: Confirm "..."? (y/N)`。
- **授权例外**: 允许通过 CLI 标志（如 `-y`, `--trusted`）在启动任务时预授权，跳过针对该特定任务的确认提示。

### 2.3 上下文传播 (Context Propagation)
- **环境变量同步**:
  - 采用 **白名单 + 前缀过滤** 模式。
  - 默认同步: `PATH`, `USER`, `LANG` 及其它 `VIBE_` 开头的变量。
- **动态目录**: 指令包可包含可选的 `cwd` 字段，Worker 必须在执行前尝试切换至该目录。
- **跨 Shell 适配 (Shell Polyfill)**: 根据 Worker 所在的 Shell 环境（自动探测），自动处理环境变量设置命令的语法差异（如 `export` vs `$env:`）。

## 3. 技术约束与规范 (Technical Constraints)

- **协议扩展**: UDS 协议需新增 `ExecuteIntent`, `GateRequest`, `GateResponse` 等消息类型。
- **阻塞逻辑**: 在等待人工确认期间，Worker 进程必须阻塞子任务的启动，并保持与 Master 的心跳连接。

## 4. 下游代理指令 (Directives for Research/Planning)

- **研究员 (Researcher)**: 重点研究如何在不中断 PTY 运行的情况下，在终端窗口中“干净地”弹出一个阻塞式的人工确认交互提示。同时研究跨平台 Shell 探测技术。
- **规划者 (Planner)**: 任务分解需包含：UDS 协议升级、Worker 阻塞式网关实现、Shell 语法适配层、以及 `vibe run` 命令中信任标志的逻辑实现。

---
*Last updated: 2026-04-14 after Phase 4 Discussion*
