# Phase 17 Context: Bi-directional Flow & Reliability

## Core Challenge: The "Enter/Submit" Reliability
**Problem**: Commands injected into AI CLIs (Claude/Gemini) and signals injected into the Master's stdin often fail to trigger a "Submit" action, requiring manual intervention. This is the "Huge Problem" blocking autonomous workflows.
**Goal**: Achieve 100% automated trigger reliability.

## Implementation Decisions

### 1. Signal Transport: File-based Bus
- **Mechanism**: Move from TTY injection to a directory-based signal bus located at `.vibe/bus/`.
- **Reasoning**: Completely bypasses the TTY "Enter" problem for signaling. Worker A writes a file, and Master reads it directly. Reliability: 100%.
- **Workflow**: 
  - `vibe signal` writes a JSON file.
  - `vibe wait` uses file system watchers (via `notify` crate) to respond instantly.

### 2. Payload Standard: JSON Envelope
- **Structure**: All signals use a standard JSON envelope with `sender`, `name`, and `payload`.
- **Large Files**: Support `@path` references. `vibe wait` outputs the path, not the content.

### 3. Command Injection Fix (For Master -> Worker)
- **TTY Encoding**: Implement `vibe_core::adapter::TTYEncoder` to send precise byte sequences. 
- **The "Enter" Mystery**: We will research if `\r\n` or a specific escape sequence is needed for Raw Mode CLIs.
- **Throttling**: Add a 5ms delay between characters to ensure the target CLI's input buffer can keep up.

### 4. Routing: Hub-Spoke
- **Master as Router**: Worker -> Master communication via File Bus.
- **Control**: Master -> Worker communication via improved (reliable) TTY Injection.

## Locked Constraints
- **Reliability First**: Task 17.1 must prove "Enter" triggering works 100% before starting Task 17.2.
- **No Manual Keys**: The success metric is a Worker finishing a task and Master receiving the signal WITHOUT the user pressing any keys.

## Next Steps
- Research the exact byte sequence required by WezTerm/Tmux to trigger a "Submit" in Raw Mode.
- Implement the `.vibe/bus/` file-based signaling system.
- Add throttling to `adapter.inject_text`.
