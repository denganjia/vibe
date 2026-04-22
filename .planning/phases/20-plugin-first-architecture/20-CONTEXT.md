# Phase 20: Plugin-first 架构与迁移边界 - Context

**Gathered:** 2026-04-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 20 不实现完整 runtime，也不迁移全部旧 CLI。它只锁定 plugin-first 产品架构和迁移边界：plugin 包结构、skills/commands/references/scripts 分工、`.vibe` 工作区职责、旧 Rust CLI 能力分类，以及后续 Phase 21-24 的规划约束。

</domain>

<decisions>
## Implementation Decisions

### 产品形态
- **D-01:** v6.0 采用彻底 plugin-first，而不是 CLI-first 或 plugin-first + 独立 CLI runtime。
- **D-02:** 用户入口是当前 AI 终端可安装的 plugin。Plugin 注入 skills、commands、references 和 scripts，让当前主模型成为 Conductor。
- **D-03:** 不再把 `vibe-cli` 作为主产品形态；旧 CLI 能力需要被分类为迁入 plugin/scripts、保留兼容或移除。

### Runtime 边界
- **D-04:** 仍然需要 runtime，但 runtime 是 plugin 内的轻量 JS/Python scripts，不是独立重型 CLI、daemon 或数据库。
- **D-05:** scripts 只负责最低必要的确定性动作：初始化 `.vibe`、写 task JSON、文件锁、启动 Agent subprocess、记录 logs/results、生成 release summary。
- **D-06:** 协作智能留在模型和 skill 中，scripts 不实现复杂调度大脑，避免在脚本层重建重型系统。

### `.vibe` 工作区
- **D-07:** `.vibe` 是项目级可观察工作区，默认包含 `Agents/`、`tasks/`、`runs/`、`locks/`、`reviews/`、`logs/` 和配置文件。
- **D-08:** `.vibe/Agents` 是角色与模型定义的核心位置，可定义 planner、executor、reviewer、release 等 Agent 以及使用 claude/gemini/codex 的命令。
- **D-09:** `.vibe` 文件格式必须足够简单，让当前主模型能直接读取并推理，不依赖隐藏状态。

### 多模型协作协议
- **D-10:** 主模型负责与用户多轮问答完善计划细节，然后按计划大小拆分为多个 task。
- **D-11:** executor Agent 执行 task 后，reviewer Agent 审查执行结果并要求修复，主模型聚合最终结果。
- **D-12:** 子 Agent 通过 shell/subprocess 启动 claude、gemini、codex 等 CLI；不要求 terminal pane 编排作为默认路径。

### Phase 20 规划取舍
- **D-13:** Plugin 包结构采用通用 plugin 目录设计，但 Phase 20 先适配 Codex plugin 格式，避免过早绑定单一平台。
- **D-14:** Scripts runtime 默认使用 JS 实现；Python 可作为未来可选 runtime，但 Phase 20 不做双实现。
- **D-15:** 旧 Rust CLI 暂时保留为 compatibility reference，用于迁移分类和行为对照，不在 Phase 20 直接大规模删除。
- **D-16:** 子 Agent 默认通过 subprocess 非交互执行；terminal pane 编排保留为可选兼容模式，不作为默认执行路径。
- **D-17:** Phase 20 不只产出架构文档和迁移表，也要 scaffold 最小 plugin 目录，让后续 Phase 21/22 能直接接着扩展。
- **D-18:** 产品名定为 **Vibe**。`vibe-cli` 只表示旧 Rust CLI 实现或兼容层，不再作为主产品名。
- **D-19:** Plugin package root 使用 `plugin/vibe/`，内部放置 `.codex-plugin/plugin.json`、`skills/`、`commands/`、`references/`、`scripts/`、`templates/` 和 `examples/`。
- **D-20:** 旧 Rust workspace 暂时保留在 `apps/` 与 `crates/`，Phase 20 只建立迁移分类与 plugin scaffold，不做大规模目录搬迁。

