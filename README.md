# Vibe: Universal AI Development Plugin & MCP Server

## 介绍 (Introduction)

Vibe 是一个 **Plugin-first (插件优先) 的多模型协作协议与轻量级运行时**。它的核心目标是打破 AI 与本地开发环境之间的“次元壁”，将您的终端从单纯的字符输入框升级为分布式 AI 协作的物理调度室。

在 Vibe 7.0 里程碑中，**Vibe 已经完全摒弃了笨重的独立 Rust CLI 编排器**，全面转型为基于 **Model Context Protocol (MCP)** 的标准化 AI 终端插件。这意味着您可以直接使用现有的 AI 终端（如 Gemini CLI、Claude Code、Codex）无缝加载 Vibe 的能力，让大语言模型（LLM）直接管理项目工作区、分配任务、获取状态并进行资源锁定。

---

## 当前实现 (Current Implementation)

Vibe 目前完全采用 **Node.js** 和 **MCP 协议** 实现，其核心架构设计如下：

1. **MCP Server 原生集成**：
   位于 `plugin/vibe/mcp-server.js` 的轻量级 Node.js 服务器。它将底层的文件读写、锁定（Locking）、任务 JSON 落盘等原子操作暴露为标准化的 MCP 工具（如 `vibe_create_task`, `vibe_acquire_lock`）。
2. **纯态项目级工作区 (JSON State)**：
   所有的运行状态、任务定义、日志和审查记录都静态地、安全地保存在项目根目录的 `.vibe/` 文件夹下。没有全局数据库，AI 的思考与调度过程如同 Git 历史一样可被人类读取、审计和接续。
3. **动态技能发现引擎 (Dynamic Skill Discovery)**：
   MCP 服务器启动时，会自动扫描 `plugin/vibe/skills/` 目录。只要定义了标准的 `SKILL.md`（包含 YAML 元数据）以及对应的执行脚本，该技能就会被自动注册为大模型可调用的工具（例如 `vibe_skill_init`, `vibe_skill_conductor`），实现了极高的系统可扩展性。
4. **跨平台兼容 (Universal Manifests)**：
   通过 `gemini-extension.json`, `.claude-plugin/plugin.json`, 以及 `.codex-plugin/plugin.json`，确保主流 AI 命令行工具均可开箱即用。

---

## 如何安装 (How to Install)

### 推荐方式：Gemini CLI 一键安装 (免 npm install)

得益于 Gemini CLI 的扩展机制，您可以直接从 GitHub Release 安装预构建的自包含版本，**完全无需手动运行 `npm install`**。

在您的终端中执行：
```bash
gemini plugin add https://github.com/anjia/vibe-cli
# 或者如果使用特定版本标签：
# gemini plugin add https://github.com/anjia/vibe-cli --ref v7.0.0
```
Gemini CLI 会自动下载最新 Release 中打包好的 `.zip` 或 `.tar.gz` 文件，直接识别 `gemini-extension.json` 并激活 Vibe 技能。

---

### 源码安装方式 (适用于 Claude, Codex 或二次开发)

由于 Vibe 是一个纯 MCP 插件，您**不需要**安装任何底层的 Rust 编译环境，只需具备 Node.js (v18+) 即可。

#### 1. 获取源码与安装依赖
```bash
git clone https://github.com/anjia/vibe-cli.git
cd vibe-cli/plugin/vibe
npm install --production
```

#### 2. 挂载到 AI 终端
根据您使用的 AI 助手，配置 Vibe 插件的绝对路径：

**对于 Claude Code 用户**：
如果您在包含 `.claude-plugin` 的 vibe 目录下运行，Claude 会自动加载。若要全局使用，请在您的 Claude 配置文件中添加 MCP 服务器配置：
```json
{
  "mcpServers": {
    "vibe": {
      "command": "node",
      "args": ["/绝对路径/to/vibe-cli/plugin/vibe/mcp-server.js"]
    }
  }
}
```

**对于 Codex 用户**：
您可以直接将官方的市场配置文件添加到 Codex 中，实现一键订阅和安装：
```bash
codex marketplace add https://github.com/anjia/vibe-cli/releases/latest/download/codex-marketplace.json
codex plugin install vibe
```

**对于本地测试的 Gemini CLI 用户**：
```bash
gemini --extension /绝对路径/to/vibe-cli/plugin/vibe
```

---

## 如何使用 (How to Use)

Vibe 的核心理念是**“您不需要手动敲击命令，只需要与 AI 对话”**。所有的底层操作都会由 AI 自动调用 MCP 工具完成。

请打开您挂载了 Vibe 插件的 AI 终端（Gemini / Claude 等），并在您的目标项目目录中尝试以下自然语言指令：

