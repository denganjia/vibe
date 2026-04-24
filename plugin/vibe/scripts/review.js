#!/usr/bin/env node

/**
 * review.js
 * Captures structured findings from Reviewer Agent output.
 * 
 * Usage:
 * node review.js --task-id=T1 --run-id=run_abc --mock-input='{"status":"fail", "findings":[{"severity":"high", "message":"error"}]}'
 */

const fs = require('fs');
const path = require('path');
const { sanitizeId } = require('./utils');

function saveReviewResult(taskId, runId, reviewInput, workspaceRoot = process.cwd()) {
  let reviewResult;
  try {
    if (typeof reviewInput === 'string') {
      // Try to extract JSON from the input (might contain markdown or extra text)
      const jsonMatch = reviewInput.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        reviewResult = JSON.parse(jsonMatch[0]);
      } else {
        reviewResult = JSON.parse(reviewInput);
      }
    } else {
      reviewResult = reviewInput;
    }
  } catch (e) {
    throw new Error(`Failed to parse review input as JSON: ${e.message}`);
  }

  // Validate format
  if (!reviewResult.status) {
    throw new Error('Review result missing "status".');
  }

  const vibeDir = path.resolve(workspaceRoot, '.vibe');
  const reviewsDir = path.join(vibeDir, 'reviews');
  if (!fs.existsSync(reviewsDir)) fs.mkdirSync(reviewsDir, { recursive: true });

  const safeTaskId = sanitizeId(taskId);
  const safeRunId = sanitizeId(runId);
  const fileName = `${safeTaskId}_${safeRunId}.json`;
  const filePath = path.join(reviewsDir, fileName);

  const findings = (reviewResult.findings || []).map(f => ({
    file: f.file || 'unknown',
    line: f.line || 0,
    severity: f.severity || 'low',
    message: f.message || 'No message provided'
  }));

  const structuredOutput = {
    task_id: taskId,
    run_id: runId,
    status: reviewResult.status, // 'pass' or 'fail'
    findings: findings,
    reviewed_at: new Date().toISOString()
  };

  fs.writeFileSync(filePath, JSON.stringify(structuredOutput, null, 2));
  return structuredOutput;
}

module.exports = {
  saveReviewResult,
  runSkill: (params, workspaceRoot) => saveReviewResult(params.taskId, params.runId, params.reviewInput, workspaceRoot)
};

if (require.main === module) {
  const args = process.argv.slice(2);
  let taskId = '';
  let runId = '';
  let inputStr = '';

  args.forEach(arg => {
    if (arg.startsWith('--task-id=')) taskId = arg.split('=')[1];
    if (arg.startsWith('--run-id=')) runId = arg.split('=')[1];
    if (arg.startsWith('--mock-input=')) inputStr = arg.split('=')[1];
  });

  if (!taskId || !runId) {
    console.error('Usage: node review.js --task-id=<id> --run-id=<id> [--mock-input=<json>]');
    process.exit(1);
  }

  // In real use, inputStr might come from stdin if not provided via --mock-input
  if (!inputStr) {
    try {
      inputStr = fs.readFileSync(0, 'utf8'); // Read from stdin
    } catch (e) {
      console.error('Error: Failed to read from stdin.');
      process.exit(1);
    }
  }

  try {
    const result = saveReviewResult(taskId, runId, inputStr);
    console.log(`Structured review saved to .vibe/reviews/${taskId}_${runId}.json`);
    console.log(JSON.stringify(result));
  } catch (e) {
    console.error(`Error: ${e.message}`);
    process.exit(1);
  }
}
