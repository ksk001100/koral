use crate::{Context, KoralResult};
use std::any::Any;

pub struct WithApp;
pub struct WithoutApp;
pub struct IgnoreApp;

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

// Handler taking only Context (New Preferred Style)
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
