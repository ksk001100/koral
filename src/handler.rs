use crate::{Context, KoralResult};
use std::any::Any;

/// Marker for handlers that expect (App, Context)
pub struct WithApp;
/// Marker for handlers that expect (Context) but have access to App via context
pub struct WithoutApp; // Historically named
/// Marker for handlers that ignore arguments
pub struct IgnoreApp;

/// Marker for typed handlers (Context<A>)
pub struct TypedApp;

/// Trait representing an action handler
pub trait Handler<A: ?Sized, Marker> {
    /// Execute the handler
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()>;
}

// Handler strictly taking (app, ctx) - Old style or manual style
impl<A, F> Handler<A, WithApp> for F
where
    A: Any,
    F: Fn(&mut A, Context) -> KoralResult<()>,
{
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
        // We do NOT inject app into ctx for WithApp handlers to avoid double mutable borrow.
        // The handler receives `app` explicitly as the first argument.
        (self)(app, ctx)
    }
}

// Handler taking only Context (New Preferred Style - Typed)
impl<A, F> Handler<A, TypedApp> for F
where
    A: Any, // A must be Any because Context<A> expects A: ?Sized, and our App trait impls are sized usually but Any covers it.
    // Actually A doesn't need to be Any for Context<A>, but usually is for dynamic fallback.
    F: Fn(Context<A>) -> KoralResult<()>,
{
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
        // reconstruct typed context
        let typed_ctx = Context {
            flags: ctx.flags,
            args: ctx.args,
            state: ctx.state,
            app: Some(app),
        };
        (self)(typed_ctx)
    }
}

// Handler taking only Context (Old Style - Dynamic)
impl<A, F> Handler<A, WithoutApp> for F
where
    A: Any,
    F: Fn(Context) -> KoralResult<()>,
{
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
        let ctx = ctx.with_app(app);
        (self)(ctx)
    }
}

impl<A, F> Handler<A, IgnoreApp> for F
where
    A: Any,
    F: Fn() -> KoralResult<()>,
{
    fn call(&self, _app: &mut A, _ctx: Context) -> KoralResult<()> {
        (self)()
    }
}

/// Helper to invoke a handler with the correct marker type
pub fn call_handler<A: Any, M, H>(handler: H, app: &mut A, ctx: Context) -> KoralResult<()>
where
    H: Handler<A, M>,
{
    handler.call(app, ctx)
}
