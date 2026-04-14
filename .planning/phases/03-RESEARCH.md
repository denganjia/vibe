# Phase 3: State Persistence & IPC Layer - Research

**Researched:** 2024-03-21
**Domain:** IPC (UDS), Daemonization, Persistent State (SQLite)
**Confidence:** HIGH

## Summary

本研究针对 Phase 3 的核心需求（状态持久化与 IPC 层）进行了技术方案分析。核心架构采用 **Master-Worker 模型**，通过 **Unix Domain Sockets (UDS)** 进行低延迟通信，并使用 **Newline-delimited JSON (NDJSON)** 作为消息协议。为了确保跨平台的鲁棒性，研究了 Windows 与 Unix 的守护进程化（Daemonization）差异，以及 Windows 10+ 对 `AF_UNIX` 的支持特性。

**Primary recommendation:**
在 Master 进程启动时，必须先清理可能存在的陈旧 UDS 路径，然后进行 `bind`；在 Windows 上，通过 `CommandExt::creation_flags` 配合 `DETACHED_PROCESS` 和 `CREATE_NO_WINDOW` 实现无感知后台化。

## Project Constraints (from GEMINI.md)

- **Always use Chinese response**: 本研究报告及后续产物均遵循此规范。
- **Commit docs**: 所有的规划文档及研究报告需提交至 git (已在配置中开启)。

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| STAT-01 | 持久化状态 (Persistent State) | 定义了 SQLite 扩展模式与串行化写入并发模型。 |
| STAT-02 | 基于 UDS 的实时状态同步 | 验证了跨平台 UDS 支持、NDJSON 协议格式及 `tokio` 异步实现。 |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `tokio` | `1.x` | 异步运行时 | [VERIFIED: Cargo.toml] 支持异步 UDS 监听与并发任务调度。 |
| `rusqlite` | `0.32` | SQLite 驱动 | [VERIFIED: Cargo.toml] `bundled` 模式减少环境依赖，支持复杂状态持久化。 |
| `serde_json` | `1.0` | 协议序列化 | [VERIFIED: Cargo.toml] 高性能处理 NDJSON 消息格式。 |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `tokio-util` | `0.7` | 编解码工具 | 处理 NDJSON 的 `LinesCodec` 帧解析。[ASSUMED] |
| `daemonize` | `0.5` | Unix 守护进程化 | Unix 平台下标准的后台脱离实现。[ASSUMED] |
| `windows-sys` | `0.59` | Win32 API 访问 | [VERIFIED: Cargo.toml] 用于设置 Windows 进程启动标志及 Job Objects。 |
| `uuid` | `1.0` | 唯一 ID 生成 | 为 `vibe_id` 生成全局唯一标识符。[ASSUMED] |
| `tempfile` | `3.x` | 测试支持 | 用于测试 UDS 文件清理与数据库迁移。[ASSUMED] |

**Installation:**
```bash
# 修改 Cargo.toml 添加缺失依赖
cargo add tokio-util --features codec
cargo add uuid --features v4
#[cfg(unix)]
cargo add daemonize
```

## Architecture Patterns

### Recommended Project Structure
```
crates/vibe-core/src/
├── ipc/
│   ├── mod.rs        # IPC 核心入口
│   ├── server.rs     # Master UDS Listener
│   ├── client.rs     # Worker/Client UDS Stream
│   └── protocol.rs   # NDJSON 消息定义 (Register, Heartbeat, etc.)
├── state/
│   ├── mod.rs        # 状态库入口
│   ├── db.rs         # SQLite Manager (串行化写入)
│   └── schema.rs     # 模式定义与迁移
└── os/
    ├── mod.rs
    ├── windows.rs    # 已存在: Job Objects, 补充 Daemon logic
    └── unix.rs       # 补充 Fork/Daemon logic
```

### Pattern 1: On-demand Master Launch (按需启动)
**What:** 任何 `vibe` 指令作为 Client 尝试连接 UDS。若失败（`NotFound`），则 Fork/Spawn Master 进程，等待其就绪后再重试连接。
**When to use:** 确保用户无需手动启动 Master 服务，保持 CLI 体验流畅。

### Pattern 2: Serialized DB Operations (串行化写入)
**What:** Master 进程内部开启一个专用的 `mpsc` 通道接收 `DbRequest`，由单个任务/线程持有 SQLite 连接进行操作。
**Example:**
```rust
// 伪代码示例
let (db_tx, mut db_rx) = mpsc::channel::<DbRequest>(100);
tokio::spawn(async move {
    let mut conn = Connection::open(db_path)?;
    while let Some(req) = db_rx.recv().await {
        handle_db_request(&mut conn, req);
    }
});
```

### Anti-Patterns to Avoid
- **Shared Mutable Connection:** 不要在多个 `tokio` 任务间共享同一个 `rusqlite::Connection` (非 Thread-safe)。
- **Implicit Socket Cleanup:** 不要依赖进程正常退出来清理 `.sock` 文件；必须在 `bind` 前显式尝试删除。

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| NDJSON Framing | 手动处理 `\n` 切分 | `tokio_util::codec::LinesCodec` | 处理粘包、不完整行等边缘情况更健壮。 |
| Windows Backgrounding | 自己调用复杂的 `CreateProcess` | `std::process::CommandExt` (Windows) | Rust 标准库已封装 `creation_flags`，足以设置 `DETACHED_PROCESS`。 |
| Unix Daemonization | 手动双重 `fork()` | `daemonize` crate | 封装了 `setsid`, `umask`, `std*` 重定向等繁琐步骤。 |

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `AF_UNIX` (UDS) | IPC Layer | ✓ | Win10 17063+ | Windows 10 以下无法原生支持，需报错提示或降级。 |
| `sqlite3` | Persistence | ✓ | 3.x (bundled) | 使用 `rusqlite` 的 `bundled` 特性，无需系统预装。 |
| `JobObjects` | Process Mgmt | ✓ | Windows NT | Windows 专属，用于确保 Master 退出时清理子进程。 |

