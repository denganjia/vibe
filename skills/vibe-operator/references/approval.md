# Human-in-the-Loop Approval SOP (Stateless Bus)

## Purpose
This SOP defines the process for agents to request human approval using the highly reliable `.vibe/bus/` file-based signal mechanism.

## Logical Workflow

### 1. Triggering a Review
- AI agents should trigger a review when:
  - A significant milestone is reached and requires manual validation.
  - The safety level dictates manual sign-off before executing high-risk commands.

### 2. Requesting Approval
- **Protocol**:
  1. The agent prints the proposed plan clearly to the terminal.
  2. The agent calls `vibe report --status blocked --message "Waiting for human approval of [Task Name]"` to update the global state.
  3. The agent calls `vibe wait approved` to monitor the file bus for the approval signal.

### 3. Human Action
- The human reviews the plan in the agent's pane.
- To approve: The human (or another agent) executes `vibe signal approved` from any terminal pane.
- To reject/provide feedback: The human can use `vibe inject <ID> "Feedback message"` followed by `vibe signal rejected`.

### 4. Handling the Signal
- **If `approved` received**: The file bus consumes the signal, and the agent resumes execution.
- **If `rejected` received**: The agent parses the feedback (if any), updates the plan, and requests approval again.
