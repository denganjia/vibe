{
  "found": true,
  "phase_number": "21",
  "phase_name": "`.vibe` 工作区与 Agent 定义",
  "goal": "Plugin 启用后可以非破坏性创建项目级 `.vibe` 工作区，并用 `.vibe/Agents` 定义 planner、executor、reviewer、release 等角色和模型命令。",
  "success_criteria": [
    "初始化会创建 `.vibe/Agents`、`.vibe/tasks`、`.vibe/runs`、`.vibe/locks`、`.vibe/reviews`、`.vibe/logs` 和配置文件。",
    "Agent 文件可以声明角色、模型命令、prompt/reference、允许工具和预期输出。",
    "`.vibe/config.json` 可以记录默认模型、Agent 定义、并发限制、任务路径、锁策略、review 策略和 release 设置。",
    "初始化不会覆盖用户修改过的 `.vibe` 文件，除非显式 force。",
    "`.vibe` 格式足够直观，当前模型可以直接读取并理解当前协作状态。"
  ],
  "section": "### Phase 21: `.vibe` 工作区与 Agent 定义\n**Goal**: Plugin 启用后可以非破坏性创建项目级 `.vibe` 工作区，并用 `.vibe/Agents` 定义 planner、executor、reviewer、release 等角色和模型命令。\n**Depends on**: Phase 20\n**Requirements**: VIBE-01, VIBE-02, VIBE-03, VIBE-04, VIBE-05\n**Success Criteria** (what must be TRUE):\n  1. 初始化会创建 `.vibe/Agents`、`.vibe/tasks`、`.vibe/runs`、`.vibe/locks`、`.vibe/reviews`、`.vibe/logs` 和配置文件。\n  2. Agent 文件可以声明角色、模型命令、prompt/reference、允许工具和预期输出。\n  3. `.vibe/config.json` 可以记录默认模型、Agent 定义、并发限制、任务路径、锁策略、review 策略和 release 设置。\n  4. 初始化不会覆盖用户修改过的 `.vibe` 文件，除非显式 force。\n  5. `.vibe` 格式足够直观，当前模型可以直接读取并理解当前协作状态。\n**Plans**: TBD"
}