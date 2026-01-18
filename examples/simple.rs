use koral::traits::App;
use koral::{Context, Flag, KoralResult};

#[derive(Flag, Debug)]
#[flag(name = "verbose", short = 'v', help = "Run with verbose output")]
struct VerboseFlag;

#[derive(Flag, Debug)]
#[flag(name = "count", default = "1", help = "Number of times to say hello")]
struct CountFlag(i32);

#[derive(koral::App)]
#[app(name = "simple", version = "1.0", action = run)]
struct SimpleApp {
    verbose: VerboseFlag,
    count: CountFlag,
}

fn run(ctx: Context) -> KoralResult<()> {
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
    let mut app = SimpleApp {
        verbose: VerboseFlag,
        count: CountFlag(0),
    };
    app.run(std::env::args().skip(1).collect())
}
