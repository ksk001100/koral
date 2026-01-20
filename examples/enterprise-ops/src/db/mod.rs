use koral::prelude::*;

pub mod postgres;
pub mod redis;

#[derive(Subcommand)]
#[subcommand(name = "db", about = "Manage managed databases")]
#[subcommand(subcommands(postgres::PostgresCmd, redis::RedisCmd))]
pub enum DbCmd {
    Postgres(postgres::PostgresCmd),
    Redis(redis::RedisCmd),
}

impl Default for DbCmd {
    fn default() -> Self {
        Self::Postgres(postgres::PostgresCmd::default())
    }
}
