# Phase 12: Workflow Templates & Optimization - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-16
**Phase:** 12-workflow-templates-optimization
**Areas discussed:** Templates, Variable Format, SOP Design, Optimization

---

## Scenario Templates

| Option | Description | Selected |
|--------|-------------|----------|
| Refactoring Workflow | Specialized templates for code refactoring | ✓ |
| Multi-scenario (TDD/Diag) | Add TDD and Diagnostics as well | |

**User's choice:** 目前仅引入代码重构工作流模板。

---

## Variable Format & Context

| Option | Description | Selected |
|--------|-------------|----------|
| Prompt Injection | Use dynamic semantic variables | ✓ |
| $[VARIABLE_NAME] | Preferred placeholder format | ✓ |
| {{VARIABLE_NAME}} | Jinja2/Mustache style | |
| Explicit Labeling | Use [See <FILE>] for cross-referencing | ✓ |

**User's choice:** 使用 Prompt 变量注入，占位符格式采用 $[VAR]。上下文感知通过显示标注。

---

## SOP Design (Refactoring)

| Option | Description | Selected |
|--------|-------------|----------|
| Standard Refactor | Analyze -> Refactor -> Test -> Review | ✓ |
| Safe Refactor | Include Snapshot/Backup nodes | |

**User's choice:** 采用标准重构流，不强制要求快照节点。

---

## Optimization & Delivery (Claude's Discretion)

- Token Optimization: Compressing tool descriptions in SKILL.md.
- Reliability Benchmark: Defining Edge Case tests for model compliance.
- Quick Start: Adding a command Cheat Sheet to SKILL.md.

---

## Deferred Ideas

- TDD workflow templates.
- Deep diagnostics/networking templates.
- Runtime validation of dynamic variables.
