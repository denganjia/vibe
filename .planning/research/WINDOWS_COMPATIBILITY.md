# Windows Compatibility Research: vibe-cli

**Domain:** Cross-platform Terminal Orchestration
**Researched:** 2024-05-22
**Overall confidence:** HIGH

## Executive Summary

`vibe-cli` 在 Windows 10/11 上的支持是完全可行的，且可以通过共享大部分 Rust 代码逻辑来实现。核心挑战在于进程管理模型（Job Objects vs Process Groups）和 IPC 路径的细微差异。研究表明，Windows 10 (1803+) 对 Unix Domain Sockets (UDS) 的原生支持允许我们维持跨平台的通信架构，而 Windows Job Objects 则提供了确保 Worker 进程不泄露的健壮机制。WezTerm 的 CLI 工具在 Windows 上依然通过命名管道提供一致的交互体验。

## Key Findings

### 1. 跨平台 IPC 策略
*   **推荐方案:** 继续使用 **Unix Domain Sockets (UDS)**。
*   **理由:** 现代 Windows (10 Build 17063+ / Windows 11) 通过 `AF_UNIX` 原生支持 UDS。Tokio 1.43+ 提供了良好的封装。
*   **注意事项:**
    *   **路径长度:** 严格限制在 108 个字符以内。
    *   **文件清理:** Windows 不会自动清理关闭后的 Socket 文件，必须在 `bind` 前显式执行 `std::fs::remove_file`。
    *   **路径格式:** 使用标准的 Windows 文件路径（例如 `C:\Users\...\socket.sock`），而非 Unix 路径。

### 2. 进程组与生命周期管理
*   **挑战:** Unix 的进程组和信号机制在 Windows 上不直接适用。
*   **解决方案:** 使用 **Windows Job Objects**。
*   **核心功能:**
    *   通过 `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` 标志，确保当 Master 进程句柄关闭（包括崩溃）时，所有关联的 Worker 进程会被 Windows 内核自动终止。
    *   这是防止“僵尸 Worker”出现在 Windows 任务管理器中的最可靠方法。

### 3. WezTerm 深度集成
*   **通信层:** Windows 版 WezTerm 使用 **命名管道 (Named Pipes)** 代替 UDS 进行 CLI 与 GUI 间的通信。
*   **环境变量:**
    *   `WEZTERM_PANE`: 与 Unix 一致，用于标识当前窗格 ID。
    *   `WEZTERM_UNIX_SOCKET`: 在 Windows 上此变量通常指向命名管道路径（如 `\\.\pipe\wezterm-gui-...`）。
*   **CLI 调用:** 使用 `wezterm.exe cli` 配合相应的子命令（`split-pane`, `send-text`, `get-text`）。

## Recommended Stack for Windows

| Component | Technology | Why |
|-----------|------------|-----|
| **IPC** | `tokio::net::UnixStream` | 跨平台一致性，减少 `#[cfg(windows)]` 分支。 |
| **Process Control** | `windows-sys` | 轻量级 Win32 API 绑定，用于管理 Job Objects。 |
| **Environment** | `std::env` | 处理 Windows 路径（`PathBuf`）与环境变量。 |
| **Terminal UI** | `ratatui` / `crossterm` | 现代 Rust 库已完美支持 Windows VT100/ANSI 序列。 |

## Architecture Patterns for Windows

### Job Object Wrapper
为了确保 Worker 进程的清理，建议在 Master 启动时创建一个 Job Object 包装器：

```rust
// 伪代码：Windows 进程生命周期绑定
struct WindowsProcessGuard {
    job_handle: HANDLE,
}

impl WindowsProcessGuard {
    fn spawn_and_assign(&self, mut cmd: Command) -> Child {
        let child = cmd.spawn().unwrap();
        // 将子进程句柄分配给 Job Object
        assign_process_to_job(self.job_handle, child.raw_handle());
        child
    }
}
```

### Path Normalization
在处理持久化（`.vibe/state.db`）和 IPC 路径时，必须使用 `dirs` crate 来定位正确的目录：
*   Unix: `~/.local/share/vibe/`
*   Windows: `C:\Users\Name\AppData\Local\vibe\`

## Critical Pitfalls

### Pitfall 1: Socket File Residue
*   **现象:** Master 重启时 `UnixListener::bind` 报错 "Address already in use"。
*   **预防:** 在 Windows 上，即使进程正常退出，Socket 文件有时也可能残留在磁盘上。代码必须具备“启动前自清理”逻辑。

### Pitfall 2: Path Length Limit
*   **现象:** 嵌套过深的目录会导致 UDS 绑定失败。
*   **预防:** 优先在 `$TEMP` 或 `$LOCALAPPDATA` 下使用简短的 Socket 名称，并进行长度检查。

### Pitfall 3: `nix` Crate Incompatibility
*   **现象:** 现有的 `SUMMARY.md` 提到使用 `nix` crate 处理进程组，这在 Windows 上编译失败。
*   **预防:** 必须将 `nix` 依赖放入 `[target.'cfg(unix)'.dependencies]`，并在 Windows 上使用 `windows-sys` 作为替代方案。

## Implications for Roadmap

1.  **Phase 1 (Terminal Abstraction):** 增加 `TerminalAdapter` 对 `wezterm.exe` 后缀的处理，并引入 `dirs` crate 统一路径管理。
2.  **Phase 2 (IPC):** 在 Master 启动逻辑中增加对 Windows 环境下旧 Socket 文件的清理步骤。
3.  **Phase 4 (Safety):** 引入 Windows Job Objects 实现跨平台进程收割机制。

## Sources

- [Microsoft: Unix Domain Sockets on Windows](https://learn.microsoft.com/en-us/windows/win32/winsock/af-unix-reference)
- [Tokio Documentation: UnixListener on Windows](https://docs.rs/tokio/latest/tokio/net/struct.UnixListener.html)
- [WezTerm CLI Windows Guide](https://wezfurlong.org/wezterm/cli/index.html)
- [Rust `windows-sys` Crate Docs](https://docs.rs/windows-sys/latest/windows_sys/)
