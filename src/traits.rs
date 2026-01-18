use crate::error::KoralResult;
use crate::context::Context;
use std::fmt::Display;
use std::str::FromStr;

/// Trait for types that can be parsed from command line arguments.
pub trait FlagValue: Clone + Send + Sync + ToString + 'static {
    type Err: Display;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
    
    /// Whether this flag type requires a value to be passed.
    /// Defaults to true. Override for boolean flags.
    fn takes_value() -> bool {
        true
    }
}

impl<T> FlagValue for T 
where 
    T: FromStr + Clone + Send + Sync + ToString + 'static,
    <T as FromStr>::Err: Display 
{
    type Err = <T as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
    }

    fn takes_value() -> bool {
        if std::any::type_name::<T>() == "bool" {
            false
        } else {
            true
        }
    }
}

/// Trait for accessing flag metadata. 
pub trait Flag {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn aliases(&self) -> Vec<&str>; 
    fn takes_value(&self) -> bool;
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
    fn flags(&self) -> Vec<&dyn Flag> {
        vec![]
    }

    /// Returns a list of subcommands.
    fn subcommands(&self) -> Vec<&dyn App> {
        vec![]
    }

    /// Execute the application logic.
    fn execute(&mut self, ctx: Context) -> KoralResult<()>;

    /// Run the application with the given arguments.
    /// This handles common tasks like help and version checks, and argument parsing.
    fn run(&mut self, args: Vec<String>) -> KoralResult<()> {
        // 1. Basic help check for root command (naive)
        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
           self.print_help();
           return Ok(());
        }

        if args.contains(&"--version".to_string()) {
            println!("{} version {}", self.name(), self.version());
            return Ok(());
        }

        // 2. Parse arguments
        // We need to know about subcommands to route correctly.
        // For now, let's just parse for THIS command.
        
        let ctx = {
            let mut parser = crate::parser::Parser::new(self.flags());
            parser.parse(&args)?
        };

        // 3. Subcommand routing
        // Check if the first positional argument matches a subcommand name.
        if let Some(cmd_name) = ctx.args.first() {
            for sub in self.subcommands() {
                if sub.name() == cmd_name {
                    // Found a subcommand!
                    // We need to construct a new args vector for the subcommand
                    // effectively shifting the args. 
                    // Note: This is a bit simplistic because `ctx.args` has stripped flags out.
                    // But `ctx.args` contains "positional" args. 
                    // Ideally we should pass the *remainder* of the original unparsed args.
                    // But since we already parsed everything into `ctx`, we might have consumed flags meant for subcommand?
                    // 
                    // Actually, usually subcommands are `app subcommand [flags]`.
                    // So if we see a subcommand, we should stop parsing flags for the parent?
                    // For now, let's trust that the user didn't mix parent flags after subcommand name if we use strict parsing.
                    // But wait, `parser` consumes flags.
                    
                    // Let's delegate to the subcommand. 
                    // We can't really "re-use" the parent ctx easily if subcommands have different flags.
                    // So we probably should have checked for subcommand BEFORE full parsing?
                    // Or `Parser` should support "stop at first non-flag argument"?
                    
                    // For this refactor, let's keep it simple:
                    // If we find a subcommand match in the *original* args, dispatch to it.
                    // But `run` takes `Vec<String>`.
                    
                    // Let's look at `args` again.
                    // Skip arg[0] which is usually the binary name if called from shell, but `run` usually gets args without binary name?
                    // `example/simple_app.rs`: .run(std::env::args().skip(1).collect())
                    // So args[0] is the first argument.
                }
            }
        }
        
        // Re-implementing subcommand logic properly requires a bit more sophisticated parsing (like "commands" vs "args").
        // For now, let's do the parsing and then execute. Subcommand handling might need to be explicit in `execute` OR we enforce a structure.
        // The implementation plan said: "Update `App` trait: Add `subcommands()`... Update `run` to perform parsing *before* calling `execute`."
        
        // Let's stick to the plan: `execute` takes `Context`. 
        // If the user wants subcommands, they can register them. 
        // If we want automatic dispatch, we need to do it here.
        
        if !self.subcommands().is_empty() {
             if let Some(_first) = args.first() {
                 // Check if 'first' matches a subcommand
                 // We have to iterate mutably if we want to call run on it... but `subcommands()` returns `&dyn App`.
                 // `&dyn App` doesn't allow calling `run` (which takes &mut self).
                 // This is a flaw in the `subcommands()` returning `&dyn App` design if we want generic dispatch.
                 // We need `&mut dyn App`.
                 // But `subcommands` usually returns a list of *owned* or *reference* to apps.
                 // If `self` owns them, we need `subcommands_mut`.
                 
                 // For now, let's skip the complicated automatic dispatch implementation in the default `run` method
                 // and focus on getting `execute(ctx)` working first for single apps.
                 // The 'Complex App' example manually handles dispatch, which is fine for now, 
                 // but we ideally want to automate it.
                 // 
                 // Let's implement basic `execute` call for now.
            }
        }
        
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
                let aliases = flag.aliases();
                let alias_str = if aliases.is_empty() {
                    "".to_string()
                } else {
                    format!(", -{}", aliases.join(", -"))
                };
                let val_hint = if flag.takes_value() { " <value>" } else { "" };
                println!("  --{}{} {}  {}", flag.name(), alias_str, val_hint, flag.description());
        }
        
        let subs = self.subcommands();
        if !subs.is_empty() {
            println!("\nCommands:");
            for sub in subs {
                println!("  {}  {}", sub.name(), sub.description());
            }
        }
    }
}
