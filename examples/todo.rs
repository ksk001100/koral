use koral::prelude::*;
use std::sync::{Arc, Mutex};

// --- State ---

struct TodoState {
    tasks: Vec<String>,
}

// --- Flags ---

#[derive(Flag, Debug, Default)]
#[flag(name = "verbose", short = 'v', help = "Enable verbose output")]
struct VerboseFlag;

#[derive(Flag, Debug, Default)]
#[flag(help = "Show all tasks including completed ones")]
struct AllFlag;

#[derive(FlagValue, Clone, Debug, PartialEq, Default)]
enum ListFormat {
    #[default]
    Simple,
    Detailed,
}

fn validate_format(s: &str) -> Result<(), String> {
    match s {
        "simple" | "detailed" => Ok(()),
        _ => Err("Format must be 'simple' or 'detailed'".to_string()),
    }
}

#[derive(Flag, Debug, Default)]
#[flag(
    name = "format",
    short = 'f',
    default = "simple",
    help = "Output format (simple, detailed)",
    validator = validate_format,
    aliases = "fmt"
)]
struct FormatFlag(#[allow(dead_code)] ListFormat);

// --- Subcommands ---

#[derive(Default, koral::App)]
#[app(name = "add", action = add_task)]
struct AddCmd;

fn add_task(ctx: Context) -> KoralResult<()> {
    if ctx.args.is_empty() {
        println!("Error: Task description required.");
        return Ok(());
    }
    let task = ctx.args.join(" ");

    // Access state (Arc<Mutex<TodoState>>)
    // We passed &mut Arc<...>, so ctx.state() gives &Arc<...>.
    let state = ctx
        .state::<Arc<Mutex<TodoState>>>()
        .ok_or_else(|| KoralError::Validation("State not found".to_string()))?;

    let mut guard = state
        .lock()
        .map_err(|_| KoralError::Validation("Lock poisoned".to_string()))?;
    guard.tasks.push(task.clone());

    println!("Added task: '{}'", task);
    Ok(())
}

#[derive(Default, koral::App)]
#[app(name = "list", action = list_tasks)]
#[app(flags(AllFlag, FormatFlag))]
struct ListCmd;

fn list_tasks(ctx: Context) -> KoralResult<()> {
    let show_all = ctx.get::<AllFlag>().unwrap_or(false);
    let format = ctx.get::<FormatFlag>().expect("Default value");

    // Access state
    let state = ctx
        .state::<Arc<Mutex<TodoState>>>()
        .ok_or_else(|| KoralError::Validation("State not found".to_string()))?;

    let guard = state
        .lock()
        .map_err(|_| KoralError::Validation("Lock poisoned".to_string()))?;

    println!("Tasks (Format: {:?}):", format);
    for (i, task) in guard.tasks.iter().enumerate() {
        match format {
            ListFormat::Simple => println!("  - {}", task),
            ListFormat::Detailed => println!("  [{}] {}", i + 1, task),
        }
    }

    if show_all {
        println!("  (Showing all tasks - dummy impl)");
    }
    Ok(())
}

#[derive(Default, koral::App)]
#[app(name = "done", action = complete_task)]
struct DoneCmd;

fn complete_task(ctx: Context) -> KoralResult<()> {
    if let Some(id_str) = ctx.args.first() {
        if let Ok(id) = id_str.parse::<usize>() {
            let state = ctx
                .state::<Arc<Mutex<TodoState>>>()
                .ok_or_else(|| KoralError::Validation("State not found".to_string()))?;

            let mut guard = state
                .lock()
                .map_err(|_| KoralError::Validation("Lock poisoned".to_string()))?;

            if id > 0 && id <= guard.tasks.len() {
                let removed = guard.tasks.remove(id - 1);
                println!("Marked task '{}' as done.", removed);
            } else {
                println!("Error: Invalid task ID.");
            }
        } else {
            println!("Error: Task ID must be a number.");
        }
    } else {
        println!("Error: Task ID required.");
    }
    Ok(())
}

// --- Main App ---

#[derive(koral::App)]
#[app(name = "todo", version = "0.1.0", action = run_todo)]
#[app(flags(VerboseFlag))]
struct TodoApp {
    #[app(subcommand)]
    cmd: TodoCmd,
}

#[derive(koral::Subcommand)]
enum TodoCmd {
    #[subcommand(name = "add", aliases = "a")]
    Add(AddCmd),
    #[subcommand(name = "list", aliases = "ls")]
    List(ListCmd),
    #[subcommand(name = "done", aliases = "d")]
    Done(DoneCmd),
}

impl Default for TodoCmd {
    fn default() -> Self {
        Self::List(ListCmd::default())
    }
}

fn run_todo(ctx: Context) -> KoralResult<()> {
    // If we're here, no subcommand was matched (or args were empty)
    // Print usage

    // Check global flags (Optional - demonstrated here but doesn't affect flow)
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

    // We need to pass &mut Arc<...> as &mut dyn Any
    // But Mutex needs to be inside Arc.
    // The handlers expect `ctx.state::<Arc<Mutex<TodoState>>>()`.
    // So we pass `&mut state` where `state` IS `Arc<Mutex<...>>`.

    app.run_with_state(&mut state, std::env::args().collect())
}
