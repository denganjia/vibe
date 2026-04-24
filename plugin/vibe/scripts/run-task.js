const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { setTaskStatus } = require('./status');

/**
 * Task Execution Script with Dependency Enforcement
 */

function generateRunId() {
  return `run_${crypto.randomBytes(4).toString('hex')}`;
}

async function checkDependencies(workspaceRoot, task) {
  if (!task.dependencies || task.dependencies.length === 0) {
    return true;
  }

  const tasksDir = path.join(workspaceRoot, '.vibe', 'tasks');
  const unmet = [];

  for (const depId of task.dependencies) {
    const depPath = path.join(tasksDir, `${depId}.json`);
    if (!fs.existsSync(depPath)) {
      unmet.push({ id: depId, reason: 'not found' });
      continue;
    }
    const depTask = JSON.parse(fs.readFileSync(depPath, 'utf8'));
    if (depTask.status !== 'completed') {
      unmet.push({ id: depId, reason: depTask.status });
    }
  }

  if (unmet.length > 0) {
    console.error(`Blocked by unmet dependencies: ${unmet.map(d => `${d.id} (${d.reason})`).join(', ')}`);
    setTaskStatus(workspaceRoot, task.id, 'blocked');
    return false;
  }

  return true;
}

function runTaskSync(workspaceRoot, taskId, agentIdOverride = null) {
  return new Promise(async (resolve, reject) => {
    const vibeDir = path.join(workspaceRoot, '.vibe');
    const taskPath = path.join(vibeDir, 'tasks', `${taskId}.json`);
    
    if (!fs.existsSync(taskPath)) return reject(new Error(`Task ${taskId} not found`));
    
    const task = JSON.parse(fs.readFileSync(taskPath, 'utf8'));
    
    // Cycle Count Check (Max 3 attempts)
    const runCount = (task.run_count || 0) + 1;
    if (runCount > 3) {
      console.error(`Task ${taskId} has reached the maximum 3-cycle threshold.`);
      setTaskStatus(workspaceRoot, taskId, 'failed', { 
        error: 'Maximum 3-cycle threshold exceeded' 
      });
      return reject(new Error(`Task ${taskId} exceeded max run cycles`));
    }

    // Dependency Check
    const depsOk = await checkDependencies(workspaceRoot, task);
    if (!depsOk) {
      return reject(new Error(`Task ${taskId} is blocked by dependencies`));
    }

    // Update status and increment run_count
    setTaskStatus(workspaceRoot, taskId, 'running', { run_count: runCount });

    const agentId = agentIdOverride || task.executor || 'executor';
    const agentPath = path.join(vibeDir, 'agents', `${agentId}.json`);
    
    if (!fs.existsSync(agentPath)) return reject(new Error(`Agent ${agentId} not found`));
    
    const agent = JSON.parse(fs.readFileSync(agentPath, 'utf8'));
    const runId = generateRunId();
    const startTime = new Date().toISOString();
    
    const logsDir = path.join(vibeDir, 'logs');
    if (!fs.existsSync(logsDir)) fs.mkdirSync(logsDir, { recursive: true });
    
    const logFilePath = path.join(logsDir, `${runId}.log`);
    const logStream = fs.createWriteStream(logFilePath);
    
    logStream.write(`--- VIBE RUN START: ${runId} ---\n`);
    logStream.write(`Task ID: ${taskId}\n`);
    logStream.write(`Agent ID: ${agentId}\n`);
    logStream.write(`Command: ${agent.model_command}\n`);
    logStream.write(`Start Time: ${startTime}\n`);
    logStream.write(`---\n\n`);

    const commandParts = agent.model_command.split(' ');
    const cmd = commandParts[0];
    const args = commandParts.slice(1);
    
    const child = spawn(cmd, args, {
      cwd: workspaceRoot,
      env: { ...process.env, VIBE_TASK_ID: taskId, VIBE_RUN_ID: runId },
      shell: true,
      stdio: ['pipe', 'pipe', 'pipe']
    });

    child.stdin.write(JSON.stringify({ task, agent }, null, 2));
    child.stdin.end();

    child.stdout.on('data', (data) => {
        process.stdout.write(data);
    });
    child.stderr.on('data', (data) => {
        process.stderr.write(data);
    });

    child.stdout.pipe(logStream, { end: false });
    child.stderr.pipe(logStream, { end: false });

    child.on('error', (error) => {
      logStream.write(`\n--- SPAWN ERROR: ${error.message} ---\n`);
      setTaskStatus(workspaceRoot, taskId, 'failed', { error: error.message });
      reject(error);
    });

    child.on('exit', (code) => {
      const endTime = new Date().toISOString();
      logStream.write(`\n---\nExit Code: ${code}\nEnd Time: ${endTime}\n--- VIBE RUN END ---\n`);
      logStream.end();

      const runResult = {
        id: runId,
        task_id: taskId,
        agent_id: agentId,
        exit_code: code,
        log_path: `.vibe/logs/${runId}.log`,
        started_at: startTime,
        finished_at: endTime
      };

      const runsDir = path.join(vibeDir, 'runs');
      if (!fs.existsSync(runsDir)) fs.mkdirSync(runsDir, { recursive: true });
      fs.writeFileSync(path.join(runsDir, `${runId}.json`), JSON.stringify(runResult, null, 2));

      // Note: We don't automatically set to completed here, 
      // as it might need review or the agent might have set it itself.
      // But for simple cases, if exit code is 0, it might be completed.
      // However, the contract usually expects an explicit status update.
      
      resolve(runResult);
    });
  });
}

if (require.main === module) {
  const args = process.argv.slice(2);
  const taskId = args[0];
  const agentId = args[1];

  if (!taskId) {
    console.error('Usage: node run-task.js <task_id> [agent_id]');
    process.exit(1);
  }

  runTaskSync(process.cwd(), taskId, agentId)
    .then(result => {
      console.log(`\nRun completed with exit code ${result.exit_code}`);
      console.log(`Results: .vibe/runs/${result.id}.json`);
      if (result.exit_code !== 0) process.exit(result.exit_code);
    })
    .catch(error => {
      console.error('Execution failed:', error.message);
      process.exit(1);
    });
}

module.exports = {
  runTask: runTaskSync,
  runSkill: (params, workspaceRoot) => runTaskSync(workspaceRoot, params.taskId, params.agentId)
};
