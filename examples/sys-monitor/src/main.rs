use koral::prelude::*;
use std::sync::{Arc, Mutex};

mod commands;
mod middleware;
mod state;

use crate::commands::Commands;
use crate::middleware::{AuthMiddleware, TimingMiddleware, User};
use crate::state::AppState;

#[derive(Flag, Debug)]
#[flag(name = "verbose", short = 'v', help = "Enable verbose logging")]
struct VerboseFlag(#[allow(dead_code)] bool);

#[derive(Flag, Debug)]
#[flag(name = "user", required = true, help = "User name (required)")]
struct UserFlag(#[allow(dead_code)] String);

#[derive(App, Default)]
#[app(name = "comprehensive", version = "2.0", action = main_handler)]
#[app(flags(VerboseFlag, UserFlag))]
#[app(middleware(TimingMiddleware))] // Static registration
struct MyApp {
    #[app(subcommand)]
    commands: Commands,

    #[app(middleware)] // Dynamic injection
    auth: AuthMiddleware,
}

// Main handler using Dependency Injection
fn main_handler(
    state: State<AppState>,
    verbose: FlagArg<VerboseFlag>,
    user_flag: FlagArg<UserFlag>,
    user_ext: Extension<User>,
    args: Args,
) -> KoralResult<()> {
    if verbose.0 {
        println!("Verbose mode enabled.");
    }

    // Modify shared state
    {
        let mut cnt = state.counter.lock().unwrap();
        *cnt += 1;
        println!("Counter incremented to {}", *cnt);
    }

    println!("Hello, {} (Role: {})!", user_flag.0, user_ext.role);
    println!("Authenticated as: {}", user_ext.name);

    if !args.is_empty() {
        println!("Extra positional args: {:?}", *args);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run_app() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_app() -> KoralResult<()> {
    // Initialize State
    let mut state = AppState {
        counter: Arc::new(Mutex::new(0)),
        db_url: "postgres://localhost:5432".to_string(),
    };

    // Initialize App with Configured Middleware
    let mut app = MyApp {
        commands: Commands::Status(crate::commands::StatusCmd),
        auth: AuthMiddleware {
            api_key: "secure-secret-123".to_string(),
        },
    };

    // Simulate args for demonstration if run without args
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("No args provided. Try running with:");
        println!("  cargo run -p sys-monitor -- --user Alice --verbose");
        println!("  cargo run -p sys-monitor -- --user Alice status --format json");
        // Print help manually
        app.print_help();
        return Ok(());
    }

    app.run_with_state(&mut state, args)
}
