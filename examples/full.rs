use koral::traits::App;
use koral::{Context, Flag, KoralResult};

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
struct AddCmd {
    // Positional args are handled in action
    #[app(skip)]
    task_name: String,
}

fn add_task(ctx: Context) -> KoralResult<()> {
    if ctx.args.is_empty() {
        println!("Error: Task description required.");
        return Ok(());
    }
    let task = ctx.args.join(" ");
    println!("Added task: '{}'", task);
    Ok(())
}

#[derive(Default, koral::App)]
#[app(name = "list", action = list_tasks)]
struct ListCmd {
    all: AllFlag,
}

fn list_tasks(ctx: Context) -> KoralResult<()> {
    let show_all = ctx.get::<AllFlag>().unwrap_or(false);

    println!("Tasks:");
    println!("  [ ] Buy groceries");
    println!("  [ ] Walk the dog");
    if show_all {
        println!("  [x] Read Koral documentation");
    }
    Ok(())
}

#[derive(Default, koral::App)]
#[app(name = "done", action = complete_task)]
struct DoneCmd {
    #[app(skip)]
    id: i32,
}

fn complete_task(ctx: Context) -> KoralResult<()> {
    if let Some(id_str) = ctx.args.first() {
        println!("Marked task {} as done.", id_str);
    } else {
        println!("Error: Task ID required.");
    }
    Ok(())
}

// --- Main App ---

#[derive(koral::App)]
#[app(name = "todo", version = "0.1.0", action = run_todo)]
struct TodoApp {
    verbose: VerboseFlag,
    #[app(subcommand)]
    cmd: TodoCmd,
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
    // Check global flags
    if ctx.get::<VerboseFlag>().unwrap_or(false) {
        println!("[DEBUG] Verbose mode enabled");
    }

    // Dispatch subcommand
    // Note: In a real app you might want to strip the flags consumed by the parent
    // before passing to subcommand, or Context does it.
    // Here we pass parsed args.

    // Check if there are any args left for subcommand
    if ctx.args.is_empty() {
        // Default behavior (List) or Help?
        // Let's run List by default as defined in Enum Default, logic needs to trigger it.
        // But FromArgs expects a subcommand name.
        println!("Usage: todo <add|list|done>");
        return Ok(());
    }

    let cmd = koral::traits::FromArgs::from_args(&ctx.args)?;
    match cmd {
        TodoCmd::Add(mut c) => c.run(ctx.args[1..].to_vec())?,
        TodoCmd::List(mut c) => c.run(ctx.args[1..].to_vec())?,
        TodoCmd::Done(mut c) => c.run(ctx.args[1..].to_vec())?,
    }

    Ok(())
}

fn main() -> KoralResult<()> {
    // Instantiate with default values (phantom flags)
    // derive(App) generates TodoApp struct.

    let mut app = TodoApp {
        verbose: VerboseFlag,
        cmd: TodoCmd::default(),
    };

    app.run(std::env::args().skip(1).collect())
}
