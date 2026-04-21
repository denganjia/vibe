# Phase 18: Standardization & Init - Research

**Researched:** 2024-03-24
**Domain:** Rust CLI UX, Configuration Management, Terminal Adapters
**Confidence:** HIGH

## Summary

本阶段的研究重点在于提升 `vibe-cli` 的易用性和健壮性。通过引入交互式向导（`vibe init`），我们可以引导用户完成初始配置并自动探测环境；通过增强终端适配器，我们可以实现静默的状态清理，解决 Vibe ID 与物理 Pane ID 不一致的问题；通过支持 `stacks` 配置，我们可以实现一键批量启动多个代理。

**Primary recommendation:** 使用 `dialoguer` 构建交互式向导，并在 `TerminalAdapter` 特性中增加 `list_all_ids` 接口以支持自动状态清理。

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `dialoguer` | 0.11 | 交互式命令行提示 | Rust 生态中功能最全、易用性最好的交互库 [VERIFIED: Cargo.toml] |
| `which` | 6.0 | 探测系统 PATH 中的二进制文件 | 跨平台且简单可靠 [VERIFIED: Cargo.toml] |
| `serde_json` | 1.0 | 处理配置文件和终端输出 | 强类型解析与灵活的 Value 操作结合 [VERIFIED: Cargo.toml] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|--------------|
| `console` | 0.15 | 终端颜色和风格控制 | 与 `dialoguer` 配合使用，提升向导视觉体验 [ASSUMED] |

## Architecture Patterns

### Pattern 1: Recursive Config Merge
为了支持配置升级（例如增加 `stacks` 字段）而不破坏用户现有的 `roles` 配置，应采用 "Default-first" 合并模式。
1. 将当前代码中的 `ProjectConfig::default()` 转换为 `serde_json::Value`。
2. 将用户磁盘上的 `config.json` 解析为 `serde_json::Value`。
3. 递归合并：如果 Key 在用户配置中存在，保留用户值；如果不存在，使用默认值。

### Pattern 2: Silent State Cleanup
在执行 `vibe list` 或 `vibe spawn` 之前，自动执行清理逻辑：
```rust
let active_physical_ids = adapter.list_all_ids()?;
let mut store = StateStore::new()?;
let saved_panes = store.list_active_panes()?;

for pane in saved_panes {
    if !active_physical_ids.contains(&pane.physical_id) {
        store.remove_pane(&pane.vibe_id)?;
    }
}
```

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| PATH 探测 | 手动分割 `env::var("PATH")` | `which` crate | 处理平台差异（如 .exe 后缀）和复杂的 PATH 逻辑 |
| 交互提示 | 手动读取 stdin 和处理 ANSI 码 | `dialoguer` | 处理退格、箭头选择、颜色主题等极度繁琐的终端交互 |

## Common Pitfalls

### Pitfall 1: WezTerm CLI Environment
**What goes wrong:** `wezterm cli list` 在非 WezTerm 窗口中运行时可能会失败或返回空结果。
**Why it happens:** CLI 需要连接到正在运行的 WezTerm 服务。
**How to avoid:** 在调用前检测 `WEZTERM_PANE` 环境变量，如果不存在，给用户明确的警告。

### Pitfall 2: Sequential Spawn Race
**What goes wrong:** 快速连续创建多个 Tab 时，第一个 Tab 还没完全初始化，第二个就由于竞争资源导致 ID 分配混乱。
**Why it happens:** 终端模拟器处理创建请求可能存在异步延迟。
**How to avoid:** 在批量创建循环中，每个 `spawn` 后保留现有的 1-2 秒等待时间，并确保上一个 `spawn` 成功返回物理 ID 后再进行下一个。

## Code Examples

### 1. Dialoguer Select with PATH Probe
```rust
use dialoguer::{theme::ColorfulTheme, Select};
use which::which;

let clis = ["claude", "gemini", "codex"];
let found_clis: Vec<&str> = clis.iter()
    .filter(|&cli| which(cli).is_ok())
    .map(|&s| s)
    .collect();

if !found_clis.is_empty() {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("检测到以下 AI CLI，请选择默认使用的工具")
        .items(&found_clis)
        .default(0)
        .interact()?;
    println!("Selected: {}", found_clis[selection]);
}
```

### 2. Deep Merge for Config Upgrade
```rust
fn deep_merge(target: &mut serde_json::Value, source: serde_json::Value) {
    match (target, source) {
        (serde_json::Value::Object(t), serde_json::Value::Object(s)) => {
            for (k, v) in s {
                deep_merge(t.entry(k).or_insert(serde_json::Value::Null), v);
            }
        }
        (t, s) => *t = s,
    }
}
```

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| wezterm | Terminal Adapter | ✓ | 20240203 | Tmux |
| tmux | Terminal Adapter | ✗ | — | WezTerm |

**Missing dependencies with fallback:**
- `tmux`: 在当前环境中未找到，但 `vibe-cli` 支持 `wezterm` 作为替代。

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` |
| Quick run command | `cargo test --lib state::tests` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| INIT-01 | `vibe init` 不破坏现有配置 | unit | `cargo test test_config_merge` | ❌ (Need Wave 0) |
| CLEAN-01 | 静默清理过时记录 | unit | `cargo test test_state_cleanup` | ❌ (Need Wave 0) |

## Sources

### Primary (HIGH confidence)
- `crates/vibe-core/src/adapter/wezterm.rs` - 确认了 WezTerm CLI 的调用方式。
- `Cargo.toml` - 确认了 `dialoguer` 和 `which` 的版本。
- 本地执行 `wezterm cli list --format json` - 验证了返回数据结构。

### Secondary (MEDIUM confidence)
- `dialoguer` 官方示例文档 - 用于构建交互向导代码模式。

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Libraries already in workspace.
- Architecture: HIGH - Patterns verified against existing core code.
- Pitfalls: MEDIUM - Based on common CLI development experience.

**Research date:** 2024-03-24
**Valid until:** 2024-04-24
