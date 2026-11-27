import { createHash } from 'node:crypto'
import { rmSync } from 'node:fs'
import { expect, it, vi } from 'vitest'

import {
  agentToolDescription, commandNames, createEngine, createHost, eventNames, eventResource, skills,
} from '../index.js'
import { makeFsFixture } from './helpers'
import { memoryPort } from './memory-port'

const identity = { id: 'u_1', displayName: 'Alice' }
const context = { identity, scope: 's1' }
const discussion =
  '---\ntopic: Two\nslug: two\nstatus: concluded\ncreated: 2026-11-01\n---\n\n' +
  '## Context\n\nContext.\n\n## Rounds\n\n## Conclusion\n\n**Decision**: Add it.\n'

it('passes the host identity and scope to the commit, and denies a command before touching storage', async () => {
  const port = memoryPort()
  const beforeCommand = vi.fn(({ command }: { command: string }) =>
    command === 'discard' ? { allow: false as const, reason: 'Kept by policy' } : undefined,
  )
  const host = createHost({ store: port.store, hooks: { beforeCommand } })
  await host.dispatch({ ...context, argv: ['new', 'change', 'demo'] })
  expect(port.commits[0]).toMatchObject({ scope: 's1', commit: { actor: identity, command: 'new-change' } })

  const calls = { ...port.calls }
  await expect(host.dispatch({ ...context, argv: ['discard', 'demo'] }))
    .rejects.toMatchObject({ code: 'refused', message: 'Kept by policy' })
  expect(port.calls).toEqual(calls)
  expect(beforeCommand).toHaveBeenLastCalledWith(expect.objectContaining({
    identity, scope: 's1', command: 'discard', params: { change: 'demo', force: false },
  }), expect.objectContaining({ read: expect.any(Function) }))
})

it('commits the change and source discussion as one operation and reports both events', async () => {
  const port = memoryPort()
  port.seed('s1', { 'config.yaml': 'schema: spec-driven\n', 'discussions/two.md': discussion })
  const afterCommit = vi.fn()
  const host = createHost({ store: port.store, hooks: { afterCommit } })

  await host.dispatch({ ...context, argv: ['new', 'change', 'demo', '--from-discussion', 'two'] })

  expect(port.calls.commit).toBe(1)
  expect(port.commits[0].commit.ops).toEqual(expect.arrayContaining([
    expect.objectContaining({ key: 'changes/demo/.meta.yaml' }),
    expect.objectContaining({ key: 'discussions/two.md' }),
  ]))
  const events = port.commits[0].commit.events
  for (const event of events) expect(eventNames).toContain(event.name)
  expect(events.map(({ name, payload }) => eventResource(name, payload))).toEqual([
    { scope: 'change', resourceId: 'demo' },
    { scope: 'discussion', resourceId: 'two' },
  ])
  await vi.waitFor(() => expect(afterCommit).toHaveBeenCalledWith(expect.objectContaining({ events })))
})

it('checks authorization on direct HTTP reads before disclosing data', async () => {
  const port = memoryPort()
  const beforeCommand = vi.fn(() => ({ allow: false, reason: 'Access denied by host policy' }))
  const host = createHost({
    store: port.store,
    scopes: () => [
      { key: 'alpha', name: 'Alpha', repos: [{ key: 'web', name: 'Web', scope: 's1', canWrite: true, canWritePolicy: true }] },
    ],
    hooks: { beforeCommand },
  })
  const response = await host.handleRequest({
    method: 'GET',
    path: '/api/speclink/v1/projects/alpha/board-order',
    headers: { 'x-speclink-api-version': '1' },
  }, identity)

  expect(response?.status).toBe(409)
  expect(JSON.parse(new TextDecoder().decode(response?.body))).toMatchObject({
    reason: 'refused', message: 'Access denied by host policy',
  })
  expect(beforeCommand).toHaveBeenCalledWith(expect.objectContaining({ command: 'get-board-order' }), expect.anything())
  expect(port.calls.commit).toBe(0)
})

