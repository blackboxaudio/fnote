/// Generic result type when using `fnote`.
pub type FNoteResult<T> = std::result::Result<T, FNoteError>;

/// Error variants for the `fnote` program.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum FNoteError {

}
