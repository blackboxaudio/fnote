/// Generic result type when using `fnote`.
pub type FNoteResult<T> = std::result::Result<T, FNoteError>;

/// Error variants for the `fnote` program.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum FNoteError {
    #[error("Unknown command")]
    UnknownCommand,

    #[error("Invalid frequency \"{0}\"")]
    InvalidFrequency(String),

    #[error("Invalid MIDI note number \"{0}\"")]
    InvalidMidiNoteNumber(String),

    #[error("Invalid music note \"{0}\"")]
    InvalidMusicNote(String),
}
