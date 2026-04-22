# Phase 20: plugin-first-architecture - Pattern Map

**Mapped:** 2026-04-22
**Files analyzed:** 25
**Analogs found:** 24 / 25

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `plugin/vibe/README.md` | reference doc | request-response | `README.md` | role-match |
| `plugin/vibe/.codex-plugin/plugin.json` | config | request-response | `20-RESEARCH.md` manifest example | no-code-analog |
| `plugin/vibe/skills/conductor/SKILL.md` | provider/skill | request-response, event-driven | `skills/vibe-operator/SKILL.md` | role-match |
| `plugin/vibe/commands/init.md` | route/command | file-I/O | `apps/vibe-cli/src/main.rs` `Commands::Init` | role-match |
| `plugin/vibe/commands/plan.md` | route/command | request-response | `.vibe/roles/Conductor.md` | role-match |
| `plugin/vibe/commands/run-task.md` | route/command | batch, file-I/O | `apps/vibe-cli/src/main.rs` `Commands::Run` / `spawn_role` | role-match |
| `plugin/vibe/commands/review-task.md` | route/command | event-driven | `skills/vibe-operator/references/verification.md` | role-match |
| `plugin/vibe/commands/status.md` | route/command | file-I/O | `apps/vibe-cli/src/main.rs` `Commands::List` / `apps/vibe-cli/src/tui.rs` | role-match |
| `plugin/vibe/commands/release-summary.md` | route/command | batch, transform | `DELIVERY.md` | partial |
| `plugin/vibe/references/plugin-architecture.md` | reference doc | request-response | `.planning/ROADMAP.md` Phase 20 | role-match |
| `plugin/vibe/references/collaboration-protocol.md` | reference doc | event-driven | `skills/vibe-operator/references/collaboration.md` | exact |
| `plugin/vibe/references/task-contract.md` | model/contract | CRUD, file-I/O | `crates/vibe-core/src/ipc/protocol.rs` | role-match |
| `plugin/vibe/references/agent-contract.md` | model/config | request-response, file-I/O | `.vibe/roles/Conductor.md` + `.vibe/config.json` | role-match |
| `plugin/vibe/references/review-protocol.md` | reference doc | event-driven | `skills/vibe-operator/references/verification.md` | exact |
| `plugin/vibe/references/workspace-layout.md` | config/reference | file-I/O | `crates/vibe-core/src/state/mod.rs` `ensure_project_vibe` | role-match |
| `plugin/vibe/references/migration-classification.md` | migration/reference | transform | `apps/vibe-cli/src/main.rs` `Commands` enum | exact |
| `plugin/vibe/scripts/README.md` | utility/script doc | file-I/O, batch | `crates/vibe-core/src/ipc/bus.rs` + `StateStore` | partial |
| `plugin/vibe/templates/.vibe/config.json` | config | file-I/O | `.vibe/config.json` | exact |
| `plugin/vibe/templates/.vibe/Agents/README.md` | config/reference | file-I/O | `.vibe/roles/*.md` | role-match |
| `plugin/vibe/templates/.vibe/tasks/README.md` | config/reference | CRUD, file-I/O | `crates/vibe-core/src/ipc/protocol.rs` | partial |
| `plugin/vibe/templates/.vibe/runs/README.md` | config/reference | batch, file-I/O | `apps/vibe-cli/src/main.rs` `Commands::Run` | partial |
| `plugin/vibe/templates/.vibe/locks/README.md` | middleware/reference | file-I/O | `StateStore::acquire_lock` | role-match |
| `plugin/vibe/templates/.vibe/reviews/README.md` | config/reference | event-driven | `verification.md` | role-match |
| `plugin/vibe/templates/.vibe/logs/README.md` | config/reference | streaming, file-I/O | `apps/vibe-cli/src/tui.rs` log reader | partial |
| `plugin/vibe/examples/README.md` | reference doc | request-response | `DELIVERY.md` | partial |

## Pattern Assignments

### `plugin/vibe/.codex-plugin/plugin.json` (config, request-response)

**Analog:** No codebase analog. Use research manifest shape from `20-RESEARCH.md`, not a hand-rolled installer.

**Manifest pattern** (`20-RESEARCH.md` lines 222-233):

