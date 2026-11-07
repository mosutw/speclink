#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  rmSync,
  statSync,
} from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ARCHITECTURES = ['arm64', 'x86_64'];
const TIMEOUT_MS = 120_000;

export function parseArgs(argv) {
  const values = {};
  for (let index = 0; index < argv.length; index += 2) {
    const key = argv[index];
    const value = argv[index + 1];
    if (!['--app', '--dmg', '--mode', '--channel', '--updater'].includes(key) || !value || value.startsWith('--')) {
      throw new Error(`參數無效：${key ?? ''}`);
    }
    const name = key.slice(2);
    if (values[name] !== undefined) throw new Error(`參數重複：${key}`);
    values[name] = value;
  }
  if (!values.app || !values.dmg || !values.mode || !values.channel) {
    throw new Error('必須提供 --app、--dmg、--mode none|full 與 --channel release|desktop-build');
  }
  if (!['none', 'full'].includes(values.mode)) throw new Error('--mode 必須是 none 或 full');
  if (!['release', 'desktop-build'].includes(values.channel)) throw new Error('--channel 必須是 release 或 desktop-build');
  if (values.channel === 'release' && !values.updater) throw new Error('Release 驗證必須提供 --updater');
  if (values.channel === 'desktop-build' && values.updater) throw new Error('Desktop Build 不接受 updater');
  return values;
}

