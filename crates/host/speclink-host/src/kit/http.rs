use super::{Caller, HostKit};
use crate::port_documents::{self as docs, ContextOutcome, ReadError};
use crate::port_exec::ExecError;
use crate::transfer::{self, SentBundle, UnreadBundle};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use speclink_core::command::{Command, CommandError, CommandOutcome, ErrorCode};
use speclink_core::fields::Fields;
use speclink_protocol::command::{
    CreateChangeRequest, CreateChangeResponse, CreateDiscussionRequest, CreateDiscussionResponse,
    PromoteDiscussionRequest, PromoteDiscussionResponse, SetFieldsRequest, SetFieldsResponse,
};
use speclink_protocol::context::ContextSnapshotRequest;
use speclink_store_port::Version;

pub const WORKSPACE_SEARCH: &str = "workspace-search";
pub const CONTEXT_SNAPSHOT: &str = "context-snapshot";
pub const SYNC_STATE: &str = "sync-state";

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ProjectAction {
    BoardOrder,
    SyncState { if_none_match: Option<String> },
    ListSpecs,
    ListChanges,
    Spec { capability: String },
    Search { query: String },
    Context { if_none_match: Option<String> },
    Export,
    Import { project: String, repo: String },
    CreateChange { repo: String },
    CreateDiscussion,
    PromoteDiscussion { slug: String },
    ChangeFields { name: String },
    DiscussionFields { slug: String },
}

#[derive(Serialize)]
pub struct ProjectReply {
    status: u16,
    etag: Option<String>,
    body: Value,
}

impl ProjectReply {
    fn json(body: impl Serialize, etag: String) -> Self {
        Self {
            status: 200,
            etag: Some(etag),
            body: json!(body),
        }
    }

    fn error(status: u16, reason: &str, message: impl ToString) -> Self {
        Self {
            status,
            etag: None,
            body: json!({ "status": status, "reason": reason, "message": message.to_string() }),
        }
    }
}

impl From<ReadError> for ExecError {
    fn from(error: ReadError) -> Self {
        match error {
            ReadError::NotFound(message) => command_error(ErrorCode::NotFound, message),
            ReadError::Store(error) => ExecError::Store(error),
        }
    }
}

impl HostKit {
    /// Answer an HTTP action after the integration has resolved the caller
    /// and checked the route's repository permissions.
    pub fn handle_project_action(
        &self,
        caller: &Caller,
        action: ProjectAction,
        body: &[u8],
        attached: Fields,
    ) -> Result<ProjectReply, ExecError> {
        let store = self.store.as_ref();
        let scope = &caller.scope;
        let reply = match action {
            ProjectAction::BoardOrder => {
                let board = self.get_board_order(caller)?;
                let etag = format!("\"{}\"", board.revision);
                ProjectReply::json(board, etag)
            }
            ProjectAction::SyncState { if_none_match } => {
                self.ask(caller, SYNC_STATE, json!({}))?;
                let etag = docs::etag(docs::scope_version(store, scope).map_err(ExecError::Store)?);
                if if_none_match.as_deref() == Some(etag.as_str()) {
                    ProjectReply {
                        status: 304,
                        etag: Some(etag),
                        body: Value::Null,
                    }
                } else {
                    ProjectReply::json(json!({}), etag)
                }
            }
            ProjectAction::ListSpecs => self.http_list(caller, true)?,
            ProjectAction::ListChanges => self.http_list(caller, false)?,
            ProjectAction::Spec { capability } => {
                let cmd = Command::Show {
                    item: Some(capability.clone()),
                    item_type: Some("spec".into()),
                };
                self.ask(caller, cmd.name(), cmd.params())?;
                let (doc, version) = docs::spec_document(store, scope, &capability)?;
                ProjectReply::json(doc, docs::etag(version))
            }
            ProjectAction::Search { query } => {
                self.ask(caller, WORKSPACE_SEARCH, json!({ "query": query }))?;
                let (response, version) = docs::search(store, scope, &query)?;
                ProjectReply::json(response, docs::etag(version))
            }
            ProjectAction::Context { if_none_match } => {
                let request: ContextSnapshotRequest = parse(body)?;
                let params = request
                    .change
                    .as_ref()
                    .map_or_else(|| json!({}), |change| json!({ "change": change }));
                self.ask(caller, CONTEXT_SNAPSHOT, params)?;
                match docs::context_snapshot(store, scope, &request, if_none_match.as_deref())? {
                    ContextOutcome::Snapshot(snapshot) => {
                        let etag = snapshot.snapshot_id.clone();
                        ProjectReply::json(snapshot, etag)
                    }
                    ContextOutcome::NotModified(version) => ProjectReply {
                        status: 304,
                        etag: Some(docs::etag(version)),
                        body: Value::Null,
                    },
                }
            }
            ProjectAction::Export => {
                let bundle = self.export_scope(caller)?;
                let version = bundle
                    .scope_version
                    .expect("an export carries a scope version");
                ProjectReply::json(bundle, format!("\"{version}\""))
            }
            ProjectAction::Import { project, repo } => {
                return self.import_http(caller, &project, &repo, body)
            }
            ProjectAction::CreateChange { repo } => {
                let req: CreateChangeRequest = parse(body)?;
                let command = Command::NewChange {
                    name: req.name,
                    description: req.description,
                    schema: req.schema,
                    agent: req.agent,
                    from_discussion: req.from_discussion,
                    last: req.last,
                    fields: req.fields,
                };
                let (outcome, version) = self.http_run(caller, command, attached)?;
                let CommandOutcome::NewChange(outcome) = outcome else {
                    unreachable!("new change outcome")
                };
                ProjectReply::json(
                    CreateChangeResponse {
                        name: outcome.name,
                        schema: Some(outcome.schema),
                        repo: Some(repo),
                        lifecycle: None,
                    },
                    docs::etag(version),
                )
            }
            ProjectAction::CreateDiscussion => {
                let req: CreateDiscussionRequest = parse(body)?;
                let command = Command::DiscussNew {
                    topic: req.topic,
                    slug: req.slug,
                    kind: req.kind,
                    fields: req.fields,
                };
                let (outcome, version) = self.http_run(caller, command, attached)?;
                let CommandOutcome::DiscussNew(info) = outcome else {
                    unreachable!("new discussion outcome")
                };
                ProjectReply::json(
                    CreateDiscussionResponse {
                        slug: info.slug,
                        topic: info.topic,
                        path: info.path,
                    },
                    docs::etag(version),
                )
            }
            ProjectAction::PromoteDiscussion { slug } => {
                let req: PromoteDiscussionRequest = parse(body)?;
                let command = Command::DiscussPromote {
                    slug,
                    name: req.name,
                    last: req.last,
                    fields: req.fields,
                };
                let (outcome, version) = self.http_run(caller, command, attached)?;
                let CommandOutcome::DiscussPromote(outcome) = outcome else {
                    unreachable!("promote discussion outcome")
                };
                ProjectReply::json(
                    PromoteDiscussionResponse {
                        change: outcome.change,
                    },
                    docs::etag(version),
                )
            }
            ProjectAction::ChangeFields { ref name }
            | ProjectAction::DiscussionFields { slug: ref name } => {
                let req: SetFieldsRequest = parse(body)?;
                let command = match action {
                    ProjectAction::ChangeFields { .. } => Command::ChangeFieldsEdit {
                        name: name.clone(),
                        set: req.set,
                        unset: req.unset,
                    },
                    _ => Command::DiscussFieldsEdit {
                        slug: name.clone(),
                        set: req.set,
                        unset: req.unset,
                    },
                };
                let (outcome, version) = self.http_run(caller, command, Fields::new())?;
                let (CommandOutcome::ChangeFieldsEdit(outcome)
                | CommandOutcome::DiscussFieldsEdit(outcome)) = outcome
                else {
                    unreachable!("field edit outcome")
                };
                ProjectReply::json(
                    SetFieldsResponse {
                        fields: outcome.fields,
                        changed: outcome.changed,
                    },
                    docs::etag(version),
                )
            }
        };
        Ok(reply)
    }

