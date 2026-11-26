/**
 * @speclink/engine — Node.js bindings for the Speclink spec-driven
 * development engine.
 *
 * The engine core is synchronous Rust; every `dispatch` runs on a background
 * worker thread and returns a Promise — the JS event loop is never blocked.
 * WARNING: never call back into the engine synchronously from inside a Store
 * method (a dispatch issued there and awaited synchronously would deadlock
 * against the store bridge).
 */

/** A Store method may return its value directly or as a Promise. */
export type MaybePromise<T> = T | Promise<T>

/** Per-change metadata (the parsed `.meta.yaml` fields). */
export interface ChangeMeta {
  schema?: string
  created?: string
  createdBy?: string
  createdWith?: string
  /** Slug of the discussion this change was promoted from. */
  fromDiscussion?: string
}

/** A change as the store reports it. */
export interface ChangeInfo {
  name: string
  /**
   * Display location of the change's documents (store-logical, e.g.
   * `changes/<name>`). Optional — defaults to `changes/<name>`.
   */
  dir?: string
  meta?: ChangeMeta
}

/** A discussion document as stored. */
export interface DiscussionDoc {
  slug: string
  text: string
  /** Display location of the document (store-logical). */
  path: string
  archived: boolean
}

/**
 * The storage interface the engine reads and writes spec documents through —
 * one-to-one with the Rust core's `Store` trait (camelCase). Artifact
 * identifiers are schema-defined output paths relative to a change (e.g.
 * `proposal.md`, `specs/<capability>/spec.md`).
 *
 * Every method is required (createEngine fails fast listing missing ones);
 * `claim` is the one optional extension — ownership is a team-system concept
 * the host adjudicates.
 */
export interface ChecksCommit {
  writes: Record<string, string>
  deletes: string[]
  /** SHA-256 of current content, or null if the document must be absent. */
  expected: Record<string, string | null>
}

export interface Store {
  // --- changes ---
  /** Active changes with metadata, sorted by name. Missing storage → empty list. */
  listChanges(): MaybePromise<ChangeInfo[]>
  findChange(name: string): MaybePromise<ChangeInfo | null>
  changeExists(name: string): MaybePromise<boolean>
  /** Create a change with the given raw metadata document; returns its display location. */
  createChange(name: string, metaText: string): MaybePromise<string>
  /** Last-modified time in whole seconds since the Unix epoch (sort key). Missing change → 0. */
  updatedAtSecs(name: string): MaybePromise<number>
  /**
   * OPTIONAL: the raw metadata document of an active change (the
   * `.meta.yaml` text). Stamping is a read-modify-write of this document,
   * so the review/verify stamp verbs require BOTH methods together with
   * `deleteArtifact` — a store missing any of the three is refused before the
   * stamp touches anything. All other verbs never call them.
   */
  readChangeMeta?(name: string): MaybePromise<string | null>
  /** OPTIONAL: overwrite the raw metadata document — see `readChangeMeta`. */
  writeChangeMeta?(name: string, content: string): MaybePromise<void>
  /**
   * OPTIONAL: delete an active change outright (the storage side of discard).
   * A store without it fails loudly only when a discard actually reaches it.
   */
  deleteChange?(name: string): MaybePromise<void>

  // --- artifacts ---
  readArtifact(change: string, artifact: string): MaybePromise<string | null>
  /** Create or overwrite an artifact; returns its display location. */
  writeArtifact(change: string, artifact: string, content: string): MaybePromise<string>
  /** Whether an artifact exists (an empty document counts). */
  artifactExists(change: string, artifact: string): MaybePromise<boolean>
  /**
   * OPTIONAL: delete a document inside an active change. Only the review and
   * verify stations delete anything (stamping removes the ticket), so a store
   * without it works everywhere else and fails loud there.
   */
  deleteArtifact?(change: string, artifact: string): MaybePromise<void>
  /** Atomically check every expected digest, then apply ALL writes/deletes, or none.
   * Required for review/verify stamping and security publication. Never emulate
   * this by separate calls to writeArtifact and deleteArtifact. */
  commitChecks?(change: string, commit: ChecksCommit): MaybePromise<void>

