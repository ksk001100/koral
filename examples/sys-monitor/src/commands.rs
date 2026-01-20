use crate::state::AppState;
use koral::prelude::*;

#[derive(FlagValue, Clone, Debug, PartialEq)]
pub enum OutputFormat {
    Json,
    Text,
    Yaml,
}

#[derive(Flag, Debug)]
#[flag(
    name = "format",
    short = 'f',
    default = "text",
    help = "Output format (json, text, yaml)"
)]
pub struct FormatFlag(#[allow(dead_code)] pub OutputFormat);

#[derive(App, Clone, Debug, Default, PartialEq)]
#[app(name = "status", action = status_handler, help = "Check system status")]
#[app(flags(FormatFlag))]
pub struct StatusCmd;

fn status_handler(state: State<AppState>, format: FlagArg<FormatFlag>) -> KoralResult<()> {
    println!("Status Check:");
    println!("  Database: {}", state.db_url);
    println!("  Counter: {}", *state.counter.lock().unwrap());
    println!("  Format: {:?}", format.0);
    Ok(())
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum Commands {
    #[subcommand(name = "status")]
    Status(StatusCmd),
}

impl Default for Commands {
    fn default() -> Self {
        Self::Status(Default::default())
    }
}
