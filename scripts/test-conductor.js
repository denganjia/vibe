const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const assert = require('assert');

const WORKDIR = path.join(process.cwd(), 'e2e_test_workdir');
const SCRIPTS_DIR = path.join(process.cwd(), 'plugin/vibe/scripts');

const SCRIPTS = {
  init: path.join(SCRIPTS_DIR, 'init.js'),
  plan: path.join(SCRIPTS_DIR, 'plan.js'),
  task: path.join(SCRIPTS_DIR, 'task.js'),
  run: path.join(SCRIPTS_DIR, 'run-task.js'),
  status: path.join(SCRIPTS_DIR, 'status.js'),
  review: path.join(SCRIPTS_DIR, 'review-task.js'),
  lock: path.join(SCRIPTS_DIR, 'lock.js'),
  sync: path.join(SCRIPTS_DIR, 'sync.js')
};

function setup() {
  console.log(`Setting up workdir: ${WORKDIR}`);
  if (fs.existsSync(WORKDIR)) {
    fs.rmSync(WORKDIR, { recursive: true, force: true });
  }
  fs.mkdirSync(WORKDIR, { recursive: true });
}

function runScript(name, args = []) {
  const quotedArgs = args.map(arg => {
    if (typeof arg === 'string' && (arg.includes('{') || arg.includes(' ') || arg.includes('"'))) {
      return `'${arg.replace(/'/g, "'\\''")}'`;
    }
    return arg;
  });
  const cmd = `node ${SCRIPTS[name]} ${quotedArgs.join(' ')}`;
  // console.log(`Running: ${cmd}`);
  return execSync(cmd, { cwd: WORKDIR, stdio: 'pipe' }).toString();
}

function createMockAgent(id, behavior = 'success') {
  const agentsDir = path.join(WORKDIR, '.vibe', 'agents');
  if (!fs.existsSync(agentsDir)) fs.mkdirSync(agentsDir, { recursive: true });
  
  let command;
  if (behavior === 'success') {
    command = 'echo "Done"';
  } else if (behavior === 'fail') {
    command = 'exit 1';
  } else {
    command = behavior; // Custom command
  }

  fs.writeFileSync(path.join(agentsDir, `${id}.json`), JSON.stringify({
    id,
    model_command: command
  }));
}

