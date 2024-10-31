use async_trait::async_trait;
use regex::Regex;

use crate::{
    cli::Command,
    error::{FNoteError, FNoteResult},
};

/// Extracts the frequency and music note from a MIDI note number (0-127).
#[derive(structopt::StructOpt)]
pub struct MidiCommand {
    /// The MIDI note number to use.
    #[structopt(parse(try_from_str = try_midi_note_number_from_str))]
    pub midi_note_number: usize,
}

#[async_trait]
impl Command for MidiCommand {
    async fn run(&self) -> FNoteResult<()> {
        println!("Converting MIDI note number: {:?}", self.midi_note_number);

        Ok(())
    }
}

/// Parses a MIDI note number from a string.
fn try_midi_note_number_from_str(arg: &str) -> FNoteResult<usize> {
    let regex = Regex::new(r"^(?:\d|[1-9]\d|1[01]\d|12[0-7])$").unwrap();
    if regex.is_match(arg) {
        Ok(arg.parse::<usize>().unwrap())
    } else {
        Err(FNoteError::InvalidMidiNoteNumber(arg.to_string()))
    }
}
