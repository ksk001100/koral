use koral::prelude::*;
use std::sync::{Mutex, OnceLock};

// 1. Define State
#[derive(Default, Clone)]
struct AppState {
    db_url: String,
}

// 2. Define Flag
#[derive(Flag, Debug, PartialEq)]
#[flag(name = "verbose", short = 'v')]
struct VerboseFlag(bool);

#[derive(App, Default)]
#[app(name = "di_test", action = run)]
#[app(flags(VerboseFlag))]
struct DiApp;

// Separate logs for each test to avoid parallel execution issues
fn di_log() -> &'static Mutex<Vec<String>> {
    static DI_LOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    DI_LOG.get_or_init(|| Mutex::new(Vec::new()))
}

fn legacy_log() -> &'static Mutex<Vec<String>> {
    static LEGACY_LOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    LEGACY_LOG.get_or_init(|| Mutex::new(Vec::new()))
}

fn legacy_msg_log() -> &'static Mutex<Vec<String>> {
    static LEGACY_MSG_LOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    LEGACY_MSG_LOG.get_or_init(|| Mutex::new(Vec::new()))
}

// 3. Define Handler with DI
fn run(state: State<AppState>, verbose: FlagArg<VerboseFlag>, args: Args) -> KoralResult<()> {
    di_log()
        .lock()
        .unwrap()
        .push(format!("State: {}", state.db_url));
    if *verbose {
        di_log().lock().unwrap().push("Verbose: on".to_string());
    } else {
        di_log().lock().unwrap().push("Verbose: off".to_string());
    }
    di_log().lock().unwrap().push(format!("Args: {:?}", *args));
    Ok(())
}

#[test]
fn test_di_handler() {
    di_log().lock().unwrap().clear();

    let mut state = AppState {
        db_url: "postgres://localhost".into(),
    };
    let mut app = DiApp;

    // args: progname, --verbose, arg1
    let args = vec![
        "di_test".to_string(),
        "--verbose".to_string(),
        "extra_arg".to_string(),
    ];

    app.run_with_state(&mut state, args).unwrap();

    let output = di_log().lock().unwrap();
    assert!(output.contains(&"State: postgres://localhost".to_string()));
    assert!(output.contains(&"Verbose: on".to_string()));
    assert!(output.contains(&"Args: [\"extra_arg\"]".to_string()));
}

// Test legacy handler compatibility
#[derive(App, Default)]
#[app(name = "legacy_test", action = run_legacy)]
struct LegacyApp;

fn run_legacy(ctx: Context) -> KoralResult<()> {
    legacy_log()
        .lock()
        .unwrap()
        .push(format!("Legacy: {}", ctx.args.len()));
    Ok(())
}

#[derive(App, Default)]
#[app(name = "legacy_msg_test", action = run_legacy_msg)]
struct LegacyMsgApp {
    msg: String,
}

fn run_legacy_msg(app: &mut LegacyMsgApp, _ctx: Context) -> KoralResult<()> {
    legacy_msg_log()
        .lock()
        .unwrap()
        .push(format!("LegacyMsg: {}", app.msg));
    Ok(())
}

#[test]
fn test_legacy_handler() {
    legacy_log().lock().unwrap().clear();
    let mut app = LegacyApp;
    app.run(vec!["legacy_test".to_string(), "arg1".to_string()])
        .unwrap();
    assert!(legacy_log()
        .lock()
        .unwrap()
        .contains(&"Legacy: 1".to_string()));
}

#[test]
fn test_legacy_msg_handler() {
    legacy_msg_log().lock().unwrap().clear();
    let mut app = LegacyMsgApp {
        msg: "Hello".into(),
    };
    app.run(vec!["legacy_msg_test".to_string()]).unwrap();
    assert!(legacy_msg_log()
        .lock()
        .unwrap()
        .contains(&"LegacyMsg: Hello".to_string()));
}
