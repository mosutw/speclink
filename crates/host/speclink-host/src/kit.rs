//! The host kit (design D1): what a host that embeds speclink builds once and
//! shares between every user. It holds the store and the hooks; each call
//! brings who acts and on which scope.
//!
//! The kit takes Engine commands, not verb text: every door (the Agent tool
//! of the Node SDK today, the HTTP door later) turns its input into a
//! [`Command`] and calls [`HostKit::run`], so the hooks see the same thing
//! from every door.

mod http;
pub use http::{ProjectAction, ProjectReply, CONTEXT_SNAPSHOT, SYNC_STATE, WORKSPACE_SEARCH};

use crate::commit::event_payload;
use crate::port_exec::{execute_as, ExecError, PortExecution};
use crate::transfer::{self, ImportOptions, ImportReport, ScopeBundle, TransferError};
use speclink_core::command::{Command, CommandError, ErrorCode, ExecutionContext};
use speclink_store_port::Timestamp;
use speclink_store_port::{Actor, Event, ScopeId, StorePort, Version};
use std::sync::Arc;
use std::collections::BTreeSet;
use speclink_core::fields::{self, Fields};

/// Who acts: an id that stays when the user is renamed, and the name that
/// documents are stamped with. Neither is empty or only white space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    actor: Actor,
}

impl Identity {
    pub fn new(
        id: impl Into<String>,
        display_name: impl Into<String>,
    ) -> Result<Identity, InvalidIdentity> {
        let (id, display_name) = (id.into(), display_name.into());
        if id.trim().is_empty() {
            return Err(InvalidIdentity { field: "id" });
        }
        if display_name.trim().is_empty() {
            return Err(InvalidIdentity {
                field: "display name",
            });
        }
        let actor = Actor::new(id, display_name).map_err(|_| InvalidIdentity { field: "id" })?;
        Ok(Identity { actor })
    }

    /// The id the commit record carries.
    pub fn id(&self) -> &str {
        self.actor.id()
    }

    /// The name the commit record and the document stamps carry.
    pub fn display_name(&self) -> &str {
        self.actor.display_name()
    }
}

/// An identity field that is empty or only white space.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidIdentity {
    field: &'static str,
}

impl InvalidIdentity {
    /// `id` or `display name`.
    pub fn field(&self) -> &'static str {
        self.field
    }
}

impl std::fmt::Display for InvalidIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "identity {} must not be empty or blank", self.field)
    }
}

impl std::error::Error for InvalidIdentity {}

/// One call's who and where.
#[derive(Debug, Clone)]
pub struct Caller {
    pub identity: Identity,
    pub scope: ScopeId,
}

/// What the before-command hook is asked about (design D4).
#[derive(Debug, Clone)]
pub struct BeforeCommand {
    pub identity: Identity,
    pub scope: ScopeId,
    /// [`Command::name`].
    pub command: &'static str,
    /// [`Command::params`].
    pub params: serde_json::Value,
    /// The keys the write changes; only the two workflow-config write
    /// commands carry them (empty when the write fails or changes nothing).
    pub changed_keys: Option<Vec<String>>,
}

/// The before-command hook's answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Allow,
    /// The call ends as `refused` with this reason.
    Deny {
        reason: String,
    },
}

/// What the after-commit hook watches (design D5).
#[derive(Debug, Clone)]
pub struct AfterCommit {
    pub identity: Identity,
    pub scope: ScopeId,
    pub command: &'static str,
    pub scope_version: Version,
    /// The events of the commit, as the store's commit received them.
    pub events: Vec<Event>,
}

pub type BeforeCommandHook = Box<dyn Fn(&BeforeCommand) -> Decision + Send + Sync>;
pub type AfterCommitHook = Box<dyn Fn(&AfterCommit) + Send + Sync>;

/// The hooks of a kit; each one is optional.
#[derive(Default)]
pub struct Hooks {
    /// Asked exactly once per call, before anything reaches the store's
    /// commit. Absent: every call is allowed.
    pub before_command: Option<BeforeCommandHook>,
    /// Called once after the store's commit succeeded; it cannot change the
    /// result, and a panic in it is caught.
    pub after_commit: Option<AfterCommitHook>,
}

/// The fixed text of a refusal that gives no reason.
pub const GET_BOARD_ORDER: &str = "get-board-order";

const NO_REASON: &str = "refused by host";

/// One kit for every user: the store and the hooks, with no state of its
/// own, so one value serves every request.
#[derive(Clone, Debug, Default)]
pub struct ReservedFieldPrefixes(BTreeSet<String>);

impl ReservedFieldPrefixes {
    pub fn new(prefixes: Vec<String>) -> anyhow::Result<Self> {
        for prefix in &prefixes {
            fields::check(&Fields::from([(prefix.clone(), String::new())]))?;
        }
        Ok(Self(prefixes.into_iter().collect()))
    }
}