it('rejects reserved fields from argv before running hooks or touching storage', async () => {
  const port = memoryPort()
  const beforeCommand = vi.fn()
  const host = createHost({ store: port.store, reservedFieldPrefixes: ['host.'], hooks: { beforeCommand } })
  await expect(
    host.dispatch({ ...context, argv: ['new', 'change', 'demo', '--field', 'host.project=p2'] }),
  ).rejects.toMatchObject({ code: 'refused', message: "field 'host.project' is reserved by the host" })
  await expect(
    host.dispatch({ ...context, argv: ['discuss', 'field', 'unset', 'topic', 'host.project'] }),
  ).rejects.toMatchObject({ code: 'refused' })
  expect(beforeCommand).not.toHaveBeenCalled()
  expect(port.calls).toEqual({ list: 0, read: 0, commit: 0 })
})

it('writes trusted creation fields and exposes only the changed keys in events', async () => {
  const port = memoryPort()
  const host = createHost({ store: port.store, reservedFieldPrefixes: ['host.'] })
  await host.dispatch({
    ...context,
    argv: ['new', 'change', 'demo', '--field', 'owner=alice'],
    fields: { 'host.project': 'p1' },
  })
  await expect(host.dispatch({ ...context, argv: ['show', 'demo', '--json'] })).resolves.toMatchObject({
    fields: { owner: 'alice', 'host.project': 'p1' },
  })
  await host.dispatch({ ...context, argv: ['change', 'field', 'set', 'demo', 'owner=bob'] })
  expect(port.recordedEvents('s1').at(-1)).toMatchObject({
    name: 'change-fields-changed', payload: { change: 'demo', keys: ['owner'] },
  })
  expect(port.recordedEvents('s1').at(-1)?.payload).not.toHaveProperty('fields')
})

it('previews the differences and then replaces the contents of one scope', async () => {
  const port = memoryPort()
  port.seed('s1', { 'specs/old/spec.md': 'Old spec' })
  const host = createHost({ store: port.store })
  const key = 'specs/new/spec.md'
  const content = 'New spec'
  const digest = 'sha256:' + createHash('sha256').update(content).digest('hex')
  const change = {
    ...context, mode: 'replace' as const, bundle: { formatVersion: 2, documents: [{ key, content, digest }] },
  }
  const expected = { added: [key], changed: [], removed: ['specs/old/spec.md'] }

  await expect(host.importScope({ ...change, dryRun: true })).resolves.toMatchObject(expected)
  expect(port.calls.commit).toBe(0)
  await expect(host.importScope(change)).resolves.toMatchObject(expected)
  expect(port.calls.commit).toBe(1)
  await expect(host.exportScope(context)).resolves.toMatchObject({ documents: [{ key, content, digest }] })
})

it('exports the local workspace and keeps existing filesystem dispatch usable', async () => {
  const root = makeFsFixture()
  try {
    const engine = createEngine({ store: { type: 'fs', root } })
    await expect(engine.dispatch(['list', '--json'])).resolves.toMatchObject({ changes: expect.any(Array) })
    const bundle = await engine.exportScope()
    expect(bundle.formatVersion).toBe(2)
    expect(bundle.documents).toEqual(expect.arrayContaining([
      expect.objectContaining({ key: 'changes/alpha/tasks.md', digest: expect.stringMatching(/^sha256:[a-f0-9]{64}$/) }),
    ]))
  } finally {
    rmSync(root, { recursive: true, force: true })
  }
})

it('exposes the host command registry and rejects unknown skill render options', () => {
  expect(commandNames).toContain('get-board-order')
  expect(agentToolDescription({ exclude: ['new-change'] })).not.toContain('new change')
  expect(() => skills.render('review', { unknownOption: true } as never)).toThrow(/unknownOption/)
})

