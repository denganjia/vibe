# Domain Pitfalls

**Domain:** AI 终端调度器 (AI Terminal Orchestrator)
**Researched:** 2024-05-24

## 严重陷阱 (Critical Pitfalls)

会导致重构或重大问题的错误。

### Pitfall 1: 上下文窗口泛滥 (Context Window Flooding)
**What goes wrong:** 将原始的终端输出（如 `npm install` 的长日志、构建过程）直接回传给 Master AI，导致 Token 迅速耗尽。
**Why it happens:** 终端命令通常会产生大量未格式化的文本、ANSI 转义码（颜色）、以及进度条带来的回车符 (`\r`) 刷新，信息密度极低。
**Consequences:** Master AI 丢失初始指令（发生“遗忘”），产生幻觉，或者直接因为超出 LLM 的 Token 限制而导致 API 调用失败。
**Prevention:** 在 Rust 层实现一个输出过滤器/摘要器。使用 `strip-ansi-escapes` 等库剥离颜色代码，过滤高频刷新的进度条。通过滑动窗口（Sliding Window）仅向 AI 截取首尾关键日志，或在长输出发生时使用大模型进行局部摘要后再回传。
**Detection:** AI 频繁触发 API Token 限制报错，或在执行长耗时任务后偏离初始目标。

### Pitfall 2: 僵尸进程与状态残留 (Zombie Processes & Leaked State)
**What goes wrong:** Worker 窗格被关闭，或 `vibe-cli` 发生崩溃，但其内部运行的子进程（如 Dev Server、文件监听器）仍然在后台挂起运行。
**Why it happens:** 在 Rust 中未正确处理进程组（Process Group）的终止，或者过度依赖 Wezterm/Tmux 的窗格关闭机制，而没有向底层的实际进程发送 `SIGTERM`/`SIGKILL` 信号。
**Consequences:** 端口冲突（如后续运行报错 "Address already in use"），内存泄漏，以及“静默失败”（AI 以为自己成功启动了服务，但其实因为端口被旧的幽灵进程占用而失败）。
**Prevention:** 严格使用进程组管理（如 Rust 的 `nix` crate 处理 `SIGCHLD`）。在 `.vibe/state.db` 中维护严格的 PID 注册表，并在任务清理阶段（Cleanup）确保整个进程树被正确收割。
**Detection:** AI 尝试启动网络服务时，立刻收到 Exit Code 1，并在日志中发现端口绑定失败。

### Pitfall 3: 交互式提示阻塞 (Interactive Prompt Blocking)
**What goes wrong:** Worker Agent 执行的命令弹出了交互式输入提示（例如 SSH 密钥确认，`apt-get` 的 `[y/N]`，或者代码冲突时的操作），导致进程无限期挂起。
**Why it happens:** AI 往往假设所有终端命令都是非交互式的，它只等待执行完毕并检查退出码，无法处理中途的阻塞式 `stdin` 请求。
**Consequences:** 终端窗格死锁。Master AI 无限期等待 Worker 完成，导致整个自动化任务闭环完全停滞。
**Prevention:** 在意图注入层面，强制 AI 尽可能使用非交互式参数（如 `-y`, `--non-interactive`）。在 Rust 监控层引入超时机制（Timeout）和“静默检测”：如果进程在 X 秒内既没有新的输出也没有退出，向 Master AI 发出“可能发生交互阻塞”的警告，并截取屏幕最后几行供 AI 决策。
**Detection:** Worker 窗格进程处于活动状态，但输出流停滞超过预设时间，且未产生 Exit Code。

## 中度陷阱 (Moderate Pitfalls)

### Pitfall 1: 多窗格间的环境变量丢失 (Environment Variable Loss)
**What goes wrong:** Master AI 在一个窗格中设置了环境变量（如 `export NODE_ENV=production`），但随后新开启的 Worker 窗格并没有继承这些配置，导致执行环境不一致。
**Prevention:** 将 `.vibe/state.db` 作为全局环境状态的单一事实来源（Single Source of Truth）。当 `vibe-cli` 通过 Wezterm/Tmux 开启新窗格时，必须显式地从状态库中读取并注入这些环境变量（例如通过 `tmux set-environment` 或在命令前拼接环境注入）。

### Pitfall 2: 状态同步的竞态条件 (Race Conditions in State Sync)
**What goes wrong:** Master AI 试图向一个刚刚被要求创建的 Worker 窗格注入命令，但此时该窗格的 Shell 尚未完全初始化完毕，导致命令丢失或执行异常。
**Prevention:** 避免使用脆弱的 `sleep()` 延迟。通过 Rust 监听 Unix Socket 或轮询终端状态，实现明确的“窗格就绪（Pane Ready）”健康检查机制（Health Check/Ack），只有收到就绪信号后才允许注入意图。

## 轻微陷阱 (Minor Pitfalls)

### Pitfall 1: 盲目信任退出码 (False Positives on Exit Codes)
**What goes wrong:** 某些不规范的 CLI 工具即使发生了概念上的错误也会返回 `0`，或者将严重错误信息输出到了 `stdout` 而非 `stderr`。AI 如果仅判断退出码为 `0` 就认为成功，会导致后续逻辑链崩溃。
**Prevention:** AI 在验收任务闭环时，除了检查退出码，还必须强制要求 Rust 监控层回传最后 N 行输出日志进行语义验证。

### Pitfall 2: Tmux/Wezterm 目标定位漂移 (Targeting Drift)
**What goes wrong:** 在自动化脚本中使用索引（如 Tmux 的 `0:1.0`）定位窗格，当其他窗格被关闭或切分时，索引发生改变，导致命令发送到错误的窗口。
**Prevention:** 在调度层始终使用唯一的标识符（如 Tmux 的 Pane ID `%1` 或 Wezterm 的内部 Pane ID）进行绝对定位，而不是相对的索引。

## 阶段性警告 (Phase-Specific Warnings)

| 阶段主题 | 可能的陷阱 | 应对策略 |
|-------------|---------------|------------|
| 窗格编排 (Orchestration) | 频繁调用 CLI 导致极高的性能开销与延迟 | 尽量合并对 Tmux/Wezterm 的 CLI 调用，或者通过底层 Unix Socket（而非频繁 spawn 进程）与终端通信。 |
| 意图注入 (Intent Injection) | AI 产生“幻觉”并生成具有破坏性的单行脚本（如 `rm -rf /`） | 在执行涉及文件系统修改、进程终止的危险命令前，实施命令白名单或要求人类/高层权限 AI 的显式确认。 |
| 进度监控 (Monitoring) | 纯文本解析导致 AI 无法区分“命令本身”与“命令输出” | Rust 层在截取输出时，应该使用明确的定界符（Delimiters）或 JSON 结构封装上下文，清晰区分标准输出与错误流。 |

## 参考来源 (Sources)

- [HIGH] Wezterm/Tmux 官方 CLI 文档与系统信号规范 (进程组生命周期)
- [MEDIUM] WebSearch: AI Agent 终端自动化与多窗格编排的常见挑战 (Context Window 溢出与 ANSI 转义码处理)
- [MEDIUM] 社区经验: Rust 终端与进程管理陷阱 (PTY 处理, 僵尸进程防范, 竞态条件)
