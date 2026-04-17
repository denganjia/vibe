# AI Agent Bus Implementation Research

**Domain:** AI Agent Coordination & IPC
**Researched:** 2024-10-27
**Overall confidence:** HIGH

## Executive Summary

本文档详细介绍了将 `vibe-cli` 演进为“AI 代理总线 (AI Agent Bus)”的核心技术实现方案。重点解决跨进程的异步信号机制（Signal/Wait）以及子进程的角色注入（Role Injection）。

核心结论是：
1. **Signal/Wait**: 利用现有的 NDJSON Over UDS 协议，在服务端实现基于 `DashMap` 的连接挂起机制。
2. **Role Injection**: 通过 `tokio::process` 的管道重定向，将 `.vibe/roles/` 下的 Markdown 提示词注入子进程 stdin，并利用 `tokio::io::copy` 实现实时终端透传。
3. **.vibe 目录**: 确立为项目的“分布式大脑”，存储角色模板、总线配置及临时状态。

---

## 1. UDS Signaling (Signal/Wait)

### 1.1 协议扩展
在 `crates/vibe-core/src/ipc/protocol.rs` 中增加以下消息类型：

```rust
pub enum Message {
    // ... 现有消息 ...
    
    /// 阻塞等待信号
    Wait {
        signal_id: String,
        timeout_ms: Option<u64>,
    },
    
    /// 发送信号并携带负载
    Signal {
        signal_id: String,
        payload: serde_json::Value,
    },
    
    /// 信号响应（由服务端发回给 Waiter）
    SignalFired {
        signal_id: String,
        payload: serde_json::Value,
    },
}
```

### 1.2 服务端实现模式
服务端（vibe daemon）需要维护一个“等待者”名录：

```rust
type Waiters = DashMap<String, Vec<tokio::sync::oneshot::Sender<serde_json::Value>>>;

// 当收到 Wait 消息时：
let (tx, rx) = oneshot::channel();
waiters.entry(signal_id).or_default().push(tx);
let payload = rx.await?;
send_to_client(Message::SignalFired { signal_id, payload });

// 当收到 Signal 消息时：
if let Some((_, txs)) = waiters.remove(&signal_id) {
    for tx in txs {
        let _ = tx.send(payload.clone());
    }
}
```

### 1.3 关键点：UDS 路径限制
- **坑点**: UDS 路径在 Linux/macOS 上限制为 104-108 字节。直接放在 `.vibe/state/bus.sock` 极易因项目目录过深而报错。
- **方案**: 推荐将 Socket 放在 `/tmp/vibe-<project_hash>.sock`，并在 `.vibe/state/bus_path` 中记录该路径，或通过环境变量透传。

---

## 2. Role Injection (角色注入)

### 2.1 Stdin 注入模式
对于 `vibe spawn --role <ROLE>` 启动的子进程，按以下步骤注入 persona：

1. **读取模板**: 从 `.vibe/roles/<role>.md` 读取角色定义。
2. **启动进程**:
   ```rust
   let mut child = Command::new(agent_cmd)
       .stdin(Stdio::piped())
       .stdout(Stdio::inherit())
       .stderr(Stdio::inherit())
       .spawn()?;
   ```
3. **写入指令**:
   ```rust
   let mut stdin = child.stdin.take().unwrap();
   stdin.write_all(role_content.as_bytes()).await?;
   stdin.write_all(b"\n\nSystem: You are now initialized. Awaiting user input...\n").await?;
   stdin.flush().await?;
   ```
4. **接管交互**: 
   若为交互式 Agent（如 Terminal），需将系统 stdin 桥接到子进程：
   ```rust
   let mut system_stdin = tokio::io::stdin();
   tokio::io::copy(&mut system_stdin, &mut stdin).await?;
   ```

### 2.2 交互式 Agent 的 TTY 问题
- 如果子进程是 `gh ssh` 或 `vim` 等需要 TTY 的程序，简单的管道重定向会导致终端异常。
- **建议**: 在 Milestone 4.0 中，对于交互式 Agent，优先使用 `wezterm/tmux` 提供的 `send-text` 命令，而非直接操作 stdin 管道，以保持 TTY 特性。
- **代码参考**: `vibe-core/src/adapter/wezterm.rs` 的 `send_text` 方法。

---

## 3. .vibe 目录最佳实践

### 3.1 推荐结构
```text
.vibe/
├── roles/               # 角色定义 (Markdown)
│   ├── developer.md     # 核心开发者角色
│   └── reviewer.md      # 代码审查角色
├── config.toml          # 总线配置（超时时间、默认模型等）
└── state/               # 运行时状态（.gitignore 忽略）
    ├── active_context.md # 当前任务进度
    └── bus_info.json    # 记录当前 Daemon 的 PID 与 Socket 路径
```

### 3.2 Git 策略
- **应提交**: `roles/`, `config.toml`。确保团队成员共享相同的 Agent 逻辑。
- **应忽略**: `state/` 目录。

---

## 4. 架构优势与坑点 (Pitfalls)

### 4.1 优势
- **极简**: 无需 SQLite 锁竞争，UDS 本身保证了顺序性。
- **透明**: AI 代理可以直接看到 `.vibe` 里的文件，具备“自我管理”意识。

### 4.2 潜在坑点
| 坑点 | 现象 | 防范 |
|------|------|------|
| **Zombie Waiters** | 客户端崩溃导致服务端挂起的 Waiter 永不释放。 | 服务端实现 `Heartbeat` 检查，并为 `Wait` 设置强制超时。 |
| **Stdin Race** | 在注入 Role 之前，子进程可能已经开始读取 stdin。 | 确保 `spawn` 后立即写入，或使用 `sleep` 微调（不推荐）。 |
| **UDS Path limit** | `ENAMETOOLONG` 错误。 | 使用 `/tmp` 路径 + 符号链接，或 `chdir` 到目标目录后 bind。 |

## 5. 结论

`vibe-cli` 的 Agent Bus 模式应当通过 **挂起 UDS 连接** 来模拟信号同步，通过 **Stdin 管道预热** 来实现角色注入。该方案保持了 Rust 异步 IO 的高性能，同时避免了外部数据库的复杂性。

---
**Sources:**
- [Tokio Documentation (Process/Net)](https://tokio.rs/tokio/tutorial/io)
- [Unix Domain Socket Path Limits](https://man7.org/linux/man-pages/man7/unix.7.html)
- [Personal Experience: AI Agent Prompt Injection Patterns]