  // --- completion evidence ---
  /**
   * The change's completion-evidence record text, or null when it has none —
   * absence is a normal state. Optional: a store without it reads as "no
   * record".
   */
  readEvidence?(change: string): MaybePromise<string | null>
  /**
   * Write the change's evidence record text. Optional, but a store that omits
   * it fails the call the moment a completion has files to record, rather than
   * dropping the evidence silently.
   */
  writeEvidence?(change: string, content: string): MaybePromise<void>

  // --- delta specs ---
  /** Capability names that have a delta spec document in the change, sorted. */
  deltaCapabilities(change: string): MaybePromise<string[]>
  /** Whether the change has any capability container at all. */
  hasCapabilityDirs(change: string): MaybePromise<boolean>

  // --- canonical specs ---
  listCanonicalCapabilities(): MaybePromise<string[]>
  canonicalSpecExists(capability: string): MaybePromise<boolean>
  readCanonicalSpec(capability: string): MaybePromise<string | null>
  writeCanonicalSpec(capability: string, content: string): MaybePromise<void>
  /** Display location of a capability's canonical spec. */
  canonicalSpecPath(capability: string): MaybePromise<string>

  // --- archive ---
  archivedChangeExists(datedName: string): MaybePromise<boolean>
  /** Move an active change into the archive under its dated name. */
  archiveChange(name: string, datedName: string): MaybePromise<void>
  readArchivedArtifact?(datedName: string, artifact: string): MaybePromise<string | null>
  readArchivedMeta(datedName: string): MaybePromise<string | null>
  writeArchivedMeta(datedName: string, content: string): MaybePromise<void>

  // --- discussions ---
  liveDiscussionExists(slug: string): MaybePromise<boolean>
  archivedDiscussionExists(slug: string): MaybePromise<boolean>
  liveDiscussionPath(slug: string): MaybePromise<string>
  readLiveDiscussion(slug: string): MaybePromise<string | null>
  writeLiveDiscussion(slug: string, content: string): MaybePromise<string>
  deleteLiveDiscussion(slug: string): MaybePromise<void>
  /** Resolve a slug to its document: live first, then the newest archived candidate. */
  readDiscussion(slug: string): MaybePromise<DiscussionDoc | null>
  listLiveDiscussions(): MaybePromise<DiscussionDoc[]>
  /** Archived discussions, ordered by stored name (archive date order). */
  listArchivedDiscussions(): MaybePromise<DiscussionDoc[]>
  /**
   * Move a live discussion into the archive, named by its creation date;
   * returns the stored archive name, or null when no live document exists.
   */
  archiveDiscussion(slug: string, created: string): MaybePromise<string | null>

  // --- workflow config / shared vocabulary ---
  /** Raw workflow configuration document (config.yaml content), or null. */
  readWorkflowConfig(): MaybePromise<string | null>
  /** The project's LANGUAGE document (shared vocabulary), or null. */
  readLanguage(): MaybePromise<string | null>

  /**
   * OPTIONAL: claim a change for implementation. The host adjudicates
   * ownership (see the verb contract); a conflict should reject with an
   * Error whose `code` is the 409 reason (e.g. `ownership_lost`) and whose
   * message states who holds the change and what to do. Without this method,
   * `dispatch(['claim', …])` fails loud.
   */
  claim?(name: string): MaybePromise<unknown>
}

/** The built-in filesystem store: a local project root with a spec directory. */
export interface FsStoreOptions {
  type: 'fs'
  /** Project root (the directory containing the spec directory). */
  root: string
  /**
   * Spec directory name relative to root. Default: the CLI's resolution —
   * `.speclink.yaml`'s `spec_dir`, else `speclink/`, else a legacy `openspec/`,
   * else `"speclink"`.
   */
  specDir?: string
}

