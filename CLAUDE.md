<!-- GSD:project-start source:PROJECT.md -->
## Project

**vibe-cli**

`vibe-cli` 是一个基于 Rust 构建的物理调度层，专为终端（Wezterm/Tmux）中的 AI Agent 设计。它将 AI Agent 转化为能够自主操控多窗格协作、共享上下文并实现任务闭环的“终端虚拟操作员”，让开发者通过指挥 AI 团队在真实的窗口与文件系统中“并联作业”。

**Core Value:** 打破 AI 与本地开发环境之间的“次元壁”，将终端从单纯的字符输入框升级为 AI 协作的物理调度室。

### Constraints

- **Tech Stack**: Rust — 确保作为系统级工具的稳定性和分发效率。
- **Dependency**: Wezterm/Tmux CLI — MVP 依赖于终端自带的 CLI 工具（如 `wezterm cli`）。
- **Environment**: MacOS/Linux — 终端开发者集中的主流操作系统。
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- Rust 2024 edition - Backend/CLI logic
- None detected
## Runtime
- Rust Toolchain
- Cargo
- Lockfile: missing (new project)
## Frameworks
- None detected (Standard Library only)
- Built-in `cargo test`
- Cargo
## Key Dependencies
- None (Standard Library only)
- None
## Configuration
- Not configured
- `Cargo.toml`
## Platform Requirements
- Rust SDK
- Binary executable
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Patterns
- snake_case for all Rust source files.
- snake_case for function names (standard Rust).
- snake_case for variable names.
- PascalCase for structs, enums, and traits (standard Rust).
## Code Style
- `rustfmt` (standard).
- `clippy` (recommended standard).
## Import Organization
- None detected.
## Error Handling
- Standard Rust Result and Option types.
## Logging
- `println!` for CLI output.
## Comments
- Complexity documentation.
- Rust doc comments (`///` and `//!`) for API documentation.
## Function Design
## Module Design
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- Single entry point
- Minimal footprint
- Standard Rust structure
## Layers
- Purpose: Entry point for the CLI
- Location: `src/main.rs`
- Contains: `main()` function
- Depends on: Standard Library
- Used by: User (via CLI)
## Data Flow
- Stateless
## Key Abstractions
- No custom abstractions implemented yet.
## Entry Points
- Location: `src/main.rs`
- Triggers: Execution of the compiled binary
- Responsibilities: Initialize and run the application logic
## Error Handling
- None implemented yet
## Cross-Cutting Concerns
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->
## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, or `.github/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
