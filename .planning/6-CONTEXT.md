# Phase 6 CONTEXT: Status Dashboard & UX

## 1. 核心目标与范围 (Goal & Scope)

本阶段旨在构建 `vibe-cli` 的“指挥塔”——一个全屏交互式的状态仪表盘。用户必须能够通过该界面实时监控所有 Agent 的进度、阅读最新日志，并快速执行物理干预（聚焦、销毁）。

## 2. 关键架构决策 (Architectural Decisions)

### 2.1 交互式 TUI 实现 (Ratatui)
- **技术栈**: 使用 `ratatui` (Rust) 构建。
- **运行模式**: 独占终端全屏，支持鼠标/键盘双交互（优先快捷键）。
- **退出策略**: 按 `q` 退出 TUI 回到 shell；按 `Enter` 触发物理聚焦并退出。

### 2.2 实时数据同步 (UDS Subscription)
- **推送架构**: Master 维护一个活跃监控客户端列表。每当收到 `Heartbeat`, `Report`, `ExitStatus` 时，Master 异步通过 UDS 向监控客户端广播全量或增量状态。
- **协议扩展**: 
  - `Subscribe`: 监控进程告知 Master 自己是“观察者”。
  - `Broadcast { states: Vec<WorkerState> }`: Master 推送的结构化数据。

### 2.3 仪表盘布局与功能 (Layout)
- **总览表 (Table)**: 显示 VibeID, Role, Status (Running/Idle/Failed/Done), Elapsed Time, 和最新的 Summary 内容。
- **日志预览 (Log Preview)**: 选中某一行时，仪表盘下方 1/3 区域显示对应的 `{vibe_id}.log` 文件尾部内容（实时刷新）。
- **交互动作 (Interactions)**:
  - `f` (Focus): 发送 `FocusRequest` 给 Master 或直接调用 `TerminalAdapter` 置顶窗格。
  - `k` (Kill): 发送 `KillRequest` 给 Master 以物理销毁该窗格及进程。

### 2.4 性能与资源 (Performance)
- **低开销**: TUI 进程在背景无变化时不应占用过多 CPU。
- **日志分流**: 预览窗格应使用 `tokio::fs` 的异步读取，避免阻塞 TUI 渲染循环。

## 3. 技术约束与规范 (Technical Constraints)

- **UI 库**: `ratatui` + `crossterm`。
- **错误处理**: 如果无法连接到 Master UDS，TUI 应显示清晰的错误页面并提示启动服务。

## 4. 下游代理指令 (Directives for Research/Planning)

- **研究员 (Researcher)**: 研究 `ratatui` 的 `Table` widget 如何实现流畅的长文本溢出处理（Summary 可能很长）。研究如何在一个异步循环中同时监听 UDS 消息和键盘输入。
- **规划者 (Planner)**: 任务分解需包含：UDS 广播机制实现、TUI 渲染框架搭建、日志实时追踪组件、以及各项交互快捷键的路由实现。

---
*Last updated: 2026-04-14 after Phase 6 Discussion*
