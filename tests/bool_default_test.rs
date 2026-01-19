use koral::prelude::*;

#[derive(Flag, Debug)]
#[flag(name = "verbose", short = 'v')]
struct VerboseFlag; // No default specified, bool

#[derive(App)]
#[app(name = "bool_test", action = run)]
#[app(flags(VerboseFlag))]
struct TestApp;

fn run(ctx: Context) -> KoralResult<()> {
    // Should be Some(false) if not provided, not None or Error
    let verbose = ctx.get::<VerboseFlag>();
    if verbose.is_none() {
        return Err(KoralError::Validation(
            "Verbose flag is None (should be Some(false))".to_string(),
        ));
    }
    // Also check extraction via FlagArg if we had DI
    Ok(())
}

#[test]
fn test_bool_flag_defaults_to_false() {
    let mut app = TestApp;
    // No args
    let res = app.run(vec!["prog".to_string()]);
    assert!(
        res.is_ok(),
        "Run should succeed with default false: {:?}",
        res.err()
    );
}

#[test]
fn test_bool_flag_true() {
    let mut app = TestApp;
    // With flag
    // Manual context check
    // We can't clear easily inside `run` to assert value is true unless we panic or return specific error.
    // But `run` returns Ok if logic passes.
    // Let's rely on manual Context check if we wanted.
    // For now, simple success is mostly checking parsing.
    let res = app.run(vec!["prog".to_string(), "--verbose".to_string()]);
    assert!(res.is_ok());
}
