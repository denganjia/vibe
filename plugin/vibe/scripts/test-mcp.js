const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

/**
 * MCP Verification Script
 * 
 * Verifies that the MCP server:
 * 1. Starts correctly
 * 2. Responds to initialization
 * 3. Correctely executes the vibe_ping tool
 * 4. Correctely executes workspace tools: create_task, acquire_lock, get_status, release_lock
 */

async function testMcp() {
  console.log("Starting MCP Server verification...");

  // Setup test workspace environment
  const workspaceRoot = process.cwd();
  const vibeDir = path.join(workspaceRoot, '.vibe');
  
  // Clean up previous test runs if any
  if (fs.existsSync(vibeDir)) {
    // We don't delete everything to avoid destroying dev environment, 
    // but we should be aware of potential state pollution.
    console.log("Found existing .vibe directory, proceeding with caution.");
  }

  const serverPath = path.join(__dirname, '..', 'mcp-server.js');
  const server = spawn('node', [serverPath], {
    stdio: ['pipe', 'pipe', 'inherit'],
    env: { ...process.env, NODE_ENV: 'test' }
  });

  let outputBuffer = '';
  server.stdout.on('data', (data) => {
    outputBuffer += data.toString();
  });

  // Helper to send a JSON-RPC request and wait for specific response
  const callTool = (name, args, id) => {
    return new Promise((resolve, reject) => {
      console.log(`Calling tool: ${name}...`);
      const request = JSON.stringify({
        jsonrpc: "2.0",
        id,
        method: "tools/call",
        params: { name, arguments: args }
      }) + "\n";
      
      server.stdin.write(request);

      const timeout = setTimeout(() => reject(new Error(`Timeout waiting for ${name} response`)), 5000);
      const interval = setInterval(() => {
        // Find the line that matches our ID
        const lines = outputBuffer.split('\n');
        for (const line of lines) {
          if (line.trim() === '') continue;
          try {
            const response = JSON.parse(line);
            if (response.id === id) {
              clearInterval(interval);
              clearTimeout(timeout);
              // Remove this line from buffer for future calls
              outputBuffer = lines.filter(l => l !== line).join('\n');
              resolve(response);
              return;
            }
          } catch (e) {
            // Not a complete JSON or different line
          }
        }
      }, 100);
    });
  };

  const sendInitialize = (id) => {
    return new Promise((resolve, reject) => {
      console.log("Sending initialize request...");
      const request = JSON.stringify({
        jsonrpc: "2.0",
        id,
        method: "initialize",
        params: {
          protocolVersion: "2024-11-05",
          capabilities: {},
          clientInfo: { name: "test-client", version: "1.0.0" }
        }
      }) + "\n";
      
      server.stdin.write(request);

      const timeout = setTimeout(() => reject(new Error("Timeout waiting for initialize response")), 5000);
      const interval = setInterval(() => {
        const lines = outputBuffer.split('\n');
        for (const line of lines) {
          if (line.trim() === '') continue;
          try {
            const response = JSON.parse(line);
            if (response.id === id) {
              clearInterval(interval);
              clearTimeout(timeout);
              outputBuffer = lines.filter(l => l !== line).join('\n');
              resolve(response);
              return;
            }
          } catch (e) {}
        }
      }, 100);
    });
  };

  try {
    // 1. Initialize
    await sendInitialize(1);
    console.log("Initialization successful.");

    // 2. vibe_ping
    const pingRes = await callTool("vibe_ping", {}, 2);
    if (!JSON.stringify(pingRes).includes('pong')) {
      throw new Error("vibe_ping failed: " + JSON.stringify(pingRes));
    }
    console.log("vibe_ping successful.");

    // 3. vibe_create_task
    const taskRes = await callTool("vibe_create_task", {
      goal: "Test MCP integration",
      context: ["@README.md"],
      file_scope: ["test-file.txt"]
    }, 3);
    
    const taskContent = taskRes.result.content[0].text;
    const taskIdMatch = taskContent.match(/ID: ([a-f0-9]+)/);
    if (!taskIdMatch) {
      throw new Error("Failed to extract Task ID from response: " + taskContent);
    }
    const taskId = taskIdMatch[1];
    console.log(`vibe_create_task successful. Task ID: ${taskId}`);

    // Verify file exists
    const taskFile = path.join(vibeDir, 'tasks', `${taskId}.json`);
    if (!fs.existsSync(taskFile)) {
      throw new Error(`Task file not found at ${taskFile}`);
    }

    // 4. vibe_acquire_lock
    await callTool("vibe_acquire_lock", {
      taskId,
      filePaths: ["test-file.txt"]
    }, 4);
    
    const safeName = Buffer.from("test-file.txt").toString('base64').replace(/\//g, '_');
    const lockFile = path.join(vibeDir, 'locks', `${safeName}.lock`);
    if (!fs.existsSync(lockFile)) {
      throw new Error(`Lock file not found at ${lockFile}`);
    }
    console.log("vibe_acquire_lock successful.");

    // 5. vibe_get_status
    const statusRes = await callTool("vibe_get_status", { taskId }, 5);
    if (!statusRes.result.content[0].text.includes('queued')) {
      throw new Error("Unexpected status: " + statusRes.result.content[0].text);
    }
    console.log("vibe_get_status successful.");

    // 6. vibe_release_lock
    await callTool("vibe_release_lock", { taskId }, 6);
    if (fs.existsSync(lockFile)) {
      throw new Error(`Lock file still exists at ${lockFile}`);
    }
    console.log("vibe_release_lock successful.");

    console.log("\nAll MCP tool verifications passed!");

  } catch (error) {
    console.error("\nVerification failed:", error.message);
    if (outputBuffer) console.error("Remaining output buffer:", outputBuffer);
    server.kill();
    process.exit(1);
  } finally {
    server.kill();
  }
}

testMcp();
