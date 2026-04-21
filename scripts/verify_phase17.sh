#!/bin/bash
set -e

# E2E Verification Script for Phase 17
# This script tests the file-based signal bus reliability.

echo "--- Phase 17 E2E Verification ---"

# 1. Clean up old bus signals
rm -rf .vibe/bus/*.json || true

# 2. Test local signal/wait loop (Simulating Master-Worker on same machine)
echo "[Test 1/2] Local Signal/Wait Loop..."
SIGNAL_NAME="verify_p17_$(date +%s)"
PAYLOAD='{"status": "ok", "step": 17}'

# Run wait in background
./target/release/vibe-cli wait "$SIGNAL_NAME" --timeout 5 > test_output.txt &
WAIT_PID=$!

# Give it a moment to start polling
sleep 1

# Send signal
./target/release/vibe-cli signal "$SIGNAL_NAME" "$PAYLOAD"

# Wait for background process to finish
wait $WAIT_PID

# Verify output
if grep -q "step\": 17" test_output.txt; then
    echo "SUCCESS: Signal received with correct payload via FileBus."
else
    echo "FAILED: Signal not received or payload mismatch."
    cat test_output.txt
    exit 1
fi

# 3. Test @path payload
echo "[Test 2/2] Large Payload (@path) support..."
echo '{"large": true, "data": "somedata"}' > large_payload.json
./target/release/vibe-cli wait large_sig --timeout 2 > path_output.txt &
WAIT_PID_2=$!
sleep 1
./target/release/vibe-cli signal large_sig @large_payload.json
wait $WAIT_PID_2

if grep -q "large\": true" path_output.txt; then
    echo "SUCCESS: Large payload via @path verified."
else
    echo "FAILED: Large payload failed."
    cat path_output.txt
    exit 1
fi

echo "--- ALL TESTS PASSED ---"
rm test_output.txt path_output.txt large_payload.json || true
