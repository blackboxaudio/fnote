use crate::error::{FNoteError, FNoteResult};

/// Converts a frequency to the nearest MIDI note number.
pub(crate) fn frequency_to_midi_note(frequency: f32) -> Option<u8> {
    let midi_note = 69.0 + 12.0 * (frequency / 440.0).log2();
    if (0..=127).contains(&(midi_note.round() as i32)) {
        Some(midi_note.round() as u8)
    } else {
        None
    }
}

/// Converts a MIDI note number to a frequency (Hz).
pub(crate) fn midi_note_number_to_frequency(midi_note_number: u8) -> f32 {
    let frequency = 440.0 * 2.0f32.powf((midi_note_number as f32 - 69.0) / 12.0);
    (frequency * 100.0).round() / 100.0
}

/// Converts a MIDI note number to a music note.
pub(crate) fn midi_note_number_to_music_note(midi_note_number: u8) -> Option<String> {
    let semitone_to_note = ["C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];

    let octave = (midi_note_number / 12) as i8 - 1;
    let semitone_offset = (midi_note_number % 12) as usize;
    let note = semitone_to_note.get(semitone_offset).unwrap();

    Some(format!("{}{}", note, octave))
}

/// Converts a music note to a MIDI note number.
pub(crate) fn music_note_to_midi_note_number(music_note: &str) -> FNoteResult<u8> {
    let note_to_semitone = [
        ("Cb", -1), // -1 instead of 11 because Cb = B, meaning decrement octave by 1
        ("C", 0),
        ("C#", 1),
        ("Db", 1),
        ("D", 2),
        ("D#", 3),
        ("Eb", 3),
        ("E", 4),
        ("Fb", 4),
        ("F", 5),
        ("E#", 5),
        ("F#", 6),
        ("Gb", 6),
        ("G", 7),
        ("G#", 8),
        ("Ab", 8),
        ("A", 9),
        ("A#", 10),
        ("Bb", 10),
        ("B", 11),
        ("B#", 12), // 12 instead of 0 because B# = C, meaning increment octave by 1
    ]
    .iter()
    .cloned()
    .collect::<std::collections::HashMap<_, _>>();

    if music_note.len() < 2 || music_note.len() > 4 {
        Err(FNoteError::InvalidMusicNote(music_note.to_string()))
    } else {
        let base_note = if music_note.chars().nth(1) == Some('#') || music_note.chars().nth(1) == Some('b') {
            &music_note[..2]
        } else {
            &music_note[..1]
        };

        let octave_str = &music_note[base_note.len()..];
        let octave: i8 = octave_str.parse().unwrap();
        if !(-1..=9).contains(&octave) {
            return Err(FNoteError::InvalidMusicNote(music_note.to_string()));
        }

        let semitone_offset = *note_to_semitone.get(base_note).unwrap();
        let midi_number = ((octave + 1) * 12 + semitone_offset as i8) as i16;
        if (0..=127).contains(&midi_number) {
            Ok(midi_number as u8)
        } else {
            Err(FNoteError::InvalidMusicNote(music_note.to_string()))
        }
    }
}
