//! The reference server's existing route handlers still name documents using
//! TeamStore ids. This adapter keeps that wire surface while writes, guards
//! and the event log go through the new native store.

use super::{scope_id, ServerPort};
use chrono::{DateTime, Utc};
use speclink_host::transfer::{document_of_key, key_of_document, legacy_document_id};
use speclink_store::{
    content_digest, Bundle, BundleDoc, Capability, CapabilityLevel, CommandContext, DocRef,
    Document, DocumentId, EventRecord, ExpectedRevision, ImportMode, ImportOutcome, ImportReport,
    ImportedDoc, Manifest, OutboxCursor, OutboxEntry, Revision, RevisionRecord, Scope, Snapshot,
    StagedOp, StoreError, TeamStore, UnitOfWork, BUNDLE_FORMAT_VERSION, CONTRACT_VERSION,
};
use speclink_store_port::{
    Actor, Commit, DocKey, Event, Expected, KeyPrefix, Op, PortError, ScopeId, Timestamp, Version,
};
use std::collections::BTreeMap;

pub(super) struct NativeTeamStore(pub Box<dyn ServerPort>);

fn backend(error: impl ToString) -> StoreError {
    StoreError::Backend {
        source: error.to_string(),
    }
}

fn key_of(doc: &DocumentId) -> Result<DocKey, StoreError> {
    key_of_document(&legacy_document_id(doc.clone()))
        .ok_or_else(|| backend("invalid document address"))
}

fn document_id(key: &DocKey) -> Result<DocumentId, StoreError> {
    document_of_key(key)
        .map(speclink_host::transfer::team_document_id)
        .ok_or_else(|| StoreError::Corrupt {
            reason: format!("unrecognized document key {}", key.as_str()),
        })
}

fn port_scope(scope: &Scope) -> Result<ScopeId, StoreError> {
    scope_id(scope.project.as_str(), scope.repo.as_str()).map_err(backend)
}

fn store_error(scope: &Scope, error: PortError) -> StoreError {
    match error {
        PortError::Conflict {
            key,
            expected,
            actual,
        } => match document_id(&key) {
            Ok(doc) => StoreError::RevisionConflict {
                doc: DocRef {
                    project: scope.project.clone(),
                    repo: scope.repo.clone(),
                    doc,
                },
                expected: match expected {
                    Expected::Absent => ExpectedRevision::Absent,
                    Expected::At(version) => ExpectedRevision::At(Revision(version.get())),
                },
                actual: actual.map(|version| Revision(version.get())),
            },
            Err(error) => error,
        },
        PortError::Unavailable { .. } => StoreError::Unavailable,
        PortError::Backend { message } => backend(message),
    }
}

fn required_revision(value: Revision) -> Result<Version, StoreError> {
    Version::new(value.0).map_err(backend)
}

fn expectation(value: ExpectedRevision) -> Result<Expected, StoreError> {
    match value {
        ExpectedRevision::Absent => Ok(Expected::Absent),
        ExpectedRevision::At(revision) => Ok(Expected::At(required_revision(revision)?)),
    }
}

struct Materialized {
    revision: Revision,
    documents: BTreeMap<DocumentId, Document>,
}

impl Snapshot for Materialized {
    fn revision(&self) -> Revision {
        self.revision
    }

    fn read(&self, doc: &DocumentId) -> Result<Option<Document>, StoreError> {
        Ok(self.documents.get(doc).cloned())
    }

    fn history(&self, _doc: &DocumentId) -> Result<Vec<RevisionRecord>, StoreError> {
        Err(backend(
            "the native store retains current documents and scope events",
        ))
    }
}

