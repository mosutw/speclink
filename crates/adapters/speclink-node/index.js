'use strict'

// Public entry of @speclink/engine. The generated `binding.js` loads the
// platform-specific native addon; this wrapper shapes the public API:
// createEngine forms, Promise-based dispatch, and Error objects carrying a
// semantic `code` property.

const binding = require('./binding.js')

function unwrapResult(result) {
  if (result.ok) return result.value
  const err = new Error(result.message)
  err.code = result.code
  for (const key of ['reason', 'change', 'path']) {
    if (result[key] !== undefined) err[key] = result[key]
  }
  throw err
}

function whoAndWhere(call) {
  const text = (value) => (typeof value === 'string' ? value : '')
  return [text(call?.identity?.id), text(call?.identity?.displayName), text(call?.scope)]
}

class Engine {
  #native

  constructor(native) {
    this.#native = native
  }

  async exportScope() {
    return unwrapResult(await this.#native.exportScope())
  }

  /**
   * Dispatch one speclink verb. `argv` mirrors the CLI vocabulary
   * (e.g. ['list', '--json']); `options.stdin` carries content for verbs
   * that read stdin in the CLI (e.g. `new artifact --stdin`).
   */
  async dispatch(argv, options) {
    if (!Array.isArray(argv) || argv.some((a) => typeof a !== 'string')) {
      throw new TypeError('dispatch(argv): argv must be an array of strings')
    }
    const envelope = await this.#native.dispatch(argv, options?.stdin)
    if (envelope.ok) return envelope.value
    const err = new Error(envelope.message)
    err.code = envelope.code
    throw err
  }
}

/**
 * Build an engine over one of the two storage forms:
 * - `{ store: { type: 'fs', root, specDir? } }` — built-in filesystem store
 * - `{ store: <object implementing the Store interface> }` — host storage
 *
 * `options.actor` ("Name <email>") is the operator every stamp this engine
 * writes is attributed to. Both forms accept it; without it the fs form falls
 * back to the workspace's git identity and a host store stamps nothing.
 */
/**
 * The generic store invoker the native bridge calls through one
 * ThreadsafeFunction: executes `store[method](...args)` (plain values and
 * Promises both accepted, synchronous throws captured) and settles exactly
 * once with `(error, value)`.
 */
function makeInvoker(store) {
  return (method, args, settle) => {
    if (typeof store[method] !== 'function') {
      // Optional methods (e.g. claim) surface as a marker the native side
      // translates into a semantic "not supported by this store" error.
      settle({ message: `store does not implement ${method}`, code: '__missing__' }, null)
      return
    }
    Promise.resolve()
      .then(() => store[method](...args))
      .then(
        (value) => settle(null, value === undefined ? null : value),
        (e) =>
          settle(
            {
              message: e && e.message !== undefined ? String(e.message) : String(e),
              code: e && e.code != null ? String(e.code) : null,
            },
            null,
          ),
      )
  }
}

function createEngine(options) {
  const store = options?.store
  if (!store || typeof store !== 'object') {
    throw new TypeError("createEngine: options.store is required — pass { type: 'fs', root } or a Store object")
  }
  // The operator identity is bound here, once — dispatch never takes one.
  const actor = options?.actor
  if (actor !== undefined && typeof actor !== 'string') {
    throw new TypeError("createEngine: options.actor must be a string (\"Name <email>\")")
  }
  if (store.type === 'fs') {
    if (typeof store.root !== 'string') {
      throw new TypeError("createEngine: fs store requires a string 'root'")
    }
    return new Engine(binding.engineFromFs(store.root, store.specDir, actor))
  }
  return new Engine(binding.engineFromStore(store, makeInvoker(store), actor))
}

/**
 * The invoker the native port bridge calls for a host's store port: it runs
 * `target[method](...args)` (a value or a Promise) and always settles with a
 * value — `{ ok: true, value }`, or `{ ok: false, error }` with the thrown
 * object's `code`, `message`, `key` and `actual` — so the native side reads the
 * fields of any thrown object, an Error or a plain one.
 */
function makePortInvoker(target) {
  return (method, args, settle) => {
    Promise.resolve()
      .then(() => target[method](...args))
      .then(
        (value) => settle(null, { ok: true, value: value === undefined ? null : value }),
        (e) => settle(null, { ok: false, error: thrownFields(e) }),
      )
  }
}

function thrownFields(e) {
  const isObject = e !== null && typeof e === 'object'
  return {
    code: isObject && e.code != null ? String(e.code) : null,
    message: isObject && e.message !== undefined ? String(e.message) : String(e),
    key: isObject && typeof e.key === 'string' ? e.key : null,
    actual: isObject && typeof e.actual === 'number' ? e.actual : null,
  }
}

const PORT_FUNCTIONS = ['list', 'read', 'commit']

/** The store port's three functions, checked once; missing names are listed. */
function portFunctions(store, caller) {
  if (!store || typeof store !== 'object') {
    throw new TypeError(`${caller}: store must be an object with list, read and commit`)
  }
  const missing = PORT_FUNCTIONS.filter((name) => typeof store[name] !== 'function')
  if (missing.length > 0) {
    throw new TypeError(`${caller}: store is missing required functions: ${missing.join(', ')}`)
  }
  return {
    list: (...args) => store.list(...args),
    read: (...args) => store.read(...args),
    commit: (...args) => store.commit(...args),
  }
}

/** A store's version conflict: `actual` is the document's current version, or null when it does not exist. */
class StoreConflictError extends Error {
  constructor(key, actual) {
    super(`conflict on ${key}`)
    this.name = 'StoreConflictError'
    this.code = 'conflict'
    this.key = key
    this.actual = actual === undefined ? null : actual
  }
}

/** A temporary store failure; the caller can retry. */
class StoreUnavailableError extends Error {
  constructor(message) {
    super(message)
    this.name = 'StoreUnavailableError'
    this.code = 'unavailable'
  }
}

const HOOK_NAMES = ['beforeCommand', 'afterCommit']

const PROJECT_HTTP_ROUTES = [
  ['GET', /^\/binding$/, 'binding'],
  ['GET', /^\/sync-state$/, 'sync-state'],
  ['GET', /^\/whoami$/, 'project-whoami'],
  ['GET', /^\/changes$/, 'list-changes'],
  ['GET', /^\/specs$/, 'list-specs'],
  ['GET', /^\/board-order$/, 'board-order'],
  ['GET', /^\/specs\/([^/]+)\/document$/, 'spec'],
  ['GET', /^\/search$/, 'search'],
  ['POST', /^\/context$/, 'context'],
  ['GET', /^\/export$/, 'export', true],
  ['POST', /^\/import$/, 'import', true],
  ['POST', /^\/changes$/, 'create-change', true, true],
  ['POST', /^\/discussions$/, 'create-discussion', true, true],
  ['POST', /^\/discussions\/([^/]+)\/promote$/, 'promote-discussion', true, true],
  ['PUT', /^\/changes\/([^/]+)\/fields$/, 'change-fields', true],
  ['PUT', /^\/discussions\/([^/]+)\/fields$/, 'discussion-fields', true],
]

function projectHttpRoute(method, path) {
  for (const [verb, pattern, kind, write, creates] of PROJECT_HTTP_ROUTES) {
    if (method !== verb) continue
    const match = pattern.exec(path)
    if (match) return { kind, write, creates, subject: match[1] }
  }
  return null
}

class Host {
  #native
  #scopes
  #authMethods

