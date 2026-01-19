use koral::prelude::*;
use std::path::PathBuf;

mod commands;
mod store;

use crate::commands::Commands;
use crate::store::Store;

#[derive(App, Default)]
#[app(name = "kv", version = "0.1.0", action = main_action)]
struct KvApp {
    #[app(subcommand)]
    cmd: Commands,
}

fn main_action(_ctx: Context) -> KoralResult<()> {
    println!("Usage: kv <set|get|del|list>");
    Ok(())
}

fn main() -> KoralResult<()> {
    let mut app = KvApp::default();

    // Using a local JSON file for the store
    let mut store = Store::new(PathBuf::from("kv.json"));

    app.run_with_state(&mut store, std::env::args().collect())
}
