use std::collections::HashMap;

use crate::{flag, traits::Flag};

#[derive(Debug, Clone)]
pub struct Context {
    pub args: Vec<String>,
    pub flags: HashMap<String, Option<<crate::flag::Flag as crate::traits::Flag>::Value>>,
}

impl Context {
    pub fn new<F: Flag<Kind = flag::FlagKind, Value = flag::FlagValue>>(
        args: Vec<String>,
        flags: Vec<F>,
    ) -> Self {
        let mut hash = HashMap::new();
        for flag in flags {
            hash.insert(flag.clone().name(), flag.clone().value(&args));
        }
        Context { args, flags: hash }
    }

    pub fn bool_flag<T: Into<String>>(&self, name: T) -> bool {
        match self.flags.get(&name.into()).unwrap() {
            Some(flag::FlagValue::Boolean(b)) => *b,
            _ => false,
        }
    }

    pub fn value_flag<T: Into<String>>(&self, name: T) -> Option<String> {
        match self.flags.get(&name.into()).unwrap() {
            Some(flag::FlagValue::Value(s)) => Some(s.to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flag::{Flag, FlagKind};

    #[test]
    fn test_context() {
        let args = vec![
            "test".to_string(),
            "--flag".to_string(),
            "value".to_string(),
            "--bool".to_string(),
        ];
        let ctx = Context::new(
            args.clone(),
            vec![
                Flag::new("flag", FlagKind::Value),
                Flag::new("bool", FlagKind::Boolean),
            ],
        );
        assert_eq!(ctx.value_flag("flag"), Some("value".to_string()));
        assert_eq!(ctx.bool_flag("bool"), true);
    }
}
