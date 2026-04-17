# Phase 14: 信号总线实现 (Stateless Bus) - Context

**Gathered:** 2026-04-17
**Status:** Approved for implementation (Stateless Injection Model)

<domain>
## Phase Boundary

本阶段将实现基于终端注入的“无状态”信号总线。我们将废弃传统的 UDS 背景进程模式，转而利用 WezTerm/Tmux 原生的文本注入能力（send-text）实现代理间的同步。这使得 `vibe-cli` 成为一个纯粹的、无常驻守护进程的轻量级工具。

</domain>

<decisions>
## Implementation Decisions

### 1. 无状态通信架构 (Stateless Injection Bus)
- **D-01: 废弃 Daemon**: 移除对后台 `master` 进程的依赖。所有的状态管理均通过 `.vibe/` 目录下的 JSON 文件完成，通信通过终端适配器的文本注入完成。
- **D-02: 物理 ID 继承**: `vibe spawn` 或 `vibe run` 启动子任务时，必须获取当前窗格的物理 ID (如 `WEZTERM_PANE`) 并作为环境变量 `VIBE_MASTER_ID` 注入子进程。
- **D-03: 定向注入**: `vibe signal <NAME> [PAYLOAD]` 通过终端适配器（WezTerm/Tmux）直接向 `$VIBE_MASTER_ID` 对应的窗格发送（注入）文本。
- **D-04: 信号格式**: 注入的文本格式约定为：`\n[vibe-signal:<NAME>] <JSON_PAYLOAD>\n`。

### 2. Wait 端阻塞逻辑
- **D-05: Stdin 捕获**: `vibe wait <NAME>` 运行后，将作为前台进程独占该窗格的 `stdin`。它会扫描输入流，匹配对应的 `[vibe-signal:<NAME>]` 标记。
- **D-06: 载荷解析**: 一旦匹配成功，`vibe wait` 提取紧随其后的 JSON 内容并输出到控制台，随后以退出码 0 结束。
- **D-07: 超时机制**: 默认超时 300 秒，通过内部计时器实现。

### 3. 状态与隔离
- **D-08: 本地状态机**: 所有的“注册”信息（哪个窗格是什么角色）直接写入 `.vibe/state/panes.json`。
- **D-09: 广播支持**: 如果 `vibe signal` 未指定目标，则遍历 `panes.json` 向所有已知窗格注入信号。

</decisions>

<canonical_refs>
## Canonical References

### 待修改的核心模块
- `crates/vibe-core/src/state/mod.rs` — 增强 `StateStore` 对 `panes.json` 的并发安全读写（使用文件锁或原子写入）。
- `crates/vibe-core/src/adapter/mod.rs` — 确保 `write_to_pane` 接口在 WezTerm/Tmux 下均能稳定支持“文本注入”。
- `apps/vibe-cli/src/main.rs` — 实现无守护进程版的 `signal` 和 `wait` 命令。

### 废弃逻辑
- `crates/vibe-core/src/ipc/server.rs` — 本阶段及以后将不再使用 UDS 服务端逻辑，可进行清理。

</canonical_refs>

<deferred>
## Deferred Ideas

- **输入冲突处理**: 当用户正在输入时，注入信号可能导致混淆。未来可能考虑更复杂的“伪 TTY”劫持，但目前保持简单。

</deferred>

---

*Phase: 14-bus-core (Stateless Edition)*
*Context gathered: 2026-04-17*