  constructor(native, scopes, authMethods) {
    this.#native = native
    this.#scopes = scopes
    this.#authMethods = authMethods
  }

  async handleRequest(request, identity, extras) {
    const projectRoute = /^\/api\/speclink\/v1\/projects\/([^/]+)(\/.*)$/.exec(request?.path ?? '')
    const rootRoute = ['/api/speclink/v1/auth-methods', '/api/speclink/v1/whoami', '/api/speclink/v1/scopes'].includes(request?.path)
    const head = request?.method === 'HEAD'
    const method = head ? 'GET' : request?.method
    const route = projectRoute && projectHttpRoute(method, projectRoute[2])
    if (!route && !(method === 'GET' && rootRoute)) return null

    const reply = (status, body, etag) => ({
      status,
      headers: {
        ...(status === 304 ? {} : { 'content-type': 'application/json' }),
        ...(etag == null ? {} : { etag }),
      },
      body: head || status === 304 ? new Uint8Array() : new TextEncoder().encode(JSON.stringify(body)),
    })
    const fail = (status, reason, message) => reply(status, { status, reason, message })
    if (rootRoute && request.path.endsWith('/auth-methods')) {
      return this.#authMethods ? reply(200, this.#authMethods) : fail(404, 'not_found', 'This host declares no login methods')
    }
    if (!identity || typeof identity.id !== 'string' || !identity.id.trim() ||
        typeof identity.displayName !== 'string' || !identity.displayName.trim()) {
      return fail(401, 'permission_denied', 'An identity is required')
    }
    if (rootRoute && request.path.endsWith('/whoami')) {
      return reply(200, { user: { name: identity.displayName, handle: identity.id } })
    }
    const header = (name) => Object.entries(request.headers ?? {}).find(([key]) => key.toLowerCase() === name)?.[1]
    if (route && header('x-speclink-api-version') !== '1') {
      return fail(409, 'refused', "incompatible api version — this server speaks version '1'")
    }
    let projectKey
    let subject
    try {
      projectKey = projectRoute && decodeURIComponent(projectRoute[1])
      subject = route?.subject && decodeURIComponent(route.subject)
    } catch {
      return fail(400, 'invalid_argument', 'Invalid path encoding')
    }
    let projects
    try {
      projects = this.#scopes && await this.#scopes(identity)
      if (this.#scopes && (!Array.isArray(projects) || !projects.every((project) =>
        typeof project?.key === 'string' && typeof project.name === 'string' &&
        Array.isArray(project.repos) && project.repos.every((repo) =>
          typeof repo?.key === 'string' && typeof repo.name === 'string' && typeof repo.scope === 'string' && !!repo.scope.trim()),
      ))) throw new TypeError('Invalid scope list')
    } catch {
      return fail(503, 'unavailable', 'The host could not read project scopes')
    }
    if (rootRoute) {
      if (!projects) return fail(404, 'not_found', 'This host lists no scopes')
      return reply(200, { projects: projects.map(({ key, name, repos }) => ({
        id: `prj_${key}`, key, name, repos: repos.map(({ key, name }) => ({ id: `repo_${key}`, key, name })),
      })) })
    }
    const project = projects?.find((item) => item.key === projectKey)
    if (!project) return fail(404, 'not_found', 'Project not found')
    if (route.kind === 'project-whoami') {
      return reply(200, {
        user: { name: identity.displayName, handle: identity.id },
        repos: project.repos.map((repo) => ({ name: repo.key, gitUrl: '' })),
      })
    }
    const repoKey = header('x-speclink-repo')
    if (repoKey === undefined && project.repos.length > 1) {
      return fail(409, 'refused', 'Select a repository using X-Speclink-Repo')
    }
    const repo = repoKey === undefined ? project.repos[0] : project.repos.find((item) => item.key === repoKey)
    if (!repo) return fail(404, 'not_found', 'Repository not found')
    if (route.kind === 'binding') {
      return reply(200, {
        actor: { id: identity.id, name: identity.displayName },
        project: { id: `prj_${project.key}`, name: project.name, key: project.key },
        repo: { id: `repo_${repo.key}`, name: repo.name, key: repo.key },
        apiVersion: '1', engineVersion: binding.engineVersion(),
        capabilities: { fields: true, contextSnapshots: true, events: { transports: [], polling: { url: '/sync-state', etag: true } } },
      })
    }
    if (route.write && repo.canWrite !== true) return fail(403, 'permission_denied', 'The caller cannot write to this repository')
    if (request.body !== undefined && !(request.body instanceof Uint8Array)) {
      return fail(400, 'invalid_argument', 'Request body must be bytes')
    }

