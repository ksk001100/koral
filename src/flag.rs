use crate::traits::Flag as FlagTrait;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flag {
    name: String,
    alias: Vec<String>,
    kind: FlagKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlagKind {
    Boolean,
    Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias.push(alias.into());
        self
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

    fn alias(self) -> Vec<String> {
        self.alias.clone()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag() {
        let flag = Flag::new("flag", FlagKind::Value);
        assert_eq!(flag.clone().name(), "flag");
        assert_eq!(flag.clone().kind(), FlagKind::Value);
    }

    #[test]
    fn test_flag_value() {
        let flag = Flag::new("flag", FlagKind::Value);
        let args = vec![
            "test".to_string(),
            "--flag".to_string(),
            "value".to_string(),
        ];
        assert_eq!(
            flag.value(&args),
            Some(FlagValue::Value("value".to_string()))
        );
    }

    #[test]
    fn test_flag_boolean() {
        let flag = Flag::new("flag", FlagKind::Boolean);
        let args = vec!["test".to_string(), "--flag".to_string()];
        assert_eq!(flag.value(&args), Some(FlagValue::Boolean(true)));
    }
}
