# Phase 24: Release 总结与 CLI 瘦身收束 - Context

**Status:** DECIDED
**Last Updated:** 2026-04-23
**Phase Goal:** 实现基于 Git 历史的自动化 Release 总结，并按 v6.0 规划对旧 Rust CLI 进行瘦身或归档。

## Locked Decisions

### 1. Release 总结规则 (Release Summary Rules)
- **分类策略**：采用启发式 Conventional Commits 识别。
    - 匹配前缀：`feat`, `fix`, `docs`, `test`, `refactor`, `chore`。
    - 模糊匹配：对无前缀消息进行关键词识别，无法匹配的归类为 `Internal Changes`。
- **范围自动识别**：
    - 默认范围：`latest tag` -> `HEAD`。
    - 回退方案：无 tag 时从首个 commit 开始。
    - 手动覆盖：支持 `--from <commit/tag>` 和 `--to <commit/tag>`。

### 2. 变更关联性 (Linkage)
- **核心产物**：默认生成 `.vibe/RELEASE_DRAFT.md` (GitHub 兼容 Markdown)。
- **包含内容**：
    - 分类后的 Commit 列表及作者。
    - 变更文件统计 (Changed Files summary)。
    - Task 关联：若消息中含 `(task: <id>)`，则尝试关联 `.vibe/tasks/` 中的目标。
- **数据导出**：支持 `--json` 参数输出结构化数据。

### 3. CLI 瘦身与归档策略 (CLI Slimming & Archiving Strategy)
- **Rust 保留边界**：
    - 保留作为轻量级跨平台二进制分发入口的功能。
    - 保留核心 CLI 命令的分发 (Dispatch) 逻辑（调用 plugin scripts）。
- **归档/移除范围**：
    - 移除 `vibe-core` 中冗余的 Bus、State (SQLite 遗留) 和 Role (旧 Markdown 格式) 管理逻辑。
    - 归档旧的 `vibe spawn` 等复杂编排代码至 `archive/` 目录（或从主分支移除）。
- **脚本化替代**：确保所有被移除的功能在 `plugin/vibe/scripts/` 中都有对应的轻量级替代方案。

### 4. 命令集成与 GitHub 交互
- **命令形态**：通过 Plugin Command 暴露，底层执行 Node.js 脚本。
- **GitHub 策略**：**本地生成为主**。
    - 脚本不直接调用 GitHub API。
    - 用户需手动复制 Markdown 内容，或通过 shell 管道配合 `gh release create` 使用。

## User Constraints
- 总结必须是确定性的，不依赖 AI 模型的概率性推断（除非用户显式要求 AI 润色总结）。
- 瘦身过程不得破坏 Plugin 执行脚本所需的基础环境。

## Next Steps
1. **Phase 24 Research**: 研究 Git 命令的 Node.js 调用（`child_process`）及 Commit 消息解析的最佳实践。
2. **Phase 24 Planning**: 制定具体的 Release 脚本编写及 Rust 代码清理计划。
