use koral::help::generate_help;
use koral::prelude::*;

#[derive(Flag, Debug, Clone, PartialEq)]
#[flag(name = "verbose", short = 'v', help = "Enable verbose output")]
struct VerboseFlag;

#[derive(App, Default)]
#[app(name = "helpy", description = "A helpful app", action = run)]
#[app(flags(VerboseFlag))]
struct HelperApp;

fn run(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

#[test]
fn test_help_contains_description() {
    let app = HelperApp;
    let help = generate_help(&app);
    std::fs::write("help_debug.txt", &help).unwrap();
    assert!(
        help.contains("A helpful app"),
        "Help should contain description"
    );
}

#[test]
fn test_help_contains_flags() {
    let app = HelperApp;
    let help = generate_help(&app);
    assert!(
        help.contains("--verbose"),
        "Help should contain flag long name"
    );
    assert!(help.contains("-v"), "Help should contain flag short name");
    assert!(
        help.contains("Enable verbose output"),
        "Help should contain flag help text"
    );
}

#[test]
fn test_help_contains_usage() {
    let app = HelperApp;
    let help = generate_help(&app);
    assert!(
        help.contains("Usage: helpy"),
        "Help should contain usage line"
    );
}
