use koral::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// =============================================================================
// 1. STATE
// =============================================================================

#[derive(Clone)]
struct AppState {
    // Determine if we need thread safety options or just simple clone
    // For this example, let's use a counter wrapped in Arc<Mutex> to show mutable shared state
    counter: Arc<Mutex<u32>>,
    db_url: String,
}

// =============================================================================
// 2. MIDDLEWARE
// =============================================================================

// Static Middleware (Timing)
#[derive(Default)]
struct TimingMiddleware {
    start: Arc<Mutex<Option<Instant>>>,
}

impl Middleware for TimingMiddleware {
    fn before(&self, _ctx: &mut Context) -> KoralResult<()> {
        *self.start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn after(&self, _ctx: &mut Context) -> KoralResult<()> {
        if let Some(start) = *self.start.lock().unwrap() {
            println!("[TimingMiddleware] Execution took: {:?}", start.elapsed());
        }
        Ok(())
    }
}

// Configurable Middleware (Injected)
#[derive(Clone)]
struct AuthMiddleware {
    api_key: String,
}

impl Middleware for AuthMiddleware {
    fn before(&self, _ctx: &mut Context) -> KoralResult<()> {
        // Imitate checking an env var or just logging the config
        println!("[AuthMiddleware] Checking API Key: {}", self.api_key);
        Ok(())
    }
}

// =============================================================================
// 3. TYPES & FLAGS
// =============================================================================

#[derive(FlagValue, Clone, Debug, PartialEq)]
enum OutputFormat {
    Json,
    Text,
    Yaml,
}

#[derive(Flag, Debug)]
#[flag(name = "verbose", short = 'v', help = "Enable verbose logging")]
struct VerboseFlag(#[allow(dead_code)] bool);

#[derive(Flag, Debug)]
#[flag(
    name = "format",
    short = 'f',
    default = "text",
    help = "Output format (json, text, yaml)"
)]
struct FormatFlag(#[allow(dead_code)] OutputFormat);

#[derive(Flag, Debug)]
#[flag(name = "user", required = true, help = "User name (required)")]
struct UserFlag(#[allow(dead_code)] String);

// =============================================================================
// 4. SUBCOMMANDS
// =============================================================================

#[derive(App, Clone, Debug, Default, PartialEq)]
#[app(name = "status", action = status_handler, help = "Check system status")]
#[app(flags(FormatFlag))]
struct StatusCmd;

fn status_handler(state: State<AppState>, format: FlagArg<FormatFlag>) -> KoralResult<()> {
    // DI automatically extracts State and FormatFlag
    println!("Status Check:");
    println!("  Database: {}", state.db_url);
    println!("  Counter: {}", *state.counter.lock().unwrap());
    println!("  Format: {:?}", format.0);
    Ok(())
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
enum Commands {
    #[subcommand(name = "status")]
    Status(StatusCmd),
}

// =============================================================================
// 5. MAIN APP
// =============================================================================

#[derive(App)]
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
    user: FlagArg<UserFlag>,
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

    println!("Hello, {}!", user.0);

    if !args.is_empty() {
        println!("Extra positional args: {:?}", *args);
    }

    Ok(())
}

// =============================================================================
// 6. EXECUTION
// =============================================================================

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
        commands: Commands::Status(StatusCmd),
        auth: AuthMiddleware {
            api_key: "secure-secret-123".to_string(),
        },
    };

    // Simulate args for demonstration if run without args, or use env args
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("No args provided. Try running with:");
        println!("  cargo run --example comprehensive -- --user Alice --verbose");
        println!("  cargo run --example comprehensive -- --user Alice status --format json");
        // Print help manually
        app.print_help();
        return Ok(());
    }

    app.run_with_state(&mut state, args)
}
