use crate::context::AppContext;
use koral::prelude::*;

#[derive(Default, App)]
#[app(name = "runners", about = "Self-hosted runners")]
#[app(subcommands(RegisterRunnerCmd))]
pub struct RunnersCmd;

#[derive(Flag, Debug)]
#[flag(name = "token", required = true, env = "RUNNER_TOKEN")]
struct TokenFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "tags")]
struct TagsFlag(String);

#[derive(Default, App)]
#[app(name = "register")]
#[app(flags(TokenFlag, TagsFlag))]
#[app(action = register_runner)]
struct RegisterRunnerCmd;

fn register_runner(
    _ctx: State<AppContext>,
    _token: FlagArg<TokenFlag>,
    tags: FlagArg<TagsFlag>,
) -> KoralResult<()> {
    println!("Registering runner with token '******'...");
    if !tags.is_empty() {
        println!("Tags: {:?}", *tags);
    }
    Ok(())
}