it('asks the before hook for each direct HTTP read and for an unchanged context', async () => {
  const port = memoryPort()
  port.seed('s1', { 'specs/auth/spec.md': 'Auth spec' })
  let allowed = false
  const beforeCommand = vi.fn(() => allowed ? undefined : { allow: false as const, reason: 'Private scope' })
  const host = createHost({ store: port.store, scopes: () => httpScopes(), hooks: { beforeCommand } })
  const calls = [
    { method: 'GET', path: `${httpProject}/specs/auth/document`, headers: httpHeaders },
    { method: 'GET', path: `${httpProject}/search`, query: 'q=auth', headers: httpHeaders },
    { method: 'POST', path: `${httpProject}/context`, body: httpBody({}), headers: httpHeaders },
  ]
  for (const request of calls) {
    const response = await host.handleRequest(request, identity)
    expect(response?.status).toBe(409)
    expect(parseHttp(response)).toEqual({ status: 409, reason: 'refused', message: 'Private scope' })
  }
  expect(beforeCommand.mock.calls).toHaveLength(3)
  expect(port.calls).toEqual({ list: 0, read: 0, commit: 0 })

  allowed = true
  const spec = await host.handleRequest(calls[0], identity)
  expect(parseHttp(spec)).toEqual({ content: 'Auth spec' })
  const snapshot = await host.handleRequest(calls[2], identity)
  expect(snapshot?.status).toBe(200)
  expect(parseHttp(snapshot).documents).toEqual([
    expect.objectContaining({ path: 'speclink/specs/auth/spec.md', content: 'Auth spec' }),
  ])
  const conditional = { ...calls[2], headers: { ...httpHeaders, 'if-none-match': snapshot!.headers.etag } }
  expect((await host.handleRequest(conditional, identity))?.status).toBe(304)
  allowed = false
  expect((await host.handleRequest(conditional, identity))?.status).toBe(409)
  expect(port.calls.commit).toBe(0)
})

it('rejects HTTP reserved keys before the hook or store and accepts host-supplied fields', async () => {
  const port = memoryPort()
  const beforeCommand = vi.fn()
  const host = createHost({ store: port.store, scopes: () => httpScopes(), reservedFieldPrefixes: ['host.'], hooks: { beforeCommand } })
  const request = { method: 'POST', path: `${httpProject}/changes`, headers: httpHeaders }
  const response = await host.handleRequest({ ...request, body: httpBody({ name: 'demo', fields: { 'host.project': 'wrong' } }) }, identity)
  expect(response?.status).toBe(409)
  expect(parseHttp(response)).toMatchObject({ reason: 'refused' })
  const edit = await host.handleRequest({ method: 'PUT', path: `${httpProject}/changes/demo/fields`, headers: httpHeaders, body: httpBody({ unset: ['host.project'] }) }, identity)
  expect(edit?.status).toBe(409)
  expect(beforeCommand).not.toHaveBeenCalled()
  expect(port.calls).toEqual({ list: 0, read: 0, commit: 0 })

  expect((await host.handleRequest({ ...request, body: httpBody({ name: 'demo' }) }, identity, { fields: { 'host.project': 'p1' } }))?.status).toBe(200)
  const change = await host.dispatch({ ...context, argv: ['show', 'demo', '--json'] })
  expect(change).toMatchObject({ fields: { 'host.project': 'p1' } })
})