```json
{
  "name": "vibe",
  "version": "0.1.0",
  "description": "Plugin-first multi-model collaboration for project-local Agent workflows.",
  "skills": "./skills/",
  "interface": {
    "displayName": "Vibe",
    "shortDescription": "Coordinate AI Agents through project-local tasks, reviews, and logs."
  }
}
```

**Apply to:** keep identity small, point plugin surfaces with relative paths, and avoid adding runtime dependencies in Phase 20.

---

### `plugin/vibe/skills/conductor/SKILL.md` (provider/skill, request-response + event-driven)

**Analog:** `skills/vibe-operator/SKILL.md`

**Frontmatter pattern** (lines 1-4):

```markdown
---
name: vibe-operator
description: Orchestrate multiple AI agents using Vibe-CLI in a stateless terminal environment. Use when spawning sub-agents, synchronizing tasks via signal/wait, or managing terminal panes for complex multi-model development workflows.
---
```

**Skill index pattern** (lines 25-35):

```markdown
## Core SOPs

For detailed procedural guidance, refer to these references:

- **Collaboration**: [references/collaboration.md](references/collaboration.md) - Task assignment & A-D-E-V cycle.
- **State & Bus**: [references/state.md](references/state.md) - How the file-based bus and smart cleanup work.
- **Approvals**: [references/approval.md](references/approval.md) - Manual gates via `vibe wait approved`.
- **Orchestration**: [references/orchestration.md](references/orchestration.md) - Stack spawning and project init flow.
```

**Critical protocol pattern** (lines 44-52):

```markdown
## Critical Protocols

1. **Analyze-Declare-Execute-Verify Loop**: All autonomous tasks must follow this strict lifecycle.
2. **Intent Locking**: Workers MUST declare target files via `vibe report --status blocked --message "writing:path/to/file"` before modification to prevent race conditions.
3. **Verification & Retries**: Workers MUST run local tests (e.g., `cargo test`) after execution. If verification fails, automatically attempt to fix up to 3 times before signaling `BLOCKED`.
4. **File-based Signaling**: Never assume a worker is done. Use `vibe wait` to synchronize via the highly reliable `.vibe/bus/` file bus.
```

**Apply to:** rewrite for plugin-first Conductor behavior: clarify, plan, split tasks, invoke command/script entry points, require reviewer output, aggregate `.vibe` artifacts. Remove old default terminal pane assumptions.

---

### `plugin/vibe/commands/*.md` (route/command, request-response)

**Analogs:** `skills/vibe-operator/SKILL.md` tool reference and `apps/vibe-cli/src/main.rs` command routing.

**Command surface pattern** (`skills/vibe-operator/SKILL.md` lines 10-23):

```markdown
## Tool Reference

Use these `vibe` shell commands to manage agents and synchronization:

- `vibe init [--force]`: Interactive wizard to initialize `.vibe/config.json` and role templates.
- `vibe check`: Verify terminal orchestration (WezTerm/Tmux) support.
- `vibe list`: List all active vibe agents, roles, status, and summaries. Automatically cleans up stale panes.
- `vibe spawn [--role <ROLE> | --stack <NAME>]`: Create a new pane/tab, start the agent with auto-approve flags, and securely inject persona via the `$VIBE_PERSONA` environment variable.
```

**Rust command taxonomy to classify** (`apps/vibe-cli/src/main.rs` lines 17-119):

```rust
#[derive(Subcommand, Debug)]
enum Commands {
    Split { /* ... */ },
    List { json: bool },
    Status,
    Check { json: bool },
    Kill,
    Signal { name: String, payload: Option<String> },
    Wait { name: String, timeout: u64 },
    Run { command: Vec<String> },
    Inject { vibe_id: String, command: String, cwd: Option<String> },
    Focus { vibe_id: String },
    Spawn { role_flag: Option<String>, role: Option<String>, cmd: Option<String>, pane: bool, stack: Option<String> },
    Report { status: String, message: String },
    Init { force: bool },
}
```

**Apply to specific command files:**

