use koral::prelude::*;
use std::sync::{Arc, Mutex};

// Middleware that marks execution
#[derive(Clone, Default)]
struct FlagMiddleware {
    executed: Arc<Mutex<bool>>,
}

impl Middleware for FlagMiddleware {
    fn before(&self, _ctx: &mut Context) -> KoralResult<()> {
        let mut g = self.executed.lock().unwrap();
        *g = true;
        Ok(())
    }
}

#[derive(App)]
#[app(name = "mw_test", action = run)]
#[app(middleware)] // empty list? No we need to inject or register.
                   // We can't use DI middleware in test easily without struct definition?
                   // Let's use `middleware(FlagMiddleware)`? But FlagMiddleware needs state.
                   // We can use dynamic injection.
struct TestApp {
    #[app(middleware)]
    mw: FlagMiddleware,
}

fn run(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

#[test]
fn test_middleware_skipped_on_help() {
    let executed = Arc::new(Mutex::new(false));
    let mw = FlagMiddleware {
        executed: executed.clone(),
    };
    let mut app = TestApp { mw };

    // Capture stdout to prevent clutter?
    // tests run in parallel, capturing stdout is default by cargo test.

    // Run with --help
    let _ = app.run(vec!["prog".to_string(), "--help".to_string()]);

    assert!(
        !*executed.lock().unwrap(),
        "Middleware should NOT run on help"
    );
}

#[test]
fn test_middleware_runs_normally() {
    let executed = Arc::new(Mutex::new(false));
    let mw = FlagMiddleware {
        executed: executed.clone(),
    };
    let mut app = TestApp { mw };

    let _ = app.run(vec!["prog".to_string()]);

    assert!(*executed.lock().unwrap(), "Middleware SHOULD run normally");
}
