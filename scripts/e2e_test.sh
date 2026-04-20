#!/bin/bash
set -euo pipefail

export VIBE_BIN="$(pwd)/target/debug/vibe-cli"
export TEST_DIR="$(pwd)/e2e_test_workdir"

# Force cleanup before starting
rm -rf "$TEST_DIR"

cleanup() {
    echo "Cleaning up..."
    # rm -rf "$TEST_DIR"
}
trap cleanup EXIT

echo "--- Vibe E2E Integration Test ---"

mkdir -p "$TEST_DIR/.vibe/roles"
mkdir -p "$TEST_DIR/bin"
mkdir -p "$TEST_DIR/.vibe/state"
echo "{}" > "$TEST_DIR/.vibe/state/panes.json"
echo "# Scanner Persona" > "$TEST_DIR/.vibe/roles/Scanner.md"

mkfifo "$TEST_DIR/wait_pipe"

cat << 'MOCK' > "$TEST_DIR/mock_agent.sh"
#!/bin/bash
exec >> "$TEST_DIR/mock_agent.log" 2>&1
echo "Mock Agent: Received persona on stdin"
while read line || [ -n "$line" ]; do
    echo "Persona line: $line"
done

echo "Mock Agent: Simulating work..."
sleep 2

echo "Mock Agent: Reporting success..."
"$VIBE_BIN" report --status success --message "E2E Scan complete"

echo "Mock Agent: Signaling done..."
"$VIBE_BIN" signal done
MOCK
chmod +x "$TEST_DIR/mock_agent.sh"

cat << 'WEZ' > "$TEST_DIR/bin/wezterm"
#!/bin/bash
if [ "$1" == "cli" ] && [ "$2" == "split-pane" ]; then
    echo "81"
elif [ "$1" == "cli" ] && [ "$2" == "send-text" ] && [ "$3" == "--pane-id" ] && [ "$4" == "81" ]; then
    TEXT="$5"
    echo -n "$TEXT" >> "$TEST_DIR/injected.txt"
    if [[ "$TEXT" == *mock_agent.sh* ]]; then
        # Signal the main test script to execute the mock agent
        touch "$TEST_DIR/ready_to_run"
    fi
    exit 0
elif [ "$1" == "cli" ] && [ "$2" == "send-text" ] && [ "$3" == "--pane-id" ] && [ "$4" == "80" ]; then
    TEXT="$5"
    echo "$TEXT" > "$TEST_DIR/wait_pipe" &
    exit 0
elif [ "$1" == "cli" ] && [ "$2" == "list" ] && [ "$3" == "--format" ] && [ "$4" == "json" ]; then
    echo '[{"pane_id": 80, "window_id": 1, "tab_id": 1, "workspace": "test"}, {"pane_id": 81, "window_id": 1, "tab_id": 1, "workspace": "test"}]'
    exit 0
else
    exit 0
fi
WEZ
chmod +x "$TEST_DIR/bin/wezterm"

export PATH="$TEST_DIR/bin:$PATH"
export WEZTERM_PANE="80"

cat << CONF > "$TEST_DIR/.vibe/config.json"
{
  "agent_command": "$TEST_DIR/mock_agent.sh"
}
CONF

cd "$TEST_DIR"

echo "Step 1: Spawning Scanner agent..."
"$VIBE_BIN" spawn Scanner

echo "Step 1.5: Executing spawned agent in background..."
# This reliably simulates the terminal environment executing the injected script
# independent of the fake wezterm process.
(
    # Wait for the text to be injected
    for i in {1..50}; do
        if [ -f "$TEST_DIR/ready_to_run" ]; then
            break
        fi
        sleep 0.1
    done
    
    if [ -f "$TEST_DIR/ready_to_run" ]; then
        cat "$TEST_DIR/injected.txt" | VIBE_MASTER_ID=80 WEZTERM_PANE=81 bash "$TEST_DIR/mock_agent.sh"
    else
        echo "FAIL: ready_to_run signal file not created by wezterm mock" >&2
        exit 1
    fi
) &
AGENT_PID=$!

echo "Step 2: Waiting for 'done' signal (timeout 20s)..."
"$VIBE_BIN" wait done --timeout 20 < "$TEST_DIR/wait_pipe" > signal_payload.txt &
WAIT_PID=$!

# We need something to keep the pipe open so it doesn't close immediately before wezterm writes
tail -f /dev/null > "$TEST_DIR/wait_pipe" &
TAIL_PID=$!

wait $WAIT_PID
kill $TAIL_PID || true
wait $AGENT_PID || true

echo "Step 3: Validating results..."

if [ -s signal_payload.txt ]; then
    echo "Signal received:"
    cat signal_payload.txt
else
    echo "FAIL: Signal not received or wait failed."
    exit 1
fi

if command -v python3 &>/dev/null; then
    STATUS=$(python3 -c "import json; print(json.load(open('.vibe/state/panes.json'))['81']['status'])" 2>/dev/null || echo "unknown")
    SUMMARY=$(python3 -c "import json; print(json.load(open('.vibe/state/panes.json'))['81']['summary'])" 2>/dev/null || echo "unknown")
else
    STATUS=$(grep "\"status\"" .vibe/state/panes.json | head -n 1 | cut -d'"' -f4)
    SUMMARY=$(grep "\"summary\"" .vibe/state/panes.json | head -n 1 | cut -d'"' -f4)
fi

echo "Detected Agent Status: $STATUS"
echo "Detected Agent Summary: $SUMMARY"

if [ "$STATUS" == "success" ] && [ "$SUMMARY" == "E2E Scan complete" ]; then
    echo "SUCCESS: E2E Autonomous flow verified."
else
    echo "FAIL: State data incorrect (Status: $STATUS, Summary: $SUMMARY)."
    exit 1
fi

echo "--- E2E Test PASSED ---"
