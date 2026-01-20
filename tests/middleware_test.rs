use koral::prelude::*;
use std::sync::{Mutex, OnceLock};

fn log() -> &'static Mutex<Vec<String>> {
    static LOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    LOG.get_or_init(|| Mutex::new(Vec::new()))
}

#[derive(Default)]
struct GlobalLogMiddleware;

impl Middleware for GlobalLogMiddleware {
    fn before(&self, _ctx: &mut Context) -> KoralResult<()> {
        log().lock().unwrap().push("before".to_string());
        Ok(())
    }

    fn after(&self, _ctx: &mut Context) -> KoralResult<()> {
        log().lock().unwrap().push("after".to_string());
        Ok(())
    }
}

#[derive(App, Default)]
#[app(name = "mw_test", action = run)]
#[app(middleware(GlobalLogMiddleware))]
struct MiddlewareApp;

fn run(_ctx: Context) -> KoralResult<()> {
    log().lock().unwrap().push("run".to_string());
    Ok(())
}

#[test]
fn test_middleware_execution_order() {
    // Reset log
    log().lock().unwrap().clear();

    let mut app = MiddlewareApp;
    app.run(vec!["mw_test".to_string()]).unwrap();

    let l = log().lock().unwrap();
    assert_eq!(
        *l,
        vec!["before".to_string(), "run".to_string(), "after".to_string()]
    );
}
