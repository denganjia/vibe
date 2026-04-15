# Phase 5 CONTEXT: Output Monitoring & Lifecycle Safety

## 1. 核心目标与范围 (Goal & Scope)

本阶段旨在为上层 Skills 提供“视觉”反馈能力和进程链安全保障。系统不再尝试自动理解日志，而是提供一组高效的工具，让 AI 在 Skill 约束下能够主动上报进度、切换焦点，并确保整个物理环境的清理。

## 2. 关键架构决策 (Architectural Decisions)

### 2.1 极简监控模式 (Active Reporting)
- **核心逻辑**: 移除所有自动侦测（Heuristics）逻辑。
- **职责划分**: 
  - **Rust 核心**: 负责输出流的 Pipe 捕获，实时剥离 ANSI 码，并持久化到本地临时文件（供 AI 随时 cat 读取）。
  - **AI Skill**: 负责在任务逻辑结束时，显式调用 `vibe report` 或 `vibe focus`。

### 2.2 汇报与交互工具集 (Reporting Commands)
- **`vibe focus <vibe_id>`**: 封装 `TerminalAdapter::focus`（Wezterm/Tmux 置顶窗格）。AI 根据 Skill 指令在需要人类查看时主动调用。
- **`vibe report --status <S> --msg <M>`**: Worker 发送结构化消息到 Master UDS，更新数据库中的任务最终摘要。

### 2.3 物理清理与安全 (Lifecycle Safety)
- **Job Object 绑定**: Windows 下，Master 进程启动时即创建一个 Job Object，所有 Worker 及任务进程均加入该 Job。
- **清理策略**: Master 正常或异常退出时，利用操作系统特性瞬间回收所有关联进程，防止僵尸进程。

### 2.4 日志规范
- **路径**: 日志文件存储在 `LocalData/vibe/logs/{vibe_id}.log`。
- **清洗**: 在流式写入时即刻完成 ANSI Stripping。

## 3. 技术约束与规范 (Technical Constraints)

- **库选择**: 使用 `strip-ansi` 或正则表达式进行清洗；使用 `dialoguer` 进行必要的阻塞。
- **并发控制**: 确保多 Worker 同时向 Master 发送 `report` 消息时，Master 的消息队列能正确按序处理。

## 4. 下游代理指令 (Directives for Research/Planning)

- **研究员 (Researcher)**: 重点研究 WezTerm/Tmux 如何通过 CLI 实现物理焦点切换（focus-pane）。研究高效的 ANSI 剥离算法。
- **规划者 (Planner)**: 任务分解需包含：日志文件写流实现、`vibe focus` 指令开发、`vibe report` 协议打通、以及 Job Object 的全链路绑定。

---
*Last updated: 2026-04-14 after Phase 5 Discussion*
