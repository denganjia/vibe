# Phase 16: 全流程集成测试 (E2E Integration) - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-20
**Phase:** 16-e2e-integration
**Areas discussed:** 测试覆盖范围 (Coverage), CI 集成 (CI Integration), Mock 策略 (Mock Strategy)

---

## 测试覆盖范围 (Coverage)

| Option | Description | Selected |
|--------|-------------|----------|
| Happy Path (主流程) | 验证最核心的主流程成功即可 | ✓ |
| 包括异常场景 (Error Paths) | 补充测试超时和失败等情况 | |

**User's choice:** Happy Path (主流程)
**Notes:** 仅需验证全链路核心流程跑通。

---

## CI 集成 (CI Integration)

| Option | Description | Selected |
|--------|-------------|----------|
| 加入 GitHub Actions | 在 .github/workflows 中增加 E2E 的 CI 检查 | |
| 仅本地运行 | 作为本地测试即可，不跑在 CI 上 | ✓ |

**User's choice:** 仅本地运行
**Notes:** 轻量化本地验证即可。

---

## Mock 策略 (Mock Strategy)

| Option | Description | Selected |
|--------|-------------|----------|
| 使用 Bash Mock | 继续使用 scripts/e2e_test.sh 里用 Bash mock 的 Agent 方式 | ✓ |
| 引入轻量真实 CLI | 启动一个真实的 CLI 程序扮演 Agent | |

**User's choice:** 使用 Bash Mock
**Notes:** Bash mock 快且稳定。

## Claude's Discretion

None

## Deferred Ideas

None
