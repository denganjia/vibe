#!/usr/bin/env node

/**
 * plan.js
 * Generates .vibe/plan.json and .vibe/tasks/*.json from a structured plan.
 * 
 * Usage:
 * node plan.js --mock-input='{"goal": "...", "tasks": [...], "notes": "..."}'
 */

const fs = require('fs');
const path = require('path');
const { sanitizeId } = require('./utils');

function generatePlan(plan, workspaceRoot = process.cwd()) {
  if (!plan.goal || !plan.tasks || !Array.isArray(plan.tasks)) {
    throw new Error('Invalid plan format. Must include "goal" and "tasks" array.');
  }

  // Topological check for circular dependencies
  const taskIds = new Set(plan.tasks.map(t => t.id));
  for (const task of plan.tasks) {
    for (const dep of (task.dependencies || [])) {
      if (!taskIds.has(dep)) {
        console.warn(`Warning: Task "${task.id}" depends on unknown task "${dep}".`);
      }
    }
  }

  if (hasCircularDependencies(plan.tasks)) {
    throw new Error('Circular dependencies detected in tasks.');
  }

  const vibeDir = path.resolve(workspaceRoot, '.vibe');
  const tasksDir = path.join(vibeDir, 'tasks');

  if (!fs.existsSync(vibeDir)) fs.mkdirSync(vibeDir, { recursive: true });
  if (!fs.existsSync(tasksDir)) fs.mkdirSync(tasksDir, { recursive: true });

  // Generate .vibe/plan.json
  const planManifest = {
    id: plan.id || `plan_${Date.now()}`,
    goal: plan.goal,
    tasks: plan.tasks.map(t => t.id),
    status: 'pending',
    created_at: new Date().toISOString()
  };
  fs.writeFileSync(path.join(vibeDir, 'plan.json'), JSON.stringify(planManifest, null, 2));

  // Generate .vibe/tasks/*.json
  plan.tasks.forEach(task => {
    const safeTaskId = sanitizeId(task.id);
    const taskContent = {
      id: task.id,
      goal: task.goal || '',
      file_scope: task.file_scope || [],
      verification: task.verification || '',
      dependencies: task.dependencies || [],
      status: 'pending',
      created_at: new Date().toISOString(),
      ...task
    };
    fs.writeFileSync(path.join(tasksDir, `${safeTaskId}.json`), JSON.stringify(taskContent, null, 2));
  });

  // Generate .vibe/planning_notes.md
  const notesPath = path.join(vibeDir, 'planning_notes.md');
  const notesContent = plan.notes || `# Planning Notes\n\n**Goal:** ${plan.goal}\n\n*Generated on ${new Date().toISOString()}*`;
  fs.writeFileSync(notesPath, notesContent);

  return { success: true, message: 'Successfully generated plan manifest and task files.' };
}

function processReview(taskId, workspaceRoot = process.cwd()) {
  const safeTaskId = sanitizeId(taskId);
  const vibeDir = path.resolve(workspaceRoot, '.vibe');
  const taskPath = path.join(vibeDir, 'tasks', `${safeTaskId}.json`);
  const reviewsDir = path.join(vibeDir, 'reviews');

  if (!fs.existsSync(taskPath)) {
    throw new Error(`Task ${taskId} not found.`);
  }

  const task = JSON.parse(fs.readFileSync(taskPath, 'utf8'));

  // Find latest review for this task
  if (!fs.existsSync(reviewsDir)) {
    throw new Error(`No reviews found for task ${taskId}.`);
  }

  const reviews = fs.readdirSync(reviewsDir)
    .filter(f => f.startsWith(`${safeTaskId}_`) && f.endsWith('.json'))
    .map(f => ({
      name: f,
      time: fs.statSync(path.join(reviewsDir, f)).mtime.getTime()
    }))
    .sort((a, b) => b.time - a.time);

  if (reviews.length === 0) {
    throw new Error(`No reviews found for task ${taskId}.`);
  }

  const latestReviewPath = path.join(reviewsDir, reviews[0].name);
  const review = JSON.parse(fs.readFileSync(latestReviewPath, 'utf8'));

  if (review.status === 'pass') {
    task.status = 'completed';
    console.log(`Task ${taskId} review passed. Status set to completed.`);
  } else {
    task.status = 'fix-needed';
    // Aggregate findings
    task.review_findings = review.findings;
    
    // Add findings to goal or context for visibility
    const findingsSummary = review.findings
      .map(f => `- [${f.severity}] ${f.file}:${f.line}: ${f.message}`)
      .join('\n');
    
    if (!task.original_goal) {
      task.original_goal = task.goal;
    }
    
    task.goal = `${task.original_goal}\n\n### Fix Needed (from review)\n${findingsSummary}`;
    
    console.log(`Task ${taskId} review failed. Status set to fix-needed. Findings aggregated.`);
  }

  fs.writeFileSync(taskPath, JSON.stringify(task, null, 2));
  return { success: true, message: `Task ${taskId} processed.` };
}

function hasCircularDependencies(tasks) {
  const adj = {};
  tasks.forEach(t => adj[t.id] = t.dependencies || []);

  const visited = new Set();
  const recStack = new Set();

  function isCyclic(v) {
    if (!visited.has(v)) {
      visited.add(v);
      recStack.add(v);

      const neighbors = adj[v] || [];
      for (const neighbor of neighbors) {
        if (!visited.has(neighbor)) {
          if (isCyclic(neighbor)) return true;
        } else if (recStack.has(neighbor)) {
          return true;
        }
      }
    }
    recStack.delete(v);
    return false;
  }

  for (const task of tasks) {
    if (isCyclic(task.id)) return true;
  }
  return false;
}

module.exports = {
  generatePlan,
  processReview,
  runSkill: (params, workspaceRoot) => generatePlan(params, workspaceRoot)
};

if (require.main === module) {
  const args = process.argv.slice(2);
  let inputStr = '';

  const processReviewArg = args.find(a => a.startsWith('--process-review='));
  if (processReviewArg) {
    const taskId = processReviewArg.split('=')[1];
    try {
      processReview(taskId);
    } catch (e) {
      console.error(`Error: ${e.message}`);
      process.exit(1);
    }
    process.exit(0);
  }

  const mockInputArg = args.find(a => a.startsWith('--mock-input='));
  if (mockInputArg) {
    inputStr = mockInputArg.split('=')[1];
  } else {
    if (args[0] && args[0].startsWith('{')) {
      inputStr = args[0];
    } else {
      console.error('Usage: node plan.js --mock-input=\'{"goal":"...", "tasks":[]}\'');
      process.exit(1);
    }
  }

  let plan;
  try {
    plan = JSON.parse(inputStr);
    generatePlan(plan);
    console.log('Successfully generated plan manifest and task files.');
  } catch (e) {
    console.error(`Error: ${e.message}`);
    process.exit(1);
  }
}
