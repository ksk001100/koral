use std::collections::HashMap;

pub trait App {
    fn name(&self) -> String;
    fn action(&self, ctx: Context) -> Result<(), Box<dyn std::error::Error>>;
    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    fn flags(&self) -> Vec<Flag> {
        vec![]
    }
}

pub trait FlagTrait {
    type Kind;
    type Value;

    fn new<T: Into<String>>(name: T, kind: Self::Kind) -> Self;
    fn alias(self, alias: impl Into<String>) -> Self;
    fn option_index(&self, v: &[String]) -> Option<usize>;
    fn value(&self, args: &[String]) -> Option<Self::Value>;
}

pub struct Koral {
    name: String,
    apps: Vec<Box<dyn App>>,
    action: Action,
    flags: Vec<Flag>,
}

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

#[derive(Debug, Clone)]
pub struct Context {
    pub args: Vec<String>,
    pub flags: HashMap<String, Option<FlagValue>>,
}

impl Context {
    pub fn new(app: &dyn App, args: Vec<String>) -> Self {
        let mut flags = HashMap::new();
        for flag in app.flags() {
            flags.insert(flag.name.clone(), flag.value(&args));
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

impl FlagTrait for Flag {
    type Kind = FlagKind;
    type Value = FlagValue;

    fn new<T: Into<String>>(name: T, kind: Self::Kind) -> Self {
        Flag {
            name: name.into(),
            alias: vec![],
            kind,
        }
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

pub type Action = fn(Context) -> Result<(), Box<dyn std::error::Error>>;

impl Koral {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Koral {
            name: name.into(),
            apps: vec![],
            flags: vec![],
            action: |_| Ok(()),
        }
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    pub fn app(mut self, app: impl App + 'static) -> Self {
        self.apps.push(Box::new(app));
        self
    }

    pub fn flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    pub fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        match args.get(1) {
            Some(app_name) => {
                let app = self.apps.iter().find(|app| app.name() == *app_name);
                match app {
                    Some(app) => app.run(args[1..].to_vec()),
                    None => {
                        if Self::is_help(args.clone()) {
                            self.help();
                            return Ok(());
                        }
                        let ctx = Context::new(self, args);
                        (self.action)(ctx)
                    }
                }
            }
            None => {
                let ctx = Context::new(self, args);
                (self.action)(ctx)
            }
        }
    }

    pub fn help(&self) {
        println!("App Name: {}", self.name);
        if self.flags.len() > 0 {
            println!("Flags:");
            for flag in &self.flags {
                println!("\t--{} {:?}", flag.name, flag.kind);
            }
        }

        if self.apps.len() > 0 {
            println!("Commands:");
            for app in &self.apps {
                println!("\t{}", app.name());
                for flag in app.flags() {
                    println!("\t\t--{} {:?}", flag.name, flag.kind);
                }
            }
        }
    }

    fn is_help(args: Vec<String>) -> bool {
        args.contains(&"--help".to_string()) || args.contains(&"-h".to_string())
    }
}

impl App for Koral {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn action(&self, ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
        (self.action)(ctx)
    }

    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.run(args)
    }

    fn flags(&self) -> Vec<Flag> {
        self.flags.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_koral() {
        let app = Koral::new("cli");
        assert_eq!(app.name, "cli".to_string());
        assert_eq!(app.apps.len(), 0);
        assert_eq!(app.flags.len(), 0);
        assert!(matches!(app.run(vec![]), Ok(())));
    }

    #[test]
    fn test_action() {
        let app = Koral::new("cli").action(|_| Err(Box::from("Action Error".to_string())));
        assert!(matches!(app.run(vec![]), Err(_)));
    }

    #[test]
    fn test_app() {
        struct TestApp;

        impl App for TestApp {
            fn name(&self) -> String {
                "test".to_string()
            }
            fn action(&self, _ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            fn run(&self, _args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
        }

        let app = Koral::new("cli").app(TestApp);
        assert_eq!(app.apps.len(), 1);
    }

    #[test]
    fn test_flag() {
        let flag = Flag::new("flag", FlagKind::Boolean).alias("f");
        assert_eq!(flag.name, "flag".to_string());
        assert_eq!(flag.alias[0], "f".to_string());
        assert!(matches!(flag.kind, FlagKind::Boolean));
    }

    #[test]
    fn test_context_new() {
        struct TestApp;
        impl App for TestApp {
            fn name(&self) -> String {
                "test".to_string()
            }
            fn action(&self, _ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            fn run(&self, _args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            fn flags(&self) -> Vec<Flag> {
                vec![Flag::new("flag", FlagKind::Boolean)]
            }
        }

        let app = TestApp;
        let ctx = Context::new(&app, vec!["--flag".to_string()]);
        assert_eq!(ctx.flags.len(), 1);
    }

    #[test]
    fn test_context_bool_flag() {
        struct TestApp;
        impl App for TestApp {
            fn name(&self) -> String {
                "test".to_string()
            }
            fn action(&self, _ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            fn run(&self, _args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            fn flags(&self) -> Vec<Flag> {
                vec![Flag::new("flag", FlagKind::Boolean)]
            }
        }

        let app = TestApp;
        let ctx = Context::new(&app, vec!["--flag".to_string()]);
        assert!(ctx.bool_flag("flag"));
    }

    #[test]
    fn test_context_value_flag() {
        struct TestApp;
        impl App for TestApp {
            fn name(&self) -> String {
                "test".to_string()
            }
            fn action(&self, _ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            fn run(&self, _args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
            fn flags(&self) -> Vec<Flag> {
                vec![Flag::new("flag", FlagKind::Value)]
            }
        }

        let app = TestApp;
        let ctx = Context::new(&app, vec!["--flag".to_string(), "value".to_string()]);
        assert_eq!(ctx.value_flag("flag"), Some("value".to_string()));
    }

    #[test]
    fn test_is_help() {
        assert!(Koral::is_help(vec!["--help".to_string()]));
        assert!(Koral::is_help(vec!["-h".to_string()]));
        assert!(!Koral::is_help(vec![]));
    }
}
