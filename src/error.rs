/// core error type for Koral
pub type KoralError = clap::Error;

/// Result type alias for Koral operations
pub type KoralResult<T> = Result<T, KoralError>;

/// Extension trait for easier error conversion
pub trait KoralResultExt<T> {
    /// Convert any error into a KoralResult
    fn koral_err(self) -> KoralResult<T>;
}

impl<T, E> KoralResultExt<T> for Result<T, E>
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn koral_err(self) -> KoralResult<T> {
        self.map_err(|e| {
            clap::Error::raw(
                clap::error::ErrorKind::Io, // Generic Kind for external errors
                format!("Error: {}", e.into()),
            )
        })
    }
}
