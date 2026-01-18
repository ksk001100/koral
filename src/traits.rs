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
    /// Parse from arguments.
    fn from_args(args: &[String]) -> KoralResult<Self>;
    /// Get subcommand definitions.
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

    /// Returns whether strict mode is enabled.
    fn is_strict(&self) -> bool {
        false
    }

    /// Execute the application logic.
    fn execute(&mut self, ctx: Context) -> KoralResult<()>;

    /// Run the application with a shared state.
    fn run_with_state(
        &mut self,
        state: &mut dyn std::any::Any,
        args: Vec<String>,
    ) -> KoralResult<()> {
        // Check for help flag, but respect subcommands
        let help_invoked = args.iter().position(|a| a == "--help" || a == "-h");
        let subcommands = self.subcommands();

        let should_print_help = if let Some(h_idx) = help_invoked {
            // Check if a known subcommand appears BEFORE help
            // args[0] is prog name, start checking from 1
            let sub_idx = args.iter().enumerate().skip(1).find_map(|(i, arg)| {
                if subcommands.iter().any(|s| s.name == *arg) {
                    Some(i)
                } else {
                    None
                }
            });

            match sub_idx {
                Some(s_idx) => h_idx < s_idx, // Print help only if help appears BEFORE subcommand
                None => true,                 // No subcommand, help invoked -> print help
            }
        } else {
            false
        };

        if should_print_help {
            self.print_help();
            return Ok(());
        }

        if args.contains(&"--version".to_string()) {
            println!("{} version {}", self.name(), self.version());
            return Ok(());
        }

        let ctx = {
            let parser = crate::parser::Parser::new(self.flags()).strict(self.is_strict());
            // Skip argv[0] (program name)
            let args_to_parse = if args.len() > 0 {
                &args[1..]
            } else {
                &args[..]
            };
            parser.parse(args_to_parse)?
        };

        // Inject state
        let ctx = ctx.with_state(state);

        self.execute(ctx)
    }

    /// Run the application with the given arguments.
    /// This handles common tasks like help and version checks, and argument parsing.
    fn run(&mut self, args: Vec<String>) -> KoralResult<()> {
        // Check for help flag, but respect subcommands
        let help_invoked = args.iter().position(|a| a == "--help" || a == "-h");
        let subcommands = self.subcommands();

        let should_print_help = if let Some(h_idx) = help_invoked {
            // Check if a known subcommand appears BEFORE help
            // args[0] is prog name, start checking from 1
            let sub_idx = args.iter().enumerate().skip(1).find_map(|(i, arg)| {
                if subcommands.iter().any(|s| s.name == *arg) {
                    Some(i)
                } else {
                    None
                }
            });

            match sub_idx {
                Some(s_idx) => h_idx < s_idx, // Print help only if help appears BEFORE subcommand
                None => true,                 // No subcommand, help invoked -> print help
            }
        } else {
            false
        };

        if should_print_help {
            self.print_help();
            return Ok(());
        }

        if args.contains(&"--version".to_string()) {
            println!("{} version {}", self.name(), self.version());
            return Ok(());
        }

        let ctx = {
            let parser = crate::parser::Parser::new(self.flags()).strict(self.is_strict());
            // Skip argv[0] (program name)
            let args_to_parse = if args.len() > 0 {
                &args[1..]
            } else {
                &args[..]
            };
            parser.parse(args_to_parse)?
        };

        self.execute(ctx)
    }

    /// Print help message to stdout.
    fn print_help(&self) {
        println!("Usage: {} [options] [command]", self.name());
        let desc = self.description();
        if !desc.is_empty() {
            println!("{}", desc);
        }

        println!("\nOptions:");

        struct HelpItem {
            name: String,
            desc: String,
        }

        let mut items = Vec::new();

        // Built-in flags
        items.push(HelpItem {
            name: "--version".to_string(),
            desc: "Show version information".to_string(),
        });
        items.push(HelpItem {
            name: "--help, -h".to_string(),
            desc: "Show help information".to_string(),
        });

        for flag in self.flags() {
            let mut name_part = format!("--{}", flag.name);
            if let Some(s) = flag.short {
                name_part.push_str(&format!(", -{}", s));
            }

            if flag.takes_value {
                name_part.push_str(" <value>");
            }

            items.push(HelpItem {
                name: name_part,
                desc: flag.help,
            });
        }

        // Calculate max width for alignment
        let max_width = items.iter().map(|i| i.name.len()).max().unwrap_or(0);
        let padding = 2; // Extra space between name and desc

        for item in items {
            // Manual padding since dynamic width in format! macro can be tricky with string length vs char counts,
            // but here we deal with ASCII mostly.
            let pad_len = max_width.saturating_sub(item.name.len()) + padding;
            let pad = " ".repeat(pad_len);
            println!("  {}{}{}", item.name, pad, item.desc);
        }

        let subs = self.subcommands();
        if !subs.is_empty() {
            println!("\nCommands:");
            let max_sub_width = subs.iter().map(|s| s.name.len()).max().unwrap_or(0);

            for sub in subs {
                let pad_len = max_sub_width.saturating_sub(sub.name.len()) + padding;
                let pad = " ".repeat(pad_len);
                println!("  {}{}{}", sub.name, pad, sub.description);
            }
        }
    }
}
