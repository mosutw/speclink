import assert from 'node:assert/strict';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, unlinkSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { parseArgs, verifyArtifacts } from './macos-artifact-verify.mjs';

const app = 'target/universal-apple-darwin/release/bundle/macos/Speclink.app';
const dmg = 'target/universal-apple-darwin/release/bundle/dmg/Speclink_0.12.1_universal.dmg';
const updater = 'target/universal-apple-darwin/release/bundle/macos/Speclink.app.tar.gz';

function fixture(t) {
  const root = mkdtempSync(path.join(os.tmpdir(), 'macos-artifact-verify-test-'));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const app = path.join(root, 'Speclink.app');
  for (const directory of ['Contents/MacOS', 'Contents/_CodeSignature']) mkdirSync(path.join(app, directory), { recursive: true });
  for (const [file, value] of [
    ['Contents/Info.plist', 'info'],
    ['Contents/MacOS/speclink-desktop', 'desktop-binary'],
    ['Contents/MacOS/speclink', 'cli-binary'],
    ['Contents/_CodeSignature/CodeResources', 'resources'],
  ]) writeFileSync(path.join(app, file), value);
  const dmg = path.join(root, 'speclink.dmg');
  const updater = path.join(root, 'speclink.app.tar.gz');
  writeFileSync(dmg, 'dmg-fixture');
  writeFileSync(updater, 'archive-fixture');
  return { root, app, dmg, updater };
}

function fakeRunner(f, overrides = {}) {
  const commands = [];
  return {
    commands,
    run(command, args, options) {
      commands.push({ command, args, options });
      if (overrides.fail?.(command, args)) return { status: 1, stdout: '', stderr: 'fixture failure' };
      if (command === 'codesign' && args[0] === '--display') {
        return { status: 0, stdout: '', stderr: `${overrides.signature ?? 'Signature=adhoc'}\n` };
      }
      if (command === '/usr/libexec/PlistBuddy') return { status: 0, stdout: '0.12.1\n', stderr: '' };
      if (command === 'lipo') {
        const archs = overrides.archs?.(args) ?? 'arm64 x86_64';
        return { status: 0, stdout: `${archs}\n`, stderr: '' };
      }
      if (command === 'hdiutil' && args[0] === 'attach') {
        const mountPoint = args[args.indexOf('-mountpoint') + 1];
        copyApp(f.app, path.join(mountPoint, 'Speclink.app'));
        if (overrides.dmgMismatch) writeFileSync(path.join(mountPoint, 'Speclink.app/Contents/MacOS/speclink'), 'different-cli');
        return { status: 0, stdout: '/dev/disk9 Apple_HFS\n', stderr: '' };
      }
      if (command === 'tar') {
        const destination = args[args.indexOf('-C') + 1];
        copyApp(f.app, path.join(destination, 'Speclink.app'));
        if (overrides.updaterMismatch) writeFileSync(path.join(destination, 'Speclink.app/Contents/MacOS/speclink'), 'different-cli');
      }
      if (overrides.timeout?.(command, args)) {
        const error = new Error('timed out');
        error.code = 'ETIMEDOUT';
        return { status: null, error, stdout: '', stderr: '' };
      }
      return { status: 0, stdout: '', stderr: '' };
    },
  };
}

function copyApp(source, destination) {
  mkdirSync(path.dirname(destination), { recursive: true });
  const copy = (from, to) => {
    mkdirSync(path.dirname(to), { recursive: true });
    writeFileSync(to, readFileSync(from));
  };
  for (const relative of [
    'Contents/Info.plist',
    'Contents/MacOS/speclink-desktop',
    'Contents/MacOS/speclink',
    'Contents/_CodeSignature/CodeResources',
  ]) copy(path.join(source, relative), path.join(destination, relative));
}

test('Release 參數必須包含 updater；Desktop Build 明確不接受 updater', () => {
  assert.throws(() => parseArgs(['--app', app, '--dmg', dmg, '--mode', 'none', '--channel', 'release']), /--updater/);
  assert.throws(() => parseArgs(['--app', app, '--dmg', dmg, '--mode', 'none', '--channel', 'desktop-build', '--updater', updater]), /不接受 updater/);
  assert.equal(parseArgs(['--app', app, '--dmg', dmg, '--mode', 'full', '--channel', 'release', '--updater', updater]).updater, updater);
});

