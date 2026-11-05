// @vitest-environment node
import assert from 'node:assert/strict';
import { test } from 'vitest';

import { buildSync } from 'esbuild';
import { fileURLToPath } from 'node:url';

// Bundle pure logic with this test's config; the optional Claude host config is absent in CI.
const built = buildSync({
  entryPoints: [fileURLToPath(new URL('../../../../integrations/claude-code/speclink-skills/hooks/focus.ts', import.meta.url))],
  bundle: true, platform: 'node', format: 'cjs', write: false, tsconfigRaw: {},
});
const pure = { exports: {} as Record<string, (...args: any[]) => any> };
new Function('module', 'exports', built.outputFiles[0].text)(pure, pure.exports);
const { afterPrompt, commandChange, withVerb, focusFromHistory } = pure.exports;
const initial = { verb: 'apply', change: 'add-auth' };

test('非變更技能清除舊焦點，背景查詢不冒充正在處理的變更', () => {
  for (const verb of ['discuss', 'improve', 'audit', 'trace', 'manual', 'baseline', 'config']) {
    const focus = afterPrompt(initial, `/speclink-${verb} add-auth`, ['add-auth'], 'speclink-panel');
    assert.deepEqual(focus, { verb, change: null });
    assert.equal(commandChange(focus, 'speclink show add-auth', ['add-auth']), null);
  }
});

test('propose 清除前一個變更，建立後可確認新的名稱', () => {
  const focus = withVerb(initial, 'propose');
  assert.deepEqual(focus, { verb: 'propose', change: null });
  assert.equal(commandChange(focus, 'speclink status --change add-login --json', ['add-login']), 'add-login');
});

test('沒帶名稱的 quality 保留目前變更，一般文字不換焦點', () => {
  assert.deepEqual(afterPrompt(initial, '/speclink-quality', ['add-auth'], 'speclink-panel'), { verb: 'quality', change: 'add-auth' });
  assert.deepEqual(afterPrompt(initial, '幫我看 /speclink-apply 怎麼用', ['add-auth'], 'speclink-panel'), initial);
});

test('resume 依非變更技能清除焦點，不把後續背景 Bash 查詢綁回變更', () => {
  assert.deepEqual(focusFromHistory([
    { role: 'user', text: '/speclink-apply add-auth', toolUses: [] },
    { role: 'user', text: '/speclink-discuss add-auth', toolUses: [
      { tool: 'Bash', input: { command: 'speclink show add-auth' } },
    ] },
  ], ['add-auth'], 'speclink-panel'), { verb: 'discuss', change: null });
});
