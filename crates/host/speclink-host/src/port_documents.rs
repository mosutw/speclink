//! Document reads on the store port for the embedded HTTP entry point.

use crate::port_view::{parse_key, spec_capability, KeyKind};
use crate::projection::content_digest;
use speclink_core::workspace::DEFAULT_SPEC_DIR;
use speclink_protocol::context::{ContextDocument, ContextSnapshot, ContextSnapshotRequest};
use speclink_protocol::query::{SearchHit, SearchResponse, SpecDocumentResponse};
use speclink_store_port::{
    keys, DocKey, Document, KeyPrefix, Listing, PortError, ScopeId, StorePort, Version,
};
use std::collections::{BTreeMap, BTreeSet};

const SNIPPET_CONTEXT: usize = 30;

#[derive(Debug)]
pub(crate) enum ReadError {
    NotFound(String),
    Store(PortError),
}

impl From<PortError> for ReadError {
    fn from(error: PortError) -> Self {
        Self::Store(error)
    }
}

/// The ETag of a scope version: the number in double quotes.
pub fn etag(version: Version) -> String {
    format!("\"{}\"", version.get())
}

/// One of the contract's fixed prefixes (`keys::CHANGES` and the like).
fn prefix(fixed: &'static str) -> KeyPrefix {
    KeyPrefix::new(fixed).expect("a fixed prefix of the contract")
}

/// A prefix that lists exactly one document, for the scope version that
/// comes with every list. A document key follows every rule of a prefix.
fn just(key: &DocKey) -> KeyPrefix {
    KeyPrefix::new(key.as_str()).expect("a document key is a prefix")
}

fn read_one(
    port: &dyn StorePort,
    scope: &ScopeId,
    key: DocKey,
) -> Result<Option<Document>, PortError> {
    Ok(port.read(scope, &[key])?.into_iter().next().flatten())
}

/// The scope version, then one document: the version first, so it is never
/// newer than the content (D2).
fn versioned(
    port: &dyn StorePort,
    scope: &ScopeId,
    key: DocKey,
) -> Result<(Version, Option<Document>), PortError> {
    let version = port.list(scope, &just(&key))?.version;
    Ok((version, read_one(port, scope, key)?))
}

/// The documents of `keys`, by key, in one read. No read for no key.
fn read_all(
    port: &dyn StorePort,
    scope: &ScopeId,
    keys: Vec<DocKey>,
) -> Result<BTreeMap<DocKey, Document>, PortError> {
    if keys.is_empty() {
        return Ok(BTreeMap::new());
    }
    let docs = port.read(scope, &keys)?;
    Ok(keys
        .into_iter()
        .zip(docs)
        .filter_map(|(key, doc)| Some((key, doc?)))
        .collect())
}

/// The key of a document of an active change, or `None` when the names
/// cannot make one that reads back as that same document.
pub(crate) fn artifact_key(change: &str, artifact: &str) -> Option<DocKey> {
    let key = keys::change_artifact(change, artifact).ok()?;
    let same = matches!(
        parse_key(key.as_str()),
        Some(KeyKind::ChangeArtifact { change: c, artifact: a }) if c == change && a == artifact
    );
    same.then_some(key)
}

/// The scope's current version (the ETag and the polling token), from the
/// narrowest list there is: the key of the workflow config. Every list
/// returns the scope version, and this one returns one document at most.
pub fn scope_version(port: &dyn StorePort, scope: &ScopeId) -> Result<Version, PortError> {
    Ok(port.list(scope, &just(&keys::config()))?.version)
}

/// One canonical spec and the scope version.
pub fn spec_document(
    port: &dyn StorePort,
    scope: &ScopeId,
    capability: &str,
) -> Result<(SpecDocumentResponse, Version), ReadError> {
    let not_found = || ReadError::NotFound(format!("Spec '{capability}' not found."));
    let key = keys::spec(capability).map_err(|_| not_found())?;
    let (version, doc) = versioned(port, scope, key)?;
    let content = doc.ok_or_else(not_found)?.content;
    Ok((SpecDocumentResponse { content }, version))
}