test('參數拒絕未知、重複、缺值與不支援的模式', () => {
  const valid = ['--app', app, '--dmg', dmg, '--mode', 'none', '--channel', 'desktop-build'];
  assert.throws(() => parseArgs([...valid, '--extra', 'value']), /參數無效/);
  assert.throws(() => parseArgs([...valid, '--app', 'second.app']), /參數重複/);
  assert.throws(() => parseArgs([...valid, '--mode']), /參數無效/);
  assert.throws(() => parseArgs([...valid.slice(0, 4), 'unsigned', ...valid.slice(6)]), /mode/);
});

test('拒絕非 macOS runner，且在執行 native 工具前失敗', () => {
  let invoked = false;
  assert.throws(() => verifyArtifacts({ app, dmg, mode: 'none', channel: 'desktop-build' }, {
    platform: 'linux',
    runner: () => { invoked = true; return { status: 0, stdout: '', stderr: '' }; },
  }), /只能在 macOS runner/);
  assert.equal(invoked, false);
});

test('成功時驗證 App、DMG 與 updater，none 不呼叫 Gatekeeper 或 stapler', (t) => {
  const f = fixture(t);
  const native = fakeRunner(f);
  const result = verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, { platform: 'darwin', runner: native.run });
  assert.deepEqual(result, { mode: 'none', version: '0.12.1', channel: 'release', updater: true });
  assert.ok(native.commands.some(({ command, args }) => command === 'hdiutil' && args[0] === 'verify'));
  assert.ok(native.commands.some(({ command, args }) => command === 'tar' && args[0] === '-xzf'));
  assert.ok(native.commands.some(({ command, args }) => command === 'hdiutil' && args[0] === 'detach' && args[1] === '/dev/disk9'));
  assert.ok(native.commands.every(({ options }) => options.timeout === 120_000));
  assert.equal(native.commands.some(({ command }) => command === 'spctl' || command === 'xcrun'), false);
});

test('拒絕 DMG 或 updater 內與建置 App 不同的執行檔', (t) => {
  for (const mismatch of ['dmgMismatch', 'updaterMismatch']) {
    const f = fixture(t);
    const native = fakeRunner(f, { [mismatch]: true });
    assert.throws(() => verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, { platform: 'darwin', runner: native.run }), /內容不同/);
    assert.ok(native.commands.some(({ command, args }) => command === 'hdiutil' && args[0] === 'detach'));
  }
});

test('檢查兩支程式都帶 arm64 與 x86_64，也會拒絕簽章失敗', (t) => {
  const f = fixture(t);
  assert.throws(() => verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, {
    platform: 'darwin', runner: fakeRunner(f, { archs: () => 'arm64' }).run,
  }), /缺少 x86_64 架構/);
  assert.throws(() => verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, {
    platform: 'darwin', runner: fakeRunner(f, { fail: (command) => command === 'codesign' }).run,
  }), /codesign.*失敗/);
  assert.throws(() => verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, {
    platform: 'darwin', runner: fakeRunner(f, { signature: 'Signature=Developer ID Application: Speclink' }).run,
  }), /簽章模式不符.*ad-hoc/);
});

test('缺 App、DMG、sidecar 或 CodeResources 時在掛載前失敗', (t) => {
  const f = fixture(t);
  const noNative = () => { throw new Error('不應執行 native 命令'); };
  assert.throws(() => verifyArtifacts({ ...f, app: `${f.app}-missing`, mode: 'none', channel: 'release' }, { platform: 'darwin', runner: noNative }), /建置 App 不存在/);
  assert.throws(() => verifyArtifacts({ ...f, dmg: `${f.dmg}-missing`, mode: 'none', channel: 'release' }, { platform: 'darwin', runner: noNative }), /DMG 不存在/);
  for (const relative of ['Contents/MacOS/speclink', 'Contents/_CodeSignature/CodeResources']) {
    unlinkSync(path.join(f.app, relative));
    assert.throws(() => verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, { platform: 'darwin', runner: noNative }), /不存在/);
    copyAppFixtureFile(f, relative);
  }
});

