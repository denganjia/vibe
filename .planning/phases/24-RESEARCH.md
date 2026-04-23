# Phase 24: Release Summary and CLI Slimming - Research

**Researched:** 2026-04-23
**Domain:** Git Automation, Node.js Scripting, Rust CLI Refactoring
**Confidence:** HIGH

## Summary

Phase 24 旨在实现自动化的 Release 总结功能，并对现有的 Rust CLI 进行“瘦身”，使其更符合 v6.0 的插件化架构。Release 总结将基于 Git 历史，利用 Node.js 脚本通过 `child_process` 调用 `git` 命令，并结合启发式算法识别 Conventional Commits。Rust CLI 的瘦身策略则是将复杂的业务逻辑（如 `spawn` 编排）归档，保留核心的跨平台适配器和轻量级的命令分发逻辑。

**Primary recommendation:** 使用 Node.js `execSync` 配合 `git log` 和 `git diff --numstat` 实现零依赖的 Release 总结脚本，并将 Rust `main.rs` 重构为简单的命令分发器。

## User Constraints (from 24-CONTEXT.md)

### Locked Decisions
- **分类策略**：采用启发式 Conventional Commits 识别（feat, fix, docs, test, refactor, chore 等）。
- **范围自动识别**：默认 `latest tag` -> `HEAD`，回退至首个 commit。支持手动覆盖。
- **变更关联性**：生成 `.vibe/RELEASE_DRAFT.md`，包含分类 Commit、作者、文件统计，并尝试关联 `.vibe/tasks/`。
- **数据导出**：支持 `--json` 参数。
- **Rust 保留边界**：保留作为轻量级跨平台二进制分发入口，保留核心命令分发逻辑。
- **归档/移除范围**：移除 `vibe-core` 中冗余逻辑，归档旧的 `vibe spawn` 等复杂编排代码至 `archive/`。
- **脚本化替代**：确保移除功能在 `plugin/vibe/scripts/` 中有替代方案。
- **GitHub 交互**：本地生成为主，不直接调用 API。

### the agent's Discretion
- 启发式匹配的关键词定义。
- `archive/` 目录的具体组织形式。
- `release-summary.js` 的内部实现细节。

### Deferred Ideas (OUT OF SCOPE)
- AI 自动润色总结（除非用户显式要求）。
- 自动发布到 GitHub Release。

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Node.js | v22+ | 脚本执行环境 | 插件系统默认环境 [VERIFIED: local env] |
| Git CLI | 2.49+ | 版本控制交互 | 事实来源 [VERIFIED: local env] |
| Rust | 2024 | 二进制分发入口 | 高性能跨平台入口 [VERIFIED: Cargo.toml] |

### Supporting (Node.js Internal)
| Module | Purpose | When to Use |
|---------|---------|-------------|
| `child_process` | 调用 git 命令 | `execSync` 适合同步获取结果 |
| `fs`, `path` | 文件操作与路径处理 | 读取 `.vibe/tasks/` 及写入 Markdown |

## Architecture Patterns

### Recommended Project Structure
```
.vibe/
├── RELEASE_DRAFT.md      # 生成的发布草稿
└── tasks/                # 任务关联来源
archive/                  # 归档的旧 Rust 代码
├── vibe-cli/             # 旧 main.rs 逻辑
└── vibe-core/            # 冗余的 Bus/State 逻辑
plugin/vibe/
├── scripts/
│   └── release-summary.js # 核心逻辑脚本
└── commands/
    └── release-summary.md # 命令合约
```

### Pattern 1: Heuristic Conventional Commits
使用正则表达式捕获标准格式，对不符合格式的提交，通过关键词匹配分类。

**Example Heuristics:**
- `fix`, `solve`, `bug` -> `fix`
- `add`, `feat`, `new` -> `feat`
- `doc`, `readme` -> `docs`
- `refactor`, `clean` -> `refactor`
- `test`, `spec` -> `test`

### Anti-Patterns to Avoid
- **Hard-coding Tag Names:** 必须动态检测最新的 tag。
- **Large Git Buffers:** 对极大型仓库，`execSync` 可能因 buffer 限制失效，应考虑 `spawn` 或设置 `maxBuffer`。

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Git API client | 自建 REST 请求 | Git CLI | 避免认证复杂性，保持本地运行。 |
| Commit Parser | 复杂的 AST 解析 | Regex + Keyword Map | 启发式需求更适合简单的字符串匹配。 |

