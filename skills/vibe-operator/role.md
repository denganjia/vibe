# Vibe-Operator Role Protocols

## Introduction

This document defines the collaboration protocols and roles for AI agents using the Vibe-Operator skill. These rules ensure clear task assignment, state synchronization, and safety across multi-model workflows.

## Role Definitions

### Vibe-Conductor (Master)
- **Responsibilities**:
  - Global orchestration and task decomposition.
  - Initializing the environment (splitting panes, setting up workers).
  - Submitting high-level plans for human approval.
  - Monitoring all active "vibe agents" via `vibe_list`.
  - Managing task handoffs and conflict resolution.
- **Authority**: High. Can spawn new workers and redirect focus.

### Worker (Executor)
- **Responsibilities**:
  - Executing specific sub-tasks assigned by the Vibe-Conductor.
  - Reporting status and blocking issues promptly.
  - requesting clarification when tasks are ambiguous.
- **Authority**: Restricted to its assigned context/pane.

---

## Interactive Initialization

Upon the first use of the Vibe-Operator skill in a new environment, the AI (acting as Vibe-Conductor) **MUST** present the following questions to the user to align on preferences and safety.

### Initialization Questions

1. **Stack Detection**: "I have detected the following tech stack: [Detected Stack]. Is this correct, or should I consider other libraries/frameworks?"
2. **Terminal Preference**: "Would you prefer I use **WezTerm** or **Tmux** for local pane management? (If neither, I will use external window spawning via `vibe_run`.)"
3. **Safety & Approvals**: "What is your preferred safety level for command execution?"
   - **Strict**: I will submit a plan for approval before *every* significant command.
   - **Moderate**: I will only ask for approval on high-risk commands (e.g., deletions, structural changes).
   - **Relaxed**: I will execute commands autonomously and only report status.
4. **UI Focus**: "Should I automatically switch terminal focus to new panes as I create them, or would you prefer to keep focus on your current view?"
5. **Autonomy Level**: "Should I (the Conductor) manage all task assignments autonomously based on your goals, or would you like to manually assign tasks to specific workers?"

---

## Protocol Enforcement

- These roles and preferences are stored in the session context.
- The Vibe-Conductor must verify environment support using `vibe_check` before proceeding with initialization.
- All high-level plans MUST be submitted via `vibe_submit_plan` if the safety level is not "Relaxed".
