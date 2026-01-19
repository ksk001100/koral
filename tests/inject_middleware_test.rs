use koral::prelude::*;
use std::sync::{Mutex, OnceLock};

// 1. Define Configurable Middleware
#[derive(Clone, Default)]
struct ConfigMiddleware {
    id: String,
}

impl Middleware for ConfigMiddleware {
    fn before(&self, _ctx: &mut Context) -> KoralResult<()> {
        log().lock().unwrap().push(format!("ConfigMW({})", self.id));
        Ok(())
    }
}

// Global log
fn log() -> &'static Mutex<Vec<String>> {
    static LOG: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    LOG.get_or_init(|| Mutex::new(Vec::new()))
}

// 2. Define App with Injected Middleware
#[derive(App)]
#[app(name = "inject_test", action = run)]
struct InjectApp {
    #[app(middleware)]
    config_mw: ConfigMiddleware,
    // Add a normal flag to ensure mixing works
    #[app(flags(VerboseFlag))]
    _phantom: (),
}

#[derive(Flag, Debug, PartialEq)]
#[flag(name = "verbose", short = 'v')]
#[allow(dead_code)]
struct VerboseFlag(bool);

fn run(_ctx: Context) -> KoralResult<()> {
    log().lock().unwrap().push("run".to_string());
    Ok(())
}

#[test]
fn test_inject_middleware() {
    log().lock().unwrap().clear();

    // Create app with configured middleware
    let mut app = InjectApp {
        config_mw: ConfigMiddleware {
            id: "secret-123".to_string(),
        },
        _phantom: (),
    };

    app.run(vec!["inject_test".to_string()]).unwrap();

    let l = log().lock().unwrap();
    assert_eq!(
        *l,
        vec!["ConfigMW(secret-123)".to_string(), "run".to_string()]
    );
}
