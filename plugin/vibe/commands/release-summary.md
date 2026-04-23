# vibe release-summary

## Command Contract Only

This file documents the intended workflow contract and does not claim executable Codex command binding.

## Purpose

自动化生成高质量的 Release Summary，支持基于 Conventional Commits 和启发式规则的分类，并关联 Vibe 任务信息。

## Usage

```bash
node plugin/vibe/scripts/release-summary.js [--from <ref>] [--to <ref>] [--json]
```

## Inputs

- `--from`: 起始提交、标签或引用。如果省略，则尝试寻找最近的标签；若无标签则从首个提交开始。
- `--to`: 结束提交、标签或引用。默认为 `HEAD`。
- `--json`: 以 JSON 格式输出结果，而不是生成 Markdown 文件。

## Reads

- 本地 Git 历史记录。
- `.vibe/tasks/`: 尝试根据 `(task: <id>)` 模式读取任务 JSON 文件以提取标题和状态。

## Writes

- `.vibe/RELEASE_DRAFT.md`: 默认生成的 Markdown 格式 Release Summary。
- stdout: 运行状态信息或 JSON 输出。

## Categorization Logic

脚本采用启发式分类：
1. **标准 CC 匹配**: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`。
2. **模糊匹配**: 
   - `feat`: add, new, feat, feature, introduce
   - `fix`: fix, bug, patch, resolve, hotfix
   - `docs`: doc, readme, changelog
   - `test`: test, spec, e2e
   - `refactor`: refactor, cleanup, clean
   - `chore`: chore, deps, build, ci
3. **回退方案**: 归类为 `Internal Changes`。

## Expected Output

- 包含按类别（Features, Bug Fixes, Documentation 等）分组的提交列表。
- 每个提交如果关联了任务，会显示任务 ID 和标题。
- 最终生成的 `.vibe/RELEASE_DRAFT.md` 文件路径。