function copyAppFixtureFile(f, relative) {
  writeFileSync(path.join(f.app, relative), relative);
}

test('DMG checksum 或 updater 解包失敗時拒絕交付', (t) => {
  const f = fixture(t);
  const badDmg = fakeRunner(f, { fail: (command, args) => command === 'hdiutil' && args[0] === 'verify' });
  assert.throws(() => verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, { platform: 'darwin', runner: badDmg.run }), /hdiutil verify.*失敗/);
  assert.equal(badDmg.commands.some(({ command, args }) => command === 'hdiutil' && args[0] === 'attach'), false);

  const badTar = fakeRunner(f, { fail: (command) => command === 'tar' });
  assert.throws(() => verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, { platform: 'darwin', runner: badTar.run }), /tar -xzf.*失敗/);
  assert.ok(badTar.commands.some(({ command, args }) => command === 'hdiutil' && args[0] === 'detach'));
});

test('full 模式要求每個 App 的 Gatekeeper 與公證票證驗證', (t) => {
  const f = fixture(t);
  const native = fakeRunner(f, { signature: 'Signature=Developer ID Application: Speclink' });
  verifyArtifacts({ ...f, mode: 'full', channel: 'release' }, { platform: 'darwin', runner: native.run });
  assert.equal(native.commands.filter(({ command }) => command === 'spctl').length, 3);
  assert.equal(native.commands.filter(({ command, args }) => command === 'xcrun' && args[0] === 'stapler').length, 3);
  const rejected = fakeRunner(f, {
    signature: 'Signature=Developer ID Application: Speclink',
    fail: (command) => command === 'xcrun',
  });
  assert.throws(() => verifyArtifacts({ ...f, mode: 'full', channel: 'release' }, { platform: 'darwin', runner: rejected.run }), /xcrun.*失敗/);
});

test('native timeout 失敗且先卸載已掛載 DMG', (t) => {
  const f = fixture(t);
  const native = fakeRunner(f, {
    timeout: (command, args) => command === 'codesign' && args.at(-1)?.includes(`${path.sep}dmg${path.sep}Speclink.app`),
  });
  assert.throws(() => verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, { platform: 'darwin', runner: native.run }), /無法執行|失敗/);
  assert.ok(native.commands.some(({ command, args }) => command === 'hdiutil' && args[0] === 'detach'));
});

test('卸載失敗時回報掛載點並保留暫存目錄', (t) => {
  const f = fixture(t);
  const native = fakeRunner(f, { fail: (command, args) => command === 'hdiutil' && args[0] === 'detach' });
  let error;
  try {
    verifyArtifacts({ ...f, mode: 'none', channel: 'release' }, { platform: 'darwin', runner: native.run });
  } catch (caught) {
    error = caught;
  }
  assert.match(error?.message ?? '', /卸載失敗.*掛載點留在 runner 暫存目錄/);
  const preserved = error.message.match(/runner 暫存目錄 (.+)：/)?.[1];
  assert.ok(preserved && existsSync(preserved));
  rmSync(preserved, { recursive: true, force: true });
});

test('兩條 workflow 都在 macOS artifact 上傳前執行共用 verifier', () => {
  const release = readFileSync(new URL('../../.github/workflows/release.yml', import.meta.url), 'utf8');
  const desktop = readFileSync(new URL('../../.github/workflows/desktop-build.yml', import.meta.url), 'utf8');
  for (const [workflow, name] of [[release, 'Verify macOS app, DMG, and updater'], [desktop, 'Verify macOS app and DMG']]) {
    const verifyAt = workflow.indexOf(name);
    const uploadAt = workflow.indexOf('uses: actions/upload-artifact@v4', verifyAt);
    assert.notEqual(verifyAt, -1, `workflow 缺少 ${name}`);
    assert.ok(uploadAt > verifyAt, '產物驗證必須早於上傳');
    assert.match(workflow.slice(verifyAt, uploadAt), /macos-artifact-verify\.mjs/);
  }
  assert.match(release, /--channel release[\s\S]*--updater/);
  assert.match(desktop, /--channel desktop-build/);
});
