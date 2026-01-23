use koral::prelude::*;

#[derive(App, Default)]
#[app(name = "neg_test", action = run)]
struct NegApp;

fn run(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

#[test]
fn test_negative_number_as_positional() {
    let app = NegApp;
    let args = vec![
        "neg_test".to_string(),
        "-100".to_string(),
        "-5.5".to_string(),
    ];

    // Using parser directly via App internal logic is hard to inspect result without custom handler.
    // Instead we can use `App::run_with_state` or check args in handler with DI or modifying `NegApp`
    // Or just use `Parser` directly.

    let parser = koral::internal::parser::Parser::new(app.flags());
    let ctx = parser.parse(&args[1..]).unwrap();

    assert_eq!(ctx.args.len(), 2);
    assert_eq!(ctx.args[0], "-100");
    assert_eq!(ctx.args[1], "-5.5");
}

#[derive(Flag, Debug, PartialEq)]
#[flag(name = "number", short = '1')] // '1' is a flag!
struct NumberFlag;

#[derive(App, Default)]
#[app(name = "conflict_test")]
#[app(flags(NumberFlag))]
struct ConflictApp;

#[test]
fn test_negative_number_conflict() {
    // If we define a flag '1', then '-100' should NOT be a negative number,
    // but flag '1' and group '0', '0'.

    let app = ConflictApp;
    let parser = koral::internal::parser::Parser::new(app.flags()).strict(true);
    let args = vec!["-100".to_string()];

    // Should fail because '0' is not a known flag
    let result = parser.parse(&args);
    assert!(result.is_err());

    if let Err(koral::KoralError::UnknownFlag(msg)) = result {
        assert!(msg.contains("'0'"));
    } else {
        panic!("Expected UnknownFlag error");
    }
}
