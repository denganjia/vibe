# Phase 1 CONTEXT: Terminal Orchestration Foundation

## 1. 核心目标与范围 (Goal & Scope)

本阶段旨在构建 `vibe-cli` 的“物理肢体”——终端编排层。它必须能够跨平台（Windows/Unix）稳定地操作 WezTerm 和 Tmux 的窗格，并保持状态的一致性。

## 2. 关键架构决策 (Architectural Decisions)

### 2.1 终端适配器模式 (Terminal Adapter)
- **Trait 定义**: 必须实现 `TerminalAdapter` trait，包含以下核心方法：
  - `split(direction: SplitDirection, target: VibeID) -> Result<VibeID>`: 物理切分窗格并返回逻辑 ID。
  - `send_keys(id: VibeID, keys: &str) -> Result<()>`: 向指定窗格发送原始指令字符串。
  - `close(id: VibeID) -> Result<()>`: 关闭物理窗格并清理逻辑映射。
  - `get_metadata(id: VibeID) -> Result<PaneMetadata>`: 获取物理 ID 和当前状态。
- **物理布局**: 默认为 **“Tab 内多窗格并联 (Panes)”**，暂不支持“跨 Tab 调度”。

### 2.2 持久化状态管理 (Persistence)
- **数据库**: 在 Phase 1 即引入 **SQLite (rusqlite)** 作为唯一事实来源 (SSOT)。
- **逻辑 ID 映射**: `vibe-cli` 维护逻辑 `VibeID` 到物理 `PaneID` (WezTerm 的唯一 `pane_id` 或 Tmux 的 `%id`) 的映射。
- **目的**: 解决 CLI 工具多次调用间的状态丢失问题，并防止因终端切分导致的“物理 ID 漂移”。

### 2.3 跨平台集成策略 (Cross-Platform Strategy)
- **终端检测**:
  - Windows: 优先检测并调用 `wezterm.exe cli`。
  - Unix: 检测 `WEZTERM_PANE` 或 `TMUX` 环境变量，自动切换适配器。
- **环境要求**: `vibe-cli` 采取 **Fail Fast** 策略。它必须在目标终端会话内部运行，若环境不匹配则直接报错退出并给出引导建议。

### 2.4 配置与路径 (Config & Paths)
- **配置文件**: 采用 **TOML** 格式 (`vibe.toml`)。
- **目录规范 (dirs-based)**:
  - **Config**: Windows (`%APPDATA%\vibe`), Linux/macOS (`~/.config/vibe`)。
  - **State (DB)**: 存储在 `LocalData` 目录下，确保数据库的本地唯一性。

## 3. 技术约束与规范 (Technical Constraints)

- **运行时**: 基于 **Tokio (async runtime)**，尽管 CLI 是同步交互，但为了后续 UDS (Phase 2) 需预留异步架构。
- **错误处理**: 使用 `thiserror` 或 `anyhow` 提供详细的跨平台错误上下文。

## 4. 下游代理指令 (Directives for Research/Planning)

- **研究员 (Researcher)**: 重点研究 `wezterm cli list --format json` 的具体结构解析，以及如何在 Rust 中稳定地获取当前活动的 `pane_id`。
- **规划者 (Planner)**: 任务分解需包含：适配器 Trait 开发、SQLite 模式定义、CLI 包装器实现、以及针对 Windows/Unix 的环境检测逻辑。

---
*Last updated: 2026-04-14 after Phase 1 Discussion*
