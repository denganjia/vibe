#!/usr/bin/env node

/**
 * review-task.js
 * Orchestrates the review phase for a task.
 * 
 * Usage:
 * node review-task.js <task_id> [--mock-input=<json>]
 */

const fs = require('fs');
const path = require('path');
const { sanitizeId } = require('./utils');
const { saveReviewResult } = require('./review');
const { processReview } = require('./plan');

function runReviewPhase(taskId, mockInput, workspaceRoot = process.cwd()) {
  const safeTaskId = sanitizeId(taskId);
  const vibeDir = path.resolve(workspaceRoot, '.vibe');
  const taskPath = path.join(vibeDir, 'tasks', `${safeTaskId}.json`);

  if (!fs.existsSync(taskPath)) {
    throw new Error(`Task ${taskId} not found.`);
  }

  // 1. Get runId (latest run)
  const runsDir = path.join(vibeDir, 'runs');
  if (!fs.existsSync(runsDir)) {
      throw new Error(`No runs found for task ${taskId}.`);
  }
  
  const runs = fs.readdirSync(runsDir)
    .filter(f => f.endsWith('.json'))
    .map(f => {
        try {
            return JSON.parse(fs.readFileSync(path.join(runsDir, f), 'utf8'));
        } catch (e) {
            return null;
        }
    })
    .filter(r => r && r.task_id === taskId)
    .sort((a, b) => new Date(b.finished_at) - new Date(a.finished_at));

  if (runs.length === 0) {
    throw new Error(`No runs found for task ${taskId}.`);
  }

  const runId = runs[0].id;

  // 2. Process review findings
  console.log(`Processing review for task ${taskId} (run ${runId})...`);
  saveReviewResult(taskId, runId, mockInput, workspaceRoot);

  // 3. Update task status and aggregate findings
  console.log(`Updating task ${taskId} state...`);
  processReview(taskId, workspaceRoot);

  console.log(`Review phase for task ${taskId} completed.`);
  return { success: true };
}

module.exports = {
  runReviewPhase,
  runSkill: (params, workspaceRoot) => runReviewPhase(params.taskId, params.mockInput, workspaceRoot)
};

if (require.main === module) {
  const args = process.argv.slice(2);
  const taskId = args.find(a => !a.startsWith('--')) || '';
  const mockInputArg = args.find(a => a.startsWith('--mock-input='));
  let mockInput = '';

  if (!taskId) {
    console.error('Usage: node review-task.js <task_id> [--mock-input=<json>]');
    process.exit(1);
  }

  if (mockInputArg) {
    mockInput = mockInputArg.split('=')[1];
  } else {
      // Read from stdin if not mock-input
      try {
          mockInput = fs.readFileSync(0, 'utf8');
      } catch (e) {}
  }

  try {
    runReviewPhase(taskId, mockInput);
  } catch (e) {
    console.error(`Error: ${e.message}`);
    process.exit(1);
  }
}
