//! `evidence show <change>` — read task evidence for an active or archived change.

use clap::{Args, Subcommand};
use speclink_core::command::{Command, CommandError};

use crate::{CommandTarget, ToCommand, VerbInputs};

#[derive(Args, Debug)]
pub struct EvidenceArgs {
    #[command(subcommand)]
    pub command: EvidenceCommands,
}

#[derive(Subcommand, Debug)]
pub enum EvidenceCommands {
    /// Show task evidence recorded for a change (active or archived)
    Show {
        /// Change name
        change: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

impl EvidenceArgs {
    pub fn name(&self) -> &'static str {
        match &self.command {
            EvidenceCommands::Show { .. } => "evidence show",
        }
    }
}

impl ToCommand for EvidenceArgs {
    fn to_command(&self, _inputs: VerbInputs) -> Result<CommandTarget, CommandError> {
        Ok(CommandTarget::Command(match &self.command {
            EvidenceCommands::Show { change, .. } => Command::EvidenceShow { change: change.clone() },
        }))
    }
}
