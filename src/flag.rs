use crate::traits::Flag as FlagTrait;

#[derive(Clone)]
pub struct Flag {
    name: String,
    alias: Vec<String>,
    kind: FlagKind,
}

#[derive(Debug, Clone)]
pub enum FlagKind {
    Boolean,
    Value,
}

#[derive(Debug, Clone)]
pub enum FlagValue {
    Boolean(bool),
    Value(String),
}

impl Flag {
    pub fn new<T: Into<String>>(name: T, kind: FlagKind) -> Self {
        Flag {
            name: name.into(),
            alias: vec![],
            kind,
        }
    }
}

impl FlagTrait for Flag {
    type Kind = FlagKind;
    type Value = FlagValue;

    fn name(self) -> String {
        self.name.clone()
    }

    fn kind(self) -> Self::Kind {
        self.kind.clone()
    }

    fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias.push(alias.into());
        self
    }

    fn option_index(&self, v: &[String]) -> Option<usize> {
        v.iter().position(|r| {
            r == &format!("--{}", &self.name) || self.alias.iter().any(|a| r == &format!("-{}", a))
        })
    }

    fn value(&self, args: &[String]) -> Option<Self::Value> {
        match self.kind {
            FlagKind::Boolean => {
                if let Some(_) = self.option_index(args) {
                    return Some(FlagValue::Boolean(true));
                }
                None
            }
            FlagKind::Value => {
                if let Some(index) = self.option_index(args) {
                    if index + 1 < args.len() && !args[index + 1].starts_with('-') {
                        return Some(FlagValue::Value(args[index + 1].clone()));
                    }
                }
                None
            }
        }
    }
}
