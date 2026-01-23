use crate::state::SharedState;
use koral::prelude::*;

// --- Flags ---

#[derive(Flag, Debug, Default)]
#[flag(name = "verbose", short = 'v', help = "Enable verbose output")]
pub struct VerboseFlag;

#[derive(Flag, Debug, Default)]
#[flag(help = "Show all tasks including completed ones")]
pub struct AllFlag;

#[derive(FlagValue, Clone, Debug, PartialEq, Default)]
pub enum ListFormat {
    #[default]
    Simple,
    Detailed,
}

fn validate_format(s: &str) -> Result<(), String> {
    match s {
        "simple" | "detailed" => Ok(()),
        _ => Err("Format must be 'simple' or 'detailed'".to_string()),
    }
}

#[derive(Flag, Debug, Default)]
#[flag(
    name = "format",
    short = 'f',
    default = "simple",
    help = "Output format (simple, detailed)",
    validator = validate_format,
    aliases = "fmt"
)]
pub struct FormatFlag(#[allow(dead_code)] pub ListFormat);

// --- Command ---

#[derive(Default, koral::App)]
#[app(name = "list", action = list_tasks)]
#[app(flags(AllFlag, FormatFlag))]
pub struct ListCmd;

fn list_tasks(ctx: Context) -> KoralResult<()> {
    let show_all = ctx.get::<AllFlag>().unwrap_or(false);
    let format = ctx.get::<FormatFlag>().expect("Default value");

    let state = ctx.state::<SharedState>().ok_or_else(|| {
        koral::clap::Error::raw(
            koral::clap::error::ErrorKind::InvalidValue,
            "State not found",
        )
    })?;

    let guard = state.lock().map_err(|_| {
        koral::clap::Error::raw(koral::clap::error::ErrorKind::InvalidValue, "Lock poisoned")
    })?;

    println!("Tasks (Format: {:?}):", format);
    for (i, task) in guard.tasks.iter().enumerate() {
        match format {
            ListFormat::Simple => println!("  - {}", task),
            ListFormat::Detailed => println!("  [{}] {}", i + 1, task),
        }
    }

    if show_all {
        println!("  (Showing all tasks - dummy impl)");
    }
    Ok(())
}
