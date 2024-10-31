use async_trait::async_trait;

use crate::{
    cli::Command,
    convert::{convert_midi_note_number_to_frequency, convert_midi_note_number_to_music_note},
    error::FNoteResult,
    parse::try_midi_note_number_from_str,
};

/// Extracts the frequency and music note from a MIDI note number (0-127).
#[derive(structopt::StructOpt)]
pub struct MidiCommand {
    /// The MIDI note number to use.
    #[structopt(parse(try_from_str = try_midi_note_number_from_str))]
    pub midi_note_number: u8,
}

#[async_trait]
impl Command for MidiCommand {
    async fn run(&self) -> FNoteResult<()> {
        println!("MIDI: {}", self.midi_note_number);
        println!(
            "Note: {}",
            convert_midi_note_number_to_music_note(self.midi_note_number).unwrap()
        );
        println!(
            "Frequency: {}Hz",
            convert_midi_note_number_to_frequency(self.midi_note_number)
        );

        Ok(())
    }
}
