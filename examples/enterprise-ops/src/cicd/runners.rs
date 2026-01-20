use crate::context::AppContext;
use koral::prelude::*;

#[derive(Subcommand)]
#[subcommand(name = "runners", about = "Self-hosted runners")]
#[subcommand(subcommands(RegisterRunnerCmd))]
pub enum RunnersCmd {
    #[subcommand(name = "register")]
    Register(RegisterRunnerCmd),
}

impl Default for RunnersCmd {
    fn default() -> Self {
        Self::Register(RegisterRunnerCmd::default())
    }
}

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
pub struct RegisterRunnerCmd;

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
