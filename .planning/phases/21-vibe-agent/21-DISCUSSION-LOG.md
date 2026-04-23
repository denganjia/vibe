## 讨论记录：第 21 阶段 (`.vibe` 工作区与 Agent 定义)

### Q1: Agent 定义文件的格式
**问题:** 新的 Agent 定义文件存放在 `.vibe/agents/`（原计划为大写 `Agents`，根据反馈调整为小写 `agents`），我们应该采用什么格式？
**用户反馈:** 由你决定，另外调整 `Agents` 目录为 `agents`（小写）。
**决策:** 采用 **纯 JSON 格式 (Pure JSON)**。
**理由:** 第 22 阶段将引入轻量级的执行脚本（scripts runtime）。纯 JSON 格式无需额外的 YAML 解析器即可被各类脚本语言（如 Node.js、Python 等）原生支持，更加符合轻量级的架构原则，能够可靠地从中提取 `model_command` 和 `allowed_tools`。

### Q2: Legacy Config 的迁移策略
**问题:** `init` 脚本在初始化时应如何处理现有的旧版 `.vibe/config.json` 结构（例如旧的 `roles` 映射）？
**用户反馈:** 由你决定。
**决策:** 采用 **严格/手动迁移 (Strict/Manual Migration)**。
**理由:** 根据契约规定，初始化过程默认是非破坏性的（non-destructive）。`init` 脚本将仅仅创建新的配置结构，并跳过旧的映射。由于现有用户配置可能会被复杂依赖使用，为了避免破坏原有工作流，我们选择让用户参考文档或日志手动将旧的 `roles` 迁移到新的 `.vibe/agents/` 目录中。

---

### 其他说明
*   **目录命名调整:** 明确将所有 Agent 定义目录约定为全小写的 `.vibe/agents/`。