**Missing dependencies with no fallback:**
- **Older Windows (Pre-17063)**: 无法使用 `tokio::net::UnixListener`。由于 `vibe` 目标为现代环境，暂不提供降级。

## Common Pitfalls

### Pitfall 1: Stale Socket Files (陈旧套接字)
**What goes wrong:** Master 崩溃后留下 `.sock` 文件，重启时 `bind` 报 `AddrInUse` 错误。
**How to avoid:** 在 `bind` 前调用 `tokio::fs::remove_file(path).await`，并忽略 `NotFound` 错误。

### Pitfall 2: Windows UDS Path Length (路径长度限制)
**What goes wrong:** Windows UDS 路径长度限制为 108 字符。如果用户配置目录路径极长，`bind` 会失败。
**How to avoid:** 建议使用 `dirs::config_dir` 并检查最终路径长度，必要时在根目录下使用简短路径。

### Pitfall 3: Daemon Standard Streams (标准流阻塞)
**What goes wrong:** 守护进程如果不关闭 `stdout/stderr`，可能会导致父进程（启动它的 CLI）在等待子进程关闭管道时阻塞。
**How to avoid:** 必须显式设置 `stdin/stdout/stderr` 为 `Stdio::null()`。

## Code Examples

### 1. Cross-platform Daemon (Windows/Unix)
```rust
#[cfg(windows)]
fn spawn_daemon() {
    use std::os::windows::process::CommandExt;
    const DETACHED_PROCESS: u32 = 0x00000008;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    
    Command::new(env::current_exe().unwrap())
        .arg("master")
        .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
}

#[cfg(unix)]
fn spawn_daemon() {
    use daemonize::Daemonize;
    Daemonize::new()
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .start()?;
}
```

### 2. IPC Protocol Definition (NDJSON Schema)
```json
// Register: Worker -> Master
{
  "version": "v1",
  "type": "register",
  "vibe_id": "uuid-v4",
  "payload": {
    "pid": 1234,
    "physical_id": "pane-1",
    "terminal": "wezterm",
    "role": "executor"
  }
}

// Heartbeat: Worker -> Master (Every 5s)
{
  "version": "v1",
  "type": "heartbeat",
  "vibe_id": "uuid-v4"
}
```

### 3. SQLite Schema Extension
```sql
-- 在现有 panes 表基础上增加
ALTER TABLE panes ADD COLUMN last_heartbeat_at DATETIME;
ALTER TABLE panes ADD COLUMN role TEXT DEFAULT 'executor';
ALTER TABLE panes ADD COLUMN status TEXT DEFAULT 'starting';
ALTER TABLE panes ADD COLUMN pid INTEGER;
```

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `tokio-util` 处理 NDJSON 效率最高 | Standard Stack | 引入额外依赖。 |
| A2 | 10 分钟闲置超时满足需求 | Summary | 用户可能会有长时间运行的任务导致 Master 意外退出（需通过心跳计数避免）。 |
| A3 | Windows 10 17063+ 已普及 | Environment | 极少数老旧系统用户无法使用。 |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | `cargo test` |
| Quick run command | `cargo test --lib ipc::server` |
| Full suite command | `cargo test` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command |
|--------|----------|-----------|-------------------|
| STAT-01 | SQLite 记录心跳并更新状态 | Integration | `cargo test test_db_persistence` |
| STAT-02 | Worker 通过 UDS 注册并发送心跳 | Integration | `cargo test test_ipc_handshake` |

### Wave 0 Gaps
- [ ] `tests/ipc_test.rs`: 模拟 Master/Worker 握手流程。
- [ ] `tests/db_test.rs`: 验证 Schema 迁移与并发写入。

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V5 Input Validation | yes | `serde_json` 严格模式解析 NDJSON，防止非法消息注入。 |
| V12 File System | yes | 确保 `.sock` 和 `.db` 文件权限仅限当前用户读写。 |

### Known Threat Patterns

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Local Privilege Escalation | Tampering | UDS 文件权限限制 (chmod 600 on Unix)。 |
| DoS via Socket Flooding | Denial of Service | Master 设置最大并发连接数限制。 |

## Sources

### Primary (HIGH confidence)
- [Official Docs] `tokio::net::UnixListener` - Windows 支持细节。
- [Official Docs] `rusqlite` - 线程安全性说明。
- [Codebase] `crates/vibe-core/src/os/windows.rs` - 现有 Job Object 实现。

### Metadata

**Confidence breakdown:**
- Standard stack: HIGH - 已验证 workspace 现有依赖。
- Architecture: HIGH - Master-Worker 为此类工具标准模式。
- Pitfalls: MEDIUM - Windows UDS 边缘情况需在实操中微调。

**Research date:** 2024-03-21
**Valid until:** 2024-04-20
