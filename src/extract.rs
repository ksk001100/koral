use crate::{Context, Flag, KoralResult};
use std::ops::Deref;

/// Trait for extracting data from the context.
pub trait FromContext<'a>: Sized {
    /// Extract data from the context.
    fn from_context(ctx: &'a Context) -> KoralResult<Self>;
}

/// Extractor for positional arguments.
pub struct Args(pub Vec<String>);

impl Deref for Args {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromContext<'a> for Args {
    fn from_context(ctx: &'a Context) -> KoralResult<Self> {
        Ok(Args(ctx.args.clone()))
    }
}

/// Extractor for typed flags.
pub struct FlagVal<F: Flag>(pub F::Value);

impl<F: Flag> Deref for FlagVal<F> {
    type Target = F::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, F: Flag> FromContext<'a> for FlagVal<F>
where
    <F::Value as std::str::FromStr>::Err: std::fmt::Display,
{
    fn from_context(ctx: &'a Context) -> KoralResult<Self> {
        match ctx.get::<F>() {
            Some(v) => Ok(FlagVal(v)),
            None => Err(crate::KoralError::MissingArgument(format!(
                "Flag '{}' not found",
                F::name()
            ))),
        }
    }
}

/// Extractor for shared state.
/// This expects the state to be of type T and T must be Clone.
/// For large state, use Arc<T>.
#[derive(Debug, Clone)]
pub struct State<T>(pub T);

impl<T> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: 'static + Clone> FromContext<'a> for State<T> {
    fn from_context(ctx: &'a Context) -> KoralResult<Self> {
        match ctx.state::<T>() {
            Some(s) => Ok(State(s.clone())),
            None => Err(crate::KoralError::Validation(
                "Shared state not found or type mismatch".to_string(),
            )),
        }
    }
}

impl<'a, T> FromContext<'a> for Option<T>
where
    T: FromContext<'a>,
{
    fn from_context(ctx: &'a Context) -> KoralResult<Self> {
        match T::from_context(ctx) {
            Ok(v) => Ok(Some(v)),
            Err(crate::KoralError::MissingArgument(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flag::Flag;

    struct TestFlag;
    impl Flag for TestFlag {
        type Value = String;
        fn name() -> &'static str {
            "test"
        }
        fn short() -> Option<char> {
            None
        }
        fn takes_value() -> bool {
            true
        }
    }

    #[test]
    fn test_option_extract_present() {
        let mut map = std::collections::HashMap::new();
        map.insert("test".to_string(), Some("val".to_string()));
        let ctx = Context::new(map, vec![]);

        // We need FlagVal<TestFlag> which expects TestFlag to be Flag.
        // And String to be FlagValue (it is).

        let res = Option::<FlagVal<TestFlag>>::from_context(&ctx).unwrap();
        assert!(res.is_some());
        assert_eq!(res.unwrap().0, "val");
    }

    #[test]
    fn test_option_extract_missing() {
        let map = std::collections::HashMap::new();
        let ctx = Context::new(map, vec![]);

        let res = Option::<FlagVal<TestFlag>>::from_context(&ctx).unwrap();
        assert!(res.is_none());
    }
}

/// Extractor for extensions.
pub struct Extension<T>(pub T);

impl<T> Deref for Extension<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Clone + Send + Sync + 'static> FromContext<'a> for Extension<T> {
    fn from_context(ctx: &'a Context) -> KoralResult<Self> {
        match ctx.get_extension::<T>() {
            Some(v) => Ok(Extension(v.clone())),
            None => Err(crate::KoralError::MissingArgument(format!(
                "Extension of type '{}' not found",
                std::any::type_name::<T>()
            ))),
        }
    }
}
