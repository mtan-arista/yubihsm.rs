use std::fmt;

/// `Algorithm`-related errors
pub type Error = crate::Error<ErrorKind>;

/// Kinds of `Algorithm`-related errors
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ErrorKind {
    /// Invalid algorithm tag
    TagInvalid,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ErrorKind::TagInvalid => "invalid tag",
        })
    }
}
