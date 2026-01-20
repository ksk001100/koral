use crate::context::AppContext;
use koral::prelude::*;

#[derive(Subcommand)]
#[subcommand(name = "redis", about = "Manage Redis clusters")]
#[subcommand(subcommands(FlushCacheCmd))]
pub enum RedisCmd {
    #[subcommand(name = "flush")]
    Flush(FlushCacheCmd),
}

impl Default for RedisCmd {
    fn default() -> Self {
        Self::Flush(FlushCacheCmd::default())
    }
}

#[derive(Flag, Debug)]
#[flag(name = "cluster", required = true)]
struct ClusterFlag(String);

#[derive(Default, App)]
#[app(name = "flush", about = "Flush all keys")]
#[app(flags(ClusterFlag))]
#[app(action = flush_redis)]
pub struct FlushCacheCmd;

fn flush_redis(_ctx: State<AppContext>, cluster: FlagArg<ClusterFlag>) -> KoralResult<()> {
    println!("Flushing redis cluster '{}'...", *cluster);
    Ok(())
}
