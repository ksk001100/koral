use koral::prelude::*;
use std::sync::{Arc, Mutex};

// Shared state to track execution
#[derive(Default, Debug)]
struct ExecutionLog {
    path: Vec<String>,
}

// LEVEL 2
#[derive(App, Default)]
#[app(name = "level2", action = level2_action)]
struct Level2App;

fn level2_action(ctx: Context) -> KoralResult<()> {
    if let Some(log) = ctx.state::<Arc<Mutex<ExecutionLog>>>() {
        log.lock().unwrap().path.push("level2".to_string());
    }
    Ok(())
}

// LEVEL 1
#[derive(Subcommand)]
enum Level1Commands {
    #[subcommand(name = "level2")]
    Level2(Level2App),
}

#[derive(App)]
#[app(name = "level1", action = level1_action)]
struct Level1App {
    #[app(subcommand)]
    cmd: Level1Commands,
}

impl Default for Level1App {
    fn default() -> Self {
        Self {
            cmd: Level1Commands::Level2(Level2App::default()),
        }
    }
}

fn level1_action(ctx: Context) -> KoralResult<()> {
    if let Some(log) = ctx.state::<Arc<Mutex<ExecutionLog>>>() {
        log.lock().unwrap().path.push("level1".to_string());
    }
    Ok(())
}

// ROOT
#[derive(Subcommand)]
enum RootCommands {
    #[subcommand(name = "level1")]
    Level1(Level1App),
}

#[derive(App)]
#[app(name = "root", action = root_action)]
struct RootApp {
    #[app(subcommand)]
    cmd: RootCommands,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {
            cmd: RootCommands::Level1(Level1App::default()),
        }
    }
}

fn root_action(ctx: Context) -> KoralResult<()> {
    if let Some(log) = ctx.state::<Arc<Mutex<ExecutionLog>>>() {
        log.lock().unwrap().path.push("root".to_string());
    }
    Ok(())
}

fn run_test(args: Vec<&str>) -> Vec<String> {
    let log = Arc::new(Mutex::new(ExecutionLog::default()));
    let mut app = RootApp {
        cmd: RootCommands::Level1(Level1App {
            cmd: Level1Commands::Level2(Level2App),
        }),
    };
    let mut state: Arc<Mutex<ExecutionLog>> = log.clone();

    let args: Vec<String> = args.into_iter().map(|s| s.to_string()).collect();
    app.run_with_state(&mut state, args).unwrap();

    let guard = log.lock().unwrap();
    guard.path.clone()
}

#[test]
fn test_root_only() {
    let path = run_test(vec!["root"]);
    assert_eq!(path, vec!["root"]);
}

#[test]
fn test_nested_level1() {
    let path = run_test(vec!["root", "level1"]);
    assert_eq!(path, vec!["level1"]);
}

#[test]
fn test_nested_level2() {
    let path = run_test(vec!["root", "level1", "level2"]);
    assert_eq!(path, vec!["level2"]);
}
