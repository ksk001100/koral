use koral::prelude::*;

pub mod policies;
pub mod users;

#[derive(Subcommand)]
#[subcommand(name = "iam", about = "Identity Management")]
#[subcommand(subcommands(users::UsersCmd, policies::PoliciesCmd))]
pub enum IamCmd {
    #[subcommand(name = "users")]
    Users(users::UsersCmd),
    #[subcommand(name = "policies")]
    Policies(policies::PoliciesCmd),
}

impl Default for IamCmd {
    fn default() -> Self {
        Self::Users(users::UsersCmd::default())
    }
}
