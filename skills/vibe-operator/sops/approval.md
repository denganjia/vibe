# Human-in-the-Loop Approval SOP

## Purpose
This SOP defines the process for agents to request human approval for significant actions and how to handle the outcome.

## Logical Workflow

### 1. Triggering a Review
- **AI Discretion**: AI agents (primarily the Conductor) should trigger a review when:
  - A significant milestone is reached.
  - A high-risk command is about to be executed (based on `role.md` safety levels).
  - Structural changes to the codebase are proposed.
  - The agent is unsure of the next step.

### 2. Submitting the Plan
- **Tool**: Call `vibe_submit_plan`.
- **Requirements**:
  - `vibeId`: The ID of the agent requesting approval.
  - `plan`: A clear Markdown description of the proposed actions, risks, and expected outcomes.

### 3. Monitoring Approval Status
- **Tool**: Call `vibe_query_approval`.
- **States**:
  - `pending`: The agent MUST wait and NOT execute the proposed plan.
  - `approved`: The agent may proceed with the plan.
  - `rejected`: The agent MUST read the `rejection_reason` and revise the plan.

### 4. Handling Feedback
- If **Approved**: Execute the plan and report progress.
- If **Rejected**:
  - Analyze the reason for rejection.
  - Ask clarifying questions if necessary.
  - Modify the plan and re-submit via `vibe_submit_plan`.

## Best Practices
- **Wait Patiently**: Do not "nag" the user. Use a reasonable polling interval for `vibe_query_approval`.
- **Contextual Plans**: Ensure the plan submitted is self-contained and provides enough context for a human to make an informed decision without reading logs.
- **Safety First**: When in doubt, request approval.
