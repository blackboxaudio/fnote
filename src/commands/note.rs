use async_trait::async_trait;

use crate::{
    cli::Command,
    convert::{midi_note_number_to_frequency, music_note_to_midi_note_number},
    error::FNoteResult,
    parse::try_music_note_from_str,
};

/// Extracts the frequency and MIDI note number from a music note (e.g. C5).
#[derive(structopt::StructOpt)]
pub struct NoteCommand {
    /// The music note to use.
    #[structopt(parse(try_from_str = try_music_note_from_str))]
    pub music_note: String,
}

#[async_trait]
impl Command for NoteCommand {
    async fn run(&self) -> FNoteResult<()> {
        let midi_note_number = music_note_to_midi_note_number(self.music_note.as_str())?;
        let frequency = midi_note_number_to_frequency(midi_note_number);

        println!("Note: {}", self.music_note);
        println!("MIDI: {}", midi_note_number);
        println!("Frequency: {}Hz", frequency);

        Ok(())
    }
}
