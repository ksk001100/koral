use koral::prelude::*;

#[derive(App, Default)]
#[app(name = "hello", description = "A greeting application", action = hello)]
struct HelloApp;

fn hello(_ctx: Context) -> KoralResult<()> {
    println!("Hello, Koral!");
    Ok(())
}

fn main() -> KoralResult<()> {
    let mut app = HelloApp::default();
    app.run(std::env::args().collect())
}
