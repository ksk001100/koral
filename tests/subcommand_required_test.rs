use koral::prelude::*;

#[derive(Flag, Debug)]
#[flag(name = "user", required = true)]
struct UserFlag(#[allow(dead_code)] String);

#[derive(App, Default)]
#[app(name = "child", action = child_run)]
struct ChildCmd;

fn child_run(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

#[derive(Subcommand)]
enum Commands {
    #[subcommand(name = "child")]
    Child(ChildCmd),
}

impl Default for Commands {
    fn default() -> Self {
        Self::Child(ChildCmd::default())
    }
}

#[derive(App, Default)]
#[app(name = "parent", action = parent_run)]
#[app(flags(UserFlag))]
struct ParentApp {
    #[app(subcommand)]
    cmd: Commands,
}

fn parent_run(_ctx: Context) -> KoralResult<()> {
    Ok(())
}

#[test]
fn test_parent_required_missing_fails() {
    let mut app = ParentApp {
        cmd: Commands::Child(ChildCmd),
    };
    let res = app.run(vec!["parent".to_string()]);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    assert!(err.to_string().contains("user"));
}

#[test]
fn test_child_runs_without_parent_flag() {
    // This is the core fix verification: child execution should NOT trigger parent `user` check
    let mut app = ParentApp {
        cmd: Commands::Child(ChildCmd),
    };
    let res = app.run(vec!["parent".to_string(), "child".to_string()]);
    assert!(
        res.is_ok(),
        "Child should run without parent required flag: {:?}",
        res.err()
    );
}

#[test]
fn test_parent_runs_with_flag() {
    let mut app = ParentApp {
        cmd: Commands::Child(ChildCmd),
    };
    let res = app.run(vec![
        "parent".to_string(),
        "--user".to_string(),
        "Alice".to_string(),
    ]);
    assert!(res.is_ok());
}
