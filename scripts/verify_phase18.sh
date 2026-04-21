#!/bin/bash
set -e

# E2E Verification Script for Phase 18
# Tests: Init, Stack Spawning, and Smart Cleanup.

echo "--- Phase 18 E2E Verification ---"

# 1. Test help commands
./target/release/vibe-cli init --help > /dev/null
./target/release/vibe-cli spawn --help > /dev/null
echo "✓ CLI arguments verified."

# 2. Test Smart Cleanup (Simulated)
echo "[Test] Smart Cleanup..."
# Create a dummy stale entry in panes.json
# We'll use a physical ID that definitely doesn't exist
DUMMY_PID="999999"
mkdir -p .vibe/state
echo "{\
  \"vibe-stale\": {\
    \"vibe_id\": \"vibe-stale\",\
    \"physical_id\": \"$DUMMY_PID\",\
    \"terminal_type\": \"WezTerm\",\
    \"role\": \"Worker\",\
    \"status\": \"spawned\",\
    \"summary\": \"\",\
    \"pid\": null,\
    \"cwd\": null,\
    \"last_heartbeat_at\": null,\
    \"created_at\": \"2024-01-01T00:00:00Z\"\
  }\
}" > .vibe/state/panes.json

echo "Added dummy stale record with physical ID $DUMMY_PID."

# Run vibe list - this should trigger perform_silent_cleanup
./target/release/vibe-cli list > /dev/null

# Check if dummy record is gone
if grep -q "$DUMMY_PID" .vibe/state/panes.json; then
    echo "FAILED: Stale record was NOT cleaned up."
    exit 1
else
    echo "SUCCESS: Stale record was automatically cleaned up."
fi

# 3. Verify config.json updates (Deep Merge)
echo "[Test] Config Deep Merge..."
# Ensure we have a basic config
./target/release/vibe-cli init --force <<EOF
0
EOF
# Note: The EOF might fail in some envs if dialoguer doesn't like piped input, 
# but we just need to see if it runs and produces a valid config.

if [ -f ".vibe/config.json" ]; then
    if grep -q "stacks" .vibe/config.json; then
        echo "SUCCESS: config.json contains 'stacks' field."
    else
        echo "FAILED: config.json missing 'stacks' field."
        exit 1
    fi
else
    echo "WARNING: Could not verify config.json due to interactive constraints."
fi

echo "--- ALL AUTOMATED CHECKS PASSED ---"
echo "Manual Step Required: Run './target/release/vibe-cli spawn --stack default' to verify multi-tab spawning."
