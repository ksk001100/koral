use koral::flag::FlagDef;
use koral::traits::App as AppTrait;
use koral::{Context, Flag, KoralResult};

struct VerboseFlag;
impl Flag for VerboseFlag {
    type Value = bool;
    fn name() -> &'static str {
        "verbose"
    }
    fn short() -> Option<char> {
        Some('v')
    }
    fn takes_value() -> bool {
        false
    }
}

struct NameFlag;
impl Flag for NameFlag {
    type Value = String;
    fn name() -> &'static str {
        "name"
    }
    fn short() -> Option<char> {
        Some('n')
    }
    fn default_value() -> Option<Self::Value> {
        Some("Guest".to_string())
    }
}

struct MyApp {
    verbose: bool,
    name: String,
}

impl MyApp {
    fn new() -> Self {
        Self {
            verbose: false,
            name: "Guest".to_string(),
        }
    }
}

impl AppTrait for MyApp {
    fn name(&self) -> &str {
        "my-app"
    }

    fn flags(&self) -> Vec<FlagDef> {
        vec![
            FlagDef::from_trait::<VerboseFlag>(),
            FlagDef::from_trait::<NameFlag>(),
        ]
    }

    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if let Some(name) = ctx.get::<NameFlag>() {
            self.name = name;
        }

        if ctx.get::<VerboseFlag>().unwrap_or(false) {
            self.verbose = true;
        }

        println!("Hello, {}!", self.name);
        if self.verbose {
            println!("(Verbose mode enabled)");
        }
        Ok(())
    }
}

fn main() -> KoralResult<()> {
    let mut app = MyApp::new();
    // In a real app, environment args would be passed. here we simulate.
    let args: Vec<String> = std::env::args().skip(1).collect();
    app.run(args)
}