| Command file | Copy pattern from | Phase 20 boundary |
|--------------|-------------------|-------------------|
| `init.md` | `Commands::Init` lines 399-458 and `ensure_project_vibe` lines 334-372 | Document non-destructive `.vibe` initialization; no full JS implementation yet. |
| `plan.md` | `.vibe/roles/Conductor.md` lines 4-15 | Document clarification and task decomposition entry point. |
| `run-task.md` | `Commands::Run` lines 322-344 and `spawn_role` lines 486-559 | Document subprocess default and artifact expectations; do not implement scheduler brain. |
| `review-task.md` | `verification.md` lines 3-12 | Document reviewer trigger and findings contract. |
| `status.md` | `Commands::List` lines 183-211 and TUI polling lines 120-190 | Document future status as `.vibe/tasks`, `runs`, `reviews`, `logs`; classify pane status as compatibility. |
| `release-summary.md` | `DELIVERY.md` lines 11-24 | Document local summary shape only; implementation belongs to Phase 24. |

---

### `plugin/vibe/references/collaboration-protocol.md` (reference doc, event-driven)

**Analog:** `skills/vibe-operator/references/collaboration.md`

**Core workflow pattern** (lines 3-31):

```markdown
## 1. 核心流: Analyze-Declare-Execute-Verify

### Step A: Analyze (分析)
- Conductor 分配任务后，Worker 须在新环境中读取目标范围内的代码和相关上下文文档。

### Step B: Declare (声明 / Intent Locking)
- **强制约束**：在修改任何文件前，Worker 必须声明对该文件的修改意图，防止多代理同时修改造成冲突。

### Step C: Execute (执行)
- 应用代码修改、配置更新等。

### Step D: Verify & Fix (验证与自愈)
- **强制门禁**：任务执行后，必须运行相关验证（如 `cargo test` 或 `npm run lint`）。
```

**Signal workflow pattern** (lines 33-43):

```markdown
## 2. 信号驱动的工作流 (Signal-driven Workflow)

- **同步协议**:
  1. **Conductor**: `vibe spawn --role Worker`。
  2. **Conductor**: 执行 `vibe wait done` 进入等待。
  3. **Worker**: 在新窗格中完成 `A-D-E-V` 任务循环。
  4. **Worker**: 发出 `vibe signal done`，内容包含 JSON Envelope。
```

**Apply to:** keep A-D-E-V and review loop, but replace default `vibe spawn`/pane wording with plugin command/script and `.vibe/tasks` artifacts.

---

### `plugin/vibe/references/task-contract.md` (model/contract, CRUD + file-I/O)

**Analog:** `crates/vibe-core/src/ipc/protocol.rs`

**Structured JSON contract pattern** (lines 21-49):

```rust
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct WorkerState {
    pub vibe_id: String,
    pub physical_id: String,
    pub role: Option<String>,
    pub status: String,
    pub summary: String,
    pub last_seen: String,
    pub cwd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SignalInfo {
    pub name: String,
    pub payload: serde_json::Value,
}
```

**Serialization pattern** (lines 81-90):

```rust
impl Message {
    pub fn to_ndjson(&self) -> serde_json::Result<String> {
        let mut json = serde_json::to_string(self)?;
        json.push('\n');
        Ok(json)
    }

    pub fn from_str(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s.trim())
    }
}
```

**Apply to:** define task JSON fields in plain Markdown: `id`, `goal`, `context`, `file_scope`, `constraints`, `expected_output`, `verification`, `reviewer_requirements`, `status`, `created_at`, `updated_at`. Phase 20 documents the contract only.

---

### `plugin/vibe/references/agent-contract.md` and `templates/.vibe/Agents/README.md` (model/config, file-I/O)

**Analogs:** `.vibe/config.json`, `.vibe/roles/*.md`, `RoleManager`

**Current config shape** (`.vibe/config.json` lines 1-19):

```json
{
  "roles": {
    "Worker": "gemini -y",
    "Evaluator": "gemini -y",
    "Conductor": "gemini -y"
  },
  "default_command": "gemini -y",
  "stacks": {
    "default": [
      "Conductor",
      "Worker"
    ],
    "full": [
      "Conductor",
      "Worker",
      "Evaluator"
    ]
  }
}
```

**Role file pattern** (`.vibe/roles/Conductor.md` lines 4-15):

