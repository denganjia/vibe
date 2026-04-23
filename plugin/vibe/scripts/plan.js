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

function main() {
  const args = process.argv.slice(2);
  let inputStr = '';

  const mockInputArg = args.find(a => a.startsWith('--mock-input='));
  if (mockInputArg) {
    inputStr = mockInputArg.split('=')[1];
  } else {
    // If not mock-input, check if there's a positional argument that is valid JSON
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
  } catch (e) {
    console.error('Error: Failed to parse input JSON.');
    console.error(e.message);
    process.exit(1);
  }

  if (!plan.goal || !plan.tasks || !Array.isArray(plan.tasks)) {
    console.error('Error: Invalid plan format. Must include "goal" and "tasks" array.');
    process.exit(1);
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
    console.error('Error: Circular dependencies detected in tasks.');
    process.exit(1);
  }

  const vibeDir = path.resolve(process.cwd(), '.vibe');
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
    fs.writeFileSync(path.join(tasksDir, `${task.id}.json`), JSON.stringify(taskContent, null, 2));
  });

  // Generate .vibe/planning_notes.md
  const notesPath = path.join(vibeDir, 'planning_notes.md');
  const notesContent = plan.notes || `# Planning Notes\n\n**Goal:** ${plan.goal}\n\n*Generated on ${new Date().toISOString()}*`;
  fs.writeFileSync(notesPath, notesContent);

  console.log('Successfully generated plan manifest and task files.');
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

main();