export interface CreateEngineOptions {
  store: FsStoreOptions | Store
  /**
   * The operator identity every stamp this engine writes is attributed to
   * (`created_by`, `reviewed_by`, `verified_by`), in `"Name <email>"` form.
   *
   * Bound once at construction — one engine instance, one identity. A
   * multi-tenant host builds one engine per request (or per identity); there
   * is deliberately no way to pass an identity at dispatch time, so a caller
   * cannot claim someone else's. Who may claim which identity is the host's
   * call: the SDK only takes the result.
   *
   * Omitted (or blank after trimming): the fs form falls back to the
   * workspace's git identity, a host store stamps no identity at all.
   */
  actor?: string
}

export interface DispatchOptions {
  /**
   * Content for verbs that read stdin in the CLI (the `--stdin` flag), e.g.
   * `dispatch(['new', 'artifact', 'proposal', '--change', 'x', '--stdin'], { stdin })`.
   */
  stdin?: string
}

export interface Engine {
  exportScope(): Promise<ScopeBundle>
  /**
   * Dispatch one speclink verb. `argv` mirrors the CLI vocabulary one-to-one
   * (shell argv without the program name, e.g. `['list', '--json']`).
   *
   * Resolves to the same structured object the CLI prints with `--json`
   * (camelCase); verbs without a `--json` form resolve to `{ output: string }`.
   * The verb surface and payload contract are described in
   * docs/verb-contract.md.
   *
   * Rejects with an `Error` whose `message` is the CLI's semantic message and
   * whose `code` property classifies the failure (`error`, `not_found`,
   * `invalid_argv`, a host store's 409 reason such as `ownership_lost`, or
   * `store_error`/`panic`).
   */
  dispatch(argv: string[], options?: DispatchOptions): Promise<unknown>
}

/**
 * Build an engine over the built-in fs store (`{ type: 'fs', root }`) or a
 * host-implemented Store object. A Store object missing required methods
 * throws synchronously, listing the missing method names.
 */
export function createEngine(options: CreateEngineOptions): Engine

/** The render matrix: target × invocation. */
export interface RenderOptions {
  target: 'claude' | 'codex' | 'copilot' | 'antigravity' | 'neutral'
  /** How the harness executes speclink verbs. Default: `'cli'`. */
  invocation?: 'cli' | 'tool-call'
  /** Spec directory name substituted into rendered content. Default: `"speclink"`. */
  specDir?: string
  /** Neutral target only: the identity in the skill body (`{{TOOL}}`, e.g. `--agent`). Default: `"speclink"`. */
  toolName?: string
  /** The available skill names, including the skill being rendered. */
  skills?: string[]
  checkout?: boolean
  /** Whether the host can run Git commands. Defaults to the value of checkout. */
  gitHistory?: boolean
}

export interface SkillInfo {
  name: string
  description: string
}

export const skills: {
  /** The skill registry: names and descriptions of every generated skill. */
  list(): SkillInfo[]
  /**
   * Render one skill's SKILL.md content for a matrix point — identical to
   * what `speclink init` generates for equivalent parameters.
   */
  render(name: string, options: RenderOptions): string
}

/** A document of the store port. Versions and times are integers from 0 to 2^53 − 1 (`Number.MAX_SAFE_INTEGER`); times are milliseconds since the Unix epoch. */
export interface PortDocument {
  content: string
  /** The scope version of the commit that last wrote this document. */
  version: number
  modifiedAt: number
}

export interface PortListEntry {
  key: string
  version: number
  modifiedAt: number
}

/** The scope's current version and the matching documents, in byte order of their keys. */
export interface PortListing {
  version: number
  entries: PortListEntry[]
}

/** One change in a commit. `expected: null` means the document must not exist; a number is the version it must have. */
export type PortOp =
  | { op: 'put'; key: string; content: string; expected: number | null }
  | { op: 'delete'; key: string; expected: number }

export interface PortEvent {
  name: string
  payload: unknown
}

/** Everything one command writes, in one commit. */
export interface PortCommit {
  actor: { id: string; displayName: string }
  /** The command's stable name, e.g. `new-change`. */
  command: string
  at: number
  ops: PortOp[]
  /** The domain events of the command: the only way events reach the host. */
  events: PortEvent[]
}

