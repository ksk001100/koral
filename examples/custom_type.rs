use koral::{Flag, FlagValue, KoralResult, Context};
use koral::traits::App as AppTrait;
use std::str::FromStr;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
enum Environment {
    Development,
    Staging,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

// User-defined type must implement FromStr to be used as a Flag.
// It also needs Clone, Send, Sync, 'static (derived/auto).
impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" | "development" => Ok(Environment::Development),
            "stage" | "staging" => Ok(Environment::Staging),
            "prod" | "production" => Ok(Environment::Production),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}

struct MyTool {
    env: Environment,
    env_flag: Flag<Environment>,
}

impl MyTool {
    fn new() -> Self {
        Self {
            env: Environment::Development,
            env_flag: Flag::<Environment>::new("env")
                .alias("e")
                .default_value(Environment::Development),
        }
    }
}

impl AppTrait for MyTool {
    fn name(&self) -> &str {
        "deploy-tool"
    }

    fn flags(&self) -> Vec<&dyn koral::traits::Flag> {
        vec![&self.env_flag]
    }

    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if let Some(e) = ctx.get("env") {
            self.env = e;
        } else {
             // Fallback to default if not present in context?
             // Since Parser doesn't fill defaults, and ctx.get returns parsed one.
             // But my flag declaration has default_value.
             // Ideally we should use it. 
             // But `Flag` struct is accessible here.
             if let Some(def) = &self.env_flag.default_value {
                 self.env = def.clone();
             }
        }

        println!("Deploying to {:?} environment...", self.env);
        Ok(())
    }
}

fn main() -> KoralResult<()> {
    let mut tool = MyTool::new();
    // Simulate args: --env prod
    // We explicitly call run, which now does parsing.
    let args = vec!["--env".to_string(), "prod".to_string()];
    tool.run(args)
}
