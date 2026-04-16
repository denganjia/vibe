# REQUIREMENTS - Milestone 2

## Functional Requirements

### 1. Controlled Workflow (SCO-01)
- **Workflow Nodes**: The system must support a multi-node task flow (e.g., `Plan -> Review -> Implement`).
- **Human Confirmation**: Transitions between critical nodes must block until manual approval is received.
- **MCP Extension**: The MCP server should expose tools to "Submit Plan" and "Query Approval Status".
- **Visual Feedback**: The TUI should clearly indicate when an agent is "Waiting for Approval".

### 2. State Evolution & Migration (INF-01)
- **Automatic Migration**: On startup, the Master must check the database version and apply necessary schema updates.
- **No Data Loss**: Migrations must preserve existing Logical-to-Physical pane mappings.
- **Rollback Safety**: Failed migrations should not corrupt the existing database.

## Operations & Distribution

### 1. Binary Packaging (OPS-01)
- **Cross-platform**: Support macOS (Intel/M1), Linux (x86_64), and Windows.
- **Single Binary**: The TUI, Master, and MCP server should all be bundled in the `vibe` binary.
- **Release Automation**: Trigger binary builds on Git tags.

### 2. One-line Installation (OPS-02)
- **Install Script**: Provide a `curl | bash` (and PowerShell equivalent) script.
- **Auto-PATH**: The script should attempt to add `vibe` to the user's PATH.

## Non-Functional Requirements

- **Reliability**: Database migrations must be atomic.
- **UX**: TUI must remain responsive during high-volume workflow events.
- **Security**: Installation scripts must use HTTPS and verify checksums if possible.
