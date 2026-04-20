# Vibe-CLI

Vibe-CLI 是一个强大的多智能体（Multi-agent）终端编排工具，旨在将您的终端转变为一个自主协作的开发环境。通过 **Stateless Bus（无状态总线）** 架构，它允许不同的 AI 智能体在独立的终端窗格中协同工作。

## 核心特性

- **多窗格编排**：支持 WezTerm 和 Tmux，自动管理终端布局。
- **自主派生 (Autonomous Spawning)**：智能体可以根据任务需求自主创建子智能体。
- **无状态通信**：基于信号（Signal）和等待（Wait）机制的跨进程同步。
- **角色化系统**：预设 Conductor, Worker, Evaluator 等角色，适配不同的开发场景。

## 安装指南

### 本地构建
确保您的系统已安装 Rust 工具链。

```bash
cargo build --release
```

编译后的二进制文件位于 `target/release/vibe`。

### 脚本安装
```bash
# macOS / Linux
curl -sSL https://raw.githubusercontent.com/anjia/vibe-cli/master/scripts/install.sh | bash
```

## 快速开始

1. **检查环境**：
   ```bash
   vibe check
   ```

2. **启动主控智能体**：
   ```bash
   vibe spawn --role Conductor
   ```

3. **同步信号**：
   - 智能体 A：`vibe signal task_complete`
   - 智能体 B：`vibe wait task_complete`

## 技能系统

项目内置了 `vibe-operator` 技能，通过 `vibe spawn` 自动注入 persona 及其相关 SOP 和模板，指导 AI 智能体遵循标准化的协作流程。

## 许可证

[Apache-2.0](LICENSE)
