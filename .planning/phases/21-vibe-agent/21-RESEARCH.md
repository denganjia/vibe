# Phase 21: `.vibe` 工作区与 Agent 定义 - Research

**Researched:** 2024-05
**Domain:** 配置管理、工作区初始化、JSON Schema
**Confidence:** HIGH

## Summary

本阶段主要在项目中确立 `.vibe` 工作区的物理结构及 Agent 定义的标准规范。根据讨论和约束，弃用原有混合了 Markdown 和 YAML 的复杂配置（`roles/*.md`），转而拥抱纯 JSON 格式（`agents/*.json`），以便于后续阶段的各种轻量级脚本能够不借助复杂的第三方库即可读取。

**Primary recommendation:** 使用 `tokio::fs` 和 `serde_json` 在初始化命令（如 `vibe init`）中实现结构化的非破坏性目录与配置生成，完全放弃向下兼容（即不自动迁移旧配置文件），将新架构彻底切入 JSON-first。

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- 采用 **纯 JSON 格式 (Pure JSON)** 存储 Agent 定义，方便 Phase 22 脚本读取。
- Agent 目录变更为全小写的 `.vibe/agents/`。
- 初始化过程必须是非破坏性的（non-destructive），通过 plugin-level script 实现。
- Legacy Config（如包含 `roles` 映射的旧版 `.vibe/config.json`）的迁移采用 **严格/手动迁移 (Strict/Manual Migration)**，`init` 不会自动转换，以免破坏工作流。
- `.vibe/config.json` schema 改为嵌套结构（如 `concurrency.max_parallel_tasks`, `runtime.scripts`），舍弃扁平的 `roles`。
- Agent 定义 (`.vibe/agents/*.json`) 必须显式包含 `model_command`（可执行 shell 命令，如 `gemini -y`）。

### the agent's Discretion
- (无) - 格式、路径及迁移策略均已明确锁定。

### Deferred Ideas (OUT OF SCOPE)
- 自动读取或转换旧的 Markdown (`.vibe/roles/*.md`) 和旧版 config（用户需手动迁移）。
</user_constraints>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `serde` | 1.0 | 序列化/反序列化基础框架 | Rust 事实上的序列化标准，项目中已包含 |
| `serde_json` | 1.0 | JSON 解析和生成 | 满足纯 JSON Agent 定义文件的生成和读取要求 |
| `tokio::fs` | 1.x | 异步文件 I/O | 在非阻塞环境中安全地进行目录和文件创建 |
| `std::path::PathBuf` | N/A | 跨平台路径构建 | 确保在 Win/Mac/Linux 上安全构建工作区路径 |

## Architecture Patterns

### Recommended Project Structure
```
.vibe/
├── agents/             # Agent 角色定义目录 (Pure JSON)
│   ├── planner.json
│   ├── executor.json
│   └── reviewer.json
├── tasks/              # 任务数据存储
├── runs/               # 执行记录存储
├── locks/              # 锁策略和锁定文件
├── reviews/            # Review 数据存放
├── logs/               # 协作日志和执行日志
└── config.json         # 新版嵌套结构的配置，替代原有的扁平配置
```

### Pattern 1: Non-Destructive Initialization (非破坏性初始化)
**What:** 使用存在性检查确保只有在目录或文件不存在时才进行创建。
**When to use:** 在触发工作区初始化操作时。
**Example:**
```rust
use std::path::Path;
use tokio::fs;

pub async fn init_workspace(base_dir: &Path, force: bool) -> anyhow::Result<()> {
    let dirs = ["agents", "tasks", "runs", "locks", "reviews", "logs"];
    let vibe_dir = base_dir.join(".vibe");
    
    if !vibe_dir.exists() {
        fs::create_dir_all(&vibe_dir).await?;
    }

    for dir in dirs {
        let dir_path = vibe_dir.join(dir);
        if !dir_path.exists() {
            fs::create_dir(&dir_path).await?;
        }
    }
    
    let config_path = vibe_dir.join("config.json");
    if !config_path.exists() || force {
        let default_config = serde_json::json!({
            "concurrency": { "max_parallel_tasks": 1 },
            "runtime": { "scripts": true }
        });
        fs::write(&config_path, serde_json::to_string_pretty(&default_config)?).await?;
    }
    Ok(())
}
```