```markdown
## Responsibilities
- **Task Planning & Delegation**: Break down complex objectives and use `vibe spawn` to deploy Workers to handle specific sub-tasks.
- **Intelligence-First Routing**: Monitor `.vibe/bus/` signals using `vibe wait`. Analyze the JSON payloads (or file paths starting with `@`) and the current state of panes (`vibe list`).
- **Direct Control**: Use `vibe inject` to send direct instructions, context, or corrections to specific Workers when they are stalled or need guidance.
- **Consolidation**: Once all tasks are complete, aggregate the results and generate final documentation (e.g., `DELIVERY.md`).
```

**Role loader pattern** (`crates/vibe-core/src/state/mod.rs` lines 312-331):

```rust
pub struct RoleManager {
    roles_dir: PathBuf,
}

impl RoleManager {
    pub fn new() -> Result<Self> {
        let vibe_dir = resolve_project_vibe_dir()?;
        Ok(Self {
            roles_dir: vibe_dir.join("roles"),
        })
    }

    pub fn get_persona(&self, role_name: &str) -> Result<String> {
        let role_file = self.roles_dir.join(format!("{}.md", role_name));
        if role_file.exists() {
            Ok(fs::read_to_string(role_file)?)
        } else {
            Err(crate::error::VibeError::Internal(format!("Role template not found: {}", role_name)))
        }
    }
}
```

**Apply to:** document `.vibe/Agents` as richer than old `.vibe/roles`: command, model/provider, prompt/reference files, allowed tools, expected outputs, review policy. Do not copy old roles verbatim.

---

### `plugin/vibe/references/review-protocol.md` and `templates/.vibe/reviews/README.md` (reference doc, event-driven)

**Analog:** `skills/vibe-operator/references/verification.md`

**Review pattern** (lines 3-12):

```markdown
## 1. 任务后逻辑审计 (Post-task Logic Audit)

Evaluator (审计者) 必须在任务完成后、标记为 SUCCESS 前执行逻辑审计。

- **审计流程**:
  1. **读取意图**: 从初始计划中提取该任务的原始意图，核实 Intent Locking 记录 (`vibe list`)。
  2. **代码/产出走查**: 检查生成的代码、配置或文档是否实现了所有要求。
  3. **环境核实**: 运行自动化测试脚本（如 `cargo test`）验证物理变更。
  4. **反馈提交**: 如果发现偏差，通过 `vibe report` 提交审计结果并通过 `.vibe/bus/` 发送信号通知 Conductor。
```

**Checklist pattern** (lines 13-20):

```markdown
## 2. 意图对齐检查表 (Intent Alignment Checklist)

- [ ] **意图完整性 (Intent Completeness)**: 是否所有子任务要求都已达成？
- [ ] **逻辑严密性 (Logic Integrity)**: 代码逻辑是否闭环？是否存在明显的竞态条件？
- [ ] **A-D-E-V 遵守情况 (A-D-E-V Compliance)**: Worker 是否在修改前声明了 Intent Locks？
- [ ] **状态一致性 (State Consistency)**: 变更是否已正确汇入最终的 `DELIVERY.md` 或全局状态中？
```

**Apply to:** reviewer output should be a structured artifact under `.vibe/reviews`; replace pane/status references with task/run/review artifacts.

---

### `plugin/vibe/references/workspace-layout.md` and `templates/.vibe/*/README.md` (config/reference, file-I/O)

**Analog:** `crates/vibe-core/src/state/mod.rs` and `crates/vibe-core/src/env.rs`

**Project-local `.vibe` resolution** (`crates/vibe-core/src/env.rs` lines 49-62):

```rust
pub fn resolve_project_vibe_dir() -> Result<PathBuf> {
    let mut current = env::current_dir()?;
    loop {
        let vibe_dir = current.join(".vibe");
        if vibe_dir.is_dir() {
            return Ok(vibe_dir);
        }
        if !current.pop() {
            break;
        }
    }
    // Return a default if not found
    Ok(env::current_dir()?.join(".vibe"))
}
```

**Initialization pattern** (`crates/vibe-core/src/state/mod.rs` lines 334-372):

