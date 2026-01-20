use koral::prelude::*;

pub mod vpc;

#[derive(Default, App)]
#[app(name = "network", about = "Network Resources")]
#[app(subcommands(vpc::VpcCmd))]
pub struct NetworkCmd;
