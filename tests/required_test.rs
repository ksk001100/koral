use koral::prelude::*;

#[derive(Flag, Debug, PartialEq)]
#[flag(name = "token", required = true)]
struct TokenFlag(String);

#[derive(Flag, Debug, PartialEq)]
#[flag(name = "optional")]
struct OptionalFlag(String);

#[derive(App, Default)]
#[app(name = "required_test_app", action = run)]
#[app(flags(TokenFlag, OptionalFlag))]
struct TestApp;

fn run(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

#[test]
fn test_missing_required_flag() {
    let mut app = TestApp;
    let err = app.run(vec!["required_test_app".to_string()]).unwrap_err();
    match err {
        koral::KoralError::MissingArgument(msg) => {
            assert!(msg.contains("Required flag '--token' is missing"));
        }
        _ => panic!("Expected MissingArgument error, got {:?}", err),
    }
}

#[test]
fn test_present_required_flag() {
    let mut app = TestApp;
    let res = app.run(vec![
        "required_test_app".to_string(),
        "--token".to_string(),
        "abc".to_string(),
    ]);
    assert!(res.is_ok());
}

#[test]
fn test_required_and_optional() {
    let mut app = TestApp;
    let res = app.run(vec![
        "required_test_app".to_string(),
        "--token".to_string(),
        "abc".to_string(),
        "--optional".to_string(),
        "def".to_string(),
    ]);
    assert!(res.is_ok());
}
