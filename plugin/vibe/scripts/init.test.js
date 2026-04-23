const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const assert = require('assert');

const TEST_DIR = path.join(__dirname, 'test_target');
const INIT_SCRIPT = path.join(__dirname, 'init.js');

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

function runInit(args = []) {
  execSync(`node ${INIT_SCRIPT} ${args.join(' ')}`, { stdio: 'pipe' });
}

try {
  // Test 1: Given an empty directory, creates .vibe/ with all subdirectories and copies all template files.
  setup();
  runInit([TEST_DIR]);
  const configPath = path.join(TEST_DIR, '.vibe', 'config.json');
  assert.ok(fs.existsSync(configPath), 'config.json should exist');
  
  const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
  assert.strictEqual(config.default_model, 'claude', 'config.json should have default_model');
  assert.ok(config.lock_policy, 'config.json should have lock_policy');

  const plannerPath = path.join(TEST_DIR, '.vibe', 'agents', 'planner.json');
  assert.ok(fs.existsSync(plannerPath), 'planner.json should exist');
  
  const planner = JSON.parse(fs.readFileSync(plannerPath, 'utf8'));
  assert.ok(planner.prompt, 'planner.json should have prompt');
  assert.ok(planner.reference, 'planner.json should have reference');

  assert.ok(fs.existsSync(path.join(TEST_DIR, '.vibe', 'tasks')), 'tasks dir should exist');
  assert.ok(fs.existsSync(path.join(TEST_DIR, '.vibe', 'runs')), 'runs dir should exist');
  assert.ok(fs.existsSync(path.join(TEST_DIR, '.vibe', 'locks')), 'locks dir should exist');
  assert.ok(fs.existsSync(path.join(TEST_DIR, '.vibe', 'reviews')), 'reviews dir should exist');
  assert.ok(fs.existsSync(path.join(TEST_DIR, '.vibe', 'logs')), 'logs dir should exist');

  // Test 2: Given an existing .vibe/config.json, does not overwrite it unless --force is passed.
  setup();
  fs.mkdirSync(path.join(TEST_DIR, '.vibe'), { recursive: true });
  fs.writeFileSync(path.join(TEST_DIR, '.vibe', 'config.json'), '{"existing": true}');
  
  runInit([TEST_DIR]);
  let configContent = fs.readFileSync(path.join(TEST_DIR, '.vibe', 'config.json'), 'utf8');
  assert.strictEqual(configContent, '{"existing": true}', 'config.json should NOT be overwritten without --force');

  // Run with --force
  runInit([TEST_DIR, '--force']);
  configContent = fs.readFileSync(path.join(TEST_DIR, '.vibe', 'config.json'), 'utf8');
  assert.notStrictEqual(configContent, '{"existing": true}', 'config.json should be overwritten with --force');

  // Test 3: Even with --force, does not delete unrecognized *.json files in .vibe/agents/
  setup();
  runInit([TEST_DIR]);
  const customAgentPath = path.join(TEST_DIR, '.vibe', 'agents', 'custom.json');
  fs.writeFileSync(customAgentPath, '{"custom": true}');
  
  runInit([TEST_DIR, '--force']);
  assert.ok(fs.existsSync(customAgentPath), 'custom.json in agents should not be deleted even with --force');

  console.log('All tests passed.');
} catch (error) {
  console.error('Test failed:', error.message);
  process.exit(1);
} finally {
  cleanup();
}