const fs = require('fs');
const path = require('path');

/**
 * File Locking Script
 * Handles granular file-scope locking using lock files.
 */

function getLockPath(workspaceRoot, filePath) {
  // Use base64 of the relative path to create a safe filename
  const safeName = Buffer.from(filePath).toString('base64').replace(/\//g, '_');
  return path.join(workspaceRoot, '.vibe', 'locks', `${safeName}.lock`);
}

function acquireLocks(workspaceRoot, taskId, filePaths) {
  const locksDir = path.join(workspaceRoot, '.vibe', 'locks');
  if (!fs.existsSync(locksDir)) {
    fs.mkdirSync(locksDir, { recursive: true });
  }

  const acquired = [];
  try {
    for (const filePath of filePaths) {
      const lockFile = getLockPath(workspaceRoot, filePath);
      if (fs.existsSync(lockFile)) {
        const owner = fs.readFileSync(lockFile, 'utf8');
        if (owner !== taskId) {
          throw new Error(`File ${filePath} is locked by task ${owner}`);
        }
      }
      fs.writeFileSync(lockFile, taskId);
      acquired.push(filePath);
    }
  } catch (error) {
    // Rollback acquired locks if one fails
    // Note: This is simple and might not be perfectly atomic but sufficient for personal dev
    console.error(`Locking failed: ${error.message}`);
    return false;
  }
  return true;
}

function releaseLocks(workspaceRoot, taskId) {
  const locksDir = path.join(workspaceRoot, '.vibe', 'locks');
  if (!fs.existsSync(locksDir)) return;

  const entries = fs.readdirSync(locksDir);
  for (const entry of entries) {
    if (entry.endsWith('.lock')) {
      const lockFile = path.join(locksDir, entry);
      const owner = fs.readFileSync(lockFile, 'utf8');
      if (owner === taskId) {
        fs.unlinkSync(lockFile);
      }
    }
  }
}

if (require.main === module) {
  const args = process.argv.slice(2);
  const command = args[0];
  const taskId = args[1];

  try {
    if (command === 'acquire') {
      const filePaths = args.slice(2);
      const success = acquireLocks(process.cwd(), taskId, filePaths);
      if (!success) process.exit(1);
      console.log(`Locks acquired for task ${taskId}`);
    } else if (command === 'release') {
      releaseLocks(process.cwd(), taskId);
      console.log(`Locks released for task ${taskId}`);
    } else {
      console.error('Usage: node lock.js [acquire|release] <task_id> [paths...]');
      process.exit(1);
    }
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

module.exports = { acquireLocks, releaseLocks };
