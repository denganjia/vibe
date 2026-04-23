const test = require('node:test');
const assert = require('node:assert');
const fs = require('node:fs');
const path = require('node:path');

// We will implement these in release-summary.js
// For now, we'll try to require them and handle the failure if the file doesn't exist yet
let releaseSummary;
try {
  releaseSummary = require('./release-summary.js');
} catch (e) {
  // Mock for initial Red state in TDD
  releaseSummary = {
    categorize: () => 'Internal Changes',
    getTaskInfo: () => null,
    generateMarkdown: () => ''
  };
}

test('Categorization logic', async (t) => {
  await t.test('should identify standard CC feat', () => {
    assert.strictEqual(releaseSummary.categorize('feat(ui): add button'), 'feat');
  });

  await t.test('should identify standard CC fix', () => {
    assert.strictEqual(releaseSummary.categorize('fix: crash on start'), 'fix');
  });

  await t.test('should fuzzy match add to feat', () => {
    assert.strictEqual(releaseSummary.categorize('add new component'), 'feat');
  });

  await t.test('should fuzzy match patch to fix', () => {
    assert.strictEqual(releaseSummary.categorize('patch security vulnerability'), 'fix');
  });

  await t.test('should fallback to Internal Changes', () => {
    assert.strictEqual(releaseSummary.categorize('some random work'), 'Internal Changes');
  });
});

test('Task association logic', async (t) => {
  const mockVibeDir = path.join(process.cwd(), '.vibe', 'tasks');
  if (!fs.existsSync(mockVibeDir)) {
    fs.mkdirSync(mockVibeDir, { recursive: true });
  }

  const taskId = 'TEST-123';
  const taskFile = path.join(mockVibeDir, `${taskId}.json`);
  const taskData = { id: taskId, title: 'Test Task', status: 'done' };
  
  fs.writeFileSync(taskFile, JSON.stringify(taskData));

  await t.test('should extract and load task info when file exists', () => {
    const info = releaseSummary.getTaskInfo(`completed (task: ${taskId})`);
    assert.ok(info);
    assert.strictEqual(info.id, taskId);
    // Note: The actual implementation might return the whole object or formatted string
    // We expect it to at least return the ID if it matches
  });

  await t.test('should handle missing task file gracefully', () => {
    const info = releaseSummary.getTaskInfo('completed (task: NONEXISTENT)');
    assert.ok(info);
    assert.strictEqual(info.id, 'NONEXISTENT');
    assert.strictEqual(info.missing, true);
  });

  await t.test('should return null when no task pattern found', () => {
    assert.strictEqual(releaseSummary.getTaskInfo('just a commit'), null);
  });

  // Clean up
  if (fs.existsSync(taskFile)) {
    fs.unlinkSync(taskFile);
  }
});

test('Boundary cases', async (t) => {
  await t.test('should handle empty commit message', () => {
    assert.strictEqual(releaseSummary.categorize(''), 'Internal Changes');
  });
});