## Runtime State Inventory

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | `.vibe/state/panes.json` | 确保瘦身后的 CLI 仍能读写此文件（或改为调用脚本处理）。 |
| Secrets/env vars | `VIBE_MASTER_ID` | 脚本需要感知此变量以保持兼容。 |
| Build artifacts | `target/release/vibe` | 瘦身后的构建产物应显著减小。 |

## Common Pitfalls

### Pitfall 1: No Tags in Repository
**What goes wrong:** `git describe --tags --abbrev=0` 报错。
**How to avoid:** 使用 `try-catch` 捕获异常，回退到首个 commit (`git rev-list --max-parents=0 HEAD`)。

### Pitfall 2: Conventional Commits with Multiple Scopes
**What goes wrong:** 正则表达式可能无法正确捕获复杂的 scope。
**How to avoid:** 使用非贪婪匹配，或仅保留 type 和 subject 的核心匹配。

## Code Examples

### Git Log Extraction (Node.js)
```javascript
// Source: [CITED: Node.js Documentation / Git Manual]
const { execSync } = require('child_process');

function getGitLog(from, to) {
  const range = from ? `${from}..${to || 'HEAD'}` : '';
  const logFormat = '%h|%an|%s';
  const cmd = `git log ${range} --pretty=format:"${logFormat}"`;
  return execSync(cmd, { encoding: 'utf8' }).trim().split('\n');
}

function getDiffStat(from, to) {
  const range = from ? `${from}..${to || 'HEAD'}` : '';
  return execSync(`git diff --numstat ${range}`, { encoding: 'utf8' }).trim();
}
```

### Heuristic Parsing
```javascript
const CATEGORIES = {
  feat: ['feat', 'add', 'new'],
  fix: ['fix', 'solve', 'bug', 'patch'],
  docs: ['docs', 'readme', 'guide'],
  refactor: ['refactor', 'clean', 'rename'],
  test: ['test', 'spec'],
  chore: ['chore', 'bump', 'deps']
};

function categorize(message) {
  const standardMatch = message.match(/^(feat|fix|docs|test|refactor|chore)(?:\(.*\))?:/i);
  if (standardMatch) return standardMatch[1].toLowerCase();

  const lower = message.toLowerCase();
  for (const [cat, keywords] of Object.entries(CATEGORIES)) {
    if (keywords.some(k => lower.includes(k))) return cat;
  }
  return 'internal';
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Rust-heavy orchestration | Node.js scripted logic | Phase 24 | 更易于热更新和扩展，减少二进制体积。 |
| Manual Changelog | Automated Heuristic Summary | Phase 24 | 提高发布效率，减少人为遗漏。 |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Node.js exists on user machine | Standard Stack | 瘦身后的 Rust CLI 依赖 Node.js 执行脚本。 |
| A2 | .vibe/state/panes.json 格式稳定 | Runtime State | 如果格式变化，Rust 的 PaneRecord 结构需同步更新。 |

## Open Questions

1. **Dispatcher Implementation**: 是否需要 Rust CLI 静态链接某些脚本，还是完全通过文件系统调用？
   - Recommendation: 优先通过 `plugin/vibe/scripts/` 调用，方便用户自定义。

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | Script execution | ✓ | v22.16.0 | — |
| Git | History extraction | ✓ | 2.49.0 | — |
| Cargo | CLI building | ✓ | 1.85.0 | — |

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Jest (for scripts), Cargo Test (for Rust) |
| Quick run command | `node plugin/vibe/scripts/release-summary.test.js` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| REQ-RS-01 | Git log extraction | unit | `npm test` | ❌ Wave 0 |
| REQ-RS-02 | Heuristic categorization | unit | `npm test` | ❌ Wave 0 |
| REQ-SLIM-01 | Dispatch logic | integration | `cargo test` | ❌ Wave 0 |

## Sources

### Primary (HIGH confidence)
- `24-CONTEXT.md` - Phase goals and locked decisions.
- `main.rs` - Current Rust implementation.
- Node.js `child_process` documentation.

### Secondary (MEDIUM confidence)
- Conventional Commits specification.
