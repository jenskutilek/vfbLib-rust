pub use error_stack::Report;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VfbError {
    #[error("Error opening file: {0}")]
    FileOpenError(std::io::Error),
    #[error("Error reading from file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Bad value: {0}, expected {1}")]
    BadValue(String, String),
    #[error("Value out of range: {0}")]
    Overflow(u32),
    #[error("Invalid UTF-8 sequence: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),
    #[error("Attempted to decompile an entry {0} that has no data")]
    UninitializedEntry(String),
    #[error("Invalid glyph header data: {0:?}")]
    InvalidGlyphHeader(Vec<u8>),
}

/// A helper context for building error messages
#[derive(Debug)]
pub struct ReadContext(pub String);

impl fmt::Display for ReadContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while {}", self.0)
    }
}
