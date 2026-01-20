use koral::prelude::*;

fn validate_positive(s: &str) -> Result<(), String> {
    let val: i32 = s.parse().map_err(|_| "Must be a number".to_string())?;
    if val > 0 {
        Ok(())
    } else {
        Err("Must be positive".to_string())
    }
}

#[derive(Flag, Debug)]
#[flag(name = "positive", validator = validate_positive)]
struct PositiveFlag(#[allow(dead_code)] i32);

#[derive(App, Default)]
#[app(name = "validator_test", action = test_action)]
#[app(flags(PositiveFlag))]
struct TestApp;

fn test_action(_ctx: Context<TestApp>) -> KoralResult<()> {
    Ok(())
}

#[test]
fn test_validator_success() {
    let mut app = TestApp;
    let res = app.run(vec!["prog".into(), "--positive".into(), "10".into()]);
    assert!(res.is_ok());
}

#[test]
fn test_validator_fail_not_number() {
    let mut app = TestApp;
    let res = app.run(vec!["prog".into(), "--positive".into(), "abc".into()]);
    assert!(res.is_err());
    let err = res.err().unwrap().to_string();
    assert!(err.contains("Must be a number"));
}

#[test]
fn test_validator_fail_negative() {
    let mut app = TestApp;
    let res = app.run(vec!["prog".into(), "--positive".into(), "-5".into()]);
    assert!(res.is_err());
    let err = res.err().unwrap().to_string();
    assert!(err.contains("Must be positive"));
}