pub struct HostKit {
    store: Arc<dyn StorePort>,
    hooks: Hooks,
    import_limit_bytes: usize,
    reserved_field_prefixes: ReservedFieldPrefixes,
}

impl HostKit {
    pub fn new(store: Arc<dyn StorePort>, hooks: Hooks) -> HostKit {
        HostKit {
            store,
            hooks,
            import_limit_bytes: 32 * 1024 * 1024,
            reserved_field_prefixes: ReservedFieldPrefixes::default(),
        }
    }

    /// Run `cmd` for `caller`. Without hooks the result is what
    /// [`crate::port_exec::execute`] gives for the same command; the commit
    /// records the caller's identity and the command's name.
    pub fn run(&self, caller: &Caller, cmd: Command) -> Result<PortExecution, ExecError> {
        self.run_with_fields(caller, cmd, Fields::new())
    }

    pub fn with_reserved_field_prefixes(mut self, prefixes: ReservedFieldPrefixes) -> Self {
        self.reserved_field_prefixes = prefixes;
        self
    }

    pub fn run_with_fields(
        &self,
        caller: &Caller,
        mut cmd: Command,
        attached: Fields,
    ) -> Result<PortExecution, ExecError> {
        if let Some(key) = cmd.written_field_keys().into_iter().find(|key| {
            self.reserved_field_prefixes.0.iter().any(|prefix| key.starts_with(prefix.as_str()))
        }) {
            return Err(ExecError::Command(CommandError::new(
                ErrorCode::Refused,
                format!("field '{key}' is reserved by the host"),
            )));
        }
        if let Some(fields) = cmd.creation_fields_mut() {
            fields.extend(attached);
            speclink_core::fields::check(fields).map_err(|error| {
                ExecError::Command(CommandError::new(ErrorCode::InvalidArgv, error.to_string()))
            })?;
        }
        let command = cmd.name();
        let params = match &self.hooks.before_command {
            Some(_) => cmd.params(),
            None => serde_json::Value::Null,
        };
        let cmd = without_local_facts(cmd);
        // The kit reads no process environment and no checkout: the Engine
        // gets the caller's name and nothing else.
        let engine_ctx = ExecutionContext {
            actor: Some(caller.identity.display_name().to_string()),
            ..Default::default()
        };
        let (store, scope, actor) = (self.store.as_ref(), &caller.scope, &caller.identity.actor);

        self.ask(caller, command, params)?;
        let execution = execute_as(store, scope, actor, &engine_ctx, cmd)?;

        if let Some(scope_version) = execution.scope_version {
            let events = execution
                .events
                .iter()
                .map(|event| Event::new(event.kind(), event_payload(event)))
                .collect::<Result<Vec<Event>, _>>();
            // The same mapping built the commit's events, so it cannot fail here.
            if let Ok(events) = events {
                self.watch(caller, command, scope_version, events);
            }
        }
        Ok(execution)
    }

    pub fn with_import_limit(mut self, limit: usize) -> Self {
        self.import_limit_bytes = limit;
        self
    }

    pub fn export_scope(&self, caller: &Caller) -> Result<ScopeBundle, ExecError> {
        self.ask(caller, transfer::EXPORT_COMMAND, serde_json::json!({}))?;
        Ok(transfer::export(self.store.as_ref(), &caller.scope)?)
    }

    pub fn import_scope(
        &self,
        caller: &Caller,
        bundle: ScopeBundle,
        options: ImportOptions,
    ) -> Result<ImportReport, ExecError> {
        let bytes: usize = bundle.documents.iter().map(|doc| doc.content.len()).sum();
        if bytes > self.import_limit_bytes {
            return Err(TransferError::TooLarge(format!(
                "the bundle's content is {bytes} bytes, over the {}-byte import limit",
                self.import_limit_bytes
            ))
            .into());
        }
        self.ask(
            caller,
            transfer::IMPORT_COMMAND,
            serde_json::json!({
                "documents": bundle.documents.len(),
                "bytes": bytes,
                "mode": options.mode.as_str(),
                "dryRun": options.dry_run,
            }),
        )?;
        let at =
            Timestamp::new(chrono::Utc::now().timestamp_millis().max(0) as u64).map_err(|e| {
                ExecError::Store(speclink_store_port::PortError::Backend {
                    message: e.to_string(),
                })
            })?;
        let (report, events) = transfer::import_with(
            self.store.as_ref(),
            &caller.scope,
            &caller.identity.actor,
            bundle,
            at,
            options,
        )?;
        if !events.is_empty() {
            self.watch(
                caller,
                transfer::IMPORT_COMMAND,
                report.scope_version,
                events,
            );
        }
        Ok(report)
    }

