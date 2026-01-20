use koral::prelude::*;

#[derive(Flag, Debug)]
#[flag(name = "test")]
struct TestFlag(#[allow(dead_code)] String);

#[derive(App, Default)]
#[app(name = "opt_test", action = run)]
#[app(flags(TestFlag))]
struct TestApp;

// Handler using Option<FlagArg>
fn run(ctx: Context) -> KoralResult<()> {
    let opt_arg = Option::<FlagArg<TestFlag>>::from_context(&ctx);
    // We can't rely on return value of run to inspect.
    // But we check if it compiles and runs without error.
    match opt_arg {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[test]
fn test_option_extraction() {
    let mut app = TestApp;
    // Missing flag -> should suffice (Option returns None, which is Ok)
    let res = app.run(vec!["prog".to_string()]);
    assert!(res.is_ok());

    // Present flag -> should suffice
    let res = app.run(vec![
        "prog".to_string(),
        "--test".to_string(),
        "val".to_string(),
    ]);
    assert!(res.is_ok());
}
