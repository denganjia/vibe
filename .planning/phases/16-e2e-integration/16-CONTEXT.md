# Phase 16: 全流程集成测试 (E2E Integration) - Context

**Gathered:** 2026-04-20
**Status:** Ready for planning

<domain>
## Phase Boundary

验证完整的自治开发工作流，确保主代理启动 (Spawn) -> 子代理执行工作并发送信号 (Signal) -> 主代理接收信号并继续 (Wait) 这一完整链路能顺利运行，并且上下文 (`.vibe/state/`) 被正确共享。
</domain>

<decisions>
## Implementation Decisions

### 测试覆盖范围 (Coverage)
- **D-01:** 验证主流程成功即可 (Happy Path)，不要求全量覆盖错误和超时场景。

### CI 集成 (CI Integration)
- **D-02:** E2E 测试作为本地命令/脚本运行即可，不需要加入 GitHub Actions。

### Mock 策略 (Mock Strategy)
- **D-03:** 继续使用 `scripts/e2e_test.sh` 里通过 Bash 脚本进行 Mock 的轻量方式。这能保证测试在本地极快运行并且无外部依赖。

### 隐含需求实现
- **D-04:** BUS-11 (实现跨 Agent 上下文共享): 子代理任务完成后能通过上报机制 (`vibe report`) 自动更新 `.vibe/state/panes.json` 的 summary 和 status。
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### 测试规范
- `.planning/REQUIREMENTS.md` — BUS-11 (上下文共享)
- `scripts/e2e_test.sh` — 已有的 Bash 测试用例脚本，包含 Mock Agent。
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `e2e_test_workdir/mock_agent.sh` — 已包含对 `vibe signal` 和 `vibe report` 等命令的调用模拟。
- `.vibe/state/panes.json` — 现有的并发安全结构，可被子进程 Agent 读写。

### Integration Points
- 子代理启动脚本 `mock_agent.sh` 需触发 `vibe report --status success --message "..."` 更新上游状态，并触发 `vibe signal done` 发送信号。
</code_context>

<specifics>
## Specific Ideas

无特殊附加想法，遵循标准 Bash 脚本验证流程。
</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.
</deferred>
