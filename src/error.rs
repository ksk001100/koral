use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KoralError {
    FlagValueParseError(String),
    MissingArgument(String),
    UnknownFlag(String),
    Validation(String),
}

impl fmt::Display for KoralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KoralError::FlagValueParseError(msg) => write!(f, "Flag parse error: {}", msg),
            KoralError::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
            KoralError::UnknownFlag(msg) => write!(f, "Unknown flag: {}", msg),
            KoralError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for KoralError {}

pub type KoralResult<T> = Result<T, KoralError>;
