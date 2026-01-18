use crate::{Context, KoralResult};
use std::any::Any;

pub struct WithApp;
pub struct WithoutApp;
pub struct IgnoreApp;

pub struct TypedApp;

pub trait Handler<A: ?Sized, Marker> {
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

pub fn call_handler<A: Any, M, H>(handler: H, app: &mut A, ctx: Context) -> KoralResult<()>
where
    H: Handler<A, M>,
{
    handler.call(app, ctx)
}
