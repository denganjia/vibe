# TESTING.md

## Frameworks
- **Rust `cargo test`**: Standard unit and integration testing framework.
- **Bash**: Custom E2E test scripts.

## Test Structure
- **Unit Tests**: Typically located within the source files (in `mod tests` blocks).
- **Integration Tests**: Located in `crates/vibe-core/tests` for testing library-wide functionality.
- **E2E Tests**: Managed via `scripts/e2e_test.sh`. This script sets up a mock environment (`e2e_test_workdir`), spawns agents, and validates the full signal-bus lifecycle.

## Mocking
- **Terminal Mocking**: E2E tests use a fake `wezterm` binary in the `PATH` to intercept and validate terminal orchestration commands.
- **Agent Mocking**: `mock_agent.sh` is used to simulate agent behavior (consuming persona, reporting status, signaling completion).

## Continuous Integration
- **GitHub Actions**: Configured in `.github/workflows/release.yml`, though primarily focused on releases. E2E tests are currently intended for local execution.
