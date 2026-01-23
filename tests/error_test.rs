use koral::prelude::*;

#[derive(Flag, Debug, Clone, PartialEq)]
#[flag(name = "strict", short = 's')]
struct StrictFlag;

// Manual implementation to enable strict mode
struct ManualStrictApp;

impl AppTrait for ManualStrictApp {
    fn name(&self) -> &str {
        "manual_strict"
    }
    fn is_strict(&self) -> bool {
        true
    }
    fn flags(&self) -> Vec<FlagDef> {
        vec![FlagDef::from_trait::<StrictFlag>()]
    }
    fn execute(&mut self, _ctx: Context) -> KoralResult<()> {
        Ok(())
    }
}

fn run_strict(args: Vec<&str>) -> KoralResult<()> {
    let mut app = ManualStrictApp;
    let args: Vec<String> = args.into_iter().map(|s| s.to_string()).collect();
    app.run(args)
}

#[test]
fn test_strict_unknown_flag() {
    let res = run_strict(vec!["prog", "--unknown"]);
    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().kind(),
        clap::error::ErrorKind::UnknownArgument
    );
}

#[test]
fn test_strict_known_flag() {
    let res = run_strict(vec!["prog", "--strict"]);
    assert!(res.is_ok());
}

#[test]
fn test_strict_positional() {
    let res = run_strict(vec!["prog", "positional"]);
    assert!(res.is_ok());
}

#[test]
fn test_strict_dash_positional() {
    // If argument starts with -, but is not a known flag.
    let res = run_strict(vec!["prog", "-x"]);
    assert!(res.is_err());
}
