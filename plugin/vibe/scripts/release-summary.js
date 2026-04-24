const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

/**
 * Categorize a commit message based on Conventional Commits or fuzzy matching.
 * @param {string} msg 
 * @returns {string}
 */
function categorize(msg) {
  if (!msg) return 'Internal Changes';

  // Standard Conventional Commits
  const ccRegex = /^(feat|fix|docs|test|refactor|chore)(?:\(.*\))?:\s*(.*)/i;
  const match = msg.match(ccRegex);
  if (match) {
    return match[1].toLowerCase();
  }

  // Fuzzy matching
  const lowerMsg = msg.toLowerCase();
  const rules = [
    { type: 'feat', keywords: ['add', 'new', 'feat', 'feature', 'introduce'] },
    { type: 'fix', keywords: ['fix', 'bug', 'patch', 'resolve', 'hotfix'] },
    { type: 'docs', keywords: ['doc', 'readme', 'changelog'] },
    { type: 'test', keywords: ['test', 'spec', 'e2e'] },
    { type: 'refactor', keywords: ['refactor', 'cleanup', 'clean'] },
    { type: 'chore', keywords: ['chore', 'deps', 'build', 'ci'] },
  ];

  for (const rule of rules) {
    if (rule.keywords.some(kw => lowerMsg.includes(kw))) {
      return rule.type;
    }
  }

  return 'Internal Changes';
}

/**
 * Extract task info from commit message and attempt to load task details.
 * @param {string} msg 
 * @returns {object|null}
 */
function getTaskInfo(msg) {
  const taskRegex = /\(task:\s*([\w-]+)\)/i;
  const match = msg.match(taskRegex);
  if (!match) return null;

  const taskId = match[1];
  const taskPath = path.join(process.cwd(), '.vibe', 'tasks', `${taskId}.json`);

  try {
    if (fs.existsSync(taskPath)) {
      const data = JSON.parse(fs.readFileSync(taskPath, 'utf8'));
      return { ...data, id: taskId };
    }
  } catch (e) {
    // Ignore read errors
  }

  return { id: taskId, missing: true };
}

/**
 * Get git log between two points.
 * @param {string} from 
 * @param {string} to 
 * @returns {string[]}
 */
function getGitLog(from, to, workspaceRoot = process.cwd()) {
  try {
    let range = '';
    if (from && to) {
      range = `${from}..${to}`;
    } else if (from) {
      range = `${from}..HEAD`;
    } else {
      // Try to find latest tag
      try {
        const latestTag = execSync('git describe --tags --abbrev=0', { cwd: workspaceRoot, encoding: 'utf8' }).trim();
        range = `${latestTag}..HEAD`;
      } catch (e) {
        // No tags found, get all logs
        range = '';
      }
    }

    const log = execSync(`git log ${range} --pretty=format:"%s"`, { cwd: workspaceRoot, encoding: 'utf8' });
    return log.split('\n').filter(Boolean);
  } catch (e) {
    console.error('Failed to get git log:', e.message);
    return [];
  }
}

/**
 * Generate release summary markdown.
 * @param {string[]} logs 
 * @returns {string}
 */
function generateMarkdown(logs) {
  const categories = {
    feat: [],
    fix: [],
    docs: [],
    test: [],
    refactor: [],
    chore: [],
    'Internal Changes': []
  };

  const titleMap = {
    feat: '🚀 Features',
    fix: '🐛 Bug Fixes',
    docs: '📝 Documentation',
    test: '🧪 Tests',
    refactor: '♻️ Refactoring',
    chore: '🔧 Chore',
    'Internal Changes': '🏠 Internal Changes'
  };

  logs.forEach(msg => {
    const type = categorize(msg);
    const taskInfo = getTaskInfo(msg);
    let entry = msg;

    if (taskInfo) {
      const taskRef = taskInfo.missing ? `(task: ${taskInfo.id})` : `[${taskInfo.id} - ${taskInfo.title || 'Task'}]`;
      entry = `${msg} ${taskRef}`;
    }

    if (categories[type]) {
      categories[type].push(entry);
    } else {
      categories['Internal Changes'].push(entry);
    }
  });

  let md = `# Release Summary\n\n`;
  md += `Generated on ${new Date().toISOString().split('T')[0]}\n\n`;

  for (const [key, items] of Object.entries(categories)) {
    if (items.length > 0) {
      md += `## ${titleMap[key] || key}\n\n`;
      items.forEach(item => {
        md += `- ${item}\n`;
      });
      md += `\n`;
    }
  }

  return md;
}

// CLI Execution
if (require.main === module) {
  const args = process.argv.slice(2);
  let from = null;
  let to = 'HEAD';
  let isJson = false;

  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--from' && args[i + 1]) {
      from = args[i + 1];
      i++;
    } else if (args[i] === '--to' && args[i + 1]) {
      to = args[i + 1];
      i++;
    } else if (args[i] === '--json') {
      isJson = true;
    }
  }

  const logs = getGitLog(from, to);
  
  if (isJson) {
    const summary = logs.map(msg => ({
      message: msg,
      category: categorize(msg),
      task: getTaskInfo(msg)
    }));
    console.log(JSON.stringify(summary, null, 2));
  } else {
    const md = generateMarkdown(logs);
    const outputPath = path.join(process.cwd(), '.vibe', 'RELEASE_DRAFT.md');
    
    if (!fs.existsSync(path.dirname(outputPath))) {
      fs.mkdirSync(path.dirname(outputPath), { recursive: true });
    }
    
    fs.writeFileSync(outputPath, md);
    console.log(`Release summary generated at ${outputPath}`);
  }
}

function generateReleaseSummary(workspaceRoot, from, to) {
  const logs = getGitLog(from, to, workspaceRoot);
  const md = generateMarkdown(logs);
  const outputPath = path.join(workspaceRoot, '.vibe', 'RELEASE_DRAFT.md');
  
  if (!fs.existsSync(path.dirname(outputPath))) {
    fs.mkdirSync(path.dirname(outputPath), { recursive: true });
  }
  
  fs.writeFileSync(outputPath, md);
  return { success: true, path: outputPath, content: md };
}

module.exports = {
  categorize,
  getTaskInfo,
  getGitLog,
  generateMarkdown,
  generateReleaseSummary,
  runSkill: (params, workspaceRoot) => generateReleaseSummary(workspaceRoot, params.from, params.to)
};
