use koral::{traits, Context};

fn main() {
    let app = App::new("calc")
        .app(Command::Add)
        .app(Command::Sub)
        .app(Command::Mul)
        .app(Command::Div);

    match app.run(std::env::args().collect()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

struct App {
    name: String,
    apps: Vec<Box<dyn traits::App>>,
}

impl App {
    fn new<T: Into<String>>(name: T) -> Self {
        App {
            name: name.into(),
            apps: vec![],
        }
    }

    fn app(mut self, app: impl traits::App + 'static) -> Self {
        self.apps.push(Box::new(app));
        self
    }

    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
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
                        Err("unknown command".into())
                    }
                }
            }
            None => {
                self.help();
                Ok(())
            }
        }
    }

    fn help(&self) {
        println!("Usage: {} <command> [args]", self.name);
        println!();
        println!("Commands:");
        for app in &self.apps {
            println!("  {}", app.name());
        }
    }

    fn is_help(args: Vec<String>) -> bool {
        args.iter().any(|arg| arg == "--help" || arg == "-h")
    }
}

impl traits::App for App {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn action(&self, _ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
        self.help();
        Ok(())
    }

    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let ctx = Context::new(args, self.flags().clone());
        self.action(ctx)
    }
}

#[derive(Debug)]
enum Command {
    Add,
    Sub,
    Mul,
    Div,
}

impl traits::App for Command {
    fn name(&self) -> String {
        match self {
            Command::Add => "add".to_string(),
            Command::Sub => "sub".to_string(),
            Command::Mul => "mul".to_string(),
            Command::Div => "div".to_string(),
        }
    }

    fn action(&self, ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Add => {
                let a = ctx.args.get(1).unwrap().parse::<i32>()?;
                let b = ctx.args.get(2).unwrap().parse::<i32>()?;
                println!("{}", a + b);
            }
            Command::Sub => {
                let a = ctx.args.get(1).unwrap().parse::<i32>()?;
                let b = ctx.args.get(2).unwrap().parse::<i32>()?;
                println!("{}", a - b);
            }
            Command::Mul => {
                let a = ctx.args.get(1).unwrap().parse::<i32>()?;
                let b = ctx.args.get(2).unwrap().parse::<i32>()?;
                println!("{}", a * b);
            }
            Command::Div => {
                let a = ctx.args.get(1).unwrap().parse::<f32>()?;
                let b = ctx.args.get(2).unwrap().parse::<f32>()?;
                println!("{}", a / b);
            }
        }

        Ok(())
    }

    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let ctx = Context::new(args, self.flags().clone());
        self.action(ctx)
    }
}
