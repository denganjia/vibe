# Phase 14: Signal Bus Implementation (Bus Core)

## Objective
Implement `vibe signal` and `vibe wait` asynchronous communication mechanism to allow sub-agents and master agents to coordinate autonomously via signals.

## Requirements
- **BUS-04**: Implement `vibe signal <MSG>`. Notify all subscribers/waiters.
- **BUS-05**: Implement `vibe wait [SIGNAL]`. Block until signal arrives.
- **BUS-06**: Isolated routing via UDS path based on project hash (`/tmp/vibe-<hash>.sock`).

## Key Decisions
- **Signal Bus State**: `MasterServer` will maintain a `waiters` map (`Arc<Mutex<HashMap<String, Vec<mpsc::Sender<serde_json::Value>>>>>`) to track clients waiting for specific signals.
- **Message Flow**:
    1. Client A sends `Wait { signal_name, timeout_ms }`.
    2. Server registers Client A's connection as a waiter for `signal_name`.
    3. Client B sends `Signal { name, payload }`.
    4. Server finds all waiters for `name`, sends `SignalFired { name, payload }` to them, and removes them from the map.
- **UDS Path**: Use `/tmp/vibe-<hash>.sock` to avoid UDS path length limits and provide project isolation.

## Technical Details
- **Hashing**: Use a stable hash of the absolute path to the `.vibe` directory.
- **Protocol**: 
    - `Message::Signal(SignalInfo)`
    - `Message::Wait(WaitInfo)`
    - `Message::SignalFired(SignalInfo)` (New)
- **CLI**: Add `signal` and `wait` subcommands to `vibe-cli`.

## Traceability
- BUS-04 -> Plan 01, Plan 02
- BUS-05 -> Plan 01, Plan 02
- BUS-06 -> Plan 01
