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
    pub midi_note_number: u8,
}

#[async_trait]
impl Command for MidiCommand {
    async fn run(&self) -> FNoteResult<()> {
        println!("MIDI: {}", self.midi_note_number);
        println!(
            "Note: {}",
            midi_note_number_to_music_note(self.midi_note_number).unwrap()
        );
        println!("Frequency: {} Hz", midi_note_number_to_frequency(self.midi_note_number));

        Ok(())
    }
}

/// Converts a MIDI note number to a music note.
pub(crate) fn midi_note_number_to_music_note(midi_note_number: u8) -> Option<String> {
    let semitone_to_note = ["C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];

    let octave = (midi_note_number / 12) as i8 - 1;
    let semitone_offset = (midi_note_number % 12) as usize;
    let note = semitone_to_note.get(semitone_offset).unwrap();

    Some(format!("{}{}", note, octave))
}

/// Converts a MIDI note number to a frequency (Hz).
pub(crate) fn midi_note_number_to_frequency(midi_note_number: u8) -> f32 {
    let frequency = 440.0 * 2.0f32.powf((midi_note_number as f32 - 69.0) / 12.0);
    (frequency * 100.0).round() / 100.0
}

/// Parses a MIDI note number from a string.
fn try_midi_note_number_from_str(arg: &str) -> FNoteResult<u8> {
    let regex = Regex::new(r"^(?:\d|[1-9]\d|1[01]\d|12[0-7])$").unwrap();
    if regex.is_match(arg) {
        Ok(arg.parse::<u8>().unwrap())
    } else {
        Err(FNoteError::InvalidMidiNoteNumber(arg.to_string()))
    }
}
