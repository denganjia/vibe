# Phase 21 UAT: .vibe 工作区与 Agent 定义

## Test Environment
- **Workspace**: `/Users/anjia/Documents/part-time/vibe-cli/uat_v21`
- **Runner**: Node.js v22.16.0

## Test Cases

### 1. Basic Initialization
- **Goal**: Confirm `vibe init` creates the standard workspace structure.
- **Action**: Run `node plugin/vibe/scripts/init.js uat_v21`.
- **Expected**: `.vibe/` directory exists with `agents/`, `tasks/`, `runs/`, `locks/`, `reviews/`, `logs/`, and `config.json`.
- **Result**: PASSED

### 2. Configuration Schema
- **Goal**: Verify `config.json` contains required nested fields.
- **Action**: Read `uat_v21/.vibe/config.json`.
- **Expected**: Contains `default_model`, `concurrency`, `lock_policy`, etc.
- **Result**: PASSED

### 3. Agent Template Richness
- **Goal**: Confirm Agent definitions include prompt and reference fields.
- **Action**: Read `uat_v21/.vibe/agents/planner.json`.
- **Expected**: Contains `prompt` and `reference` fields.
- **Result**: PASSED

### 4. Non-Destructive Update
- **Goal**: Ensure existing user configurations are not overwritten.
- **Action**: Modify `config.json`, run `init.js` again, then check `config.json`.
- **Expected**: User changes persist.
- **Result**: PASSED

### 5. Force Override
- **Goal**: Confirm `--force` correctly overwrites configurations.
- **Action**: Run `init.js --force`, check `config.json`.
- **Expected**: Configuration is reset to template defaults.
- **Result**: PASSED
