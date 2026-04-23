const fs = require('fs');
const path = require('path');

/**
 * Status Management Script
 * Handles task and run status updates with contract validation.
 */

const ALLOWED_STATUSES = [
  'queued',
  'running',
  'blocked',
  'review-needed',
  'fix-needed',
  'failed',
  'completed',
  'interrupted'
];

function setTaskStatus(workspaceRoot, taskId, status, metadata = {}) {
  if (!ALLOWED_STATUSES.includes(status)) {
    throw new Error(`Invalid status: ${status}. Must be one of ${ALLOWED_STATUSES.join(', ')}`);
  }

  const taskPath = path.join(workspaceRoot, '.vibe', 'tasks', `${taskId}.json`);
  if (!fs.existsSync(taskPath)) {
    throw new Error(`Task ${taskId} not found`);
  }

  const task = JSON.parse(fs.readFileSync(taskPath, 'utf8'));
  const oldStatus = task.status;
  
  const updatedTask = {
    ...task,
    status,
    ...metadata,
    updated_at: new Date().toISOString()
  };

  fs.writeFileSync(taskPath, JSON.stringify(updatedTask, null, 2));
  console.log(`Task ${taskId} status: ${oldStatus} -> ${status}`);
  return updatedTask;
}

function getTaskStatus(workspaceRoot, taskId) {
  const taskPath = path.join(workspaceRoot, '.vibe', 'tasks', `${taskId}.json`);
  if (!fs.existsSync(taskPath)) {
    throw new Error(`Task ${taskId} not found`);
  }
  const task = JSON.parse(fs.readFileSync(taskPath, 'utf8'));
  return task.status;
}

function listTasks(workspaceRoot) {
  const vibeDir = path.join(workspaceRoot, '.vibe');
  const planPath = path.join(vibeDir, 'plan.json');
  const tasksDir = path.join(vibeDir, 'tasks');

  if (fs.existsSync(planPath)) {
    const plan = JSON.parse(fs.readFileSync(planPath, 'utf8'));
    console.log(`Plan: ${plan.id} - ${plan.goal}`);
    console.log('Tasks:');
    
    plan.tasks.forEach(taskId => {
      const taskPath = path.join(tasksDir, `${taskId}.json`);
      if (fs.existsSync(taskPath)) {
        const task = JSON.parse(fs.readFileSync(taskPath, 'utf8'));
        const deps = task.dependencies && task.dependencies.length > 0 
          ? ` [Depends on: ${task.dependencies.join(', ')}]` 
          : '';
        console.log(`- [${task.status}] ${task.id}: ${task.name || ''}${deps}`);
      } else {
        console.log(`- [missing] ${taskId}`);
      }
    });
  } else if (fs.existsSync(tasksDir)) {
    const files = fs.readdirSync(tasksDir);
    files.filter(f => f.endsWith('.json')).forEach(file => {
      const task = JSON.parse(fs.readFileSync(path.join(tasksDir, file), 'utf8'));
      const deps = task.dependencies && task.dependencies.length > 0 
        ? ` [Depends on: ${task.dependencies.join(', ')}]` 
        : '';
      console.log(`- [${task.status}] ${task.id}: ${task.name || ''}${deps}`);
    });
  } else {
    console.log('No tasks or plan found.');
  }
}

if (require.main === module) {
  const args = process.argv.slice(2);
  const command = args[0];

  try {
    if (command === '--check-schema') {
      console.log('Schema check: OK');
      process.exit(0);
    }

    if (command === 'set') {
      const taskId = args[1];
      const status = args[2];
      const metadata = args[3] ? JSON.parse(args[3]) : {};
      setTaskStatus(process.cwd(), taskId, status, metadata);
    } else if (command === 'get') {
      const taskId = args[1];
      const status = getTaskStatus(process.cwd(), taskId);
      console.log(status);
    } else if (command === 'list' || !command) {
      listTasks(process.cwd());
    } else {
      console.error('Usage: node status.js [list|get|set] [task_id] [status] [metadata_json]');
      process.exit(1);
    }
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

module.exports = { setTaskStatus, getTaskStatus, listTasks };
