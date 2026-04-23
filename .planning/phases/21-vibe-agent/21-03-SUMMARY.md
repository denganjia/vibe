---
phase: 21-vibe-agent
plan: 03
subsystem: vibe-agent
tags: [template, config, agent, gap-fix]
requirements: [VIBE-02, VIBE-03]
requires: ["21-01", "21-02"]
provides: ["enhanced-agent-templates", "global-config-defaults"]
tech-stack: [JSON, Node.js]
key-files:
  - plugin/vibe/templates/.vibe/agents/planner.json
  - plugin/vibe/templates/.vibe/agents/executor.json
  - plugin/vibe/templates/.vibe/agents/reviewer.json
  - plugin/vibe/templates/.vibe/agents/release.json
  - plugin/vibe/templates/.vibe/config.json
  - plugin/vibe/scripts/init.test.js
decisions:
  - "将 Agent prompt 和 reference 固化在模板中，确保初始化即具备引导能力。"
  - "在全局配置中引入 default_model，简化后续多 Agent 执行时的参数传递。"
metrics:
  duration: 10m
  completed_date: "2026-04-23"
---

# Phase 21 Plan 03: Gap Fix Summary

## One-liner
完善 Agent 和 Config 模板字段，确保初始化后的工作区完全符合 VIBE-02/VIBE-03 规范。

## Overview
本计划修复了 Phase 21 验证中发现的字段缺失问题。通过为 Agent 模板增加 `prompt` 和 `reference` 字段，以及为 `config.json` 增加 `default_model` 和 `lock_policy`，确保了 `.vibe` 工作区在初始化后即可被后续的脚本 runtime (Phase 22) 和执行引擎 (Phase 23) 正确使用。

## Key Changes

### 1. Agent 模板增强
- 所有的 Agent JSON 模板（planner, executor, reviewer, release）均已包含：
    - `"prompt"`: 对应角色的系统提示词引导。
    - `"reference"`: 指向 `plugin/vibe/references/` 下相关协议文档的路径。
- **Files**: `plugin/vibe/templates/.vibe/agents/*.json`
- **Commit**: `a57aa31`

### 2. 全局配置完善
- `config.json` 模板新增了：
    - `"default_model": "claude"`: 全局默认模型配置。
    - `"lock_policy": {}`: 锁策略占位符，符合 Roadmap 要求。
- **File**: `plugin/vibe/templates/.vibe/config.json`
- **Commit**: `7ddac59`

### 3. 测试覆盖
- 更新了 `plugin/vibe/scripts/init.test.js`，增加了对生成的 JSON 文件内容的断言，确保新字段被正确拷贝。
- **File**: `plugin/vibe/scripts/init.test.js`
- **Commit**: `81af661`

## Deviations from Plan
None - plan executed exactly as written.

## Known Stubs
- `config.json` 中的 `lock_policy` 为空对象 `{}`，其实际逻辑将在 Phase 22 实现。

## Threat Flags
None.

## Self-Check: PASSED
- [x] All Agent templates contain prompt and reference.
- [x] config.json contains default_model and lock_policy.
- [x] init.test.js passed.
- [x] All changes committed.
