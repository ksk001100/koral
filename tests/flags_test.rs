use koral::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Flag, Debug, Default, Clone, PartialEq)]
#[flag(name = "verbose", short = 'v')]
struct VerboseFlag;

#[derive(Flag, Debug, Default, Clone, PartialEq)]
#[flag(name = "name", short = 'n')]
struct NameFlag(String);

#[derive(Flag, Debug, Default, Clone, PartialEq)]
#[flag(name = "count", short = 'c', default = 1)]
struct CountFlag(i32);

#[derive(Flag, Debug, Default, Clone, PartialEq)]
#[flag(name = "opt", short = 'o')]
struct OptFlag(String);

#[derive(App, Default)]
#[app(name = "flag_test", action = test_action)]
#[app(flags(VerboseFlag, NameFlag, CountFlag, OptFlag))]
struct TestApp;

#[derive(Clone, Debug, Default)]
struct TestResult {
    verbose: bool,
    name: String,
    count: i32,
    opt: Option<String>,
}

fn test_action(ctx: Context) -> KoralResult<()> {
    let res = TestResult {
        verbose: ctx.get::<VerboseFlag>().unwrap_or(false),
        name: ctx.get::<NameFlag>().unwrap_or_default(),
        count: ctx.get::<CountFlag>().unwrap_or(1),
        opt: ctx.get::<OptFlag>(),
    };

    let state = ctx
        .state::<Arc<Mutex<Option<TestResult>>>>()
        .expect("State missing");
    let mut guard = state.lock().unwrap();
    *guard = Some(res);
    Ok(())
}

fn run_test(args: Vec<&str>) -> TestResult {
    let result_store = Arc::new(Mutex::new(None));
    let mut app = TestApp;
    let mut state: Arc<Mutex<Option<TestResult>>> = result_store.clone();

    let args: Vec<String> = args.into_iter().map(|s| s.to_string()).collect();
    app.run_with_state(&mut state, args).unwrap();

    let guard = result_store.lock().unwrap();
    guard.clone().expect("Action did not run")
}

#[test]
fn test_bool_flag() {
    let res = run_test(vec!["prog", "-v"]);
    assert!(res.verbose);

    let res = run_test(vec!["prog", "--verbose"]);
    assert!(res.verbose);

    let res = run_test(vec!["prog"]);
    assert!(!res.verbose);
}

#[test]
fn test_string_flag() {
    let res = run_test(vec!["prog", "-n", "Alice"]);
    assert_eq!(res.name, "Alice");

    let res = run_test(vec!["prog", "--name", "Bob"]);
    assert_eq!(res.name, "Bob");

    let res = run_test(vec!["prog", "--name=Charlie"]);
    assert_eq!(res.name, "Charlie");
}

#[test]
fn test_int_flag() {
    let res = run_test(vec!["prog", "-c", "10"]);
    assert_eq!(res.count, 10);

    let res = run_test(vec!["prog"]);
    assert_eq!(res.count, 1); // Default
}

#[test]
fn test_option_flag() {
    let res = run_test(vec!["prog", "-o", "value"]);
    assert_eq!(res.opt, Some("value".to_string()));

    let res = run_test(vec!["prog"]);
    assert_eq!(res.opt, None);
}
