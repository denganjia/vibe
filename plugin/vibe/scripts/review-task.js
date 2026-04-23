#!/usr/bin/env node

/**
 * review-task.js
 * Orchestrates the review phase for a task.
 * 
 * Usage:
 * node review-task.js <task_id> [--mock-input=<json>]
 */

const { spawnSync } = require('child_process');
const fs = require('fs');
const path = require('path');

function main() {
  const args = process.argv.slice(2);
  const taskId = args.find(a => !a.startsWith('--')) || '';
  const mockInputArg = args.find(a => a.startsWith('--mock-input='));
  
  if (!taskId) {
    console.error('Usage: node review-task.js <task_id> [--mock-input=<json>]');
    process.exit(1);
  }

  const vibeDir = path.resolve(process.cwd(), '.vibe');
  const taskPath = path.join(vibeDir, 'tasks', `${taskId}.json`);

  if (!fs.existsSync(taskPath)) {
    console.error(`Error: Task ${taskId} not found.`);
    process.exit(1);
  }

  const task = JSON.parse(fs.readFileSync(taskPath, 'utf8'));
  
  // 1. Get runId (latest run)
  const runsDir = path.join(vibeDir, 'runs');
  const runs = fs.readdirSync(runsDir)
    .filter(f => f.endsWith('.json'))
    .map(f => JSON.parse(fs.readFileSync(path.join(runsDir, f), 'utf8')))
    .filter(r => r.task_id === taskId)
    .sort((a, b) => new Date(b.finished_at) - new Date(a.finished_at));

  if (runs.length === 0) {
    console.error(`Error: No runs found for task ${taskId}.`);
    process.exit(1);
  }

  const runId = runs[0].id;

  // 2. Process review findings
  const reviewArgs = [
    path.join(__dirname, 'review.js'),
    `--task-id=${taskId}`,
    `--run-id=${runId}`
  ];
  if (mockInputArg) reviewArgs.push(mockInputArg);

  console.log(`Processing review for task ${taskId} (run ${runId})...`);
  const reviewProc = spawnSync('node', reviewArgs, { stdio: 'inherit' });
  
  if (reviewProc.status !== 0) {
    console.error('Error: review.js failed.');
    process.exit(1);
  }

  // 3. Update task status and aggregate findings
  console.log(`Updating task ${taskId} state...`);
  const planProc = spawnSync('node', [
    path.join(__dirname, 'plan.js'),
    `--process-review=${taskId}`
  ], { stdio: 'inherit' });

  if (planProc.status !== 0) {
    console.error('Error: plan.js --process-review failed.');
    process.exit(1);
  }

  console.log(`Review phase for task ${taskId} completed.`);
}

main();
