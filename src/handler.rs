use crate::{Context, KoralResult};

pub struct WithApp;
pub struct WithoutApp;

pub trait Handler<A, Marker> {
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()>;
}

impl<A, F> Handler<A, WithApp> for F
where
    F: Fn(&mut A, Context) -> KoralResult<()>,
{
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
        (self)(app, ctx)
    }
}

impl<A, F> Handler<A, WithoutApp> for F
where
    F: Fn(Context) -> KoralResult<()>,
{
    fn call(&self, _app: &mut A, ctx: Context) -> KoralResult<()> {
        (self)(ctx)
    }
}

pub fn call_handler<A, M, H>(handler: H, app: &mut A, ctx: Context) -> KoralResult<()>
where
    H: Handler<A, M>,
{
    handler.call(app, ctx)
}