impl NativeTeamStore {
    fn materialize(&self, scope: &Scope) -> Result<Materialized, StoreError> {
        let id = port_scope(scope)?;
        let all = KeyPrefix::new("").expect("empty key prefix");
        for _ in 0..3 {
            let listing = self.0.list(&id, &all).map_err(|e| store_error(scope, e))?;
            let mut documents = BTreeMap::new();
            let mut stable = true;
            for chunk in listing.entries.chunks(200) {
                let keys = chunk
                    .iter()
                    .map(|entry| entry.key.clone())
                    .collect::<Vec<_>>();
                let read = self.0.read(&id, &keys).map_err(|e| store_error(scope, e))?;
                for (entry, doc) in chunk.iter().zip(read) {
                    if let Some(doc) = doc.filter(|doc| doc.version == entry.version) {
                        documents.insert(
                            document_id(&entry.key)?,
                            Document {
                                content: doc.content,
                                revision: Revision(doc.version.get()),
                            },
                        );
                    } else {
                        stable = false;
                        break;
                    }
                }
                if !stable || documents.len() > listing.entries.len() {
                    break;
                }
            }
            if stable
                && documents.len() == listing.entries.len()
                && self
                    .0
                    .list(&id, &all)
                    .map_err(|e| store_error(scope, e))?
                    .version
                    == listing.version
            {
                return Ok(Materialized {
                    revision: Revision(listing.version.get()),
                    documents,
                });
            }
        }
        Err(StoreError::Unavailable)
    }
}

impl TeamStore for NativeTeamStore {
    fn manifest(&self) -> Manifest {
        Manifest {
            contract_version: CONTRACT_VERSION,
            driver: self.0.kind().into(),
            level: CapabilityLevel::SingleNode,
            capabilities: [
                Capability::Snapshot,
                Capability::Cas,
                Capability::Transaction,
                Capability::Outbox,
                Capability::Backup,
            ]
            .into(),
        }
    }

    fn health(&self) -> Result<(), StoreError> {
        self.0.probe().map_err(backend)
    }

    fn migrate(&self, target_version: u32) -> Result<(), StoreError> {
        if target_version == CONTRACT_VERSION {
            Ok(())
        } else {
            Err(backend("unsupported store contract version"))
        }
    }

