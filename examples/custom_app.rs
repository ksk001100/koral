use koral::{App, Context, Koral};

fn main() {
    let koral = Koral::new("calc")
        .app(Command::Add)
        .app(Command::Sub)
        .app(Command::Mul)
        .app(Command::Div);

    match koral.run(std::env::args().collect()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
enum Command {
    Add,
    Sub,
    Mul,
    Div,
}

impl App for Command {
    fn name(&self) -> String {
        match self {
            Command::Add => "add".to_string(),
            Command::Sub => "sub".to_string(),
            Command::Mul => "mul".to_string(),
            Command::Div => "div".to_string(),
        }
    }

    fn action(&self, context: Context) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Add => {
                let a = context.args.get(1).unwrap().parse::<i32>()?;
                let b = context.args.get(2).unwrap().parse::<i32>()?;
                println!("{}", a + b);
            }
            Command::Sub => {
                let a = context.args.get(1).unwrap().parse::<i32>()?;
                let b = context.args.get(2).unwrap().parse::<i32>()?;
                println!("{}", a - b);
            }
            Command::Mul => {
                let a = context.args.get(1).unwrap().parse::<i32>()?;
                let b = context.args.get(2).unwrap().parse::<i32>()?;
                println!("{}", a * b);
            }
            Command::Div => {
                let a = context.args.get(1).unwrap().parse::<f32>()?;
                let b = context.args.get(2).unwrap().parse::<f32>()?;
                println!("{}", a / b);
            }
        }

        Ok(())
    }

    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let context = Context::new(self, args);
        self.action(context)
    }
}