### Anti-Patterns to Avoid
- **破坏性覆盖:** 绝不能在未携带 `--force` 标志的情况下覆盖用户现存的 `.vibe/config.json` 或 `.vibe/agents/*.json`。
- **兼容性胶水代码:** 不要在 `init` 或加载逻辑中混入针对旧版 `roles` 的解析代码，严格遵守手动迁移约束。

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JSON 数据组装 | 字符串拼接 (`format!`) | `serde_json::json!` 宏 | 自动处理引号转义及嵌套结构的缩进问题 |
| 跨平台路径拼接 | 字符串连接 (`dir + "/" + file`) | `Path::join` | 避免由于分隔符不一致导致的 Windows/Unix 差异 |

## Runtime State Inventory

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | 旧版 `.vibe/roles/` 目录和其中的 Markdown 文件 | **Manual Migration:** 维持原样，不在代码中自动迁移 |
| Live service config | None — verified by standard stack | None |
| OS-registered state | None — verified by standard stack | None |
| Secrets/env vars | None — verified by standard stack | None |
| Build artifacts | 可能存在使用旧 Schema 格式的 `.vibe/config.json` | **Manual Migration:** 初始化时跳过已存在的文件 |

## Common Pitfalls

### Pitfall 1: 不完整的 `model_command` 导致下游崩溃
**What goes wrong:** 后续的运行时（Phase 22）依赖 Agent 里的 `model_command` 作为 Subprocess 触发，如果不强制验证此字段，整个 plugin 执行管线会中断。
**Why it happens:** JSON 中缺少必须的键或拼写错误。
**How to avoid:** 使用强类型的 Rust Struct 配合 `serde` 来承载默认的内置模板数据。
**Warning signs:** JSON 出现未定义的动态字典格式而非静态 Schema。

## Code Examples

### JSON Agent 强类型定义模板
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentDefinition {
    pub id: String,
    pub model_command: String,
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub expected_output: String,
    #[serde(default)]
    pub description: String,
}

// 可在初始化时提供 default 实现，快速 dump 出 JSON
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `roles/*.md` 带有 YAML Frontmatter | `agents/*.json` 纯 JSON | Phase 21 | 后续通过 bash/js/python 等脚本提取执行命令时无需 YAML 解析器 |
| Flat `roles` 对象 | Nested 结构化配置 | Phase 21 | 配置层次更分明，便于加入 concurrency 等隔离项 |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Agent 将以纯 JSON 形式在 `.vibe/agents/` 目录中持久化。 | User Constraints | 如果不一致，后续执行器解析失败。 |
| A2 | 弃用 Markdown Agent，且不需要后向兼容/自动迁移代码。 | User Constraints | 用户必须查阅日志手动迁移配置，否则原功能失效。 |

## Open Questions (RESOLVED)

1. **[Force Override Behavior]**
   - What we know: 除非显式 force，否则不会覆盖。
   - What's unclear: 具体的 `--force` 触发途径以及覆盖的边界（是仅覆盖缺失项，还是整体清空目录重建）。
   - Recommendation: 建议 `--force` 仅针对 `config.json` 及核心内置代理 (`planner.json` 等) 进行强制文件复写，不删除用户自定义的其他 `*.json` Agent。
   - **RESOLVED:** `--force` 标志仅强制覆盖 `config.json` 和核心内置代理（如 `planner.json`, `executor.json`, `reviewer.json`, `release.json`），绝不会删除或覆盖用户自定义的其他 `*.json` Agent。

## Environment Availability

Step 2.6: SKIPPED (no external dependencies identified)
