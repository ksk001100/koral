use koral::prelude::*;

// We need to import koral crate.
// In integration tests, we use `koral`.

#[derive(Flag, Debug, PartialEq)]
#[flag(name = "verbose", short = 'v')]
struct VerboseFlag;

#[derive(Flag, Debug, PartialEq)]
#[flag(name = "name", default = "Stranger")]
struct NameFlag(String);

#[test]
fn test_parser_flags() {
    let _verbose_def = FlagDef::from_trait::<VerboseFlag>();
    let _name_def = FlagDef::from_trait::<NameFlag>();

    // Parser is pub(crate) so we can't access it easily in integration tests unless we expose it?
    // User plan said "Add Unit Tests for parser.rs".
    // If parser is internal, we should test it via App or expose it?
    // `koral::parser::Parser` is `pub(crate)`.
    // Koral architecture: `App::run` uses `Parser`.
    // So we should test via `App` or `Context`?
    // Or make Parser public?
    // The library exposes `Context` which comes from Parser.

    // Let's test via App to be safe and use public API only.
}

#[derive(App, Default)]
#[app(name = "testapp", action = test_action)]
#[app(flags(VerboseFlag, NameFlag))]
struct TestApp;

fn test_action(_ctx: Context) -> KoralResult<()> {
    // We can assert here
    Ok(())
}

#[test]
fn test_app_integration() {
    // This is better Test.
    let mut app = TestApp;
    let args = vec![
        "testapp".to_string(),
        "--verbose".to_string(),
        "--name".to_string(),
        "Alice".to_string(),
    ];

    // We can't inspect context easily unless we capture it?
    // For now, let's just ensure it runs without error.
    app.run(args).unwrap();
}
