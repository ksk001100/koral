use koral::prelude::*;

#[derive(Flag, Debug)]
#[flag(
    name = "verbose",
    short = 'v',
    help = "Run with verbose output",
    aliases = "loud"
)]
struct VerboseFlag;

fn validate_count(s: &str) -> Result<(), String> {
    if let Ok(val) = s.parse::<i32>() {
        if val > 0 {
            return Ok(());
        }
    }
    Err("Count must be greater than 0".to_string())
}

#[derive(Flag, Debug)]
#[flag(name = "count", default = "1", help = "Number of times to say hello", validator = validate_count, aliases = "cnt, num")]
struct CountFlag(#[allow(dead_code)] i32);

#[derive(App)]
#[app(name = "simple", version = "1.0", action = run)]
#[app(flags(VerboseFlag, CountFlag))]
struct SimpleApp {
    call_count: i32,
}

// Handler receives Context<SimpleApp>
fn run(mut ctx: Context<SimpleApp>) -> KoralResult<()> {
    // Access application state safely via ctx.app
    if let Some(app) = &mut ctx.app {
        app.call_count += 1;
        println!("SimpleApp run count: {}", app.call_count);
    }

    let verbose = ctx.get::<VerboseFlag>().unwrap_or(false);
    let count = ctx.get::<CountFlag>().expect("Default value guaranteed");

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
}

fn main() -> KoralResult<()> {
    let mut app = SimpleApp { call_count: 0 };
    app.run(std::env::args().collect())
}
