use koral::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Flag, Debug, Default)]
#[flag(name = "verbose", short = 'v')]
struct VerboseFlag;

#[derive(Flag, Debug, Default)]
#[flag(name = "force", short = 'f')]
struct ForceFlag;

#[derive(Flag, Debug, Default)]
#[flag(name = "name", default = "")]
struct NameFlag(#[allow(dead_code)] String);

#[derive(App)]
#[app(name = "test", action = test_action)]
#[app(flags(VerboseFlag, ForceFlag, NameFlag))]
struct TestApp;

// Storage to verify results
#[derive(Clone)]
struct TestResult {
    verbose: bool,
    force: bool,
    name: String,
}

fn test_action(ctx: Context) -> KoralResult<()> {
    let res = TestResult {
        verbose: ctx.get::<VerboseFlag>().unwrap_or(false),
        force: ctx.get::<ForceFlag>().unwrap_or(false),
        name: ctx.get::<NameFlag>().unwrap_or_default(),
    };

    // We can't return the result easily from run(), so we panic if assertion fails
    // or we can use a shared state via hacks, but let's just use thread local or simple logic.
    // Actually, for a test inside `tests/`, we can't easily share state with the App unless we pass it.
    // But `App::run` takes `&mut self` and args.
    // `state` injection via `run_with_state` is possible.

    // Let's use `run_with_state`.
    let state = ctx
        .state::<Arc<Mutex<Option<TestResult>>>>()
        .expect("State missing");
    let mut guard = state.lock().unwrap();
    *guard = Some(res);

    Ok(())
}

#[test]
fn test_combined_short_flags() {
    let result_store = Arc::new(Mutex::new(None));
    let mut app = TestApp;

    // args: prog -vf
    let args = vec!["prog".into(), "-vf".into()];

    let mut state: Arc<Mutex<Option<TestResult>>> = result_store.clone();

    // This should work if implemented
    let _ = app.run_with_state(&mut state, args);

    let guard = result_store.lock().unwrap();
    if let Some(res) = &*guard {
        assert!(res.verbose, "Verbose should be true");
        assert!(res.force, "Force should be true");
    } else {
        // If parser failed silently or treated -vf as positional (unknown), action runs but flags are false.
        // Or if it failed with error (though App::run usually returns Result).
        // Wait, if -vf is unknown, it goes to positionals. Flags default to false.
        // So we expect this to fail assertion if not implemented?
        // Actually, if flags are false, assertions fail.
        panic!("Action did not run or flags not set. Verbose/Force were not detected (likely treated as positional '-vf').");
    }
}

#[test]
fn test_long_flag_equals() {
    let result_store = Arc::new(Mutex::new(None));
    let mut app = TestApp;

    // args: prog --name=Alice
    let args = vec!["prog".into(), "--name=Alice".into()];

    let mut state: Arc<Mutex<Option<TestResult>>> = result_store.clone();

    app.run_with_state(&mut state, args).unwrap();

    let guard = result_store.lock().unwrap();
    if let Some(res) = &*guard {
        assert_eq!(res.name, "Alice", "Name should be Alice");
    } else {
        panic!("Action did not run or name not set. (likely treated '--name=Alice' as positional)");
    }
}
