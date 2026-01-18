use koral::{Context, Flag, KoralResult};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Environment {
    Development,
    Staging,
    Production,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

struct EnvFlag;
impl Flag for EnvFlag {
    type Value = Environment;
    fn name() -> &'static str {
        "env"
    }
    fn short() -> Option<char> {
        Some('e')
    }
    fn default_value() -> Option<Self::Value> {
        Some(Environment::Development)
    }
}

struct MyTool {
    env: Environment,
}

impl MyTool {
    fn new() -> Self {
        Self {
            env: Environment::Development,
        }
    }
}

impl koral::traits::App for MyTool {
    fn name(&self) -> &str {
        "deploy-tool"
    }

    fn flags(&self) -> Vec<koral::flag::FlagDef> {
        vec![koral::flag::FlagDef::from_trait::<EnvFlag>()]
    }

    fn execute(&mut self, ctx: Context) -> KoralResult<()> {
        if let Some(e) = ctx.get::<EnvFlag>() {
            self.env = e;
        }
        // Default is handled by Parser application of default_value,
        // so ctx.get should return Some(Development) if not present.
        // Wait, does it?
        // Parser::parse fills from known_flags.default_value.
        // But Parser uses FlagDef. default_value is Option<String>.
        // It inserts into HashMap.
        // ctx.get::<F> calls value_t::<F::Value>.
        // value_t tries to get key. If found, parses string.
        // So yes, it should work.

        println!("Deploying to {:?} environment...", self.env);
        Ok(())
    }
}

fn main() -> KoralResult<()> {
    // We can run MyTool manually or use App wrapping if we moved logic to closure.
    // But manual implementation example usually runs manually.
    let mut tool = MyTool::new();
    let args: Vec<String> = std::env::args().skip(1).collect();
    koral::traits::App::run(&mut tool, args)
}