```rust
pub fn ensure_project_vibe() -> Result<PathBuf> {
    let vibe_dir = resolve_project_vibe_dir()?;
    if !vibe_dir.exists() {
        fs::create_dir_all(&vibe_dir)?;
        println!("Initialized .vibe directory in {:?}", vibe_dir);
    }

    let roles_dir = vibe_dir.join("roles");
    if !roles_dir.exists() {
        fs::create_dir_all(&roles_dir)?;
    }

    let state_dir = vibe_dir.join("state");
    if !state_dir.exists() {
        fs::create_dir_all(&state_dir)?;
    }

    let config_file = vibe_dir.join("config.json");
    if !config_file.exists() {
        let config = ProjectConfig::default();
        let content = serde_json::to_string_pretty(&config)?;
        fs::write(config_file, content)?;
    }

    Ok(vibe_dir)
}
```

**Apply to:** Phase 20 template docs should name future directories: `Agents/`, `tasks/`, `runs/`, `locks/`, `reviews/`, `logs/`, `config.json`. Preserve non-destructive expectations for Phase 21.

---

### `plugin/vibe/templates/.vibe/config.json` (config, file-I/O)

**Analog:** `ProjectConfig` and `.vibe/config.json`

**Default config pattern** (`crates/vibe-core/src/state/mod.rs` lines 240-263):

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub roles: HashMap<String, String>,
    pub default_command: String,
    pub stacks: HashMap<String, Vec<String>>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        let mut roles = HashMap::new();
        roles.insert("Conductor".to_string(), "claude -y".to_string());
        roles.insert("Worker".to_string(), "claude -y".to_string());
        roles.insert("Evaluator".to_string(), "claude -y".to_string());
        // ...
    }
}
```

**User override merge pattern** (`crates/vibe-core/src/state/mod.rs` lines 278-303):

```rust
pub fn load(&self) -> Result<ProjectConfig> {
    let default_config = ProjectConfig::default();
    if self.config_file.exists() {
        let content = fs::read_to_string(&self.config_file)?;
        let user_val: serde_json::Value = serde_json::from_str(&content)?;
        let mut default_val = serde_json::to_value(&default_config)?;
        Self::merge_values(&mut default_val, user_val);
        Ok(serde_json::from_value(default_val)?)
    } else {
        Ok(default_config)
    }
}
```

**Apply to:** template config should stay simple and model-readable. Include future fields only as documented placeholders if needed; do not create a hidden state schema that scripts alone understand.

---

### `plugin/vibe/scripts/README.md`, `templates/.vibe/tasks/README.md`, `runs/README.md`, `locks/README.md`, `logs/README.md` (utility/script docs, file-I/O)

**Analogs:** `FileBus`, `StateStore`, `Commands::Run`, TUI log reader.

**Atomic state write pattern** (`crates/vibe-core/src/state/mod.rs` lines 63-72):

```rust
fn save(&self) -> Result<()> {
    let panes = self.panes.lock().unwrap();
    let content = serde_json::to_string_pretty(&*panes)?;
    
    // Atomic write
    let tmp_file = self.state_file.with_extension("tmp");
    fs::write(&tmp_file, content)?;
    fs::rename(tmp_file, &self.state_file)?;
    
    Ok(())
}
```

**Lock pattern** (`crates/vibe-core/src/state/mod.rs` lines 75-93):

```rust
fn acquire_lock(&self) -> Result<LockGuard> {
    let lock_file = self.state_file.with_extension("lock");
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(5);

    while start.elapsed() < timeout {
        match fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&lock_file) {
            Ok(_) => return Ok(LockGuard { path: lock_file }),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            Err(e) => return Err(e.into()),
        }
    }
    Err(crate::error::VibeError::Internal("Timeout acquiring state lock".into()))
}
```

**FileBus send/receive pattern** (`crates/vibe-core/src/ipc/bus.rs` lines 11-38, 40-73):

```rust
pub fn send(signal_name: &str, payload: Value) -> Result<()> {
    let bus_dir = resolve_bus_dir()?;
    let filename = format!("{}-{}.json", ts, uuid);
    let final_path = bus_dir.join(filename);
    let tmp_path = final_path.with_extension("tmp");
    let signal = SignalInfo {
        name: signal_name.to_string(),
        payload,
    };
    let content = serde_json::to_string_pretty(&signal)?;
    fs::write(&tmp_path, content)?;
    fs::rename(tmp_path, final_path)?;
    Ok(())
}

