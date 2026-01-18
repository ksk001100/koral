use koral::prelude::*;

// 1. Enum for choices (mapped by name)
#[derive(FlagValue, Clone, Debug, PartialEq)]
enum OutputFormat {
    Json,
    Text,
    Table,
}

#[derive(Flag, Debug)]
#[flag(
    name = "format",
    short = 'f',
    default = "text",
    help = "Output format (json, text, table)"
)]
struct FormatFlag(#[allow(dead_code)] OutputFormat);

// 2. Wrap a primitive type (NewType pattern)
// This uses FromStr on the inner type (u32).
#[derive(FlagValue, Clone, Debug, PartialEq)]
struct Timeout(u32);

#[derive(Flag, Debug)]
#[flag(
    name = "timeout",
    short = 't',
    default = "30",
    help = "Timeout in seconds"
)]
struct TimeoutFlag(#[allow(dead_code)] Timeout);

#[derive(App)]
#[app(name = "custom_types", action = run)]
#[app(flags(FormatFlag, TimeoutFlag))]
struct CustomTypeApp;

fn run(ctx: Context<CustomTypeApp>) -> KoralResult<()> {
    let format = ctx.get::<FormatFlag>().expect("Default set");
    let timeout = ctx.get::<TimeoutFlag>().expect("Default set");

    println!("Configuration:");
    println!("  Format: {:?}", format);
    println!("  Timeout: {:?} seconds", timeout);

    Ok(())
}

fn main() -> KoralResult<()> {
    CustomTypeApp.run(std::env::args().collect())
}
