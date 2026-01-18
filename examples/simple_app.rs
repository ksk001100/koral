use koral::{App, Flag, KoralResult};

struct VerboseFlag;
impl Flag for VerboseFlag {
    type Value = bool;
    fn name() -> &'static str {
        "verbose"
    }
    fn short() -> Option<char> {
        Some('v')
    }
    fn help() -> &'static str {
        "Run with verbose output"
    }
    fn takes_value() -> bool {
        false
    }
}

struct CountFlag;
impl Flag for CountFlag {
    type Value = i32;
    fn name() -> &'static str {
        "count"
    }
    fn default_value() -> Option<Self::Value> {
        Some(1)
    }
    fn help() -> &'static str {
        "Number of times to say hello"
    }
}

fn main() -> KoralResult<()> {
    App::new("simple-app")
        .version("1.0")
        .description("A simple app example")
        .register::<VerboseFlag>()
        .register::<CountFlag>()
        .action(|ctx| {
            let verbose = ctx.get::<VerboseFlag>().unwrap_or(false);
            // Default is handled by Flag trait default_value if missing in args?
            // Wait, Parser implementation DOES fill defaults now!
            // But CountFlag returns Option<i32>.
            // So ctx.get::<CountFlag>() returns Option<i32>.
            // If default was applied by Parser, it should be Some(1).
            let count = ctx
                .get::<CountFlag>()
                .expect("Default value guaranteed by Parser");

            if verbose {
                println!("Verbose mode on");
            }

            for i in 0..count {
                println!("Hello #{}", i + 1);
            }

            if !ctx.args.is_empty() {
                println!("Positional arguments: {:?}", ctx.args);
            }
            Ok(())
        })
        .run(std::env::args().skip(1).collect())
}
