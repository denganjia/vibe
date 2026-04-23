const fs = require('fs');
const path = require('path');
const { setTaskStatus } = require('./status');

/**
 * Workspace Synchronization and Recovery Script
 * Reconciles lock state vs task status and cleans up orphaned locks.
 */

function syncWorkspace(workspaceRoot) {
  const vibeDir = path.join(workspaceRoot, '.vibe');
  const tasksDir = path.join(vibeDir, 'tasks');
  const locksDir = path.join(vibeDir, 'locks');

  if (!fs.existsSync(tasksDir)) {
    console.log('No tasks directory found.');
    return;
  }

  // 1. Scan tasks for 'running' state
  const taskFiles = fs.readdirSync(tasksDir).filter(f => f.endsWith('.json'));
  const runningTasks = [];

  for (const file of taskFiles) {
    const taskPath = path.join(tasksDir, file);
    try {
      const task = JSON.parse(fs.readFileSync(taskPath, 'utf8'));
      if (task.status === 'running') {
        runningTasks.push(task);
      }
    } catch (e) {
      console.error(`Error reading task file ${file}: ${e.message}`);
    }
  }

  // 2. Get active locks and their owners
  const lockOwners = new Set();
  if (fs.existsSync(locksDir)) {
    const lockFiles = fs.readdirSync(locksDir).filter(f => f.endsWith('.lock'));
    for (const file of lockFiles) {
      const lockPath = path.join(locksDir, file);
      try {
        const owner = fs.readFileSync(lockPath, 'utf8').trim();
        if (owner) {
          lockOwners.add(owner);
        }
      } catch (e) {
        console.error(`Error reading lock file ${file}: ${e.message}`);
      }
    }
  }

  // 3. Reconcile: If 'running' but no lock, mark as 'interrupted'
  for (const task of runningTasks) {
    if (!lockOwners.has(task.id)) {
      console.warn(`Task ${task.id} is 'running' but has no active locks. Marking as interrupted.`);
      try {
        setTaskStatus(workspaceRoot, task.id, 'interrupted', { 
          interruption_detected_at: new Date().toISOString() 
        });
      } catch (e) {
        console.error(`Failed to update status for task ${task.id}: ${e.message}`);
      }
    }
  }

  // 4. Clean up orphaned locks (locks whose owner is not 'running')
  if (fs.existsSync(locksDir)) {
    const lockFiles = fs.readdirSync(locksDir).filter(f => f.endsWith('.lock'));
    for (const file of lockFiles) {
      const lockPath = path.join(locksDir, file);
      try {
        const owner = fs.readFileSync(lockPath, 'utf8').trim();
        
        // Check if owner exists and is 'running'
        const taskPath = path.join(tasksDir, `${owner}.json`);
        let shouldRemove = false;
        
        if (!fs.existsSync(taskPath)) {
          shouldRemove = true;
        } else {
          const task = JSON.parse(fs.readFileSync(taskPath, 'utf8'));
          if (task.status !== 'running') {
            shouldRemove = true;
          }
        }
        
        if (shouldRemove) {
          console.log(`Removing orphaned lock: ${file} (Owner: ${owner})`);
          fs.unlinkSync(lockPath);
        }
      } catch (e) {
        console.error(`Error processing lock file ${file}: ${e.message}`);
      }
    }
  }
}

if (require.main === module) {
  syncWorkspace(process.cwd());
}

module.exports = { syncWorkspace };
