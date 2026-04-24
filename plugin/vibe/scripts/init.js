const fs = require('fs');
const path = require('path');

function copyDirRecursive(src, dest, force) {
  if (!fs.existsSync(dest)) {
    fs.mkdirSync(dest, { recursive: true });
  }

  const entries = fs.readdirSync(src, { withFileTypes: true });

  for (const entry of entries) {
    const srcPath = path.join(src, entry.name);
    const destPath = path.join(dest, entry.name);

    if (entry.isDirectory()) {
      copyDirRecursive(srcPath, destPath, force);
    } else {
      if (force || !fs.existsSync(destPath)) {
        fs.copyFileSync(srcPath, destPath);
      }
    }
  }
}

function initWorkspace(targetDir, force, workspaceRoot = process.cwd()) {
  const resolvedTarget = path.resolve(workspaceRoot, targetDir);
  
  if (!resolvedTarget.startsWith(workspaceRoot)) {
    throw new Error("Target directory must be within the workspace root.");
  }

  const vibeDir = path.join(resolvedTarget, '.vibe');

  // Directories to ensure exist
  const dirs = ['agents', 'tasks', 'runs', 'locks', 'reviews', 'logs'];
  for (const dir of dirs) {
    fs.mkdirSync(path.join(vibeDir, dir), { recursive: true });
  }

  // Path to templates
  const templatesDir = path.resolve(__dirname, '../templates/.vibe');
  
  if (fs.existsSync(templatesDir)) {
      copyDirRecursive(templatesDir, vibeDir, force);
  }
}

module.exports = {
  initWorkspace,
  runSkill: (params, workspaceRoot) => initWorkspace(params.targetDir || '.', params.force || false, workspaceRoot)
};

if (require.main === module) {
  const args = process.argv.slice(2);
  let force = false;
  let targetDir = '.';

  for (const arg of args) {
    if (arg === '--force') {
      force = true;
    } else if (!arg.startsWith('-')) {
      targetDir = arg;
    }
  }

  try {
    initWorkspace(targetDir, force);
  } catch (error) {
    console.error('Error initializing workspace:', error.message);
    process.exit(1);
  }
}
