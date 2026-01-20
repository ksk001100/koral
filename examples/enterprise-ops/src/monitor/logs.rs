use crate::context::AppContext;
use koral::prelude::*;

#[derive(Default, App, FromArgs)]
#[app(name = "logs", about = "Search global logs")]
#[app(subcommands(SearchLogsCmd))]
pub struct LogsCmd;

#[derive(Flag, Debug)]
#[flag(name = "query", required = true, help = "LogQL Query")]
struct LogQueryFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "limit", default = 100)]
struct LimitFlag(u32);

#[derive(Default, App)]
#[app(name = "search")]
#[app(flags(LogQueryFlag, LimitFlag))]
#[app(action = search_logs)]
struct SearchLogsCmd;

fn search_logs(
    _ctx: State<AppContext>,
    query: FlagArg<LogQueryFlag>,
    limit: FlagArg<LimitFlag>,
) -> KoralResult<()> {
    println!("Searching logs (limit={}): {}", *limit, *query);
    println!("[error] 2023-10-25T10:00:01 Connection refused");
    Ok(())
}
