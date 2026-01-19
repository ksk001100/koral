use koral::prelude::*;

#[derive(App)]
#[app(name = "strict_app", action = run, strict)]
struct StrictApp;

fn run(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

#[derive(App)]
#[app(name = "loose_app", action = run)]
struct LooseApp;

#[test]
fn test_strict_mode_unknown_long_flag() {
    let mut app = StrictApp;
    let res = app.run(vec!["strict_app".to_string(), "--unknown".to_string()]);
    assert!(res.is_err());
    match res.unwrap_err() {
        koral::KoralError::UnknownFlag(msg) => {
            assert!(msg.contains("Unknown flag '--unknown'"));
        }
        _ => panic!("Expected UnknownFlag error"),
    }
}

#[test]
fn test_strict_mode_unknown_short_flag() {
    let mut app = StrictApp;
    let res = app.run(vec!["strict_app".to_string(), "-u".to_string()]);
    assert!(res.is_err());
    match res.unwrap_err() {
        koral::KoralError::UnknownFlag(msg) => {
            assert!(msg.contains("Unknown short flag 'u'"));
        }
        _ => panic!("Expected UnknownFlag error"),
    }
}

#[test]
fn test_loose_mode_ignores_unknown() {
    let mut app = LooseApp;
    // Loose mode should treat unknown args as positionals
    let res = app.run(vec!["loose_app".to_string(), "--unknown".to_string()]);
    assert!(res.is_ok());
}