function run(command, args, runner = spawnSync) {
  const result = runner(command, args, {
    encoding: 'utf8',
    timeout: TIMEOUT_MS,
    maxBuffer: 8 * 1024 * 1024,
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  if (result.error) throw new Error(`${command} ${args[0] ?? ''} 無法執行：${result.error.message}`);
  if (result.status !== 0) {
    const detail = (result.stderr || result.stdout || '').trim();
    const reason = result.signal ? `signal ${result.signal}` : `exit ${result.status}`;
    throw new Error(`${command} ${args.join(' ')} 失敗（${reason}）${detail ? `：${detail}` : ''}`);
  }
  return result.stdout ?? '';
}

function requireSigningIdentity(target, mode, runner) {
  const result = runner('codesign', ['--display', '--verbose=2', target], {
    encoding: 'utf8',
    timeout: TIMEOUT_MS,
    maxBuffer: 8 * 1024 * 1024,
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  if (result.error || result.status !== 0) {
    throw new Error(`codesign --display ${target} 無法讀取簽章資訊：${result.error?.message ?? result.stderr ?? result.status}`);
  }
  const details = `${result.stdout ?? ''}\n${result.stderr ?? ''}`;
  const isAdHoc = /(?:^|\n)Signature=adhoc(?:\n|$)/.test(details);
  if ((mode === 'none') !== isAdHoc) {
    throw new Error(`${target} 簽章模式不符：預期 ${mode === 'none' ? 'ad-hoc' : 'Developer ID'}`);
  }
}

function requirePath(file, label, expectedType) {
  if (!existsSync(file)) throw new Error(`${label} 不存在：${file}`);
  const stat = statSync(file);
  const valid = expectedType === 'directory' ? stat.isDirectory() : stat.isFile();
  if (!valid) throw new Error(`${label} 不是${expectedType === 'directory' ? '目錄' : '檔案'}：${file}`);
}

function sha256(file) {
  return createHash('sha256').update(readFileSync(file)).digest('hex');
}

function appIdentity(app, mode, runner) {
  const contents = path.join(app, 'Contents');
  const info = path.join(contents, 'Info.plist');
  const executable = path.join(contents, 'MacOS', 'speclink-desktop');
  const sidecar = path.join(contents, 'MacOS', 'speclink');
  const resources = path.join(contents, '_CodeSignature', 'CodeResources');
  for (const [file, label] of [[info, 'Info.plist'], [executable, '主程式'], [sidecar, 'CLI sidecar'], [resources, 'CodeResources']]) {
    requirePath(file, label, 'file');
  }

  run('codesign', ['--verify', '--deep', '--strict', '--all-architectures', '--verbose=2', app], runner);
  requireSigningIdentity(app, mode, runner);
  for (const executablePath of [executable, sidecar]) {
    run('codesign', ['--verify', '--strict', '--all-architectures', '--verbose=2', executablePath], runner);
    requireSigningIdentity(executablePath, mode, runner);
    const architectures = run('lipo', ['-archs', executablePath], runner).trim().split(/\s+/);
    for (const architecture of ARCHITECTURES) {
      if (!architectures.includes(architecture)) {
        throw new Error(`${executablePath} 缺少 ${architecture} 架構：${architectures.join(', ')}`);
      }
    }
  }

  const version = run('/usr/libexec/PlistBuddy', ['-c', 'Print :CFBundleShortVersionString', info], runner).trim();
  if (!version) throw new Error(`${info} 缺少 CFBundleShortVersionString`);
  return {
    version,
    files: [info, executable, sidecar, resources].map((file) => sha256(file)),
  };
}

function compareApp(reference, app, label, mode, runner) {
  const identity = appIdentity(app, mode, runner);
  if (identity.version !== reference.version) throw new Error(`${label} 版號不符：${identity.version}，預期 ${reference.version}`);
  if (identity.files.some((hash, index) => hash !== reference.files[index])) {
    throw new Error(`${label} 的主程式、sidecar、Info.plist 或 CodeResources 與建置 App 不同`);
  }
}

function findMountedDevice(output) {
  const device = output.match(/^\s*(\/dev\/disk\S+)/m)?.[1];
  if (!device) throw new Error('hdiutil attach 沒有回傳可卸載的磁碟裝置');
  return device;
}

function verifyFullNotarization(apps, runner) {
  for (const app of apps) {
    run('spctl', ['--assess', '--type', 'execute', '--verbose=2', app], runner);
    run('xcrun', ['stapler', 'validate', '--verbose', app], runner);
  }
}

export function verifyArtifacts(options, { platform = process.platform, runner = spawnSync } = {}) {
  if (platform !== 'darwin') throw new Error('macOS 產物驗證只能在 macOS runner 執行');
  const { app, dmg, mode, updater, channel } = options;
  if (!['none', 'full'].includes(mode)) throw new Error('mode 必須是 none 或 full');
  if (!['release', 'desktop-build'].includes(channel)) throw new Error('channel 必須是 release 或 desktop-build');
  if ((channel === 'release') !== Boolean(updater)) throw new Error('Release 必須提供 updater，Desktop Build 不可提供 updater');
  for (const [file, label, type] of [
    [app, '建置 App', 'directory'],
    [dmg, 'DMG', 'file'],
    ...(updater ? [[updater, 'updater', 'file']] : []),
  ]) {
    requirePath(file, label, type);
  }

  const reference = appIdentity(app, mode, runner);
  const scratch = mkdtempSync(path.join(os.tmpdir(), 'speclink-macos-artifact-'));
  const mountPoint = path.join(scratch, 'dmg');
  const updaterDir = path.join(scratch, 'updater');
  mkdirSync(mountPoint);
  let device;
  const appsToAssess = [app];
  let attachAttempted = false;
  let detached = false;
  let failure;
  try {
    run('hdiutil', ['verify', dmg], runner);
    attachAttempted = true;
    const output = run('hdiutil', ['attach', '-readonly', '-nobrowse', '-noautoopen', '-mountpoint', mountPoint, dmg], runner);
    device = findMountedDevice(output);
    const dmgApp = path.join(mountPoint, 'Speclink.app');
    compareApp(reference, dmgApp, 'DMG 內 App', mode, runner);
    appsToAssess.push(dmgApp);

    if (updater) {
      mkdirSync(updaterDir);
      run('tar', ['-xzf', updater, '-C', updaterDir], runner);
      const updaterApp = path.join(updaterDir, 'Speclink.app');
      compareApp(reference, updaterApp, 'updater 內 App', mode, runner);
      appsToAssess.push(updaterApp);
    }
    if (mode === 'full') verifyFullNotarization(appsToAssess, runner);
  } catch (error) {
    failure = error;
  }

  if (attachAttempted) {
    try {
      run('hdiutil', ['detach', device ?? mountPoint], runner);
      detached = true;
    } catch (error) {
      failure = new Error(`${failure ? `${failure.message}；` : ''}DMG 卸載失敗，掛載點留在 runner 暫存目錄 ${mountPoint}：${error.message}`);
    }
  }
  if (attachAttempted && !device) {
    failure = new Error(`${failure ? `${failure.message}；` : ''}無法確認 DMG 掛載裝置，暫存目錄留在 runner：${scratch}`);
  }
  if (!attachAttempted || detached) {
    rmSync(scratch, { recursive: true, force: true });
  }
  if (failure) throw failure;
  return { mode, version: reference.version, channel, updater: Boolean(updater) };
}

export function formatVerificationResult(result) {
  return `macOS 產物驗證通過：channel=${result.channel} mode=${result.mode} ` +
    `signing=${result.mode === 'none' ? 'ad-hoc' : 'Developer ID'} version=${result.version} ` +
    `artifacts=${result.updater ? 'app,dmg,updater' : 'app,dmg'}`;
}

function main(argv) {
  const options = parseArgs(argv);
  const result = verifyArtifacts(options);
  console.log(formatVerificationResult(result));
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (error) {
    console.error(`macos-artifact-verify: ${error.message}`);
    process.exitCode = 1;
  }
}
