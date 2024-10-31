use async_trait::async_trait;

use crate::{
    commands::{freq::FreqCommand, midi::MidiCommand, note::NoteCommand},
    error::FNoteResult,
};

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
    /// Extracts the frequency and music note from a MIDI note number (0-127).
    Midi(MidiCommand),

    /// Extracts the frequency and MIDI note number from a music note (e.g. C5).
    Note(NoteCommand),

    /// Extracts the (nearest) MIDI note number and music note from a frequency (e.g. 440).
    Freq(FreqCommand),
}

#[async_trait]
impl Command for FNoteCommand {
    async fn run(&self) -> FNoteResult<()> {
        match self {
            Self::Midi(cmd) => cmd.run().await,
            Self::Note(cmd) => cmd.run().await,
            Self::Freq(cmd) => cmd.run().await,
        }
    }
}
