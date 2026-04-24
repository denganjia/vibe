const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const yaml = require('js-yaml');

const pluginRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(pluginRoot, '..', '..');
const skillsDir = path.join(pluginRoot, 'skills');

test('SKILL.md files have valid frontmatter', () => {
  const skillDirs = fs.readdirSync(skillsDir, { withFileTypes: true })
    .filter(dirent => dirent.isDirectory())
    .map(dirent => dirent.name);

  assert.ok(skillDirs.length > 0, 'Should have at least one skill directory');

  for (const skillName of skillDirs) {
    const skillPath = path.join(skillsDir, skillName, 'SKILL.md');
    assert.ok(fs.existsSync(skillPath), `Skill ${skillName} should have SKILL.md`);

    const content = fs.readFileSync(skillPath, 'utf8');
    const match = content.match(/^\s*---\n([\s\S]*?)\n---/);
    assert.ok(match, `Skill ${skillName}/SKILL.md should have YAML frontmatter`);

    const frontmatter = yaml.load(match[1]);
    assert.ok(frontmatter.name, `Skill ${skillName} frontmatter should have 'name'`);
    assert.ok(frontmatter.version, `Skill ${skillName} frontmatter should have 'version'`);
    assert.ok(frontmatter.description, `Skill ${skillName} frontmatter should have 'description'`);
  }
});

test('Legacy files and directories are removed', () => {
  const legacyPaths = [
    path.join(pluginRoot, 'commands'),
    path.join(pluginRoot, 'roles'),
    path.join(skillsDir, 'Conductor.md')
  ];

  for (const legacyPath of legacyPaths) {
    const relativePath = path.relative(repoRoot, legacyPath);
    assert.ok(!fs.existsSync(legacyPath), `Legacy path should be removed: ${relativePath}`);
  }
});
