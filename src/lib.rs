use std::collections::HashMap;

pub trait App {
    fn name(&self) -> String;
    fn action(&self, ctx: Context) -> Result<(), Box<dyn std::error::Error>>;
    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    fn flags(&self) -> Vec<Flag> {
        vec![]
    }
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

    pub fn option_index(&self, v: &[String]) -> Option<usize> {
        v.iter().position(|r| {
            r == &format!("--{}", &self.name) || self.alias.iter().any(|a| r == &format!("-{}", a))
        })
    }

    pub fn value(&self, args: &[String]) -> Option<FlagValue> {
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

        if self.apps.len() > 0 {
            println!("Commands:");
            for app in &self.apps {
                println!("\t{}", app.name());
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
    fn it_works() {
        let app = Koral::new("cli");
        assert_eq!(app.name, "cli".to_string());
    }
}
