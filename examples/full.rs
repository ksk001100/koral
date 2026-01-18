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
#[flag(
    name = "all",
    short = 'a',
    help = "Show all tasks including completed ones"
)]
struct AllFlag;

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
        .expect("State mismatch");
    let mut guard = state.lock().unwrap();
    guard.tasks.push(task.clone());

    println!("Added task: '{}'", task);
    Ok(())
}

#[derive(Default, koral::App)]
#[app(name = "list", action = list_tasks)]
#[app(flags(AllFlag))]
struct ListCmd;

fn list_tasks(ctx: Context) -> KoralResult<()> {
    let show_all = ctx.get::<AllFlag>().unwrap_or(false);

    // Access state
    let state = ctx
        .state::<Arc<Mutex<TodoState>>>()
        .expect("State mismatch");
    let guard = state.lock().unwrap();

    println!("Tasks:");
    for (i, task) in guard.tasks.iter().enumerate() {
        println!("  [{}] {}", i + 1, task);
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
                .expect("State mismatch");
            let mut guard = state.lock().unwrap();
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

    state: Arc<Mutex<TodoState>>,
}

#[derive(koral::Subcommand)]
enum TodoCmd {
    #[subcommand(name = "add")]
    Add(AddCmd),
    #[subcommand(name = "list")]
    List(ListCmd),
    #[subcommand(name = "done")]
    Done(DoneCmd),
}

impl Default for TodoCmd {
    fn default() -> Self {
        Self::List(ListCmd::default())
    }
}

fn run_todo(ctx: Context) -> KoralResult<()> {
    let app = ctx.app::<TodoApp>().unwrap();

    // Check global flags
    if ctx.get::<VerboseFlag>().unwrap_or(false) {
        println!("[DEBUG] Verbose mode enabled");
    }

    // Clone Arc to local variable, make it mutable so we can take &mut reference
    let mut state = app.state.clone();

    // Dispatch subcommand
    // ... args check ...
    if ctx.args.is_empty() {
        // Fallback or usage
        // Note: For 'list' default, we need logic.
        // Assuming explicit command for now.
        println!("Usage: todo <add|list|done>");
        return Ok(());
    }

    // Because Context consumes flags, we need to pass what's left in args to subcommand?
    // Actually Context::args are positional args.
    // If we have `todo -v add foo`, ctx.args = ["add", "foo"].
    // We parse "add" to determine subcommand, then pass ["foo"] to subcommand.
    //
    // However, `FromArgs` expects the subcommand name at index 0?
    // `derive_subcommand`: `let sub_name = &args[0];`
    // Yes.

    let cmd = koral::traits::FromArgs::from_args(&ctx.args)?;

    // We need to shift args for the subcommand execution (remove the subcommand name)
    // `run_with_state` will re-parse.
    let sub_args = ctx.args[1..].to_vec();

    match cmd {
        TodoCmd::Add(mut c) => c.run_with_state(&mut state, sub_args)?,
        TodoCmd::List(mut c) => c.run_with_state(&mut state, sub_args)?,
        TodoCmd::Done(mut c) => c.run_with_state(&mut state, sub_args)?,
    }

    Ok(())
}

fn main() -> KoralResult<()> {
    let mut app = TodoApp {
        cmd: TodoCmd::default(),
        state: Arc::new(Mutex::new(TodoState {
            tasks: vec!["Task1".to_string()],
        })),
    };

    app.run(std::env::args().skip(1).collect())
}