/// The desktop's search over active changes and live discussions: a fixed
/// artifact order, one hit for each change, char-safe snippets. Two lists
/// and one read; an empty query only takes the scope version.
pub fn search(
    port: &dyn StorePort,
    scope: &ScopeId,
    query: &str,
) -> Result<(SearchResponse, Version), ReadError> {
    let q = query.trim();
    if q.is_empty() {
        return Ok((
            SearchResponse { hits: Vec::new() },
            scope_version(port, scope)?,
        ));
    }
    let changes = port.list(scope, &prefix(keys::CHANGES))?;
    let discussions = port.list(scope, &prefix(keys::DISCUSSIONS))?;

    // Each change with a data file, and the artifacts it has.
    #[derive(Default)]
    struct Card<'k> {
        has_meta: bool,
        artifacts: BTreeSet<&'k str>,
        deltas: Vec<&'k str>,
    }
    let mut cards: BTreeMap<&str, Card> = BTreeMap::new();
    for entry in &changes.entries {
        match parse_key(entry.key.as_str()) {
            Some(KeyKind::ChangeMeta { change }) => {
                cards.entry(change).or_default().has_meta = true
            }
            Some(KeyKind::ChangeArtifact { change, artifact }) => {
                let card = cards.entry(change).or_default();
                card.artifacts.insert(artifact);
                if let Some(capability) = spec_capability(artifact) {
                    card.deltas.push(capability);
                }
            }
            _ => {}
        }
    }
    let order: Vec<(&str, Vec<String>)> = cards
        .iter()
        .filter(|(_, card)| card.has_meta)
        .map(|(change, card)| {
            let artifacts = ["proposal.md", "design.md", "tasks.md"]
                .into_iter()
                .map(str::to_string)
                .chain(card.deltas.iter().map(|cap| format!("specs/{cap}/spec.md")))
                .filter(|artifact| card.artifacts.contains(artifact.as_str()))
                .collect();
            (*change, artifacts)
        })
        .collect();
    let slugs: BTreeSet<&str> = discussions
        .entries
        .iter()
        .filter_map(|entry| match parse_key(entry.key.as_str()) {
            Some(KeyKind::Discussion { slug }) => Some(slug),
            _ => None,
        })
        .collect();

    let mut wanted: Vec<DocKey> = order
        .iter()
        .flat_map(|(change, artifacts)| artifacts.iter().filter_map(|a| artifact_key(change, a)))
        .collect();
    wanted.extend(slugs.iter().filter_map(|slug| keys::discussion(slug).ok()));
    let docs = read_all(port, scope, wanted)?;

    let mut hits = Vec::new();
    for (change, artifacts) in &order {
        for artifact in artifacts {
            let Some(doc) = artifact_key(change, artifact).and_then(|key| docs.get(&key)) else {
                continue;
            };
            if let Some(snippet) = find_snippet(&doc.content, q) {
                hits.push(SearchHit {
                    kind: "change".to_string(),
                    id: change.to_string(),
                    artifact: artifact.clone(),
                    snippet,
                });
                break;
            }
        }
    }
    for slug in slugs {
        let Some(doc) = keys::discussion(slug).ok().and_then(|key| docs.get(&key)) else {
            continue;
        };
        if let Some(snippet) = find_snippet(&doc.content, q) {
            hits.push(SearchHit {
                kind: "discussion".to_string(),
                id: slug.to_string(),
                artifact: format!("{slug}.md"),
                snippet,
            });
        }
    }
    Ok((SearchResponse { hits }, changes.version))
}

// --- the context snapshot ---

/// What [`context_snapshot`] answers.
#[derive(Debug)]
pub enum ContextOutcome {
    /// The caller's `If-None-Match` is the current scope version.
    NotModified(Version),
    Snapshot(ContextSnapshot),
}

/// The projection path of the workflow config: the source of the policy
/// revision.
fn config_path() -> String {
    format!("{DEFAULT_SPEC_DIR}/config.yaml")
}

/// The projection path (under `speclink/`) of a document when the request
/// selects it, or `None` to leave it out. A change narrows to its own
/// artifacts, every canonical spec, the config and the vocabulary; no change
/// is the full mirror, with every change's artifacts and the live
/// discussions too. Change data files, evidence and the archive are never
/// part of the mirror.
fn select_path(kind: KeyKind<'_>, change: Option<&str>) -> Option<String> {
    match kind {
        KeyKind::ChangeArtifact {
            change: c,
            artifact,
        } => (!artifact.starts_with("checks/") && change.is_none_or(|n| n == c))
            .then(|| format!("{DEFAULT_SPEC_DIR}/changes/{c}/{artifact}")),
        KeyKind::Spec { capability } => {
            Some(format!("{DEFAULT_SPEC_DIR}/specs/{capability}/spec.md"))
        }
        KeyKind::Config => Some(config_path()),
        KeyKind::Language => Some(format!("{DEFAULT_SPEC_DIR}/LANGUAGE.md")),
        KeyKind::Discussion { slug } if change.is_none() => {
            Some(format!("{DEFAULT_SPEC_DIR}/discussions/{slug}.md"))
        }
        _ => None,
    }
}

