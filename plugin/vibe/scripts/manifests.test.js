const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');

const pluginRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(pluginRoot, '..', '..');

const files = {
  packageJson: path.join(pluginRoot, 'package.json'),
  geminiManifest: path.join(pluginRoot, 'gemini-extension.json'),
  claudeManifest: path.join(pluginRoot, '.claude-plugin', 'plugin.json'),
  codexManifest: path.join(pluginRoot, '.codex-plugin', 'plugin.json'),
  marketplace: path.join(repoRoot, '.agents', 'plugins', 'marketplace.json')
};

function readJson(filePath) {
  assert.ok(fs.existsSync(filePath), `${path.relative(repoRoot, filePath)} should exist`);
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function assertSafeRelativePath(value, expected, label) {
  assert.equal(value, expected, `${label} should be ${expected}`);
  assert.ok(!path.isAbsolute(value), `${label} should not be absolute`);
  assert.ok(!value.includes('../'), `${label} should not traverse outside its root`);
}

test('manifest files exist and parse as JSON', () => {
  for (const filePath of Object.values(files)) {
    assert.doesNotThrow(() => readJson(filePath), `${path.relative(repoRoot, filePath)} should parse`);
  }
});

test('package.json defines the shared plugin identity and smoke test entrypoint', () => {
  const pkg = readJson(files.packageJson);

  assert.equal(pkg.name, 'vibe');
  assert.equal(pkg.version, '0.1.0');
  assert.equal(pkg.description, 'Plugin-first multi-model collaboration for Vibe workspaces.');
  assert.equal(pkg.private, true);
  assert.equal(pkg.type, 'commonjs');
  assert.deepEqual(pkg.scripts, {
    test: 'npm run test:manifests',
    'test:manifests': 'node --test scripts/manifests.test.js'
  });
  assert.deepEqual(pkg.engines, { node: '>=22.0.0' });
  assert.ok(!('mcpServers' in pkg), 'package.json should not declare mcpServers');

  const dependencies = { ...pkg.dependencies, ...pkg.devDependencies };
  assert.ok(
    !Object.prototype.hasOwnProperty.call(dependencies, '@modelcontextprotocol/sdk'),
    'package.json should not include the MCP SDK before Phase 27'
  );
});

test('provider manifests stay aligned with package identity and phase boundaries', () => {
  const pkg = readJson(files.packageJson);
  const gemini = readJson(files.geminiManifest);
  const claude = readJson(files.claudeManifest);
  const codex = readJson(files.codexManifest);

  for (const manifest of [gemini, claude, codex]) {
    assert.equal(manifest.name, pkg.name);
    assert.equal(manifest.version, pkg.version);
    assert.equal(manifest.description, pkg.description);
    assert.ok(!('mcpServers' in manifest), 'provider manifests should not declare mcpServers yet');
  }

  assert.deepEqual(Object.keys(gemini).sort(), ['description', 'name', 'version']);
  assert.deepEqual(Object.keys(claude).sort(), ['description', 'name', 'version']);

  assertSafeRelativePath(codex.skills, './skills/', 'codex skills path');
  assert.deepEqual(codex.interface, {
    displayName: 'Vibe',
    shortDescription: 'Coordinate AI Agents through project-local tasks, reviews, and logs.'
  });
  assert.ok(!('contextFileName' in gemini), 'gemini manifest should not declare contextFileName');
  assert.ok(!('plan' in gemini), 'gemini manifest should not declare plan');
  assert.ok(!('skills' in claude), 'claude manifest should stay minimal in Phase 25');
  assert.ok(!('commands' in claude), 'claude manifest should not override commands yet');
});

test('marketplace discovery remains local and policy-complete', () => {
  const marketplace = readJson(files.marketplace);
  const vibePlugin = marketplace.plugins.find((plugin) => plugin.name === 'vibe');

  assert.equal(marketplace.name, 'vibe-local');
  assert.deepEqual(marketplace.interface, { displayName: 'Vibe Local Plugins' });
  assert.ok(vibePlugin, 'marketplace should include a vibe plugin entry');
  assert.equal(vibePlugin.source?.source, 'local');
  assertSafeRelativePath(vibePlugin.source?.path, './plugin/vibe', 'marketplace source path');
  assert.equal(vibePlugin.policy?.installation, 'AVAILABLE');
  assert.equal(vibePlugin.policy?.authentication, 'ON_INSTALL');
  assert.equal(vibePlugin.category, 'Productivity');
});
