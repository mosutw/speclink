#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const RELEASE_CHANNEL = 'release';
const DESKTOP_BUILD_CHANNEL = 'desktop-build';

export function bundleConfigFor({ channel, platform, macOSSigning }) {
  if (![RELEASE_CHANNEL, DESKTOP_BUILD_CHANNEL].includes(channel)) {
    throw new Error(`未知的桌面打包 channel：${channel ?? ''}`);
  }

  if (channel === DESKTOP_BUILD_CHANNEL) {
    const bundle = { createUpdaterArtifacts: false };
    if (platform === 'darwin') bundle.macOS = { signingIdentity: '-' };
    return { bundle };
  }

  if (platform !== 'darwin') return {};
  if (macOSSigning !== 'none' && macOSSigning !== 'full') {
    throw new Error(`macOS Release 的 SPECLINK_MACOS_SIGNING 無效：${macOSSigning ?? ''}`);
  }
  return macOSSigning === 'none'
    ? { bundle: { macOS: { signingIdentity: '-' } } }
    : {};
}

function main(argv) {
  if (argv.length !== 1) {
    throw new Error('用法：node scripts/release/desktop-bundle-config.mjs <release|desktop-build>');
  }

  const config = bundleConfigFor({
    channel: argv[0],
    platform: process.platform,
    macOSSigning: process.env.SPECLINK_MACOS_SIGNING,
  });
  process.stdout.write(`${JSON.stringify(config)}\n`);
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    main(process.argv.slice(2));
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