async function runTests() {
  try {
    setup();

    // T1: Single Task Happy Path
    console.log('--- T1: Single Task Happy Path ---');
    runScript('init', ['.']);
    createMockAgent('executor', 'echo "Implementing feature X"');
    
    // Create plan manually for testing
    const plan = {
      id: 'P1',
      goal: 'T1 Goal',
      tasks: ['T1']
    };
    fs.writeFileSync(path.join(WORKDIR, '.vibe', 'plan.json'), JSON.stringify(plan, null, 2));
    
    runScript('task', ['create', JSON.stringify({ id: 'T1', goal: 'Task 1' })]);
    
    // Run Task
    runScript('run', ['T1']);
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'T1.json'))).status, 'running');
    
    // Review Task (Pass)
    runScript('review', ['T1', `--mock-input={"status":"pass", "findings":[]}`]);
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'T1.json'))).status, 'completed');
    console.log('T1 PASSED');

    // T2: Dependency Chain
    console.log('\n--- T2: Dependency Chain ---');
    setup();
    runScript('init', ['.']);
    createMockAgent('executor', 'echo "Done"');
    
    runScript('task', ['create', JSON.stringify({ id: 'A', goal: 'Task A' })]);
    runScript('task', ['create', JSON.stringify({ id: 'B', goal: 'Task B', dependencies: ['A'] })]);
    
    // Try to run B (should be blocked)
    try {
      runScript('run', ['B']);
    } catch (e) {
      // Expected to fail in run-task.js due to dependencies
    }
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'B.json'))).status, 'blocked');
    
    // Run A
    runScript('run', ['A']);
    runScript('review', ['A', `--mock-input={"status":"pass"}`]);
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'A.json'))).status, 'completed');
    
    // Now run B (should work)
    runScript('run', ['B']);
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'B.json'))).status, 'running');
    console.log('T2 PASSED');

    // T3: Review-Fix Loop
    console.log('\n--- T3: Review-Fix Loop ---');
    setup();
    runScript('init', ['.']);
    createMockAgent('executor', 'echo "Fixed"');
    runScript('task', ['create', JSON.stringify({ id: 'T3', goal: 'Task 3' })]);
    
    runScript('run', ['T3']);
    runScript('review', ['T3', `--mock-input={"status":"fail", "findings":[{"message":"Needs more work"}]}`]);
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'T3.json'))).status, 'fix-needed');
    
    // Re-run (reset to queued then running)
    runScript('run', ['T3']); 
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'T3.json'))).status, 'running');
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'T3.json'))).run_count, 2);
    
    runScript('review', ['T3', `--mock-input={"status":"pass"}`]);
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'T3.json'))).status, 'completed');
    console.log('T3 PASSED');

    // T4: Max Retries
    console.log('\n--- T4: Max Retries ---');
    setup();
    runScript('init', ['.']);
    createMockAgent('executor', 'echo "Trying again"');
    runScript('task', ['create', JSON.stringify({ id: 'T4', goal: 'Task 4' })]);
    
    // Cycle 1
    runScript('run', ['T4']);
    runScript('review', ['T4', `--mock-input={"status":"fail"}`]);
    
    // Cycle 2
    runScript('run', ['T4']);
    runScript('review', ['T4', `--mock-input={"status":"fail"}`]);
    
    // Cycle 3
    runScript('run', ['T4']);
    runScript('review', ['T4', `--mock-input={"status":"fail"}`]);
    
    // Cycle 4 (Should fail)
    try {
      runScript('run', ['T4']);
    } catch (e) {
      // Expected failure
    }
    const finalTask4 = JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'T4.json')));
    assert.strictEqual(finalTask4.status, 'failed');
    assert.ok(finalTask4.error.includes('Maximum 3-cycle threshold'));
    console.log('T4 PASSED');

    // T5: Recovery
    console.log('\n--- T5: Recovery ---');
    setup();
    runScript('init', ['.']);
    runScript('task', ['create', JSON.stringify({ id: 'T5', goal: 'Task 5', status: 'running' })]);
    
    // Case 1: Running but no lock
    runScript('sync');
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'T5.json'))).status, 'interrupted');
    
    // Case 2: Running with lock, but we manually run sync (simulating crash recovery)
    runScript('status', ['set', 'T5', 'running']);
    runScript('lock', ['acquire', 'T5', 'some_file.txt']);
    // Sync shouldn't change it if lock exists
    runScript('sync');
    assert.strictEqual(JSON.parse(fs.readFileSync(path.join(WORKDIR, '.vibe', 'tasks', 'T5.json'))).status, 'running');
    
    // Case 3: Orphaned lock
    runScript('status', ['set', 'T5', 'completed']);
    // T5 is completed but lock remains
    assert.ok(fs.existsSync(path.join(WORKDIR, '.vibe', 'locks', Buffer.from('some_file.txt').toString('base64').replace(/\//g, '_') + '.lock')));
    runScript('sync');
    // Lock should be gone
    const lockDir = path.join(WORKDIR, '.vibe', 'locks');
    const locks = fs.readdirSync(lockDir).filter(f => f.endsWith('.lock'));
    assert.strictEqual(locks.length, 0);
    console.log('T5 PASSED');

    console.log('\nALL E2E INTEGRATION TESTS PASSED!');
  } catch (error) {
    console.error('\nTest failed:');
    console.error(error.message);
    if (error.stdout) console.error('Stdout:', error.stdout.toString());
    if (error.stderr) console.error('Stderr:', error.stderr.toString());
    process.exit(1);
  } finally {
    // Keep workdir if it failed? No, clean up usually.
    // fs.rmSync(WORKDIR, { recursive: true, force: true });
  }
}

runTests();