    const action = { kind: route.kind }
    switch (route.kind) {
      case 'spec': action.capability = subject; break
      case 'search': action.query = new URLSearchParams(request.query).get('q') ?? ''; break
      case 'sync-state':
      case 'context': action.if_none_match = header('if-none-match') ?? null; break
      case 'import': action.project = project.key; action.repo = repo.key; break
      case 'create-change': action.repo = repo.key; break
      case 'change-fields': action.name = subject; break
      case 'discussion-fields':
      case 'promote-discussion': action.slug = subject; break
    }
    const fields = route.creates ? extras?.fields : undefined
    const answer = await this.#native.handleProjectAction(
      ...whoAndWhere({ identity, scope: repo.scope }), action, Buffer.from(request.body ?? []), fields,
    )
    if (answer.ok) return reply(answer.value.status, answer.value.body, answer.value.etag)
    const reason = answer.reason ?? answer.code
    if (answer.code === 'invalid_argv') {
      return fail(reason === 'bundle_too_large' ? 413 : 400, answer.reason ?? 'invalid_argument', answer.message)
    }
    if (answer.code === 'refused' || answer.code === 'revision_conflict') return fail(409, reason, answer.message)
    if (answer.code === 'not_found') return fail(404, reason, answer.message)
    if (answer.code === 'unavailable') return fail(503, reason, 'The store is temporarily unavailable')
    return fail(500, 'internal', 'The host could not handle the request')
  }

  /**
   * Run one speclink verb for `identity` on `scope`. `argv`, `stdin`, the
   * result and the errors are those of `engine.dispatch`.
   */
  async dispatch(call) {
    const argv = call?.argv
    if (!Array.isArray(argv) || argv.some((a) => typeof a !== 'string')) {
      throw new TypeError('host.dispatch: argv must be an array of strings')
    }
    // A field that is not a string reads as empty, which the native side
    // refuses with invalid_argv before any store call.
    const text = (value) => (typeof value === 'string' ? value : '')
    const identity = call.identity
    const envelope = await this.#native.dispatch(
      text(identity?.id),
      text(identity?.displayName),
      text(call.scope),
      argv,
      call.stdin,
      call.fields,
    )
    if (envelope.ok) return envelope.value
    const err = new Error(envelope.message)
    err.code = envelope.code
    throw err
  }
  async exportScope(call) {
    return unwrapResult(await this.#native.exportScope(...whoAndWhere(call)))
  }

  async importScope(call) {
    const [mode, dryRun] = importOptions(call)
    return unwrapResult(await this.#native.importScope(
      ...whoAndWhere(call), JSON.stringify(call?.bundle) ?? 'null', mode, dryRun,
    ))
  }

}

