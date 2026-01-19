use crate::context::Context;
use crate::error::KoralResult;

/// Trait for defining middleware hooks that run before and after command execution.
///
/// Middleware can be used for logging, authentication, state setup, etc.
pub trait Middleware: Send + Sync {
    /// Executed before the command handler.
    ///
    /// If this returns an error, the command execution is aborted.
    fn before(&self, _ctx: &mut Context) -> KoralResult<()> {
        Ok(())
    }

    /// Executed after the command handler.
    fn after(&self, _ctx: &mut Context) -> KoralResult<()> {
        Ok(())
    }
}
