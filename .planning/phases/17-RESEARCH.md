# Phase 17: Bi-directional Flow & Reliability - Research

**Researched:** 2024-04-21
**Domain:** Terminal Input Injection & Reliability
**Confidence:** HIGH

## Summary

本研究旨在解决向 WezTerm 和 Tmux 中运行的交互式 CLI（如 Claude/Gemini）注入指令并触发提交的可靠性问题。核心挑战在于不同终端模拟器对“回车”键的处理差异，以及“括号粘贴模式”（Bracketed Paste Mode）对自动执行的阻碍。

**Primary recommendation:** 
统一使用 `\r` (Carriage Return) 作为指令提交触发符，并在注入大量文本时引入毫秒级延迟（Throttling）。对于 WezTerm，必须显式使用 `--no-paste` 选项或在粘贴后单独发送 `\r`。

## Standard Stack

### Core
| Library / Tool | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| WezTerm CLI | 20240203+ | 终端控制与文本注入 | 现代终端模拟器，支持丰富的 CLI 接口 |
| Tmux | 3.x | 终端复用与文本注入 | 行业标准的终端复用器，`send-keys` 功能强大 |
| Rust std::process | - | 调用外部 CLI 工具 | 提供跨平台的进程调用能力，避免 Shell 字符转义问题 |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|--------------|
| `nix` / `libc` | - | 获取 TTY 状态（可选） | 需要更底层的 TTY 控制时 |
| `tokio::time` | - | 异步限流延迟 | 在异步环境中实现字符注入限流 |

## Architecture Patterns

### Recommended Injection Pattern
1. **Throttling (限流)**: 将长文本拆分为块（如 64 字节），每块之间间隔 2-5ms。
2. **Encoding (编码)**: 始终使用 UTF-8。特殊字符直接作为原始字节流发送。
3. **Triggering (触发)**: 
   - 注入文本主体。
   - 注入 `\r` (0x0D) 以模拟按下回车键。
   - 在 Tmux 中，使用 `send-keys C-m` 或 `send-keys Enter`。

### Pattern 1: Raw Byte Injection (绕过 Shell)
不通过 `bash -c` 调用，而是直接使用 `Command::args` 传递参数，防止特殊字符被当前 Shell 解析。

### Anti-Patterns to Avoid
- **依赖 `\n`**: 在 Raw Mode (如 AI CLI) 中，`\n` (LF) 往往只下移一行而不触发提交。
- **全速注入**: 注入几千个字符而不加延迟可能导致目标 CLI 的输入缓冲区溢出或语法高亮引擎崩溃。
- **默认使用粘贴模式**: 在开启了 Bracketed Paste Mode 的 CLI 中，粘贴动作会自动包裹转义序列，导致末尾的换行符被视为普通字符而非执行指令。

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| 终端模拟 | 自研 PTY 驱动 | WezTerm/Tmux CLI | 处理 PTY 状态极其复杂，利用成熟工具的注入接口更稳定 |
| 键位映射 | 手动维护 keymap | Tmux `send-keys` 键名 | Tmux 已处理了大部分 Terminfo 兼容性问题 |

## Common Pitfalls

### Pitfall 1: Bracketed Paste Mode Blocking
**What goes wrong:** 注入文本后，CLI 只是显示了文本并换行，没有开始运行。
**Why it happens:** CLI 开启了括号粘贴模式（`\e[?200h`），终端模拟器会自动将注入文本包裹在 `\e[200~` 和 `\e[201~` 之间，这会导致内部的所有换行符失去“执行”语义。
**How to avoid:** 
- WezTerm: 使用 `--no-paste` 参数。
- Tmux: `send-keys` 默认不使用粘贴模式，但如果目标 CLI 正在等待粘贴，可能需要手动发送 `\r`。

### Pitfall 2: Input Buffer Overflow
**What goes wrong:** 注入的长文本出现乱序、丢失字符。
**Why it happens:** 交互式 CLI 可能在输入时执行复杂的计算（如实时语法检查），处理速度跟不上注入速度。
**How to avoid:** 引入 Throttling。

## Code Examples

### WezTerm 可靠注入 (Rust)
```rust
// 发送文本主体
Command::new("wezterm")
    .args(["cli", "send-text", "--no-paste", "--pane-id", target_id])
    .arg(text)
    .output()?;

// 单独发送回车触发提交
Command::new("wezterm")
    .args(["cli", "send-text", "--no-paste", "--pane-id", target_id])
    .arg("\r")
    .output()?;
```

### Tmux 字符限流注入 (Logic)
```rust
fn inject_with_throttle(target_id: &str, text: &str) {
    for chunk in text.as_bytes().chunks(64) {
        let chunk_str = String::from_utf8_lossy(chunk);
        Command::new("tmux")
            .args(["send-keys", "-t", target_id, "-l"])
            .arg(&*chunk_str)
            .output().unwrap();
        std::thread::sleep(Duration::from_millis(5));
    }
    Command::new("tmux")
        .args(["send-keys", "-t", target_id, "C-m"])
        .output().unwrap();
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| 发送 `\n` | 发送 `\r` | - | 提高 Raw Mode 程序的提交成功率 |
| 全速注入 | 带延迟注入 (Throttling) | - | 避免长文本丢失和 CLI 崩溃 |
| 依赖终端默认粘贴行为 | 显式控制 Paste 模式 | - | 解决括号粘贴模式下的执行阻塞问题 |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `\r` 在所有现代 AI CLI 中都能触发提交 | Summary | 某些特定 CLI 可能强制要求 `\n` 或 `\r\n` |
| A2 | 5ms 延迟足以覆盖大部分 CLI 处理开销 | Throttling | 极慢的 CLI 可能仍会丢失数据 |

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| WezTerm | 终端适配层 | ✓ | 20240203 | — |
| Tmux | 终端适配层 | ✗ | — | 提示用户安装或仅使用 WezTerm |
| Bash | 脚本执行 | ✓ | 5.2.26 | — |

**Missing dependencies with no fallback:**
- **Tmux**: 当前环境未检测到 `tmux`。如果用户需要使用 Tmux 适配器，必须先安装。

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` |
| Config file | `Cargo.toml` |
| Quick run command | `cargo test --lib adapter` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command |
|--------|----------|-----------|-------------------|
| REL-01 | 发送 `\r` 能触发提交 | Integration | 需要运行中的交互式 CLI 进行冒烟测试 |
| REL-02 | 注入特殊字符不乱码 | Unit | `cargo test adapter::encoding` |
| REL-03 | 限流逻辑有效 | Unit | `cargo test adapter::throttle` |

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V5 Input Validation | yes | 对注入文本进行清理，防止注入终端转义序列攻击 (Terminal Escape Injection) |

### Known Threat Patterns

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Escape Sequence Injection | Tampering | 过滤或转义输入中的 `\e` (ESC) 等控制字符，除非显式允许 |

## Sources

### Primary (HIGH confidence)
- WezTerm Documentation (`wezterm cli send-text --help`)
- Tmux Manual (`man tmux`)
- Bracketed Paste Mode Specification (XTerm / terminal-guide)

### Secondary (MEDIUM confidence)
- Community discussions on StackOverflow regarding `tmux send-keys` reliability.
- Blog posts on terminal raw mode and line endings.