    fn snapshot<'a>(&'a self, scope: &Scope) -> Result<Box<dyn Snapshot + 'a>, StoreError> {
        Ok(Box::new(self.materialize(scope)?))
    }

    fn begin_unit_of_work(
        &self,
        scope: &Scope,
        ctx: CommandContext,
    ) -> Result<UnitOfWork, StoreError> {
        port_scope(scope)?;
        Ok(UnitOfWork::new(scope.clone(), ctx))
    }

    fn commit(&self, uow: UnitOfWork, events: Vec<EventRecord>) -> Result<Revision, StoreError> {
        let scope = uow.scope();
        let id = port_scope(scope)?;
        let ops = uow
            .ops()
            .iter()
            .map(|op| match op {
                StagedOp::Put {
                    doc,
                    content,
                    expected,
                } => Ok(Op::Put {
                    key: key_of(doc)?,
                    content: content.clone(),
                    expected: expectation(*expected)?,
                }),
                StagedOp::Delete { doc, expected } => Ok(Op::Delete {
                    key: key_of(doc)?,
                    expected: required_revision(*expected)?,
                }),
            })
            .collect::<Result<Vec<_>, StoreError>>()?;
        let guards = uow
            .guards()
            .iter()
            .map(|(doc, expected)| Ok((key_of(doc)?, expectation(*expected)?)))
            .collect::<Result<Vec<_>, StoreError>>()?;
        if ops.is_empty() && events.is_empty() {
            return Ok(self.materialize(scope)?.revision);
        }
        let actor = Actor::new(&uow.context().actor, &uow.context().actor).map_err(backend)?;
        let at = Timestamp::new(Utc::now().timestamp_millis().max(0) as u64).map_err(backend)?;
        let events = events
            .into_iter()
            .map(|event| Event::new(event.name, event.payload).map_err(backend))
            .collect::<Result<Vec<_>, _>>()?;
        let commit =
            Commit::new(actor, &uow.context().command, at, ops, events).map_err(backend)?;
        self.0
            .commit_guarded(&id, &commit, &guards)
            .map(|version| Revision(version.get()))
            .map_err(|e| store_error(scope, e))
    }

    fn rollback(&self, _uow: UnitOfWork) -> Result<(), StoreError> {
        Ok(())
    }

    fn export(&self, scope: &Scope) -> Result<Bundle, StoreError> {
        let snapshot = self.materialize(scope)?;
        Ok(Bundle {
            format_version: BUNDLE_FORMAT_VERSION,
            scope: scope.clone(),
            project_revision: snapshot.revision,
            documents: snapshot
                .documents
                .into_iter()
                .map(|(doc, read)| BundleDoc {
                    doc,
                    digest: content_digest(&read.content),
                    content: read.content,
                })
                .collect(),
        })
    }

    fn import(&self, bundle: Bundle, mode: ImportMode) -> Result<ImportReport, StoreError> {
        if bundle.format_version != BUNDLE_FORMAT_VERSION {
            return Err(backend("unsupported bundle format version"));
        }
        let snapshot = self.materialize(&bundle.scope)?;
        if mode == ImportMode::CreateNew && !snapshot.documents.is_empty() {
            return Err(backend(
                "import (create-new): target scope already holds documents",
            ));
        }
        let mut uow = UnitOfWork::new(
            bundle.scope,
            CommandContext {
                command: "import".into(),
                actor: "speclink-server".into(),
            },
        );
        let mut documents = Vec::with_capacity(bundle.documents.len());
        for doc in bundle.documents {
            if content_digest(&doc.content) != doc.digest {
                return Err(StoreError::Corrupt {
                    reason: "bundle content digest mismatch".into(),
                });
            }
            let current = snapshot.documents.get(&doc.doc);
            let outcome = current.map_or(ImportOutcome::Created, |_| ImportOutcome::Overwritten);
            uow.put(
                doc.doc.clone(),
                doc.content,
                current.map_or(ExpectedRevision::Absent, |read| {
                    ExpectedRevision::At(read.revision)
                }),
            );
            documents.push(ImportedDoc {
                doc: doc.doc,
                outcome,
            });
        }
        let project_revision = self.commit(uow, Vec::new())?;
        Ok(ImportReport {
            project_revision,
            documents,
        })
    }

    fn read_outbox(
        &self,
        scope: &Scope,
        from: OutboxCursor,
    ) -> Result<Vec<OutboxEntry>, StoreError> {
        let id = port_scope(scope)?;
        let mut entries = Vec::new();
        let mut cursor = from.0;
        loop {
            let batch = self
                .0
                .events_after(&id, cursor, 256)
                .map_err(|e| store_error(scope, e))?;
            let done = batch.len() < 256;
            for (seq, event) in batch {
                let at = event
                    .payload
                    .get("occurredAt")
                    .and_then(serde_json::Value::as_str)
                    .and_then(|at| DateTime::parse_from_rfc3339(at).ok())
                    .map(|at| at.with_timezone(&Utc))
                    .unwrap_or(DateTime::<Utc>::UNIX_EPOCH);
                entries.push(OutboxEntry {
                    seq,
                    revision: Revision(event.version.get()),
                    record: EventRecord {
                        name: event.name,
                        payload: event.payload,
                        actor: String::new(),
                        at,
                    },
                });
                cursor = seq;
            }
            if done {
                return Ok(entries);
            }
        }
    }

    fn ack_outbox(&self, scope: &Scope, up_to: OutboxCursor) -> Result<(), StoreError> {
        self.0
            .trim_events(&port_scope(scope)?, up_to.0)
            .map_err(|e| store_error(scope, e))
    }

    fn outbox_acked(&self, scope: &Scope) -> Result<OutboxCursor, StoreError> {
        self.0
            .event_floor(&port_scope(scope)?)
            .map(OutboxCursor)
            .map_err(|e| store_error(scope, e))
    }
}
