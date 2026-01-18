use crate::{Context, KoralResult};

pub struct WithApp;
pub struct WithoutApp;
pub struct IgnoreApp;

pub trait Handler<A: ?Sized, Marker> {
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()>;
}

// Handler strictly taking (app, ctx) - Old style or manual style
impl<A, F> Handler<A, WithApp> for F
where
    A: ?Sized,
    F: Fn(&mut A, Context<A>) -> KoralResult<()>,
{
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
        // Convert Context<()> to Context<A> but keep app as None (because it's passed separately)
        let ctx = Context {
            flags: ctx.flags,
            args: ctx.args,
            app: None,
        };
        (self)(app, ctx)
    }
}

// Handler taking only Context<A> (New Preferred Style)
impl<A, F> Handler<A, WithoutApp> for F
where
    A: ?Sized,
    F: Fn(Context<A>) -> KoralResult<()>,
{
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
        let ctx = ctx.with_app(app);
        (self)(ctx)
    }
}

// Handler taking Context<()> (Legacy Style)
impl<A, F> Handler<A, IgnoreApp> for F
where
    A: ?Sized,
    F: Fn(Context) -> KoralResult<()>,
{
    fn call(&self, _app: &mut A, ctx: Context) -> KoralResult<()> {
        (self)(ctx)
    }
}

pub fn call_handler<A: ?Sized, M, H>(handler: H, app: &mut A, ctx: Context) -> KoralResult<()>
where
    H: Handler<A, M>,
{
    handler.call(app, ctx)
}
