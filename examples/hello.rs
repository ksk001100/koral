use koral::prelude::*;

#[derive(App, Default)]
#[app(name = "hello", action = run)]
struct HelloApp;

fn run(ctx: Context<HelloApp>) -> KoralResult<()> {
    println!("Hello, {}!", ctx.args.join(" "));
    Ok(())
}

fn main() -> KoralResult<()> {
    HelloApp.run(std::env::args().collect())
}
