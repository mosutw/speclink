import { spawnSync } from 'node:child_process'
import { createRequire } from 'node:module'
import { join } from 'node:path'
import { expect, it } from 'vitest'

const require = createRequire(import.meta.url)

it('compiles a consumer of the public package types, including Antigravity', () => {
  // Allow slow CI runners to compile, but stop a stuck compiler before the test times out.
  const result = spawnSync(process.execPath, [
    require.resolve('typescript/bin/tsc'),
    '--project', join(__dirname, 'types', 'tsconfig.json'),
  ], { encoding: 'utf8', timeout: 25_000 })
  expect(result.error).toBeUndefined()
  expect(result.status, result.stdout + result.stderr).toBe(0)
}, 30_000)
