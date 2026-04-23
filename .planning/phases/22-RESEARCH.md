# Phase 22: 轻量 scripts runtime - Research

**Researched:** 2026-04-23
**Domain:** Node.js Runtime, Subprocess Management, File Locking, Task Lifecycle
**Confidence:** HIGH

## Summary

本阶段的核心是实现一组确定性、轻量级的 Node.js 脚本，作为 Vibe Plugin 的运行时原语。这些脚本将替代原先由 Rust CLI 负责的任务管理、并发锁定和 Agent 进程启动等核心功能。

脚本必须遵循“确定性”原则：它们只负责执行 `.vibe` 工作区文件定义的规则，而不参与任何策略性决策（如 Agent 选择、任务优先级等）。

## Requirements Mapping (from ROADMAP.md)

| Requirement | Implementation Detail | Script |
|-------------|-----------------------|--------|
| RUN-01: Create Task JSON | 提供标准化接口，根据 Conductor 输入生成符合 Task Contract 的 JSON 文件。 | `task.js` |
| RUN-02: File Locking | 基于文件的锁定机制，防止多个 Agent 同时修改同一文件作用域 (`file_scope`)。 | `lock.js` |
| RUN-03: Agent Subprocess | 按 `.vibe/agents/` 配置启动 Agent 命令，注入上下文并捕获输出。 | `run.js` |
| RUN-04: Log & Run Capture | 将 stdout/stderr/exit_code 及元数据持久化到 `.vibe/runs/` 和 `.vibe/logs/`。 | `run.js` |
| RUN-05: Portable Runtime | 使用原生 Node.js API，不依赖第三方库，确保在各种 AI 终端环境中的可移植性。 | ALL |

## Standard Stack (Scripts)

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `fs` | Native | 文件读写、目录创建、原子性检查 | Node.js 原生，无外部依赖 |
| `path` | Native | 跨平台路径构建 | 确保 Win/Mac/Linux 路径一致性 |
| `child_process` | Native | 启动 Agent 进程、流捕获 | 实现 Subprocess Boundary 的核心工具 |
| `crypto` | Native | 生成稳定的 Task/Run ID (UUID v4) | 替代 Rust 的 `uuid` crate |

## Architecture Patterns

### 1. Subprocess Boundary & Context Injection
`run.js` 启动 Agent 时，通过以下方式注入上下文：
- **Stdin**: 将 `task.json` 内容通过 stdin 喂给 Agent（如果配置支持）。
- **Env Vars**: 将 allowlisted 环境变量及 `VIBE_TASK_ID` 等元数据注入。
- **Files**: Agent 按照 `prompt` 和 `references` 指示自行读取 `.vibe/tasks/<id>.json`。

### 2. File Scope Locking (Deterministic Locking)
`lock.js` 采用简单的文件锁模式：
- 锁定文件存放在 `.vibe/locks/`。
- 锁名通常是 `file_scope` 中路径的 Base64 编码或 Hash。
- 脚本提供 `acquire` 和 `release` 命令。

### 3. Task Lifecycle Management
`status.js` 提供原子的状态变更逻辑，确保 `updated_at` 时间戳同步更新，并防止非法状态转移。

## Common Pitfalls to Avoid

- **过度的策略逻辑**: 脚本不应包含“如果失败则重试”或“如果忙碌则等待”的复杂逻辑。这些由 Conductor Skill 决定，脚本只负责返回 `failed` 或 `locked`。
- **非跨平台 Shell 调用**: 启动 `model_command` 时，应尽量使用 `spawn` 的非 shell 模式，以避免 shell 注入风险和 Windows 转义问题。
- **巨大的内存占用**: 在捕获 Agent 输出时，应采用流式写入（Streaming）到日志文件，而不是先读入内存。

## Assumptions Log

| # | Claim | Risk if Wrong |
|---|-------|---------------|
| A1 | Node.js 运行时在目标 AI 终端环境中可用。 | 如果不可用，需提供 Python 等效脚本或回退到 Rust。 |
| A2 | `model_command` 能够通过标准 Shell 执行。 | 某些复杂的交互式命令可能需要特殊的 TTY 处理。 |
| A3 | 基于文件的锁足以处理低并发协作。 | 高并发下可能存在竞态，但 Vibe 定位是个人协作。 |
