use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// core error type for Koral
pub enum KoralError {
    /// Error parsing a flag value
    #[error("Flag parse error: {0}")]
    FlagValueParseError(String),
    /// Required argument is missing
    #[error("Missing argument: {0}")]
    MissingArgument(String),
    /// Invalid flag or command specified
    #[error("Invalid flag/command: {0}")]
    InvalidFlag(String),
    /// Unknown flag encountered
    #[error("Unknown flag: {0}")]
    UnknownFlag(String),
    /// General validation error
    #[error("Validation error: {0}")]
    Validation(String),
    /// IO error
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for KoralError {
    fn from(err: std::io::Error) -> Self {
        KoralError::IoError(err.to_string())
    }
}

/// Result type alias for Koral operations
pub type KoralResult<T> = Result<T, KoralError>;
