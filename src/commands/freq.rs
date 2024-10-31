use async_trait::async_trait;

use crate::{
    cli::Command,
    convert::{
        convert_frequency_to_midi_note, convert_midi_note_number_to_frequency, convert_midi_note_number_to_music_note,
    },
    error::FNoteResult,
    parse::try_frequency_from_str,
};

/// Extracts the (nearest) MIDI note number and music note from a frequency.
#[derive(structopt::StructOpt)]
pub struct FreqCommand {
    /// The frequency to use.
    #[structopt(parse(try_from_str = try_frequency_from_str))]
    pub frequency: f32,
}

#[async_trait]
impl Command for FreqCommand {
    async fn run(&self) -> FNoteResult<()> {
        let midi_note_number = convert_frequency_to_midi_note(self.frequency).unwrap();
        let music_note = convert_midi_note_number_to_music_note(midi_note_number).unwrap();

        let frequency = convert_midi_note_number_to_frequency(midi_note_number);
        let difference = ((self.frequency - frequency) * 100.0).round() / 100.0;
        let sign = if difference >= 0.0 { "+" } else { "-" };
        let formatted_difference = if difference == 0.0 {
            "".to_string()
        } else {
            format!(" ({}{}Hz)", sign, difference.abs())
        };

        println!("Frequency: {}Hz{}", frequency, formatted_difference);
        println!("Note: {}", music_note);
        println!("MIDI: {}", midi_note_number);

        Ok(())
    }
}