/**
 * The storage contract a host implements: three functions, each on one scope
 * (an opaque string). Each function may return its value or a Promise.
 *
 * - `commit` applies all of its operations or none: run it as one database
 *   transaction, and check every `expected` before the first write. One
 *   expectation that does not hold rejects the whole commit. Give the written
 *   documents the new scope version as their version and `at` as their time,
 *   and keep the events in the same transaction when you need them delivered.
 * - `read` returns one result per key, in the same order (`null` for a
 *   missing document), all from one completed commit. `list` returns the
 *   documents whose key starts with `prefix`.
 *
 * Errors: throw an object whose `code` is `'conflict'` (with the `key` that
 * does not match and its `actual` version, or null when it does not exist) or
 * `'unavailable'` (a temporary failure). Anything else is another failure.
 * `StoreConflictError` and `StoreUnavailableError` set these fields for you;
 * the SDK reads only the fields, never `instanceof`.
 *
 * WARNING: do not wait for a `dispatch` of the same host inside these
 * functions — the dispatch waits for them, so it would never finish.
 */
export interface StorePort {
  list(scope: string, prefix: string): MaybePromise<PortListing>
  read(scope: string, keys: string[]): MaybePromise<(PortDocument | null)[]>
  commit(scope: string, commit: PortCommit): MaybePromise<number>
}

/** Who acts: an id that stays when the user is renamed, and the name document stamps carry. Neither may be empty or only white space. */
export interface HostIdentity {
  id: string
  displayName: string
}

export interface BeforeCommandInfo {
  identity: HostIdentity
  scope: string
  /** The engine command's stable name, e.g. `archive` or `workflow-config-edit`. */
  command: string
  /** The command's params, camelCase (e.g. `{ change: 'demo', carryReview: true, … }`). Read them; changing them changes nothing. */
  params: Record<string, unknown>
  /** Only for the workflow-config writes: the keys the write changes, e.g. `['tdd']`. */
  changedKeys?: string[]
}

/** `undefined` or `{ allow: true }` allows; `{ allow: false, reason }` refuses with that reason. */
export type BeforeCommandAnswer = void | { allow: true } | { allow: false; reason: string }

export interface AfterCommitInfo {
  identity: HostIdentity
  scope: string
  command: string
  /** The scope version the commit returned. */
  scopeVersion: number
  /** The events of the commit, the same the store's `commit` received. */
  events: PortEvent[]
}

/**
 * The host's two points to step in; both are optional. An unknown name is
 * refused at `createHost`.
 *
 * WARNING: do not wait for a `dispatch` of the same host inside a hook —
 * the dispatch waits for `beforeCommand`, so it would never finish.
 */
export interface HostHooks {
  /**
   * Asked exactly once per call, before anything is written. A refusal ends
   * the call with code `refused` and your reason; for any command but the
   * workflow-config writes it comes before the store sees a call. A throw, a
   * rejected Promise or an answer of another shape does not allow: the call
   * ends with code `internal`.
   */
  beforeCommand?(info: BeforeCommandInfo, tools: { read(argv: string[]): Promise<unknown> }): MaybePromise<BeforeCommandAnswer>
  /**
   * Called after a successful commit; it only watches. The dispatch does not
   * wait for it, and its throw or rejected Promise is swallowed. Delivery is
   * not guaranteed: a host that needs every event handles them in the store's
   * `commit`, in the same transaction as the writes.
   */
  afterCommit?(info: AfterCommitInfo): MaybePromise<void>
}

export interface HostAuthMethods {
  methods: string[]
  browser?: { issuer: string; clientId: string; scopes?: string[] }
}

export interface CreateHostOptions {
  authMethods?: HostAuthMethods
  /** Keys with one of these prefixes can only be written by the host. */
  reservedFieldPrefixes?: string[]
  limits?: { importBytes?: number }
  scopes?(identity: HostIdentity): MaybePromise<{ key: string; name: string; repos: { key: string; name: string; scope: string; canWrite?: boolean; canWritePolicy?: boolean }[] }[]>
  store: StorePort
  hooks?: HostHooks
}