/** Every key a call of `importScope` in the replace mode can have. */
const REPLACE_CALL_KEYS = ['identity', 'scope', 'bundle', 'mode', 'dryRun']

/**
 * `importScope`'s `mode` and `dryRun` as `[mode, dryRun]`, checked here,
 * before anything crosses to the native side: a replace deletes documents, so
 * a value of the wrong type or a misspelled key must not read as a default.
 * `dryRun: 'false'` is not false, and `dryrun: true` next to `mode:
 * 'replace'` would be a real replace. A `dryRun` key that is there must hold
 * a boolean: `dryRun: body.dryRun` with nothing in `body.dryRun` is refused,
 * not run for real. A `mode` that is `undefined` is the create mode, which
 * deletes nothing. The create mode keeps taking other keys, as it always
 * did. A call that can hold no key (not an object and not a function) goes
 * on to the native side, which refuses its identity.
 */
function importOptions(call) {
  const invalid = (message) => Object.assign(new Error(`host.importScope: ${message}`), { code: 'invalid_argv' })
  // A string is shown cut and quoted; any other value by its type only.
  const shown = (value) => (typeof value === 'string' ? JSON.stringify(value.slice(0, 40)) : value === null ? 'null' : typeof value)
  if (call === null || (typeof call !== 'object' && typeof call !== 'function')) return ['create', false]
  const { mode, dryRun } = call
  if (mode !== undefined && mode !== 'create' && mode !== 'replace') {
    throw invalid(`mode must be 'create' or 'replace', got ${shown(mode)}`)
  }
  if ('dryRun' in call && typeof dryRun !== 'boolean') {
    throw invalid(`dryRun must be true or false, got ${shown(dryRun)} (leave the key out for a real run)`)
  }
  if (mode === 'replace') {
    const unknown = Object.keys(call).filter((key) => !REPLACE_CALL_KEYS.includes(key))
    if (unknown.length > 0) {
      const names = unknown.map(shown).join(', ')
      throw invalid(`unknown key ${names} in a call with mode 'replace' (the keys are ${REPLACE_CALL_KEYS.join(', ')})`)
    }
  }
  return [mode ?? 'create', dryRun === true]
}

function authMethodsForWire(value) {
  if (!value || !Array.isArray(value.methods) || Array.from(value.methods).some((method) => typeof method !== 'string')) {
    throw new TypeError('createHost: authMethods.methods must be an array of strings')
  }
  const methods = [...value.methods]
  if (!methods.includes('browser') && value.browser === undefined) return { methods }
  const browser = value.browser
  const scopes = browser?.scopes === undefined ? [] : browser.scopes
  if (!browser || typeof browser.issuer !== 'string' || !browser.issuer.trim() ||
      typeof browser.clientId !== 'string' || !browser.clientId.trim() || !Array.isArray(scopes) ||
      Array.from(scopes).some((scope) => typeof scope !== 'string')) {
    throw new TypeError('createHost: authMethods.browser needs an issuer, clientId and optional string scopes')
  }
  return { methods, browser: { issuer: browser.issuer, clientId: browser.clientId, scopes: [...scopes] } }
}

/**
 * Build a host kit over a store port (`list`, `read`, `commit`) and optional
 * hooks (`beforeCommand`, `afterCommit`). One host serves every user: each
 * `dispatch` brings its own identity and scope.
 */
