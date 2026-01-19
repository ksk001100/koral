use koral::prelude::*;
use std::sync::{Arc, Mutex};

mod commands;
mod state;

use crate::commands::list::VerboseFlag;
use crate::commands::TodoCmd;
use crate::state::TodoState;

#[derive(koral::App, Default)]
#[app(name = "todo", version = "0.1.0", action = run_todo)]
#[app(flags(VerboseFlag))]
pub struct TodoApp {
    #[app(subcommand)]
    cmd: TodoCmd,
}

fn run_todo(ctx: Context) -> KoralResult<()> {
    // Check global flags
    if ctx.get::<VerboseFlag>().unwrap_or(false) {
        println!("[DEBUG] Verbose mode enabled (Parent)");
    }

    println!("Usage: todo <add|list|done>");
    Ok(())
}

fn main() -> KoralResult<()> {
    let mut app = TodoApp {
        cmd: TodoCmd::default(),
    };

    let mut state = Arc::new(Mutex::new(TodoState {
        tasks: vec!["Task1".to_string()],
    }));

    app.run_with_state(&mut state, std::env::args().collect())
}
