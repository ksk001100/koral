use koral::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Flag, Debug, Default)]
#[flag(name = "envflag", env = "KORAL_TEST_ENV")]
struct EnvFlag(#[allow(dead_code)] String);

#[derive(App)]
#[app(name = "envtest", action = test_action)]
#[app(flags(EnvFlag))]
struct EnvApp;

#[derive(Clone)]
struct TestResult {
    val: String,
}

fn test_action(ctx: Context) -> KoralResult<()> {
    let val = ctx.get::<EnvFlag>().unwrap_or_default();

    let state = ctx
        .state::<Arc<Mutex<Option<TestResult>>>>()
        .expect("State missing");
    let mut guard = state.lock().unwrap();
    *guard = Some(TestResult { val });
    Ok(())
}

#[test]
fn test_env_var_flag() {
    std::env::set_var("KORAL_TEST_ENV", "from_env");

    let result_store = Arc::new(Mutex::new(None));
    let args = vec!["prog".into()];
    let mut app = EnvApp;

    let mut state: Arc<Mutex<Option<TestResult>>> = result_store.clone();

    app.run_with_state(&mut state, args).unwrap();

    let guard = result_store.lock().unwrap();
    if let Some(res) = &*guard {
        assert_eq!(res.val, "from_env");
    } else {
        panic!("Action did not run");
    }

    std::env::remove_var("KORAL_TEST_ENV");
}

#[test]
fn test_env_override() {
    std::env::set_var("KORAL_TEST_ENV", "from_env");

    let result_store = Arc::new(Mutex::new(None));
    // CLI arg should override Env
    let args = vec!["prog".into(), "--envflag".into(), "from_cli".into()];
    let mut app = EnvApp;

    let mut state: Arc<Mutex<Option<TestResult>>> = result_store.clone();

    app.run_with_state(&mut state, args).unwrap();

    let guard = result_store.lock().unwrap();
    if let Some(res) = &*guard {
        assert_eq!(res.val, "from_cli");
    } else {
        panic!("Action did not run");
    }

    std::env::remove_var("KORAL_TEST_ENV");
}