*   **场景 1：初始化工作区**
    > **您**：“请使用 Vibe 帮我初始化当前项目的工作区。”
    *(AI 会在后台自动调用 `vibe_skill_init` 工具，生成 `.vibe` 目录结构。)*

*   **场景 2：需求拆解与任务分配**
    > **您**：“我想开发一个用户登录功能。请启动 Vibe Conductor（编排者）技能，帮我把需求拆分成多个具体的 Vibe 任务。”
    *(AI 会调用 `vibe_skill_conductor` 和 `vibe_create_task`，将任务落盘到 `.vibe/tasks/` 中。)*

*   **场景 3：查看进度与状态**
    > **您**：“目前 .vibe 里的任务进度怎么样了？哪些任务被锁定了？”
    *(AI 会调用 `vibe_list_tasks` 和 `vibe_get_status` 返回结构化的当前项目状态。)*

*   **场景 4：执行任务与防冲突**
    > **您**：“开始执行任务 1。在修改代码前，请确保获取了该文件的锁。”
    *(AI Agent 的 Worker 角色 SOP 规定了它必须先调用 `vibe_acquire_lock`，修改完成后再调用 `vibe_release_lock`。)*

---

## 如何共建 (How to Contribute)

Vibe 的“动态技能发现引擎”让开发者可以极其简单地为 AI 扩充新的工作流能力。我们非常欢迎开源社区一起共建新的 Skill！

### 为 Vibe 添加新技能 (Skill)

假设您想添加一个名为 `vibe_skill_audit` 的代码审计技能，只需两步：

**Step 1: 定义技能元数据 (SKILL.md)**
在 `plugin/vibe/skills/` 目录下新建一个文件夹 `audit`，并创建 `SKILL.md` 文件。文件顶部必须包含 YAML 前置物质（Frontmatter）：

```markdown
---
name: vibe-audit
version: 0.1.0
description: Execute a security audit workflow on the current task files.
---

# vibe audit
（这里用自然语言描述该技能的边界、输入、输出和契约，AI 会阅读这段说明来理解如何使用它。）
```

**Step 2: 编写执行脚本 (scripts/audit.js)**
在 `plugin/vibe/scripts/` 目录下创建同名的 `audit.js` 文件。该文件必须导出一个 `runSkill` 函数：

```javascript
/**
 * @param {Object} params - MCP 工具调用时传入的参数
 * @param {string} workspaceRoot - 当前大模型所处的工作区绝对路径
 * @returns {string|Object} - 返回给大模型的执行结果
 */
async function runSkill(params, workspaceRoot) {
  // 在这里编写您的底层 Node.js 逻辑
  // 例如：读取 .vibe/tasks/，调用 ESLint，或者扫描依赖
  const result = `Audit completed safely for workspace: ${workspaceRoot}`;
  return result;
}

module.exports = { runSkill };
```

**Step 3: 测试您的技能**
1. 运行 `cd plugin/vibe && npm test` 确保没有破坏已有功能。
2. 启动 MCP 服务器或 AI 终端，您会发现 `vibe_skill_audit` 已经被自动注册为可用工具！

欢迎提交 Pull Request，将您优秀的运维、测试、发布等工作流技能贡献到 Vibe 的官方插件库中。

---

## 如何发布与分发 (How to Release)

如果您正在基于 Vibe 进行二次开发并准备发布新版本，推荐通过 **GitHub Releases** 来分发预构建包。这种打包方式可以让终端用户（如 Gemini CLI 用户）免去手动运行 `npm install` 的麻烦。

1. **打包项目**：
   进入 `plugin/vibe/` 目录，确保仅安装生产依赖：
   ```bash
   cd plugin/vibe
   npm ci --omit=dev  # 仅保留运行时依赖
   ```
   然后，将当前目录下的所有内容打包成 `.zip` 或 `.tar.gz` 压缩文件。
   *⚠️ 关键：`gemini-extension.json` 和 `package.json` 必须直接位于压缩包的根目录，不能嵌套在额外的文件夹内。*

2. **创建 GitHub Release**：
   在您的 GitHub 代码库的 Releases 页面发布一个新版本（如 `v7.0.0`），并将打包好的压缩文件上传为附件 (Assets)。

3. **分发给用户**：
   用户现在只需要在终端输入 `gemini plugin add https://github.com/您的名字/您的项目`，Gemini CLI 就会自动读取最新 Release，下载解压并加载内置的 MCP Server。

---

## 旧版本兼容性 (Legacy Compatibility)
此前的 Rust `vibe-cli` 已被标记为 Archive/Legacy（归档）。如果您仍在使用旧版本的基于 Terminal 注入的方式，代码位于 `crates/` 和 `apps/` 目录下，但强烈建议您拥抱更加安全、强大且主流的基于 MCP 的 Plugin-first 架构。