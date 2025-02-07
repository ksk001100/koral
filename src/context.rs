use std::collections::HashMap;

use crate::flag::FlagValue;

#[derive(Debug, Clone)]
pub struct Context {
    pub args: Vec<String>,
    pub flags: HashMap<String, Option<<crate::flag::Flag as crate::traits::Flag>::Value>>,
}

impl Context {
    pub fn new(app: &dyn crate::traits::App, args: Vec<String>) -> Self {
        use crate::traits::Flag;

        let mut flags = HashMap::new();
        for flag in app.flags() {
            flags.insert(flag.clone().name(), flag.clone().value(&args));
        }
        Context { args, flags }
    }

    pub fn bool_flag<T: Into<String>>(&self, name: T) -> bool {
        match self.flags.get(&name.into()).unwrap() {
            Some(FlagValue::Boolean(b)) => *b,
            _ => false,
        }
    }

    pub fn value_flag<T: Into<String>>(&self, name: T) -> Option<String> {
        match self.flags.get(&name.into()).unwrap() {
            Some(FlagValue::Value(s)) => Some(s.to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use crate::flag::{Flag, FlagKind};

    #[test]
    fn test_context() {
        let app = App::new("test")
            .flag(Flag::new("flag", FlagKind::Value))
            .flag(Flag::new("bool", FlagKind::Boolean));
        let ctx = Context::new(
            &app,
            vec![
                "test".to_string(),
                "--flag".to_string(),
                "value".to_string(),
                "--bool".to_string(),
            ],
        );
        assert_eq!(ctx.value_flag("flag"), Some("value".to_string()));
        assert_eq!(ctx.bool_flag("bool"), true);
    }
}
