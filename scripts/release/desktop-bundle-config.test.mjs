import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { bundleConfigFor } from './desktop-bundle-config.mjs';

const root = fileURLToPath(new URL('../../', import.meta.url));
const script = new URL('./desktop-bundle-config.mjs', import.meta.url);
const secret = 'fake-apple-secret-value';

test('Release 在 macOS 無 Apple 憑證時明確使用 ad-hoc，full 保留 Tauri 簽章環境', () => {
  assert.deepEqual(bundleConfigFor({ channel: 'release', platform: 'darwin', macOSSigning: 'none' }), {
    bundle: { macOS: { signingIdentity: '-' } },
  });
  assert.deepEqual(bundleConfigFor({ channel: 'release', platform: 'darwin', macOSSigning: 'full' }), {});
});

test('Release 在 Windows 與 Linux 不覆寫原有 Tauri 設定', () => {
  for (const platform of ['win32', 'linux']) {
    assert.deepEqual(bundleConfigFor({ channel: 'release', platform, macOSSigning: 'none' }), {});
  }
});

test('Desktop Build 保留停用 updater，並只在 macOS 設 ad-hoc', () => {
  assert.deepEqual(bundleConfigFor({ channel: 'desktop-build', platform: 'darwin' }), {
    bundle: { createUpdaterArtifacts: false, macOS: { signingIdentity: '-' } },
  });
  for (const platform of ['win32', 'linux']) {
    assert.deepEqual(bundleConfigFor({ channel: 'desktop-build', platform }), {
      bundle: { createUpdaterArtifacts: false },
    });
  }
});

test('未知 channel 或 macOS Release 簽章模式會明確失敗', () => {
  assert.throws(() => bundleConfigFor({ channel: 'other', platform: 'darwin' }), /channel/);
  assert.throws(() => bundleConfigFor({ channel: 'release', platform: 'darwin', macOSSigning: 'partial' }), /SPECLINK_MACOS_SIGNING/);
});

test('CLI 只輸出 JSON 設定且不洩漏 secret', () => {
  const result = spawnSync(process.execPath, [script.pathname, 'release'], {
    cwd: root,
    encoding: 'utf8',
    env: { ...process.env, SPECLINK_MACOS_SIGNING: 'none', APPLE_CERTIFICATE: secret },
  });

  assert.equal(result.status, 0, result.stderr);
  assert.deepEqual(JSON.parse(result.stdout), bundleConfigFor({
    channel: 'release', platform: process.platform, macOSSigning: 'none',
  }));
  assert.equal(result.stdout.trim().split('\n').length, 1);
  assert.equal(result.stderr, '');
  assert.doesNotMatch(result.stdout + result.stderr, new RegExp(secret));
});

test('CLI 對未知 channel 或額外參數非零結束且不輸出設定 JSON', () => {
  for (const args of [['unknown'], ['release', 'extra']]) {
    const result = spawnSync(process.execPath, [script.pathname, ...args], {
      cwd: root,
      encoding: 'utf8',
      env: { ...process.env, SPECLINK_MACOS_SIGNING: 'none' },
    });
    assert.notEqual(result.status, 0);
    assert.equal(result.stdout, '');
    assert.notEqual(result.stderr, '');
  }
});

test('兩條 workflow 都用共用設定檔，Desktop Build 不改來源資訊輸出', () => {
  const release = readFileSync(new URL('../../.github/workflows/release.yml', import.meta.url), 'utf8');
  const desktop = readFileSync(new URL('../../.github/workflows/desktop-build.yml', import.meta.url), 'utf8');

  assert.match(release, /node scripts\/release\/desktop-bundle-config\.mjs release/);
  assert.match(release, /--config "\$RUNNER_TEMP\/desktop-release-config\.json"/);
  assert.match(desktop, /node scripts\/release\/desktop-bundle-config\.mjs desktop-build/);
  assert.match(desktop, /--config "\$RUNNER_TEMP\/desktop-build-config\.json"/);
  assert.match(desktop, /Repository: \$\{process\.env\.GITHUB_REPOSITORY\}/);
  assert.match(desktop, /Commit: \$\{process\.env\.GITHUB_SHA\}/);
});
