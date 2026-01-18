use koral::prelude::*;
use std::sync::{Arc, Mutex};

// State
#[derive(Default)]
struct TestState {
    executed: String,
    value: String,
}

// Subcommand definition
#[derive(Subcommand)]
enum ChildCmd {
    Add(AddCmd),
    Show(ShowCmd),
}

#[derive(App, Default)]
#[app(name = "add")]
#[app(action = add_action)]
struct AddCmd;

fn add_action(ctx: Context) -> KoralResult<()> {
    if let Some(state) = ctx.state::<Arc<Mutex<TestState>>>() {
        let mut guard = state.lock().unwrap();
        guard.executed = "Add".to_string();
        guard.value = ctx.args.join(" ");
    }
    Ok(())
}

#[derive(App, Default)]
#[app(name = "show")]
#[app(flags(VerboseFlag))]
#[app(action = show_action)]
struct ShowCmd;

#[derive(Flag, Default, Debug)]
#[flag(name = "verbose", short = 'v')]
struct VerboseFlag;

fn show_action(ctx: Context) -> KoralResult<()> {
    if let Some(state) = ctx.state::<Arc<Mutex<TestState>>>() {
        let mut guard = state.lock().unwrap();
        guard.executed = "Show".to_string();
        if ctx.is_present("verbose") {
            guard.value = "verbose".to_string();
        }
    }
    Ok(())
}

// Parent App
#[derive(App)]
#[app(name = "parent")]
#[app(subcommand)]
struct ParentApp {
    #[app(subcommand)]
    cmd: ChildCmd,
}

#[test]
fn test_auto_dispatch_add() {
    let state = Arc::new(Mutex::new(TestState::default()));

    // args: "prog", "add", "some", "args"
    let args = vec!["prog".to_string(), "add".to_string(), "item1".to_string()];

    let mut app = ParentApp {
        cmd: ChildCmd::Add(Default::default()), // Placeholder, will be replaced by parse
    };

    // Need to cast state to Any
    let mut any_state: Arc<Mutex<TestState>> = state.clone();

    app.run_with_state(&mut any_state, args).unwrap();

    let guard = state.lock().unwrap();
    assert_eq!(guard.executed, "Add");
    // "item1" is passed as argument to AddCmd
    // AddCmd parser sees "item1" as positional.
    // wait, parser for AddCmd sees `["item1"]`?
    // Dispatch logic: `args[1..]` -> `["item1"]`.
    // AddCmd parser runs on `["item1"]`. "item1" is positional.
    assert_eq!(guard.value, "item1");
}

#[test]
fn test_auto_dispatch_show() {
    let state = Arc::new(Mutex::new(TestState::default()));

    let args = vec!["prog".to_string(), "show".to_string(), "-v".to_string()];

    let mut app = ParentApp {
        cmd: ChildCmd::Add(Default::default()),
    };

    let mut any_state: Arc<Mutex<TestState>> = state.clone();

    app.run_with_state(&mut any_state, args).unwrap();

    let guard = state.lock().unwrap();
    assert_eq!(guard.executed, "Show");
    assert_eq!(guard.value, "verbose");
}
