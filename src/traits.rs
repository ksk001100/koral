use crate::context::Context;
use crate::error::KoralResult;
use std::fmt::Display;
use std::str::FromStr;

/// Trait for types that can be parsed from command line arguments.
pub trait FlagValue: Clone + Send + Sync + ToString + FromStr + 'static
where
    <Self as FromStr>::Err: Display,
{
}

impl<T> FlagValue for T
where
    T: FromStr + Clone + Send + Sync + ToString + 'static,
    <T as FromStr>::Err: Display,
{
}

/// Trait for types that can be parsed from a list of arguments (e.g. subcommands).
pub trait FromArgs: Sized {
    fn from_args(args: &[String]) -> KoralResult<Self>;
    fn get_subcommands() -> Vec<crate::command::CommandDef> {
        vec![]
    }
}

/// The core trait for a CLI application or sub-command.
pub trait App {
    /// The name of the application or command.
    fn name(&self) -> &str;

    /// The version of the application.
    fn version(&self) -> &str {
        "0.0.0"
    }

    /// The description/usage of the application.
    fn description(&self) -> &str {
        ""
    }

    /// Returns a list of flags for help generation.
    fn flags(&self) -> Vec<crate::flag::FlagDef> {
        vec![]
    }

    /// Returns a list of subcommands for help generation.
    fn subcommands(&self) -> Vec<crate::command::CommandDef> {
        vec![]
    }

    /// Execute the application logic.
    fn execute(&mut self, ctx: Context) -> KoralResult<()>;

    /// Run the application with a shared state.
    fn run_with_state(
        &mut self,
        state: &mut dyn std::any::Any,
        args: Vec<String>,
    ) -> KoralResult<()> {
        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
            self.print_help();
            return Ok(());
        }

        if args.contains(&"--version".to_string()) {
            println!("{} version {}", self.name(), self.version());
            return Ok(());
        }

        let ctx = {
            let parser = crate::parser::Parser::new(self.flags());
            parser.parse(&args)?
        };

        // Inject state
        let ctx = ctx.with_state(state);

        self.execute(ctx)
    }

    /// Run the application with the given arguments.
    /// This handles common tasks like help and version checks, and argument parsing.
    fn run(&mut self, args: Vec<String>) -> KoralResult<()> {
        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
            self.print_help();
            return Ok(());
        }

        if args.contains(&"--version".to_string()) {
            println!("{} version {}", self.name(), self.version());
            return Ok(());
        }

        let ctx = {
            let parser = crate::parser::Parser::new(self.flags());
            parser.parse(&args)?
        };

        self.execute(ctx)
    }

    fn print_help(&self) {
        println!("Usage: {} [options] [command]", self.name());
        let desc = self.description();
        if !desc.is_empty() {
            println!("{}", desc);
        }
        println!("\nOptions:");

        // Version flag (implied)
        println!("  --version  Show version information");
        println!("  --help, -h  Show help information");

        for flag in self.flags() {
            let mut aliases_parts = Vec::new();
            if let Some(s) = flag.short {
                aliases_parts.push(format!("-{}", s));
            }

            let val_hint = if flag.takes_value { " <value>" } else { "" };

            let short_str = if let Some(s) = flag.short {
                format!(", -{}", s)
            } else {
                "".to_string()
            };

            println!("  --{}{} {}  {}", flag.name, short_str, val_hint, flag.help);
        }

        let subs = self.subcommands();
        if !subs.is_empty() {
            println!("\nCommands:");
            for sub in subs {
                println!("  {}  {}", sub.name, sub.description);
            }
        }
    }
}
