const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const assert = require('assert');

const TEST_DIR = path.join(__dirname, 'test_runtime_workdir');
const SCRIPTS = {
  init: path.join(__dirname, 'init.js'),
  task: path.join(__dirname, 'task.js'),
  lock: path.join(__dirname, 'lock.js'),
  run: path.join(__dirname, 'run-task.js'),
  status: path.join(__dirname, 'status.js')
};

function setup() {
  if (fs.existsSync(TEST_DIR)) {
    fs.rmSync(TEST_DIR, { recursive: true, force: true });
  }
  fs.mkdirSync(TEST_DIR, { recursive: true });
}

function cleanup() {
  if (fs.existsSync(TEST_DIR)) {
    fs.rmSync(TEST_DIR, { recursive: true, force: true });
  }
}

function runScript(name, args = []) {
  const quotedArgs = args.map(arg => {
    if (typeof arg === 'string' && (arg.includes('{') || arg.includes(' ') || arg.includes('"'))) {
      return `'${arg.replace(/'/g, "'\\''")}'`;
    }
    return arg;
  });
  return execSync(`node ${SCRIPTS[name]} ${quotedArgs.join(' ')}`, { cwd: TEST_DIR, stdio: 'pipe' }).toString();
}

try {
  setup();
  console.log('1. Initializing workspace...');
  runScript('init', ['.']);
  assert.ok(fs.existsSync(path.join(TEST_DIR, '.vibe', 'config.json')));

  console.log('2. Creating a task...');
  const taskGoal = 'Test task execution';
  const taskOutput = runScript('task', ['create', JSON.stringify({ goal: taskGoal, file_scope: ['test.txt'] })]);
  const task = JSON.parse(taskOutput);
  assert.strictEqual(task.goal, taskGoal);
  assert.strictEqual(task.status, 'queued');

  console.log('3. Acquiring locks...');
  runScript('lock', ['acquire', task.id, 'test.txt']);
  assert.ok(fs.existsSync(path.join(TEST_DIR, '.vibe', 'locks', Buffer.from('test.txt').toString('base64').replace(/\//g, '_') + '.lock')));

  console.log('4. Setting status to running...');
  runScript('status', ['set', task.id, 'running']);
  assert.strictEqual(JSON.parse(fs.readFileSync(path.join(TEST_DIR, '.vibe', 'tasks', `${task.id}.json`))).status, 'running');

  console.log('5. Running mock agent...');
  // Create a mock agent that just echoes
  const agentPath = path.join(TEST_DIR, '.vibe', 'agents', 'mock.json');
  fs.writeFileSync(agentPath, JSON.stringify({
    id: 'mock',
    model_command: 'echo "Hello from Mock Agent"'
  }));

  const runOutput = runScript('run', [task.id, 'mock']);
  assert.ok(runOutput.includes('Run completed with exit code 0'));
  
  // Verify run artifact
  const runsDir = path.join(TEST_DIR, '.vibe', 'runs');
  const runFiles = fs.readdirSync(runsDir).filter(f => f.endsWith('.json'));
  assert.strictEqual(runFiles.length, 1);
  const runResult = JSON.parse(fs.readFileSync(path.join(runsDir, runFiles[0])));
  assert.strictEqual(runResult.exit_code, 0);

  // Verify log artifact
  const logContent = fs.readFileSync(path.join(TEST_DIR, runResult.log_path), 'utf8');
  assert.ok(logContent.includes('Hello from Mock Agent'));

  console.log('6. Releasing locks...');
  runScript('lock', ['release', task.id]);
  const lockFiles = fs.readdirSync(path.join(TEST_DIR, '.vibe', 'locks')).filter(f => f.endsWith('.lock'));
  assert.strictEqual(lockFiles.length, 0);

  console.log('7. Finalizing task status...');
  runScript('status', ['set', task.id, 'completed']);
  assert.strictEqual(JSON.parse(fs.readFileSync(path.join(TEST_DIR, '.vibe', 'tasks', `${task.id}.json`))).status, 'completed');

  console.log('\nAll runtime scripts verified successfully!');
} catch (error) {
  console.error('\nRuntime verification failed:');
  console.error(error.message);
  if (error.stdout) console.error('Stdout:', error.stdout.toString());
  if (error.stderr) console.error('Stderr:', error.stderr.toString());
  process.exit(1);
} finally {
  cleanup();
}
