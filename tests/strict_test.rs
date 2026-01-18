use koral::prelude::*;

#[derive(Flag, Debug, Default)]
#[flag(name = "verbose", short = 'v')]
struct VerboseFlag;

// Loose App (default)
#[derive(App)]
#[app(name = "loose", action = loose_action)]
#[app(flags(VerboseFlag))]
struct LooseApp;

fn loose_action(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

// Strict App
#[derive(App)]
#[app(name = "strict", action = strict_action)]
#[app(flags(VerboseFlag), strict)]
struct StrictApp;

fn strict_action(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

#[test]
fn test_loose_unknown_flags() {
    let mut app = LooseApp;
    // --unknown should be positional
    app.run(vec!["prog".into(), "--unknown".into()])
        .expect("Should pass in loose mode");

    // -x should be positional
    app.run(vec!["prog".into(), "-x".into()])
        .expect("Should pass in loose mode");

    // But -vx should fail because it's interpreted as group `-v` (valid) and `-x` (invalid inside group)
    // Wait, let's verify this behavior.
    assert!(
        app.run(vec!["prog".into(), "-vx".into()]).is_err(),
        "-vx should fail even in loose mode if v is flag"
    );
}

#[test]
fn test_strict_unknown_flags() {
    let mut app = StrictApp;

    // --unknown should fail
    assert!(
        app.run(vec!["prog".into(), "--unknown".into()]).is_err(),
        "--unknown should fail in strict mode"
    );

    // -x should fail
    assert!(
        app.run(vec!["prog".into(), "-x".into()]).is_err(),
        "-x should fail in strict mode"
    );
}

#[test]
fn test_strict_valid() {
    let mut app = StrictApp;
    app.run(vec!["prog".into(), "-v".into()])
        .expect("Valid flags should pass");
}
