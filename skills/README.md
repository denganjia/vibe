# Vibe Skills

This directory contains the skill definitions for AI agents using Vibe-CLI. These skills provide the necessary metadata, roles, SOPs, and templates for agents to operate effectively in a stateless terminal orchestration environment.

## Available Skills

### [Vibe-Operator](./vibe-operator/SKILL.md)
**Core System Skill**

The fundamental skill for multi-agent orchestration via a Stateless Bus. It enables AI agents to:
- **Orchestrate**: Manage terminal panes and workspaces dynamically using `vibe spawn`.
- **Synchronize State**: Maintain global awareness via `.vibe/state/panes.json`.
- **Collaborate Autonomously**: Use `vibe signal` and `vibe wait` for asynchronous task closure.

For detailed documentation, see the [Vibe-Operator Skill Definition](./vibe-operator/SKILL.md).
