use thiserror::Error;

#[derive(Debug, Error)]
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
    /// Other custom error
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl From<std::io::Error> for KoralError {
    fn from(err: std::io::Error) -> Self {
        KoralError::IoError(err.to_string())
    }
}

/// Result type alias for Koral operations
pub type KoralResult<T> = Result<T, KoralError>;

/// Extension trait for easier error conversion
pub trait KoralResultExt<T> {
    /// Convert any error into a KoralError::Other
    fn koral_err(self) -> KoralResult<T>;
}

impl<T, E> KoralResultExt<T> for Result<T, E>
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn koral_err(self) -> KoralResult<T> {
        self.map_err(|e| KoralError::Other(e.into()))
    }
}
