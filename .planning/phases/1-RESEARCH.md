# Phase 1: Terminal Orchestration Foundation - Research

**Researched:** 2026-04-14
**Domain:** Terminal Orchestration (WezTerm/Tmux), Cross-platform Integration, State Persistence
**Confidence:** HIGH

## Summary

本研究涵盖了 `vibe-cli` 在 Phase 1 构建“终端编排基础”所需的核心技术点。通过调研 WezTerm 和 Tmux 的 CLI 输出结构、跨平台环境检测机制、Windows 下的进程清理方案以及 SQLite 状态管理，为后续实现提供了明确的技术路径。

**Primary recommendation:** 
使用 `WEZTERM_PANE` 和 `TMUX_PANE` 环境变量作为“当前窗格”的首选标识符；利用 WezTerm 的原生 JSON 输出和 Tmux 的格式化字符串输出实现跨终端的统一元数据提取；在 Windows 上通过 `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` 确保进程树的安全清理。

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `rusqlite` | 0.31 | 本地状态持久化 | Rust 社区最成熟的 SQLite 绑定，支持同步操作，适合 CLI。 |
| `tokio` | 1.37 | 异步运行时 | 为 Phase 2 的 UDS 和并发调度预留空间。 |
| `windows-sys` | 0.52 | Win32 API 调用 | 用于实现 Windows Job Objects，比 `winapi` 更轻量且现代。 |
| `serde_json` | 1.0 | JSON 解析 | 处理 WezTerm CLI 输出。 |
| `thiserror` | 1.0 | 错误处理 | 提供结构化的、带上下文的跨平台错误。 |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|--------------|
| `dirs` | 5.0 | 跨平台路径计算 | 获取 Config 和 LocalData 目录。 |
| `which` | 6.0 | CLI 可执行文件探测 | 检查 `wezterm` 或 `tmux` 是否在 PATH 中。 |

## Architecture Patterns

### Recommended Project Structure
```
src/
├── adapter/
│   ├── mod.rs          # TerminalAdapter trait
│   ├── wezterm.rs      # WezTerm implementation
│   └── tmux.rs         # Tmux implementation
├── state/
│   ├── mod.rs          # DB abstraction
│   └── schema.sql      # Initial SQLite schema
├── env.rs              # Environment detection logic
└── os/
    ├── mod.rs          # OS-specific helpers
    └── windows.rs      # Job Objects for Windows
```

### Pattern 1: Terminal Adapter (Trait-based)
**What:** 使用 Trait 屏蔽底层终端 CLI 的差异。
**When to use:** 所有的窗格操作（切分、发送键、关闭）都应通过适配器。

### Anti-Patterns to Avoid
- **直接调用 Shell 命令而不进行环境检查**: 必须先验证 `WEZTERM_PANE` 或 `TMUX` 是否存在。
- **硬编码物理 ID**: 始终通过逻辑 `VibeID` 操作，由适配器层负责映射转换。

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| 路径计算 | 手动拼接字符串 | `dirs` crate | 处理 Windows `%APPDATA%` vs Unix `~/.config` 的复杂性。 |
| JSON 路径解析 | 正则表达式 | `serde_json` | CLI 输出结构可能随版本变化，强类型解析更稳健。 |
| 进程树清理 | 手动追踪 PID | Windows Job Objects | 防止父进程崩溃时留下“僵尸”子进程。 |

## Runtime State Inventory

> 本阶段为 Greenfield，但需考虑 CLI 会话间的持久化状态。

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | `state.db` | 存储 `VibeID <-> PhysicalID` 映射。 |
| Live service config | Terminal sessions | 需要在启动时验证映射中的物理 ID 是否仍然有效。 |

## Common Pitfalls

### Pitfall 1: Physical ID Drift
**What goes wrong:** 在 WezTerm 中，如果用户手动关闭了一个窗格，原来的 `pane_id` 可能会失效。
**How to avoid:** 在每次执行编排任务前，调用 `adapter.get_metadata(id)` 验证物理 ID 是否存在。

### Pitfall 2: Environment Variable Shadowing
**What goes wrong:** 在 Tmux 内部运行 WezTerm（或反之）时，环境变量可能并存。
**How to avoid:** 定义优先级策略。通常优先响应 `TMUX`（更内层），除非显式配置。

## Code Examples

### 1. WezTerm CLI JSON Parsing
```rust
// wezterm cli list --format json
let output = std::process::Command::new("wezterm")
    .args(["cli", "list", "--format", "json"])
    .output()?;

let panes: Vec<WezTermPane> = serde_json::from_slice(&output.stdout)?;
let current = panes.iter().find(|p| p.is_focused);
```

### 2. Tmux Format String
```rust
// tmux list-panes -F template
let template = r#"{"pane_id": "#{pane_id}", "is_active": "#{pane_active}"}"#;
let output = std::process::Command::new("tmux")
    .args(["list-panes", "-F", template])
    .output()?;
```

### 3. Windows Job Object (Cleanup)
```rust
use windows_sys::Win32::System::JobObjects::*;
// ... 创建 Job Object 并设置 JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE ...
// 确保子进程在 vibe master 退出时被杀死
```

## SQLite Schema (Initial)

```sql
CREATE TABLE panes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vibe_id TEXT NOT NULL UNIQUE,      -- 逻辑 ID (e.g., 'worker-1')
    physical_id TEXT NOT NULL,         -- 物理 ID (e.g., '5' or '%1')
    terminal_type TEXT NOT NULL,       -- 'wezterm' | 'tmux'
    role TEXT NOT NULL,                -- 'master' | 'worker'
    status TEXT DEFAULT 'active',      -- 'active' | 'closed'
    metadata TEXT,                     -- 额外的 JSON 元数据
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `wezterm` | WezTerm Adapter | ✓ | 2024+ | Fail fast |
| `tmux` | Tmux Adapter | ✓ | 3.2+ | Fail fast |
| `sqlite3` | Persistence | ✓ | 3.x | — (bundled in rusqlite) |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` |
| Mocking | `mockall` (用于模拟 Terminal CLI) |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command |
|--------|----------|-----------|-------------------|
| ORCH-01 | Split pane creates logical ID | Integration | `cargo test test_split_logic` |
| SAFE-02 | Kill switch closes all recorded panes | Integration | `cargo test test_kill_switch` |

## Security Domain

### Applicable ASVS Categories
| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V5 Input Validation | Yes | 对通过 `send_keys` 发送的指令进行基本的合法性检查（防止命令注入）。 |

## Sources

### Primary (HIGH confidence)
- [WezTerm Official Docs](https://wezfurlong.org/wezterm/cli/cli/list.html) - JSON structure and fields.
- [Tmux Man Page](https://man7.org/linux/man-pages/man1/tmux.1.html) - Format strings and `-F` usage.
- [Microsoft Learn](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects) - Job Objects documentation.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Rust ecosystem standard.
- Architecture: HIGH - Adapter pattern is proven.
- Pitfalls: MEDIUM - Real-world ID drift depends on user behavior.

**Research date:** 2026-04-14
**Valid until:** 2026-05-14
