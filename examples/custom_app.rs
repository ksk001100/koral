use koral::*;

fn main() {
    let koral = Koral::new("calc")
        .app(Command::Add)
        .app(Command::Sub)
        .app(Command::Mul)
        .app(Command::Div);

    match koral.run(std::env::args().collect()) {
        Ok(_) => {},
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

    fn action(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Add => {
                let a = args.get(2).unwrap().parse::<i32>().unwrap();
                let b = args.get(3).unwrap().parse::<i32>().unwrap();
                println!("{}", a + b);
            }
            Command::Sub => {
                let a = args.get(2).unwrap().parse::<i32>().unwrap();
                let b = args.get(3).unwrap().parse::<i32>().unwrap();
                println!("{}", a - b);
            }
            Command::Mul => {
                let a = args.get(2).unwrap().parse::<i32>().unwrap();
                let b = args.get(3).unwrap().parse::<i32>().unwrap();
                println!("{}", a * b);
            }
            Command::Div => {
                let a = args.get(2).unwrap().parse::<f32>().unwrap();
                let b = args.get(3).unwrap().parse::<f32>().unwrap();
                println!("{}", a / b);
            }
        }

        Ok(())
    }

    fn run(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        self.action(args)
    }

    fn flags(&self) -> Vec<String> {
        vec![]
    }
}


