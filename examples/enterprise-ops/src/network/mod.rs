use koral::prelude::*;

pub mod vpc;

#[derive(Subcommand)]
#[subcommand(name = "network", about = "Network Resources")]
#[subcommand(subcommands(vpc::VpcCmd))]
pub enum NetworkCmd {
    #[subcommand(name = "vpc")]
    Vpc(vpc::VpcCmd),
}

impl Default for NetworkCmd {
    fn default() -> Self {
        Self::Vpc(vpc::VpcCmd::default())
    }
}