it('checks repository permissions and import limits before touching storage', async () => {
  const port = memoryPort()
  const host = createHost({ store: port.store, scopes: () => httpScopes(), limits: { importBytes: 32 } })
  const reader = createHost({ store: port.store, scopes: () => httpScopes(false) })
  const exportRequest = { method: 'GET', path: `${httpProject}/export`, headers: httpHeaders }
  expect((await reader.handleRequest(exportRequest, identity))?.status).toBe(403)
  expect((await host.handleRequest({ method: 'POST', path: `${httpProject}/import`, body: new Uint8Array(33), headers: httpHeaders }, identity))?.status).toBe(413)
  expect(port.calls).toEqual({ list: 0, read: 0, commit: 0 })

  const empty = await host.handleRequest(exportRequest, identity)
  expect(parseHttp(empty)).toMatchObject({ formatVersion: 2, documents: [] })

  const writer = createHost({ store: port.store, scopes: () => httpScopes() })
  const key = 'specs/auth/spec.md'
  const content = 'Auth spec'
  const digest = 'sha256:' + createHash('sha256').update(content).digest('hex')
  const payload = { formatVersion: 2, documents: [{ key, content, digest }] }
  const request = { method: 'POST', path: `${httpProject}/import`, body: httpBody(payload), headers: httpHeaders }
  expect((await writer.handleRequest(request, identity))?.status).toBe(200)
  const repeat = await writer.handleRequest(request, identity)
  expect(repeat?.status).toBe(409)
  expect(parseHttp(repeat)).toMatchObject({ reason: 'scope_not_empty' })
  expect(parseHttp(await writer.handleRequest(exportRequest, identity))).toMatchObject(payload)
})

const httpProject = '/api/speclink/v1/projects/alpha'
const httpHeaders = { 'x-speclink-api-version': '1' }
const httpScopes = (canWrite = true) => [{ key: 'alpha', name: 'Alpha', repos: [{ key: 'web', name: 'Web', scope: 's1', canWrite }] }]
const httpBody = (value: unknown) => new TextEncoder().encode(JSON.stringify(value))
const parseHttp = (response: { body: Uint8Array } | null) => JSON.parse(new TextDecoder().decode(response?.body))

it('renders skills for hosts with source files but no Git history', () => {
  const options = { target: 'neutral' as const, invocation: 'tool-call' as const }
  const rendered = skills.render('review', { ...options, skills: ['review'], checkout: true, gitHistory: false })
  expect(rendered).toContain('## Harness limits')
  expect(rendered).toContain('Skip every step of this document that runs a git command')
  expect(rendered).not.toContain('Skip every step of this document that reads code files')
  expect(skills.render('review', options)).not.toContain('## Harness limits')
  expect(() => skills.render('review', { ...options, checkout: false, gitHistory: true })).toThrow(/contradicts/)
  expect(() => skills.render('review', { ...options, skills: ['verify'] })).toThrow(/must include 'review'/)
})

it('advertises the selected SDK repository in the binding and omits private store scopes', async () => {
  const port = memoryPort()
  const host = createHost({ store: port.store, scopes: () => [{
    ...httpScopes()[0],
    repos: [...httpScopes()[0].repos, { key: 'api', name: 'API', scope: 'private-internal-scope', canWrite: false }],
  }] })
  const request = { method: 'GET', path: `${httpProject}/binding`, headers: httpHeaders }
  expect((await host.handleRequest(request, identity))?.status).toBe(409)
  const binding = await host.handleRequest({ ...request, headers: { ...httpHeaders, 'X-Speclink-Repo': 'api' } }, identity)
  expect(binding?.status).toBe(200)
  expect(parseHttp(binding)).toMatchObject({ apiVersion: '1', actor: { id: identity.id }, repo: { key: 'api' } })
  expect(JSON.stringify(parseHttp(binding))).not.toContain('private-internal-scope')
  expect(port.calls).toEqual({ list: 0, read: 0, commit: 0 })
})

it('declares public login methods without a credential or a store lookup', async () => {
  const port = memoryPort()
  const host = createHost({ store: port.store, authMethods: {
    methods: ['browser', 'device'], browser: { issuer: 'https://login.example.test', clientId: 'speclink' },
  } })
  const response = await host.handleRequest({ method: 'GET', path: '/api/speclink/v1/auth-methods' }, null)
  expect(response?.status).toBe(200)
  expect(parseHttp(response)).toEqual({
    methods: ['browser', 'device'], browser: { issuer: 'https://login.example.test', clientId: 'speclink', scopes: [] },
  })
  expect(port.calls).toEqual({ list: 0, read: 0, commit: 0 })
})
