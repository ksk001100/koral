use koral::prelude::*;
use std::env;

#[derive(Flag, Debug, PartialEq)]
#[flag(name = "envflag", env = "KORAL_TEST_ENV")]
struct EnvFlag(String);

#[derive(Flag, Debug, PartialEq)]
#[flag(name = "defaultflag", default = "default_value")]
struct DefaultFlag(String);

#[derive(Default, App)]
#[app(name = "provider_test")]
#[app(flags(EnvFlag, DefaultFlag))]
struct ProviderApp;

#[test]
fn test_env_provider() {
    env::set_var("KORAL_TEST_ENV", "env_value");

    let app = ProviderApp;
    let parser = koral::internal::parser::Parser::new(app.flags());

    // Pass empty args, should pick up env and default
    let args: Vec<String> = vec![];
    let ctx = parser.parse(&args).expect("Parse failed");

    // Check env flag
    assert_eq!(
        ctx.flags.get("envflag").unwrap().as_deref(),
        Some("env_value")
    );

    // Check default flag
    assert_eq!(
        ctx.flags.get("defaultflag").unwrap().as_deref(),
        Some("default_value")
    );

    env::remove_var("KORAL_TEST_ENV");
}
