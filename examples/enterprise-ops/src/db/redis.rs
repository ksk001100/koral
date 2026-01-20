use crate::context::AppContext;
use koral::prelude::*;

#[derive(Default, App, FromArgs)]
#[app(name = "redis", about = "Manage Redis clusters")]
#[app(subcommands(FlushCacheCmd))]
pub struct RedisCmd;

#[derive(Flag, Debug)]
#[flag(name = "cluster", required = true)]
struct ClusterFlag(String);

#[derive(Default, App)]
#[app(name = "flush", about = "Flush all keys")]
#[app(flags(ClusterFlag))]
#[app(action = flush_redis)]
struct FlushCacheCmd;

fn flush_redis(_ctx: State<AppContext>, cluster: FlagArg<ClusterFlag>) -> KoralResult<()> {
    println!("Flushing redis cluster '{}'...", *cluster);
    Ok(())
}
