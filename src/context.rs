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
