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
    pub music_note: String,
}

#[async_trait]
impl Command for NoteCommand {
    async fn run(&self) -> FNoteResult<()> {
        println!("Note: {}", self.music_note);
        println!("MIDI: {}", music_note_to_midi_note_number(self.music_note.as_str())?);

        Ok(())
    }
}

// TODO: Doesn't work inputs like b#-1, which is 0 but should be 11. Also doesn't work for inputs greater than G9.
/// Converts a music note to a MIDI note number.
fn music_note_to_midi_note_number(music_note: &str) -> FNoteResult<u8> {
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

/// Parses a music note from a string.
fn try_music_note_from_str(arg: &str) -> FNoteResult<String> {
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