    fn http_list(&self, caller: &Caller, specs: bool) -> Result<ProjectReply, ExecError> {
        let command = Command::List {
            sort: "name".into(),
            specs,
            changes: !specs,
            worktrees: Default::default(),
        };
        let (outcome, version) = self.http_run(caller, command, Fields::new())?;
        let CommandOutcome::List(outcome) = outcome else {
            unreachable!("list outcome")
        };
        let body = if specs {
            json!({ "specs": outcome.specs })
        } else {
            json!({ "changes": outcome.changes })
        };
        Ok(ProjectReply::json(body, docs::etag(version)))
    }

    fn http_run(
        &self,
        caller: &Caller,
        command: Command,
        attached: Fields,
    ) -> Result<(CommandOutcome, Version), ExecError> {
        let execution = self.run_with_fields(caller, command, attached)?;
        let version = match execution.scope_version {
            Some(version) => version,
            None => {
                docs::scope_version(self.store.as_ref(), &caller.scope).map_err(ExecError::Store)?
            }
        };
        Ok((execution.outcome, version))
    }

    fn import_http(
        &self,
        caller: &Caller,
        project: &str,
        repo: &str,
        body: &[u8],
    ) -> Result<ProjectReply, ExecError> {
        if body.len() > self.import_limit_bytes {
            return Ok(ProjectReply::error(
                413,
                "payload_too_large",
                format!(
                    "request body exceeds the {}-byte import limit",
                    self.import_limit_bytes
                ),
            ));
        }
        let sent = match transfer::read_bundle(body) {
            Ok(sent) => sent,
            Err(error) => {
                let (status, reason) = match &error {
                    UnreadBundle::NotJson(_) => (400, "invalid_argument"),
                    UnreadBundle::WrongShape(_) => (422, "unprocessable"),
                    UnreadBundle::UnknownVersion(_) => (409, "refused"),
                };
                return Ok(ProjectReply::error(status, reason, error));
            }
        };
        let first = match &sent {
            SentBundle::First(bundle) => {
                if bundle.scope.project != project || bundle.scope.repo != repo {
                    return Ok(ProjectReply::error(
                        409,
                        "refused",
                        "bundle scope does not match authenticated binding",
                    ));
                }
                Some(
                    bundle
                        .documents
                        .iter()
                        .map(|entry| entry.document.clone())
                        .collect::<Vec<_>>(),
                )
            }
            SentBundle::Keyed(_) => None,
        };
        let bundle = sent.into_keyed()?;
        let report = self.import_scope(caller, bundle, Default::default())?;
        let etag = docs::etag(report.scope_version);
        Ok(match first {
            Some(documents) => ProjectReply::json(
                transfer::first_version_report(report.scope_version.get(), documents),
                etag,
            ),
            None => ProjectReply::json(
                json!({ "scopeVersion": report.scope_version.get(), "documents": report.documents }),
                etag,
            ),
        })
    }
}

fn parse<T: DeserializeOwned>(body: &[u8]) -> Result<T, ExecError> {
    serde_json::from_slice(body)
        .map_err(|error| command_error(ErrorCode::InvalidArgv, error.to_string()))
}

fn command_error(code: ErrorCode, message: impl Into<String>) -> ExecError {
    ExecError::Command(CommandError::new(code, message))
}
