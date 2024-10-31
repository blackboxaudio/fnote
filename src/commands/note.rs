use async_trait::async_trait;
use regex::Regex;

use crate::{
    cli::Command,
    error::{FNoteError, FNoteResult},
};

/// Extracts the frequency and MIDI note number from a music note (e.g. C5).
#[derive(structopt::StructOpt)]
pub struct NoteCommand {
    /// The music note to use.
    #[structopt(parse(try_from_str = try_music_note_from_str))]
    pub musical_note: String,
}

#[async_trait]
impl Command for NoteCommand {
    async fn run(&self) -> FNoteResult<()> {
        println!("Converting music note: {:?}", self.musical_note);

        Ok(())
    }
}

/// Parses a music note from a string.
fn try_music_note_from_str(arg: &str) -> FNoteResult<String> {
    let regex = Regex::new(r"^[A-Ga-g][#b]?[0-9]$").unwrap();
    if regex.is_match(arg) {
        Ok(arg.to_string())
    } else {
        Err(FNoteError::InvalidMusicNote(arg.to_string()))
    }
}
