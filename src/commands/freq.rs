use async_trait::async_trait;
use regex::Regex;

use crate::{
    cli::Command,
    commands::midi::{midi_note_number_to_frequency, midi_note_number_to_music_note},
    error::{FNoteError, FNoteResult},
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
        let midi_note_number = frequency_to_midi_note(self.frequency).unwrap();
        let music_note = midi_note_number_to_music_note(midi_note_number).unwrap();

        let frequency = midi_note_number_to_frequency(midi_note_number);
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

/// Converts a frequency to the nearest MIDI note number.
pub(crate) fn frequency_to_midi_note(frequency: f32) -> Option<u8> {
    let midi_note = 69.0 + 12.0 * (frequency / 440.0).log2();
    if (0..=127).contains(&(midi_note as u8)) {
        Some(midi_note as u8)
    } else {
        None
    }
}

fn try_frequency_from_str(arg: &str) -> FNoteResult<f32> {
    let regex = Regex::new(r"^\d+(\.\d+)?$").unwrap();
    if regex.is_match(arg) {
        Ok(arg.parse::<f32>().unwrap())
    } else {
        Err(FNoteError::InvalidFrequency(arg.to_string()))
    }
}
