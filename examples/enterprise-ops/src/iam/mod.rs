use koral::prelude::*;

pub mod policies;
pub mod users;

#[derive(Default, App)]
#[app(name = "iam", about = "Identity Management")]
#[app(subcommands(users::UsersCmd, policies::PoliciesCmd))]
pub struct IamCmd;
