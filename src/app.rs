use crate::{action::Action, context::Context, flag::Flag, traits};

pub struct App {
    name: String,
    apps: Vec<Box<dyn traits::App>>,
    action: Action,
    flags: Vec<Flag>,
}

impl App {
    pub fn new<T: Into<String>>(name: T) -> Self {
        App {
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

    pub fn app(mut self, app: impl traits::App + 'static) -> Self {
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
                        let ctx = Context::new(args, self.flags.clone());
                        (self.action)(ctx)
                    }
                }
            }
            None => {
                let ctx = Context::new(args, self.flags.clone());
                (self.action)(ctx)
            }
        }
    }

    pub fn help(&self) {
        use crate::traits::Flag;

        println!("App Name: {}", self.name);
        if self.flags.is_empty() {
            println!("Flags:");
            for flag in &self.flags {
                println!("\t--{} {:?}", flag.clone().name(), flag.clone().kind());
            }
        }

        if self.apps.is_empty() {
            println!("Commands:");
            for app in &self.apps {
                println!("\t{}", app.name());
                for flag in app.flags() {
                    println!("\t\t--{} {:?}", flag.clone().name(), flag.clone().kind());
                }
            }
        }
    }

    fn is_help(args: Vec<String>) -> bool {
        args.contains(&"--help".to_string()) || args.contains(&"-h".to_string())
    }
}

impl traits::App for App {
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
    use crate::flag::FlagKind;

    #[test]
    fn test_app() {
        let app = App::new("test")
            .flag(Flag::new("flag", FlagKind::Value))
            .action(|ctx| {
                let flag = ctx.value_flag("flag").unwrap();
                assert_eq!(flag, "value");
                Ok(())
            });

        let args = vec![
            "test".to_string(),
            "--flag".to_string(),
            "value".to_string(),
        ];
        app.run(args).unwrap();
    }

    #[test]
    fn test_app_help() {
        let app = App::new("test")
            .flag(Flag::new("flag", FlagKind::Value))
            .action(|_| Ok(()));

        let args = vec!["test".to_string(), "--help".to_string()];
        app.run(args).unwrap();
    }

    #[test]
    fn test_app_app() {
        let app = App::new("test")
            .app(App::new("sub").action(|_| Ok(())))
            .action(|_| Ok(()));

        let args = vec!["test".to_string(), "sub".to_string()];
        app.run(args).unwrap();
    }
}
