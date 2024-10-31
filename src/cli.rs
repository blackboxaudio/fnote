use async_trait::async_trait;

use crate::{commands::midi::MidiCommand, error::FNoteResult};

/// Represents an `fnote` program command.
#[async_trait]
pub trait Command {
    /// Executes the corresponding command.
    async fn run(&self) -> FNoteResult<()>;
}

/// Commands supported by `fnote`.
#[derive(structopt::StructOpt)]
#[structopt(
    author = "Matthew Maxwell",
    version = env!("CARGO_PKG_VERSION"),
)]
pub enum FNoteCommand {
    /// Extracts the frequency and musical note from a MIDI note number (0-127).
    Midi(MidiCommand),
}

#[async_trait]
impl Command for FNoteCommand {
    async fn run(&self) -> FNoteResult<()> {
        match self {
            Self::Midi(cmd) => cmd.run().await,
        }
    }
}
