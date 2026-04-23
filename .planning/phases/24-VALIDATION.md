# Phase 24 Validation Architecture

**Phase:** 24 - Release 总结与 CLI 瘦身收束
**Role:** Nyquist Validator
**Standard:** Automated Unit & Integration Testing

## Overview
第 24 阶段的验证重点是 Release 总结脚本的分类准确性、任务关联的正确性，以及 Rust CLI 瘦身后命令分发的可靠性。我们将结合 Node.js 单元测试和 Rust 集成测试来确保系统闭环。

## Test Strategy

### 1. Script Unit Tests (`release-summary.test.js`)
- **T1: Heuristic Categorization**: 验证各种格式的 commit message（标准 Conventional、模糊匹配、无匹配）是否被正确归类。
- **T2: Range Detection**: 验证在有/无 Tag 的情况下，脚本是否能正确确定 Git 日志范围。
- **T3: Task Association**: 模拟包含 `(task: <id>)` 的 commit，验证脚本是否能正确读取 `.vibe/tasks/` 下的 JSON 并提取目标信息。

### 2. Rust Dispatcher Integration Tests
- **T4: Command Forwarding**: 验证运行 `vibe release-summary` 是否能正确触发底层 Node.js 脚本。
- **T5: Error Propagation**: 验证当脚本退出码非 0 时，Rust CLI 是否能正确捕获并显示错误信息。

### 3. Archive Verification
- **T6: Code Presence**: 验证旧代码是否已被正确移动至 `archive/` 且不再参与主构建过程。
- **T7: Build Integrity**: 确保瘦身后的项目仍能成功编译。

## Truths to Verify

| Truth | Verification Method |
|-------|---------------------|
| 脚本能准确将 commit 分类（含 fuzzy 匹配）并归类不匹配项为 "Internal Changes"。 | `release-summary.test.js` (T1) |
| 包含 `(task: id)` 的 commit 能在总结中显示对应的任务目标。 | `release-summary.test.js` (T3) |
| `vibe release-summary` 命令由 Rust 入口成功分发至 JS 脚本。 | Manual / Dispatch Test (T4) |
| 项目在移除冗余 `vibe-core` 逻辑后仍能正常通过编译。 | `cargo build` (T7) |

## Artifact Check
- [ ] `.vibe/RELEASE_DRAFT.md` 文件内容格式符合 GitHub 规范。
- [ ] `archive/` 目录下包含预期的旧代码文件。
- [ ] 瘦身后可执行文件体积有可感知的减小（可选，定性检查）。
