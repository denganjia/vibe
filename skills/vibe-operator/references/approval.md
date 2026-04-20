# Human-in-the-Loop Approval SOP (Stateless Bus)

## Purpose
This SOP defines the process for agents to request human approval for significant actions using the signal-bus mechanism.

## Logical Workflow

### 1. Triggering a Review
- AI agents should trigger a review when:
  - A significant milestone is reached.
  - Structural changes to the codebase are proposed.
  - The agent is about to execute high-risk commands.

### 2. Requesting Approval
- **Protocol**:
  1. The agent prints the proposed plan clearly to the terminal.
  2. The agent calls `vibe report --status blocked --message "Waiting for human approval of [Task Name]"` to update the global state.
  3. The agent calls `vibe wait approved` to enter a blocking state.

### 3. Human Action
- The human reviews the plan in the agent's pane.
- To approve: The human (or another agent) executes `vibe signal approved` from any pane.
- To reject/provide feedback: The human can use `vibe inject <ID> "Feedback message"` followed by `vibe signal rejected`.

### 4. Handling the Signal
- **If `approved` received**: The agent resumes execution.
- **If `rejected` received**: The agent parses the feedback (if any), updates the plan, and requests approval again.

## Best Practices
- **Clear Context**: Before calling `vibe wait`, ensure the last lines in the terminal clearly describe WHAT the user needs to approve.
- **Unique Signals**: For parallel approvals, use specific signal names like `approve_feature_x`.
