use crate::{extract::FromContext, Context, KoralResult};
use std::any::Any;

/// Trait representing an action handler
pub trait Handler<A: ?Sized, Args> {
    /// Execute the handler
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()>;
}

// ===========================================================================
// Legacy / Manual Handlers
// ===========================================================================

/// Marker for legacy handlers taking (App, Context)
pub struct Legacy;

impl<A, F> Handler<A, Legacy> for F
where
    A: Any,
    F: Fn(&mut A, Context) -> KoralResult<()>,
{
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
        (self)(app, ctx)
    }
}

/// Marker for typed handlers (Context<A>)
pub struct LegacyTyped;

impl<A, F> Handler<A, LegacyTyped> for F
where
    A: Any,
    F: Fn(Context<A>) -> KoralResult<()>,
{
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
        // Reconstruct typed context
        // We cannot use ctx.with_app(app) because it returns Context<dyn Any> (since ctx is Context<dyn Any>)
        // unless we change Context definition of with_app.
        // Easier to manually construct.
        let typed_ctx = Context {
            flags: ctx.flags,
            args: ctx.args,
            state: ctx.state,
            app: Some(app),
        };
        (self)(typed_ctx)
    }
}

/// Marker for dynamic handlers (Context)
pub struct LegacyDyn;

impl<A, F> Handler<A, LegacyDyn> for F
where
    A: Any,
    F: Fn(Context) -> KoralResult<()>,
{
    fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
        // Inject app into context (dynamic)
        // Check if F expects Context<dyn Any> (default Context).
        // ctx is already Context (dyn Any) effectively?
        // But `with_app` converts `Context<T>` to `Context<U>`.
        // Wait, `with_app` takes `&mut U`.
        // If F expects `Context` (default), it expects `Context<dyn Any>`.
        // `app` is `&mut A`.
        // We can cast `app` to `&mut dyn Any`?
        // `ctx.with_app(app)` returns `Context<A>`.
        // So `LegacyTyped` covers `Context<A>`.
        // `LegacyDyn` covers `Context` (i.e. `Context<dyn Any>`).
        // For `LegacyDyn`, we want `Context` that holds `app` as `dyn Any`?
        // `ctx.app` is `Option<&mut dyn Any>`? No (`Context` definition).
        // Struct Context<'a, A: ?Sized = dyn Any>. `app: Option<&'a mut A>`.
        // So `Context` (default) has `app: Option<&mut dyn Any>`.
        // `ctx` passed to `call` is `Context` (default, `A=dyn Any`).
        // It has `app: None` initially (from `run`).
        // We want to set `app`.
        // `app` arg is `&mut A`.
        // We need `&mut dyn Any` to put into `Context<dyn Any>`.
        // `A: Any`. So we can cast?
        // `let any_app: &mut dyn Any = app;`
        // `ctx.with_app(any_app)` -> returns `Context<dyn Any>`.
        let any_app: &mut dyn Any = app;
        let dyn_ctx = ctx.with_app(any_app);
        (self)(dyn_ctx)
    }
}

// ===========================================================================
// DI Handlers (Axum-style)
// ===========================================================================

macro_rules! impl_handler {
    ( $($ty:ident),* ) => {
        #[allow(non_snake_case, unused_mut)]
        impl<A, F, $($ty,)*> Handler<A, ($($ty,)*)> for F
        where
            A: Any,
            F: Fn($($ty,)*) -> KoralResult<()>,
            $($ty: for<'a> FromContext<'a>,)*
        {
            fn call(&self, app: &mut A, ctx: Context) -> KoralResult<()> {
                // We must perform extraction.
                // Since extractors need `&Context`, we construct the context first.
                // But `call` receives `ctx` owned.
                // We can borrow it.
                // And we must inject `app` into it if needed?
                // Context returned by `parser` doesn't have `app` set yet?
                // `run_with_state` sets state.
                // `run` doesn't set app generally unless we do it in generated code?
                // `Context` has `with_app`.
                // But `app` is `&mut A`. `Handler` receives `&mut A`.
                // So we can:
                // 1. Create a logical context with app injected.
                // 2. Extract arguments from it.

                // Note: The `ctx` passed to `call` consumes the one created in `run`.
                // It holds `flags`, `args`, `state`.
                // We create a temporary borrow scope for extraction?
                // Or we update `ctx` with `app`?
                // `ctx.with_app(app)` consumes `ctx` and returns new `Context`.
                // Can we extract from it?
                // `from_context` takes `&Context`.
                // Yes.

                // However, `Context` has lifetime `'a`.
                // `Handler::call` takes `ctx: Context`.
                // `Context` inside generic `impl` doesn't name lifetime.
                // It means `Context<'static>` or similar?
                // `Context` definition is `pub struct Context<'a, A: ?Sized = dyn Any>`.
                // If we use plain `Context`, it defaults to `Context<'static, dyn Any>` if defaults used?
                // No, lifetime defaults.
                // `Handler` trait definition: `fn call(&self, app: &mut A, ctx: Context)`.
                // This implies `Context` must be valid here.
                // `extract::FromContext` takes `&'a Context`.
                // We need to match lifetimes.
                // `for<'a> FromContext<'a>` bound handles this!

                // Only issue: `app` reference lifetime.
                // `Context` created in `run` doesn't have `app`.
                // We want to add it?
                // `ctx` argument owns the data.
                // We can construct a combined context.

                #[allow(unused_variables)]
                let ctx_with_app = ctx.with_app(app);

                // Now extract
                $(
                    let $ty = $ty::from_context(&ctx_with_app)?;
                )*

                (self)($($ty,)*)
            }
        }
    };
}

impl_handler!();
impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);
impl_handler!(T1, T2, T3, T4);
impl_handler!(T1, T2, T3, T4, T5);
impl_handler!(T1, T2, T3, T4, T5, T6);
impl_handler!(T1, T2, T3, T4, T5, T6, T7);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);

/// Helper to invoke a handler with the correct marker type
pub fn call_handler<A: Any, M, H>(handler: H, app: &mut A, ctx: Context) -> KoralResult<()>
where
    H: Handler<A, M>,
{
    handler.call(app, ctx)
}