    pub fn get_board_order(
        &self,
        caller: &Caller,
    ) -> Result<speclink_protocol::query::BoardOrderResponse, ExecError> {
        use speclink_store_port::{keys, KeyPrefix};
        self.ask(caller, GET_BOARD_ORDER, serde_json::json!({}))?;
        let key = keys::board_order();
        let prefix = KeyPrefix::new(key.as_str()).expect("the board order key is a prefix");
        for _ in 0..3 {
            let before = self
                .store
                .list(&caller.scope, &prefix)
                .map_err(ExecError::Store)?
                .version;
            let docs = self
                .store
                .read(&caller.scope, std::slice::from_ref(&key))
                .map_err(ExecError::Store)?;
            let [doc] = docs.as_slice() else {
                return Err(ExecError::Store(speclink_store_port::PortError::Backend {
                    message: "the store returned the wrong number of board order documents".into(),
                }));
            };
            let after = self
                .store
                .list(&caller.scope, &prefix)
                .map_err(ExecError::Store)?
                .version;
            if before == after {
                return Ok(speclink_protocol::query::BoardOrderResponse {
                    revision: after.get(),
                    content: doc.as_ref().map(|d| d.content.clone()),
                });
            }
        }
        Err(ExecError::Store(
            speclink_store_port::PortError::Unavailable {
                message: "the board order changed while it was read; retry the request".into(),
            },
        ))
    }

    pub fn read_for_hook(&self, caller: &Caller, cmd: Command) -> Result<PortExecution, ExecError> {
        if !matches!(
            cmd,
            Command::List { .. }
                | Command::Show { .. }
                | Command::Status { .. }
                | Command::Instructions { .. }
                | Command::Validate { .. }
                | Command::Analyze { .. }
                | Command::Trace { .. }
                | Command::Plan { .. }
                | Command::PlanStrict
                | Command::ArtifactCat { .. }
                | Command::LanguageShow
                | Command::DiscussList { .. }
                | Command::DiscussShow { .. }
                | Command::DiscussSearch { .. }
                | Command::ReviewShow { .. }
                | Command::VerifyShow { .. }
        ) {
            return Err(ExecError::Command(CommandError::new(
                ErrorCode::InvalidArgv,
                "the hook read function only accepts read commands",
            )));
        }
        let engine_ctx = ExecutionContext {
            actor: Some(caller.identity.display_name().to_string()),
            ..Default::default()
        };
        execute_as(
            self.store.as_ref(),
            &caller.scope,
            &caller.identity.actor,
            &engine_ctx,
            cmd,
        )
    }

    fn ask(
        &self,
        caller: &Caller,
        command: &'static str,
        params: serde_json::Value,
    ) -> Result<(), ExecError> {
        let Some(hook) = &self.hooks.before_command else {
            return Ok(());
        };
        let decision = hook(&BeforeCommand {
            identity: caller.identity.clone(),
            scope: caller.scope.clone(),
            command,
            params,
            changed_keys: None,
        });
        match decision {
            Decision::Allow => Ok(()),
            Decision::Deny { reason } => Err(ExecError::Command(CommandError::new(
                ErrorCode::Refused,
                if reason.trim().is_empty() {
                    NO_REASON.to_owned()
                } else {
                    reason
                },
            ))),
        }
    }

    fn watch(
        &self,
        caller: &Caller,
        command: &'static str,
        scope_version: Version,
        events: Vec<Event>,
    ) {
        if let Some(hook) = &self.hooks.after_commit {
            let info = AfterCommit {
                identity: caller.identity.clone(),
                scope: caller.scope.clone(),
                command,
                scope_version,
                events,
            };
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| hook(&info)));
        }
    }
}

/// A task completion that names no changed files records none. Without this,
/// the Engine probes a local workspace, and with no workspace that is the
/// process's working directory: a host running in a git checkout would record
/// that checkout's dirty files. The hook still sees the params as the caller
/// sent them.
fn without_local_facts(cmd: Command) -> Command {
    match cmd {
        Command::TaskDone {
            task_id,
            change,
            touched_files: None,
            head_commit,
        } => Command::TaskDone {
            task_id,
            change,
            touched_files: Some(Vec::new()),
            head_commit,
        },
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task_done(touched_files: Option<Vec<String>>) -> Command {
        Command::TaskDone {
            task_id: "1".to_string(),
            change: Some("demo".to_string()),
            touched_files,
            head_commit: None,
        }
    }

    #[test]
    fn a_completion_without_files_names_none_instead_of_probing() {
        assert_eq!(
            without_local_facts(task_done(None)),
            task_done(Some(Vec::new()))
        );
        let named = task_done(Some(vec!["src/app.rs".to_string()]));
        assert_eq!(without_local_facts(named.clone()), named);
        assert_eq!(
            without_local_facts(Command::LanguageShow),
            Command::LanguageShow
        );
    }
}
