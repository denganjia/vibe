# Vibe

## 介绍 (Introduction)

Vibe 是一个 **Plugin-first (插件优先) 的多模型协作协议与轻量级运行时**，旨在将您的终端从单纯的字符输入框升级为分布式 AI 协作的物理调度室。

在最新的里程碑中，**Vibe 已经完全摒弃了原本笨重的独立 Rust CLI 编排器**，全面转型为基于 **Model Context Protocol (MCP)** 的纯粹 AI 终端插件。当前的主会话模型（如 Gemini、Claude 等）可以直接作为主控节点 (Conductor)，通过 MCP Server 提供的标准工具 (Tools)，自主地在项目 `.vibe` 目录中管理智能体、分配任务、获取状态并进行资源锁定。

核心特性：
- **基于 MCP 的纯插件架构**：提供标准化的 `vibe_create_task`、`vibe_get_status` 等 MCP 工具，与支持 MCP 的 AI 终端（如 Claude Desktop 等）无缝集成，开箱即用，**无需编译任何底层语言代码（不需要 Rust）**。
- **无缝多模型协作**：主模型负责澄清需求与拆解任务，将具体的编码执行权和代码审查权委派给专门配置的子模型（Executor 与 Reviewer），形成高质量交付闭环。
- **轻量化运行时 (Thin Runtime)**：任务落盘、进程锁和并发状态管理等底层原语均由轻量的 JavaScript 脚本实现，通过 MCP 暴露给模型使用，提供高确定性和极佳的可移植性。
- **透明的项目级工作区**：所有的运行状态、任务 JSON 和智能体定义都安全、静态地落盘于项目局部的 `.vibe` 目录，使得 AI 的思考与调度过程如同 Git 历史一样可被人类读取、审计和接续。

## 安装与配置 (Installation)

因为 Vibe 是一个纯插件产品，您**不需要**安装 Rust 环境。只需将 MCP Server 配置到您的 AI 环境即可。

### 1. 环境准备
确保您的系统中安装了：
- **Node.js** (v18+)：用于运行 Vibe 的 MCP Server 和底层控制脚本。

### 2. 获取源码与安装依赖
```bash
git clone https://github.com/anjia/vibe-cli.git
cd vibe-cli/plugin/vibe
npm install
```

### 3. 配置 MCP Server
如果您使用的是 Claude Desktop 或其他支持 MCP 的客户端，请在您的 MCP 配置文件中添加 Vibe Server。例如对于 Claude Desktop，可以在配置文件中添加：

```json
{
  "mcpServers": {
    "vibe": {
      "command": "node",
      "args": ["/path/to/vibe-cli/plugin/vibe/mcp-server.js", "/path/to/your/project"]
    }
  }
}
```
*(注：请将路径替换为您本地的实际路径，并确保传入正确的项目工作区路径)*

## 快速开始 (Quick Start)

进入您的目标项目目录中，启动配置了 Vibe MCP Server 的 AI 助手。您可以直接使用自然语言激活 Vibe 的工作流：

> "请使用 Vibe 的 MCP 工具帮我规划并实现这个需求，分派子任务完成具体的代码变更。"

作为 Conductor 的主模型，将自主使用 Vibe 提供的 MCP 工具完成以下操作：
1. **任务管理**：使用 `vibe_create_task` 创建子任务，并在 `.vibe/tasks/` 下生成任务定义。
2. **状态同步**：使用 `vibe_get_status` 和 `vibe_list_tasks` 获取任务进度。
3. **资源锁定**：工作智能体 (Worker) 修改文件前，使用 `vibe_acquire_lock` 声明意图并防止冲突，完成后通过 `vibe_release_lock` 释放锁。

在整个过程中，您无需手动干预底层命令，只需通过终端和主控模型进行高层级的架构交流与验收即可。

## 旧版本兼容性 (Legacy Compatibility)
此前的 Rust `vibe-cli` 已被标记为 Archive/Legacy，如果您仍在使用旧版本的基于 shell 调用的方式，请参考归档文档，但强烈建议升级到新的 MCP 架构。