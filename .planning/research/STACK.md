# Technology Stack

**Project:** vibe-cli
**Researched:** 2024-05-20

## Recommended Stack

### Core Framework (Skills Definition)
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| JSON Schema | 2020-12 | Parameter Validation | Standard, robust, supported by most LLM APIs for structured output. |
| Model Context Protocol (MCP) | v1.0 | Skill Interface | Industry standard for exposing tools/skills to LLMs. Provides a consistent language-agnostic interface. |
| Rust Serde/Valico | Latest | Schema Validation | High-performance JSON schema validation in Rust before executing commands. |

### Architecture (Workflows)
| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Stateful DAG (Custom) | - | Workflow Execution | We already have an SQLite state layer. A custom Directed Acyclic Graph (DAG) executor on top of it fits the `vibe-cli` footprint better than integrating heavy external engines like Temporal or cadence. |

### Supporting Libraries
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| async-trait | Latest | Abstracting Worker Roles | To define shared behaviors for different Agent types (e.g., Planner, Executor, Reviewer). |
| tokio | Latest | Parallel Execution | When orchestrating parallel LLM calls for multi-model cross-checking. |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Workflow Engine | Custom DAG (SQLite) | Temporal/Cadence | Overkill for a local CLI tool. Adds external dependencies (server setup). |
| Skill Definition | JSON Schema/MCP | Python classes/AutoGen | `vibe-cli` is Rust-based. Wrapping Python agent frameworks would complicate the deployment and architecture significantly. |

## Sources

- MCP Specification (Anthropic)
- LangChain / LangGraph standard tooling (DAGs and Stateful Graphs)