/// The context snapshot of a scope (design D4). The snapshot id is the scope
/// version's ETag, so an `If-None-Match` that equals it answers "not
/// modified" after one list and no read.
pub fn context_snapshot_once(
    port: &dyn StorePort,
    scope: &ScopeId,
    request: &ContextSnapshotRequest,
    if_none_match: Option<&str>,
) -> Result<ContextOutcome, ReadError> {
    let changes = port.list(scope, &prefix(keys::CHANGES))?;
    let snapshot_id = etag(changes.version);
    if if_none_match == Some(snapshot_id.as_str()) {
        return Ok(ContextOutcome::NotModified(changes.version));
    }
    let change = request.change.as_deref();
    // A change with no data file and no artifact does not exist.
    if let Some(name) = change {
        let exists = changes.entries.iter().any(|entry| {
            matches!(
                parse_key(entry.key.as_str()),
                Some(KeyKind::ChangeMeta { change: c } | KeyKind::ChangeArtifact { change: c, .. })
                    if c == name
            )
        });
        if !exists {
            return Err(ReadError::NotFound(format!("change '{name}' not found")));
        }
    }
    let specs = port.list(scope, &prefix(keys::SPECS))?;
    let discussions = match change {
        None => Some(port.list(scope, &prefix(keys::DISCUSSIONS))?),
        Some(_) => None,
    };

    let listed = [Some(&changes), Some(&specs), discussions.as_ref()]
        .into_iter()
        .flatten()
        .flat_map(|listing: &Listing| listing.entries.iter().map(|entry| entry.key.clone()));
    let mut selected: Vec<(DocKey, String)> = listed
        .chain([keys::config(), keys::language()])
        .filter_map(|key| {
            let path = select_path(parse_key(key.as_str())?, change)?;
            Some((key, path))
        })
        .collect();
    let docs = port.read(
        scope,
        &selected
            .iter()
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>(),
    )?;

    // A document that was listed but is gone by the read is left out.
    let mut documents: Vec<ContextDocument> = selected
        .drain(..)
        .zip(docs)
        .filter_map(|((_, path), doc)| {
            let doc = doc?;
            Some(ContextDocument {
                path,
                digest: content_digest(&doc.content),
                content: doc.content,
                revision: Some(doc.version.get()),
            })
        })
        .collect();
    documents.sort_by(|a, b| a.path.cmp(&b.path));

    let policy_revision = documents
        .iter()
        .find(|d| d.path == config_path())
        .and_then(|d| d.revision);
    let combined: Vec<&str> = documents.iter().map(|d| d.digest.as_str()).collect();
    let digest = content_digest(&combined.join("\n"));
    Ok(ContextOutcome::Snapshot(ContextSnapshot {
        snapshot_id,
        policy_revision,
        digest,
        documents,
    }))
}

// --- text helpers ---

fn find_snippet(text: &str, query: &str) -> Option<String> {
    let haystack: Vec<char> = text.chars().collect();
    let folded_haystack: Vec<char> = haystack.iter().map(lower_first).collect();
    let needle: Vec<char> = query
        .chars()
        .map(|character| lower_first(&character))
        .collect();
    if needle.is_empty() || folded_haystack.len() < needle.len() {
        return None;
    }
    let start = (0..=folded_haystack.len() - needle.len())
        .find(|&index| folded_haystack[index..index + needle.len()] == needle[..])?;
    let end = start + needle.len();
    let from = start.saturating_sub(SNIPPET_CONTEXT);
    let to = (end + SNIPPET_CONTEXT).min(haystack.len());
    let mut snippet = String::new();
    if from > 0 {
        snippet.push('…');
    }
    snippet.extend(haystack[from..to].iter().map(|character| match character {
        '\n' | '\r' => ' ',
        character => *character,
    }));
    if to < haystack.len() {
        snippet.push('…');
    }
    Some(snippet)
}

fn lower_first(character: &char) -> char {
    character.to_lowercase().next().unwrap_or(*character)
}

pub(crate) fn context_snapshot(
    port: &dyn StorePort,
    scope: &ScopeId,
    request: &ContextSnapshotRequest,
    if_none_match: Option<&str>,
) -> Result<ContextOutcome, ReadError> {
    for _ in 0..3 {
        let outcome = context_snapshot_once(port, scope, request, if_none_match)?;
        match &outcome {
            ContextOutcome::NotModified(_) => return Ok(outcome),
            ContextOutcome::Snapshot(snapshot)
                if snapshot.snapshot_id == etag(scope_version(port, scope)?) =>
            {
                return Ok(outcome);
            }
            ContextOutcome::Snapshot(_) => {}
        }
    }
    Err(PortError::Unavailable {
        message: "the scope changed while reading its context; retry the request".into(),
    }
    .into())
}
