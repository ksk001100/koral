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

    /// Returns a list of middlewares to execute.
    fn middlewares(&self) -> Vec<Box<dyn crate::middleware::Middleware>> {
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
        let flags = self.flags();
        let h_overridden = flags.iter().any(|f| f.short == Some('h'));

        let help_invoked = args.iter().position(|a| {
            if a == "--help" {
                true
            } else if a == "-h" {
                !h_overridden
            } else {
                false
            }
        });
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

        // Parse arguments
        let (mut flags_map, mut positionals) = {
            let parser = crate::parser::Parser::new(self.flags())
                .strict(self.is_strict())
                .ignore_required(true);
            // Skip argv[0] (program name)
            let args_to_parse = if !args.is_empty() {
                &args[1..]
            } else {
                &args[..]
            };
            let ctx = parser.parse(args_to_parse)?;
            (ctx.flags, ctx.args)
        };

        let middlewares = self.middlewares();
        let skip_middleware = help_invoked.is_some();

        let mut extensions = std::collections::HashMap::new();

        // Execute Middleware 'before' hooks
        // We create a temporary context for BEFORE hooks
        if !skip_middleware {
            let mut ctx = Context::new(flags_map.clone(), positionals.clone()).with_state(state);
            for mw in &middlewares {
                mw.before(&mut ctx)?;
            }
            // Capture any modifications to flags/args?
            // For now, we update our local copies.
            flags_map = ctx.flags;
            positionals = ctx.args;
            extensions = ctx.extensions;
        }

        // Execute Command
        // Create context for execution (consumes it)
        let result = {
            let ctx = Context::new(flags_map.clone(), positionals.clone())
                .with_state(state)
                .with_extensions(extensions);
            self.execute(ctx)
        };

        // Execute Middleware 'after' hooks
        if !skip_middleware && result.is_ok() {
            let mut ctx = Context::new(flags_map, positionals).with_state(state);
            for mw in middlewares.iter().rev() {
                mw.after(&mut ctx)?;
            }
        }

        result
    }

    /// Run the application with the given arguments.
    /// This handles common tasks like help and version checks, and argument parsing.
    fn run(&mut self, args: Vec<String>) -> KoralResult<()> {
        // Check for help flag, but respect subcommands
        let flags = self.flags();
        let h_overridden = flags.iter().any(|f| f.short == Some('h'));

        let help_invoked = args.iter().position(|a| {
            if a == "--help" {
                true
            } else if a == "-h" {
                !h_overridden
            } else {
                false
            }
        });
        let subcommands = self.subcommands();

        let should_print_help = if let Some(h_idx) = help_invoked {
            // Check if a known subcommand appears BEFORE help
            // args[0] is prog name, start checking from 1
            let sub_idx = args.iter().enumerate().skip(1).find_map(|(i, arg)| {
                if subcommands
                    .iter()
                    .any(|s| s.name == *arg || s.aliases.contains(arg))
                {
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

        // Parse arguments
        let (mut flags_map, mut positionals) = {
            let parser = crate::parser::Parser::new(self.flags())
                .strict(self.is_strict())
                .ignore_required(true);
            // Skip argv[0] (program name)
            let args_to_parse = if args.is_empty() {
                &args[..]
            } else {
                &args[1..]
            };
            let ctx = parser.parse(args_to_parse)?;
            (ctx.flags, ctx.args)
        };

        let middlewares = self.middlewares();
        let skip_middleware = help_invoked.is_some();

        let mut extensions = std::collections::HashMap::new();

        if !skip_middleware {
            let mut ctx = Context::new(flags_map.clone(), positionals.clone());
            for mw in &middlewares {
                mw.before(&mut ctx)?;
            }
            flags_map = ctx.flags;
            positionals = ctx.args;
            extensions = ctx.extensions;
        }

        // Execute Command
        let result = {
            let ctx =
                Context::new(flags_map.clone(), positionals.clone()).with_extensions(extensions);
            self.execute(ctx)
        };

        // Execute Middleware 'after' hooks
        if !skip_middleware && result.is_ok() {
            let mut ctx = Context::new(flags_map, positionals);
            for mw in middlewares.iter().rev() {
                mw.after(&mut ctx)?;
            }
        }

        result
    }

    /// Print help message to stdout.
    fn print_help(&self) {
        print!("{}", crate::help::generate_help(self));
    }
}
