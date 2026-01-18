use koral::prelude::*;

#[derive(Flag, Debug, Default)]
#[flag(name = "list", aliases = "ls, l")]
struct ListFlag;

#[derive(Subcommand)]
enum Commands {
    #[subcommand(name = "add", aliases = "a, new")]
    Add(AddCmd),
    #[subcommand(name = "remove", aliases = "rm, delete")]
    Remove(RemoveCmd),
}

#[derive(App, Default)]
#[app(name = "add", action = add_action)]
struct AddCmd;

fn add_action(_ctx: Context<AddCmd>) -> KoralResult<()> {
    Ok(())
}

#[derive(App, Default)]
#[app(name = "remove", action = remove_action)]
struct RemoveCmd;

fn remove_action(_ctx: Context<RemoveCmd>) -> KoralResult<()> {
    Ok(())
}

#[derive(App)]
#[app(name = "alias_test", action = test_action)]
#[app(subcommand = cmd)]
#[app(flags(ListFlag))]
struct TestApp {
    cmd: Commands,
}

fn test_action(ctx: Context<TestApp>) -> KoralResult<()> {
    if let Some(true) = ctx.get::<ListFlag>() {
        // Flag matched
    }
    Ok(())
}

#[test]
fn test_flag_alias() {
    let mut app = TestApp {
        cmd: Commands::Add(AddCmd), // Default
    };

    // Check --list
    let res = app.run(vec!["prog".into(), "--list".into()]);
    assert!(res.is_ok());

    // Check --ls
    let res = app.run(vec!["prog".into(), "--ls".into()]);
    assert!(res.is_ok());

    // Check --l (technically --l should work if alias is defined as "l")
    // Short flags usually don't use --, but aliases are parsed as long flags if mapped to name string?
    // Wait, alias "l" -> in parser, name_part == "l"?
    // If user types "--l", yes.
    // If user types "-l", parser logic for short flag check:
    // Short flag is char. Alias is string.
    // The current parser checks `flag.aliases.iter().any(|a| a == name_part)`.
    // If name_part comes from `trim_start_matches("--")`, then "--l" -> "l" -> matches alias "l".
    let res = app.run(vec!["prog".into(), "--l".into()]);
    assert!(res.is_ok());
}

#[test]
fn test_subcommand_alias() {
    let mut app = TestApp {
        cmd: Commands::Add(AddCmd),
    };

    // Check add
    let res = app.run(vec!["prog".into(), "add".into()]);
    assert!(res.is_ok());

    // Check "a"
    let res = app.run(vec!["prog".into(), "a".into()]);
    assert!(res.is_ok());

    // Check "new"
    let res = app.run(vec!["prog".into(), "new".into()]);
    assert!(res.is_ok());

    // Check "rm"
    let res = app.run(vec!["prog".into(), "rm".into()]);
    assert!(res.is_ok());
}
