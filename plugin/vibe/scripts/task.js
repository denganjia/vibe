const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

/**
 * Task Management Script
 * Handles creation and basic updates of task artifacts.
 */

function generateTaskId() {
  return crypto.randomBytes(4).toString('hex');
}

function createTask(workspaceRoot, taskData) {
  const vibeDir = path.join(workspaceRoot, '.vibe');
  const tasksDir = path.join(vibeDir, 'tasks');

  if (!fs.existsSync(tasksDir)) {
    fs.mkdirSync(tasksDir, { recursive: true });
  }

  const id = taskData.id || generateTaskId();
  const now = new Date().toISOString();

  const task = {
    id,
    goal: taskData.goal || '',
    context: taskData.context || [],
    file_scope: taskData.file_scope || [],
    constraints: taskData.constraints || [],
    expected_output: taskData.expected_output || '',
    verification: taskData.verification || '',
    reviewer_requirements: taskData.reviewer_requirements || {},
    status: taskData.status || 'queued',
    created_at: now,
    updated_at: now,
    ...taskData
  };

  const taskPath = path.join(tasksDir, `${id}.json`);
  fs.writeFileSync(taskPath, JSON.stringify(task, null, 2));
  return task;
}

function updateTask(workspaceRoot, taskId, updates) {
  const taskPath = path.join(workspaceRoot, '.vibe', 'tasks', `${taskId}.json`);
  if (!fs.existsSync(taskPath)) {
    throw new Error(`Task ${taskId} not found`);
  }

  const task = JSON.parse(fs.readFileSync(taskPath, 'utf8'));
  const updatedTask = {
    ...task,
    ...updates,
    updated_at: new Date().toISOString()
  };

  fs.writeFileSync(taskPath, JSON.stringify(updatedTask, null, 2));
  return updatedTask;
}

if (require.main === module) {
  const args = process.argv.slice(2);
  const command = args[0];

  try {
    if (command === 'create') {
      // Expecting JSON string as second argument or piped input
      const data = args[1] ? JSON.parse(args[1]) : {};
      const task = createTask(process.cwd(), data);
      console.log(JSON.stringify(task, null, 2));
    } else if (command === 'update') {
      const taskId = args[1];
      const updates = JSON.parse(args[2]);
      const task = updateTask(process.cwd(), taskId, updates);
      console.log(JSON.stringify(task, null, 2));
    } else {
      console.error('Usage: node task.js [create|update] [data_json]');
      process.exit(1);
    }
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

module.exports = { createTask, updateTask };
