use koral::prelude::*;
use std::sync::{Arc, Mutex};

// Store logs in state instead of global for better isolation
#[derive(Default, Debug)]
struct MwLog {
    events: Vec<String>,
}

#[derive(Default)]
struct Mw1;

impl Middleware for Mw1 {
    fn before(&self, ctx: &mut Context) -> KoralResult<()> {
        if let Some(log) = ctx.state::<Arc<Mutex<MwLog>>>() {
            log.lock().unwrap().events.push("mw1_before".to_string());
        }
        Ok(())
    }
    fn after(&self, ctx: &mut Context) -> KoralResult<()> {
        if let Some(log) = ctx.state::<Arc<Mutex<MwLog>>>() {
            log.lock().unwrap().events.push("mw1_after".to_string());
        }
        Ok(())
    }
}

#[derive(Default)]
struct Mw2;

impl Middleware for Mw2 {
    fn before(&self, ctx: &mut Context) -> KoralResult<()> {
        if let Some(log) = ctx.state::<Arc<Mutex<MwLog>>>() {
            log.lock().unwrap().events.push("mw2_before".to_string());
        }
        Ok(())
    }
    fn after(&self, ctx: &mut Context) -> KoralResult<()> {
        if let Some(log) = ctx.state::<Arc<Mutex<MwLog>>>() {
            log.lock().unwrap().events.push("mw2_after".to_string());
        }
        Ok(())
    }
}

#[derive(App)]
#[app(name = "order_test", action = run)]
#[app(middleware(Mw1, Mw2))]
struct OrderApp;

fn run(ctx: Context) -> KoralResult<()> {
    if let Some(log) = ctx.state::<Arc<Mutex<MwLog>>>() {
        log.lock().unwrap().events.push("run".to_string());
    }
    Ok(())
}

#[test]
fn test_multiple_middleware_order() {
    let log = Arc::new(Mutex::new(MwLog::default()));
    let mut app = OrderApp;
    let mut state: Arc<Mutex<MwLog>> = log.clone();

    app.run_with_state(&mut state, vec!["prog".to_string()])
        .unwrap();

    let events = log.lock().unwrap().events.clone();
    assert_eq!(
        events,
        vec!["mw1_before", "mw2_before", "run", "mw2_after", "mw1_after"]
    );
}
