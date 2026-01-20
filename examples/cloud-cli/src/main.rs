use koral::prelude::*;
// use std::sync::{Arc, Mutex}; // Removed unused imports

mod commands;
mod domain;
mod flags;
mod middleware;
mod state;

use crate::commands::Commands;
use crate::flags::{ProfileFlag, TokenFlag, VerboseFlag};
use crate::middleware::{AuditMiddleware, AuthMiddleware};
use crate::state::CloudState;

// --- App Definition ---

#[derive(App, Default)]
#[app(name = "cloud-cli", version = "0.1.0", author = "Cloud Corp")]
#[app(flags(VerboseFlag, ProfileFlag, TokenFlag))]
#[app(middleware(AuditMiddleware))] // Static middleware (runs for all)
struct CloudApp {
    #[app(subcommand)]
    commands: Commands,

    #[app(middleware)] // Dynamic middleware (for auth injection)
    auth: AuthMiddleware,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e); // Standard error output
        std::process::exit(1);
    }
}

fn run() -> KoralResult<()> {
    // 1. Initialize State
    let cloud_state = CloudState::default();

    // 2. Initialize App
    let mut app = CloudApp {
        commands: Commands::Login(commands::LoginCmd), // Initial dummy, Koral replaces this based on args

        auth: AuthMiddleware {
            state: cloud_state.clone(),
        },
    };

    // 3. Run
    // Koral expects `mut state` for dependency injection of `State<T>`.
    // Our `CloudState` is internally mutable (Arc<Mutex>), so we just pass it.
    // Wrap it in a way `State<CloudState>` can extract.
    // `app.run_with_state` takes `&mut S`.

    let mut state_container = cloud_state; // This object will be passed to extractors

    let args: Vec<String> = std::env::args().collect();
    app.run_with_state(&mut state_container, args)
}
