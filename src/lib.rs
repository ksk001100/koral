pub struct Koral {
    name: String,
    apps: Vec<Box<dyn App>>,
    action: fn(Vec<String>),
}

impl Koral {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Koral {
            name: name.into(),
            apps: vec![],
            action: |_| {},
        }
    }

    pub fn action(mut self, action: fn(Vec<String>)) -> Self {
        self.action = action;
        self
    }

    pub fn app(mut self, app: Box<dyn App>) -> Self {
        self.apps.push(app);
        self
    }

    pub fn run(&self, args: Vec<String>) {
        match args.get(1) {
            Some(app_name) => {
                let app = self.apps.iter().find(|app| app.name() == *app_name);
                match app {
                    Some(app) => app.run(args),
                    None => self.help(),
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

    fn action(&self, args: Vec<String>) {
        (self.action)(args);
    }

    fn run(&self, args: Vec<String>) {
        self.action(args);
    }

    fn flags(&self) -> Vec<String> {
        vec![]
    }
}

pub trait App {
    fn name(&self) -> String;
    fn action(&self, args: Vec<String>);
    fn run(&self, args: Vec<String>);
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
