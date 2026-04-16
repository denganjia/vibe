# Domain Pitfalls

**Domain:** Terminal AI Agent Orchestration (vibe-cli)
**Researched:** 2024-05
**Context:** Adding declarative AI skills, multi-model workflows, and AI cross-checking.

## Critical Pitfalls

Mistakes that cause rewrites or major issues.

### Pitfall 1: 共享内存污染与级联失败 (Shared Memory Contamination & Cascading Failures)
**What goes wrong:** In a multi-model workflow, Master and Worker agents share a single, massive context window or message history. A hallucination, irrelevant tool output, or formatting error from one agent pollutes the shared context, causing all downstream agents to make wrong decisions.
**Why it happens:** Treating orchestration simply as passing the same giant prompt string between different LLMs instead of using isolated, explicit data states.
**Consequences:** The entire workflow fails cascade-style. Debugging becomes nearly impossible because errors propagate and amplify across agents, leading to unpredictable terminal commands.
**Prevention:** Isolate agent contexts. Use a strict Manager-Worker hierarchy where the Master agent parses outputs and passes only specific, strongly-typed data (via JSON schemas) to Worker agents, rather than the raw conversational history.
**Detection:** Downstream agents start outputting unrelated information, hallucinating context that belongs to other tasks, or complaining about malformed inputs in the TUI logs.

### Pitfall 2: 错误放大与“同质化”交叉检查 (Error Amplification & Homogeneous Cross-checking)
**What goes wrong:** Using the same underlying model family (e.g., GPT-4o) for both the execution agent (Worker) and the verification agent (Cross-checker). The Cross-checker confidently approves the Worker's incorrect output because both share identical biases and blind spots.
**Why it happens:** Attempting to implement cross-checking without considering model heterogeneity. This creates an "echo chamber" or "recursive agreement" effect. Research indicates this can amplify systemic errors rather than correct them.
**Consequences:** The system confidently executes destructive or incorrect terminal commands while bypassing safety mechanisms, giving users a false sense of security.
**Prevention:** Mandate model heterogeneity for cross-checking (e.g., use Claude 3.5 Sonnet for execution, and GPT-4o for verification). Furthermore, supplement LLM-based verification with deterministic checks (e.g., bash exit codes, dry-run validations, regex matching).
**Detection:** The TUI dashboard shows a 100% approval rate from the cross-checking agent, yet the actual terminal state ends up corrupted or user intents are frequently missed.

### Pitfall 3: 声明式技能的“黑盒”盲目执行 (Blind Execution of Declarative Skills)
**What goes wrong:** AI agents execute declarative skills (like manipulating Wezterm/Tmux panes or running complex bash scripts) based purely on static intent, without validating the dynamic terminal state or seeking human approval for critical actions.
**Why it happens:** Trusting the static description of a declarative tool to handle all dynamic runtime conditions, skipping the essential "plan-confirm-execute" workflow.
**Consequences:** The agent executes commands that are logically valid but contextually destructive (e.g., closing the wrong Tmux pane because the layout changed), leading to unrecoverable local environment corruption.
**Prevention:** Implement a strict "dry-run" mechanism. For any state-mutating declarative skill, the agent must generate a proposed execution plan first. For critical nodes, enforce a Human-in-the-loop (HITL) manual confirmation via the TUI before execution.
**Detection:** Irreversible actions happen automatically; TUI logs show command executions that the user didn't explicitly authorize.

## Moderate Pitfalls

### Pitfall 4: 技能描述模糊导致的“幻觉调用” (Vague Descriptions leading to Hallucination Calls)
**What goes wrong:** Declarative skills are defined with vague descriptions (e.g., `name: "manage_files", description: "Handles files"`). The AI agent struggles to determine when to use the skill or guesses incorrect parameters.
**Prevention:** Treat declarative skill definitions as strict API documentation. Use strong verbs, explicit JSON schemas for all parameters, and explicitly define the boundaries of when *not* to use the tool.

### Pitfall 5: 代理间死锁与无限协调 (Agent Deadlocks & Infinite Coordination)
**What goes wrong:** In multi-model orchestration, the Execution Agent waits for the Verifier's approval, but the Verifier asks the Execution Agent for clarification. They enter an infinite conversational loop, consuming massive amounts of tokens with zero terminal progress.
**Prevention:** Set strict programmatic iteration limits (e.g., max 3 back-and-forths). If consensus isn't reached, force an escalation to the user (Human-in-the-loop) or fail safely.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| **Declarative Skills Integration** | Vague descriptions leading to AI hallucination calls and invalid parameter passing. | Enforce strict JSON schemas and highly detailed, API-like descriptions for every skill. |
| **Multi-model Orchestration** | Shared memory contamination and flat hierarchies causing cascading failures. | Implement isolated state management and explicit, strongly-typed Master-Worker data contracts. |
| **AI Cross-checking** | Homogeneous models echoing each other's mistakes, amplifying errors. | Mandate heterogeneous models (e.g., Claude + GPT) and add deterministic validation steps (e.g., exit codes). |

## Sources

- [HIGH] Industry Research on Multi-Agent Collaboration: [Google DeepMind: Error Amplification in Multi-Agent Systems](https://arxiv.org/)
- [HIGH] Observability & System Design for Agents: [Galileo.ai: Pitfalls in Multi-Agent System Design](https://galileo.ai/)
- [MEDIUM] Agent Orchestration Patterns: OpenAI & LangChain architectural guidelines on Manager-Worker patterns and Tool calling schemas.