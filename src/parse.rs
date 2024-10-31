use regex::Regex;

use crate::error::{FNoteError, FNoteResult};

/// Parses a frequency from a string.
pub(crate) fn try_frequency_from_str(arg: &str) -> FNoteResult<f32> {
    let regex = Regex::new(r"^\d+(\.\d+)?$").unwrap();
    if regex.is_match(arg) {
        Ok(arg.parse::<f32>().unwrap())
    } else {
        Err(FNoteError::InvalidFrequency(arg.to_string()))
    }
}

/// Parses a MIDI note number from a string.
pub(crate) fn try_midi_note_number_from_str(arg: &str) -> FNoteResult<u8> {
    let regex = Regex::new(r"^(?:\d|[1-9]\d|1[01]\d|12[0-7])$").unwrap();
    if regex.is_match(arg) {
        Ok(arg.parse::<u8>().unwrap())
    } else {
        Err(FNoteError::InvalidMidiNoteNumber(arg.to_string()))
    }
}

/// Parses a music note from a string.
pub(crate) fn try_music_note_from_str(arg: &str) -> FNoteResult<String> {
    let regex = Regex::new(r"^[A-Ga-g][#b]?(-1|[0-9])$").unwrap();
    if regex.is_match(arg) {
        let mut chars = arg.chars();
        let mut result = String::new();

        if let Some(note_char) = chars.next() {
            result.push(note_char.to_ascii_uppercase());
        }

        result.push_str(chars.as_str());

        Ok(result)
    } else {
        Err(FNoteError::InvalidMusicNote(arg.to_string()))
    }
}
