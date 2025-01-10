pub trait App {
    fn name(&self) -> String;
    fn action(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>;
    fn flags(&self) -> Vec<String> {
        vec![]
    }
}

pub struct Koral {
    name: String,
    apps: Vec<Box<dyn App>>,
    action: Action,
}

pub type Action = fn(Vec<String>) -> Result<(), Box<dyn std::error::Error>>;

impl Koral {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Koral {
            name: name.into(),
            apps: vec![],
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

    pub fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        match args.get(1) {
            Some(app_name) => {
                let app = self.apps.iter().find(|app| app.name() == *app_name);
                match app {
                    Some(app) => app.run(args[1..].to_vec()),
                    None => (self.action)(args)
                }
            }
            None => (self.action)(args),
        }
    }

    pub fn help(&self) {
        println!("App Name: {}", self.name);

        println!("Commands:");
        for app in &self.apps {
            println!("\t{}", app.name());
        }
    }
}

impl App for Koral {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn action(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        (self.action)(args)
    }

    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.run(args)
    }

    fn flags(&self) -> Vec<String> {
        vec![]
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
