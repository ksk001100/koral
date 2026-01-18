use koral::{Flag, KoralResult, Context};
use koral::traits::{App as AppTrait, Flag as FlagTrait};

struct MyApp {
    verbose: bool,
    name: String,
    v_flag: Flag<bool>,
    n_flag: Flag<String>,
}

impl MyApp {
    fn new() -> Self {
        Self {
            verbose: false,
            name: "Guest".to_string(),
            v_flag: Flag::new("verbose").alias("v"),
            n_flag: Flag::new("name").alias("n").default_value("Guest".to_string()),
        }
    }
}

impl AppTrait for MyApp {
    fn name(&self) -> &str {
        "my-app"
    }

    fn flags(&self) -> Vec<&dyn FlagTrait> {
        vec![
            &self.v_flag as &dyn FlagTrait, 
            &self.n_flag as &dyn FlagTrait
        ]
    }

    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if let Some(v) = ctx.get("verbose") {
            self.verbose = v;
        }

        if let Some(n) = ctx.get("name") {
            self.name = n;
        } else if let Some(def) = &self.n_flag.default_value {
            self.name = def.clone();
        }

        println!("Hello, {}!", self.name);
        if self.verbose {
            println!("Verbose mode is enabled.");
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