export interface HostDispatchCall {
  /** Trusted fields attached by the host when the command creates a record. */
  fields?: Record<string, string>
  /** From your own code (your session), never from the model: argv and stdin cannot change it. */
  identity: HostIdentity
  scope: string
  /** The verb text without the program name, as for `engine.dispatch`. */
  argv: string[]
  stdin?: string
}

export interface ScopeBundle {
  formatVersion: 2
  scopeVersion?: number
  documents: { key: string; content: string; digest: string }[]
}

export interface ScopeCall {
  identity: HostIdentity
  scope: string
}

export interface ScopeImportOptions extends ScopeCall {
  bundle: ScopeBundle | { formatVersion: 1; scope: { project: string; repo: string }; projectRevision: number; documents: unknown[] }
  mode?: 'create' | 'replace'
  dryRun?: boolean
}

export interface ScopeImportReport {
  scopeVersion: number
  documents: number
  added: string[]
  changed: string[]
  removed: string[]
}

export interface HostRequest {
  method: string
  path: string
  query?: string
  headers?: Record<string, string>
  body?: Uint8Array
}

export interface HostResponse {
  status: number
  headers: Record<string, string>
  body: Uint8Array
}

export interface HostRequestExtras {
  /** Trusted fields from the host, used when a request creates a record. */
  fields?: Record<string, string>
}

export interface Host {
  handleRequest(request: HostRequest, identity: HostIdentity | null, extras?: HostRequestExtras): Promise<HostResponse | null>
  importScope(call: ScopeImportOptions): Promise<ScopeImportReport>
  exportScope(call: ScopeCall): Promise<ScopeBundle>
  /**
   * Run one verb for `identity` on `scope`. The verbs, the stdin rules, the
   * result and the Error shape are those of `engine.dispatch`; the verbs that
   * need a checkout reject with `invalid_argv`. An identity or scope that is
   * empty rejects with `invalid_argv` before any store call. Store failures
   * reject with `revision_conflict`, `unavailable` or `internal`; a refusal
   * of `beforeCommand` with `refused`.
   */
  dispatch(call: HostDispatchCall): Promise<unknown>
}

/**
 * Build one host kit for every user over a store port and optional hooks. A
 * store without `list`, `read` or `commit` throws synchronously, naming the
 * missing functions.
 */
export function createHost(options: CreateHostOptions): Host

export class StoreConflictError extends Error {
  readonly code: 'conflict'
  readonly key: string
  /** The document's current version, or null when it does not exist. */
  readonly actual: number | null
  constructor(key: string, actual: number | null)
}

/** A temporary failure for a store port's function to throw; the caller can retry. */
export class StoreUnavailableError extends Error {
  readonly code: 'unavailable'
  constructor(message: string)
}

export interface ConformanceHarness {
  /** The store port under test. */
  store: StorePort
  /** A new, unused scope on every call. */
  newScope(): MaybePromise<string>
  /** OPTIONAL: the events the store kept for `scope`, oldest first, each with its commit's version. Without it, `events-with-commit` is skipped. */
  recordedEvents?(scope: string): MaybePromise<{ name: string; payload: unknown; version: number }[]>
}

export interface ConformanceCheck {
  /** The check's stable name, the same as in the Rust suite (e.g. `conflict-stale-version`). */
  name: string
  outcome: 'passed' | 'failed' | 'skipped'
  detail?: string
}

export interface ConformanceReport {
  /** True when no check failed. */
  passed: boolean
  checks: ConformanceCheck[]
}

/**
 * Run the store contract's conformance checks — the same sixteen checks the
 * Rust implementations run — on a host's store port. Runs on a background
 * thread; the two concurrent checks call the store from several threads at
 * once, so they test the host's transactions.
 */
export function runStoreConformance(harness: ConformanceHarness): Promise<ConformanceReport>

export const eventNames: readonly string[]
export function eventResource(name: string, payload: unknown): {
  scope: 'change' | 'discussion' | 'spec' | 'unknown'
  resourceId: string
}

export const commandNames: readonly string[]
export function agentToolDescription(options?: { exclude?: string[] }): string