pub fn recv(signal_name: &str, timeout_secs: u64) -> Result<Value> {
    // poll .vibe/bus, parse matching JSON, consume with remove_file, then return payload
}
```

**Subprocess launch pattern** (`apps/vibe-cli/src/main.rs` lines 322-344):

```rust
Commands::Run { command } => {
    if command.is_empty() {
        anyhow::bail!("No command provided for 'vibe run'.");
    }

    let master_pane_id = match detect_current_terminal() {
        Some(TerminalType::WezTerm) => WezTermAdapter.get_metadata()?.pane_id,
        Some(TerminalType::Tmux) => TmuxAdapter.get_metadata()?.pane_id,
        None => "0".to_string(),
    };

    let mut child = Command::new(&command[0])
        .args(&command[1..])
        .env("VIBE_MASTER_ID", &master_pane_id)
        .spawn()?;

    let status = child.wait().await?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
```

**Log reader pattern** (`apps/vibe-cli/src/tui.rs` lines 170-187):

```rust
if let Some(ref id) = app.selected_id {
    let logs_dir = vibe_core::env::resolve_logs_dir()?;
    let log_path = logs_dir.join(format!("{}.log", id));
    if log_path.exists() {
        let content = tokio::fs::read_to_string(&log_path).await.unwrap_or_default();
        let lines: Vec<&str> = content.lines().collect();
        let last_lines = if lines.len() > 20 {
            &lines[lines.len()-20..]
        } else {
            &lines[..]
        };
        app.logs = last_lines.join("\n");
    }
}
```

**Apply to:** scripts docs should explicitly reserve JS implementation for Phase 22. In Phase 20, only document deterministic responsibilities: init `.vibe`, write task JSON, acquire locks, launch subprocesses, capture logs/results, and generate release summary drafts.

---

### `plugin/vibe/references/migration-classification.md` (migration/reference, transform)

**Analog:** `apps/vibe-cli/src/main.rs`

**Migration inventory source** (`apps/vibe-cli/src/main.rs` lines 17-119):

```rust
enum Commands {
    Split { /* terminal pane */ },
    List { json: bool },
    Status,
    Check { json: bool },
    Kill,
    Signal { name: String, payload: Option<String> },
    Wait { name: String, timeout: u64 },
    Run { command: Vec<String> },
    Inject { vibe_id: String, command: String, cwd: Option<String> },
    Focus { vibe_id: String },
    Spawn { /* role, cmd, pane, stack */ },
    Report { status: String, message: String },
    Init { force: bool },
}
```

**Classification pattern from research** (`20-RESEARCH.md` lines 306-319):

```markdown
| Old CLI capability | Category | Phase 20 classification rationale |
| `init --force` wizard/config/role creation | Migrate-to-script, redesigned |
| `run <command>` with `VIBE_MASTER_ID` | Migrate-to-script |
| `signal` / `wait` file bus semantics | Migrate-to-script/reference |
| `spawn --role/--stack` Agent command construction | Migrate-to-script/reference |
| `split`, `focus`, `inject`, `kill`, pane-backed `list`, `check` | Compatibility |
| `status` TUI | Compatibility or remove |
```

**Apply to:** every old command must be categorized as `Migrate-to-script`, `Compatibility`, or `Remove`, with rationale tied to plugin-first and scripts-thin-runtime decisions.

---

### `plugin/vibe/README.md`, `references/plugin-architecture.md`, `examples/README.md` (reference docs, request-response)

**Analogs:** `README.md`, `.planning/ROADMAP.md`, `DELIVERY.md`

**Product overview pattern** (`README.md` lines 1-10):

```markdown
# Vibe-CLI

Vibe-CLI 是一个强大的多智能体（Multi-agent）终端编排工具，旨在将您的终端转变为一个自主协作的开发环境。通过 **Stateless Bus（无状态总线）** 架构，它允许不同的 AI 智能体在独立的终端窗格中协同工作。

## 核心特性

- **多窗格编排**：支持 WezTerm 和 Tmux，自动管理终端布局。
- **自主派生 (Autonomous Spawning)**：智能体可以根据任务需求自主创建子智能体。
```

**Phase boundary pattern** (`.planning/ROADMAP.md` lines 27-36):

```markdown
### Phase 20: Plugin-first 架构与迁移边界
**Goal**: 锁定彻底 plugin-first 的产品架构，明确哪些能力属于 skills、commands、references、scripts、`.vibe`，以及旧 CLI 哪些能力迁移、保留或移除。
**Success Criteria** (what must be TRUE):
  1. 项目有明确 plugin 包目录设计，说明 skills、commands、references 和 scripts 如何被 AI 终端加载。
  2. references 定义协作协议、任务合同、Agent 合同、review 协议和 `.vibe` 工作区布局。
  3. skills 明确当前主模型作为 Conductor 的行为：澄清需求、拆计划、派任务、调 reviewer、聚合结果。
  4. commands 明确暴露 init、plan、run task、review task、status、release summary 等主要入口。
  5. 旧 Rust CLI 能力被分类为 migrate-to-script、compatibility 或 remove，并说明理由。
```

**Release/example summary pattern** (`DELIVERY.md` lines 11-24):

```markdown
## Execution Log
1. **Analyze**: Worker agent analyzed `apps/vibe-cli/src/main.rs`.
2. **Declare**: Worker declared intent locking via `vibe report --status blocked --message "writing:apps/vibe-cli/src/tui.rs"`.
3. **Execute**: 
4. **Verify**: Worker executed `cargo test` and `cargo check`.
5. **Signal**: Worker sent `[vibe-signal:refactor_done]` via the `.vibe/bus/` file bus.
6. **Consolidate**: Conductor agent consumed the signal and generated this final `DELIVERY.md`.
```

**Apply to:** rewrite the README and examples around product name `Vibe`, plugin-first workflow, and `.vibe` artifacts. Keep legacy CLI language only in migration/compatibility context.

## Shared Patterns

### Thin Runtime Boundary

**Source:** `20-CONTEXT.md` lines 21-24 and `20-RESEARCH.md` lines 285-291
**Apply to:** `scripts/README.md`, commands, references, migration classification

Phase 20 must not implement the full runtime. Scripts are limited to deterministic filesystem/subprocess actions; policy and collaboration intelligence belong in skills and references.

### File-Based Atomicity

**Source:** `crates/vibe-core/src/ipc/bus.rs` lines 31-35 and `StateStore::save` lines 63-72
**Apply to:** task contract, runs/logs/locks docs, future scripts

```rust
let content = serde_json::to_string_pretty(&signal)?;
fs::write(&tmp_path, content)?;
fs::rename(tmp_path, final_path)?;
```

### Project-Local Workspace

**Source:** `crates/vibe-core/src/env.rs` lines 49-62
**Apply to:** workspace layout, config template, command docs

`.vibe` resolution starts from the current directory and walks upward before defaulting to current `.vibe`. Preserve project-local, inspectable state.

### Agent Persona and Command Separation

**Source:** `apps/vibe-cli/src/main.rs` lines 486-559
**Apply to:** agent contract, Conductor skill, run-task command

`spawn_role` separates role/persona loading, command selection, env/context injection, process spawning, and state registration. Future plugin scripts should keep this separation but default to subprocesses, not panes.

### Review Before Completion

**Source:** `skills/vibe-operator/references/verification.md` lines 3-20
**Apply to:** collaboration protocol, review protocol, task contract

Reviewer output is a required artifact before marking a task complete. Carry forward the intent-completeness and logic-integrity checklist.

### Legacy Compatibility Boundary

**Source:** `20-RESEARCH.md` lines 306-319
**Apply to:** migration classification and command docs

Pane orchestration commands are compatibility, not default plugin behavior. `signal`/`wait`, config, role/persona, subprocess, and atomic file ideas migrate as references or thin scripts.

## No Analog Found

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `plugin/vibe/.codex-plugin/plugin.json` | config | request-response | No existing plugin manifest or `.codex-plugin` package exists in this repo. Use the researched Codex manifest shape and keep Phase 20 scaffold-only. |

## Metadata

**Analog search scope:** repo root, `skills/vibe-operator/`, `.vibe/`, `apps/vibe-cli/src/`, `crates/vibe-core/src/`, `.planning/`
**Files scanned:** 51 repo files from `rg --files`, plus canonical phase/planning docs
**Pattern extraction date:** 2026-04-22
**Explicit non-targets:** do not modify `.planning/phases/19-autonomous-workflow/19-03-SUMMARY.md`; do not modify `crates/vibe-core/.vibe/bus/`; do not implement full runtime in Phase 20.