function createHost(options) {
  const target = portFunctions(options?.store, 'createHost')
  const prefixes = options.reservedFieldPrefixes === undefined ? [] : options.reservedFieldPrefixes
  if (!Array.isArray(prefixes) || Array.from(prefixes).some((prefix) => typeof prefix !== 'string')) {
    throw new TypeError('createHost: reservedFieldPrefixes must be an array of strings')
  }
  const authMethods = options.authMethods === undefined ? undefined : authMethodsForWire(options.authMethods)
  const limits = options.limits ?? {}
  if (typeof limits !== 'object' || Array.isArray(limits) || Object.keys(limits).some((key) => key !== 'importBytes')) {
    throw new TypeError('createHost: limits must be an object with importBytes')
  }
  const { importBytes } = limits
  if (importBytes !== undefined && !(Number.isSafeInteger(importBytes) && importBytes > 0)) {
    throw new TypeError('createHost: limits.importBytes must be a positive integer (bytes)')
  }
  const hooks = options?.hooks ?? {}
  if (typeof hooks !== 'object') {
    throw new TypeError('createHost: hooks must be an object')
  }
  // A misspelled hook would leave every call allowed without a word.
  const unknown = Object.keys(hooks).filter((name) => !HOOK_NAMES.includes(name))
  if (unknown.length > 0) {
    throw new TypeError(`createHost: unknown hooks: ${unknown.join(', ')} (the hooks are ${HOOK_NAMES.join(' and ')})`)
  }
  for (const name of HOOK_NAMES) {
    if (hooks[name] !== undefined && typeof hooks[name] !== 'function') {
      throw new TypeError(`createHost: hooks.${name} must be a function`)
    }
  }
  if (options.scopes !== undefined && typeof options.scopes !== 'function') {
    throw new TypeError('createHost: scopes must be a function')
  }
  let native
  const hookTarget = {
    beforeCommand: async (info) => {
      let active = true
      const who = whoAndWhere(info)
      try {
        return await hooks.beforeCommand(info, {
          read: async (argv) => {
            if (!active) throw new Error('The hook read function can only be used while beforeCommand runs')
            return unwrapResult(await native.readForHook(...who, argv))
          },
        })
      } finally {
        active = false
      }
    },
    afterCommit: (info) => hooks.afterCommit(info),
  }
  native = binding.hostFromPort(
      makePortInvoker(target),
      makePortInvoker(hookTarget),
      typeof hooks.beforeCommand === 'function',
      typeof hooks.afterCommit === 'function',
    importBytes,
    prefixes,
  )
  return new Host(native, options.scopes, authMethods)
}

/** Run the store contract's conformance checks on a host's JS store port. */
async function runStoreConformance(harness) {
  const target = portFunctions(harness?.store, 'runStoreConformance')
  if (typeof harness.newScope !== 'function') {
    throw new TypeError('runStoreConformance: newScope must be a function')
  }
  const keepsEvents = typeof harness.recordedEvents === 'function'
  target.newScope = () => harness.newScope()
  if (keepsEvents) target.recordedEvents = (scope) => harness.recordedEvents(scope)
  return binding.runStoreConformance(makePortInvoker(target), keepsEvents)
}

/** Skill knowledge: list the registry, render one SKILL.md for a matrix point. */
const skills = {
  list: () => binding.skillsList(),
  render: (name, options) => {
    if (options && typeof options === 'object') {
      const supported = ['target', 'invocation', 'specDir', 'toolName', 'skills', 'checkout', 'gitHistory']
      const unknown = Object.keys(options).filter((key) => !supported.includes(key))
      if (unknown.length) throw new TypeError(`skills.render: unknown options: ${unknown.join(', ')}`)
    }
    return binding.skillsRender(name, options)
  },
}

const commandNames = Object.freeze(binding.commandNames())
function agentToolDescription(options = {}) {
  if (!options || typeof options !== 'object' || Array.isArray(options) ||
      Object.keys(options).some((key) => key !== 'exclude')) {
    throw new TypeError('agentToolDescription: options must be an object with exclude')
  }
  const { exclude = [] } = options
  if (!Array.isArray(exclude) || exclude.some((name) => typeof name !== 'string')) {
    throw new TypeError('agentToolDescription: exclude must be an array of verbs')
  }
  return binding.agentToolDescription(exclude.map((verb) => verb.trim()))
}

const eventNames = Object.freeze(binding.eventNames())

module.exports = { commandNames, agentToolDescription, eventNames, eventResource: binding.eventResource, createEngine, createHost, skills, runStoreConformance, StoreConflictError, StoreUnavailableError }
