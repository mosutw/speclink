//! Each engine command's stable name and serializable params (spec
//! command-runtime: 引擎命令有穩定的名稱與可序列化的參數). Hosts see both in
//! their hooks and the store's commit record carries the name, so the two are a
//! contract: `tests/golden/command-describe.json` pins them per variant.

use super::Command;

impl Command {
    /// The command's kebab-case name: the variant name, fixed per kind and
    /// independent of the params. Matched with no wildcard arm, so a new
    /// variant does not compile until it is named here.
    pub fn name(&self) -> &'static str {
        match self {
            Command::List { .. } => "list",
            Command::Show { .. } => "show",
            Command::Status { .. } => "status",
            Command::Instructions { .. } => "instructions",
            Command::Validate { .. } => "validate",
            Command::Analyze { .. } => "analyze",
            Command::Trace { .. } => "trace",
            Command::Plan { .. } => "plan",
            Command::PlanStrict => "plan-strict",
            Command::ArtifactCat { .. } => "artifact-cat",
            Command::LanguageShow => "language-show",
            Command::DiscussList { .. } => "discuss-list",
            Command::DiscussShow { .. } => "discuss-show",
            Command::DiscussSearch { .. } => "discuss-search",
            Command::NewChange { .. } => "new-change",
            Command::NewArtifact { .. } => "new-artifact",
            Command::TaskDone { .. } => "task-done",
            Command::TaskUndone { .. } => "task-undone",
            Command::TaskMove { .. } => "task-move",
            Command::Claim { .. } => "claim",
            Command::InProgressAdd { .. } => "in-progress-add",
            Command::InProgressRemove { .. } => "in-progress-remove",
            Command::ChangeDepends { .. } => "change-depends",
            Command::ChangeSchema { .. } => "change-schema",
            Command::ChangeFieldsEdit { .. } => "change-fields-edit",
            Command::Archive { .. } => "archive",
            Command::Discard { .. } => "discard",
            Command::DiscussNew { .. } => "discuss-new",
            Command::DiscussContext { .. } => "discuss-context",
            Command::DiscussAddRound { .. } => "discuss-add-round",
            Command::DiscussConclude { .. } => "discuss-conclude",
            Command::DiscussPromote { .. } => "discuss-promote",
            Command::DiscussLink { .. } => "discuss-link",
            Command::DiscussSeal { .. } => "discuss-seal",
            Command::DiscussArchive { .. } => "discuss-archive",
            Command::DiscussDiscard { .. } => "discuss-discard",
            Command::DiscussFieldsEdit { .. } => "discuss-fields-edit",
            Command::ReviewAddRound { .. } => "review-add-round",
            Command::ReviewShow { .. } => "review-show",
            Command::ReviewStamp { .. } => "review-stamp",
            Command::ReviewDiscard { .. } => "review-discard",
            Command::VerifyAddRound { .. } => "verify-add-round",
            Command::VerifyShow { .. } => "verify-show",
            Command::VerifyStamp { .. } => "verify-stamp",
            Command::VerifyDiscard { .. } => "verify-discard",
        }
    }

    /// The command's params as a JSON object: one camelCase key per field,
    /// valued as the field serializes. A variant with no fields gives `{}`.
    pub fn params(&self) -> serde_json::Value {
        // Externally tagged: a struct variant is `{ "<Variant>": { fields } }`,
        // a unit variant its bare name.
        match serde_json::to_value(self).expect("command fields serialize to JSON") {
            serde_json::Value::Object(tagged) => tagged
                .into_iter()
                .next()
                .map(|(_, fields)| fields)
                .unwrap_or_else(empty),
            _ => empty(),
        }
    }
}

fn empty() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

pub const COMMAND_NAMES: &[&str] = &[
    "list",
    "show",
    "status",
    "instructions",
    "validate",
    "analyze",
    "trace",
    "plan",
    "plan-strict",
    "artifact-cat",
    "language-show",
    "discuss-list",
    "discuss-show",
    "discuss-search",
    "new-change",
    "new-artifact",
    "task-done",
    "task-undone",
    "task-move",
    "claim",
    "in-progress-add",
    "in-progress-remove",
    "change-depends",
    "change-schema",
    "change-fields-edit",
    "archive",
    "discard",
    "discuss-new",
    "discuss-context",
    "discuss-add-round",
    "discuss-conclude",
    "discuss-promote",
    "discuss-link",
    "discuss-seal",
    "discuss-archive",
    "discuss-discard",
    "discuss-fields-edit",
    "review-add-round",
    "review-show",
    "review-stamp",
    "review-discard",
    "verify-add-round",
    "verify-show",
    "verify-stamp",
    "verify-discard",
];