### the agent's Discretion
- Plugin 具体文件夹命名、JS 或 Python 的默认选择、task JSON 字段命名和 release notes 模板格式可由 planner 在 Phase 20 plan 中提出具体方案。
- 只要不违背 plugin-first 与 scripts-thin-runtime 的边界，planner 可以保留少量兼容旧 Rust CLI 的过渡方案。

</decisions>

<specifics>
## Specific Ideas

- 用户原始愿景：一个主会话接收任务描述，自动多轮问答完善计划，再拆成多个 Task，交给 claude/gemini/codex 等 Agent 执行，结束后调用 Reviewer 审查并修复。
- 新判断：市面上主流模型已经支持 skills/commands/plugins，系统应作为 plugin 注入协作能力，而不是要求用户学习独立 CLI 编排系统。
- 代码可以放在 plugin references 供模型阅读，但可执行能力应放在 plugin scripts，通过 commands/skills 调用。
- `.vibe/Agents` 中应保存不同角色的详细定义和使用模型，而不是硬编码在 Rust CLI 中。
- Phase 20 最小 scaffold 应体现最终产品方向：plugin package、references、skills/commands hooks、scripts 入口，以及示例 `.vibe` 工作区模板。
- 目标项目结构：

```text
vibe/
  plugin/
    vibe/
      .codex-plugin/
        plugin.json
      skills/
      commands/
      references/
      scripts/
      templates/
        .vibe/
          Agents/
          config.json
          tasks/
          runs/
          locks/
          reviews/
          logs/
      examples/
  apps/        # legacy Rust CLI during migration
  crates/      # legacy Rust core during migration
  docs/
  .planning/
```

</specifics>

<canonical_refs>
## Canonical References

### Planning
- `.planning/PROJECT.md` — v6.0 plugin-first pivot, product shape, active requirements, key decisions.
- `.planning/REQUIREMENTS.md` — v6.0 plugin architecture, `.vibe` workspace, scripts runtime, multi-model workflow, release/migration requirements.
- `.planning/ROADMAP.md` — Phase 20-24 execution order and phase boundaries.

### Existing implementation to classify
- `apps/vibe-cli/src/main.rs` — existing CLI command routing, spawn, signal/wait, report, init behavior to classify for migration.
- `crates/vibe-core/src/state/mod.rs` — existing `.vibe` config/state concepts to reuse or simplify.
- `crates/vibe-core/src/ipc/bus.rs` — existing file bus atomic write/consume behavior, useful as reference for scripts runtime.
- `.vibe/config.json` — current project-local config shape.
- `.vibe/roles/` — current role templates, likely precursor to `.vibe/Agents`.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ProjectConfig` and `RoleManager` in `crates/vibe-core/src/state/mod.rs`: prove `.vibe/config.json` and role files are already validated concepts.
- `FileBus` in `crates/vibe-core/src/ipc/bus.rs`: demonstrates simple file-based atomic write and consume semantics that scripts runtime can mimic.
- `spawn_role` in `apps/vibe-cli/src/main.rs`: contains useful lessons for Agent command construction, persona injection and `VIBE_ID` propagation, but should not remain the primary product surface.

### Established Patterns
- Project-local `.vibe` directory is already central and should survive the pivot.
- Role templates are already Markdown, which maps naturally to plugin references and `.vibe/Agents`.
- Current Rust CLI uses file state and no database; v6.0 should keep that simplicity.

### Integration Points
- Plugin package should become the new root for skills, commands, references and scripts.
- `.vibe` initialization should be handled by plugin scripts, with existing Rust init logic used as migration reference.
- Release summary should become plugin command/script rather than a new Rust CLI subcommand.

</code_context>

<deferred>
## Deferred Ideas

- Publishing plugin to external marketplaces — future milestone after local plugin package stabilizes.
- Direct GitHub release publishing through network APIs — future milestone after local release notes generation works.
- Agent historical success scoring — future enhancement after deterministic task/review loop exists.

</deferred>

---

*Phase: 20-plugin-first-architecture*
*Context gathered: 2026-04-22*
