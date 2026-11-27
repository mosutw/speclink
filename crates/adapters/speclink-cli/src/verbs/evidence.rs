//! `evidence show <change>` — read task evidence for an active or archived change.

use anyhow::Result;
use speclink_core as core;
use speclink_verbs::families::evidence::{EvidenceArgs, EvidenceCommands};
use speclink_verbs::render::evidence as render;

use crate::common::{emit, open_project, plain_command, render_options, run};

pub(crate) fn cmd_evidence(a: EvidenceArgs) -> Result<()> {
    let (_ws, store) = open_project()?;
    let json = match a.command {
        EvidenceCommands::Show { json, .. } => json,
    };
    let o: core::command::EvidenceShowOutcome = run(&store, None, plain_command(&a)?)?;
    emit(render::show(&o, render_options(json)));
    Ok(())
}
