use crate::common::print_output;
use crate::context::AppContext;
use koral::prelude::*;
use serde::Serialize;

#[derive(Default, App)]
#[app(name = "users", about = "Manage Users")]
#[app(subcommands(ListUsersCmd, InviteCmd))]
pub struct UsersCmd;

#[derive(Serialize, Debug)]
struct User {
    email: String,
    role: String,
    status: String,
}

#[derive(Default, App)]
#[app(name = "list")]
#[app(action = list_users)]
struct ListUsersCmd;

fn list_users(ctx: State<AppContext>) -> KoralResult<()> {
    let users = vec![
        User {
            email: "alice@corp.com".into(),
            role: "Admin".into(),
            status: "Active".into(),
        },
        User {
            email: "bob@corp.com".into(),
            role: "Developer".into(),
            status: "Invited".into(),
        },
    ];
    print_output(&users, ctx.global_flags.output);
    Ok(())
}

#[derive(Flag, Debug)]
#[flag(name = "email", required = true)]
struct EmailFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "role", default = "Developer")]
struct RoleFlag(String);

#[derive(Default, App)]
#[app(name = "invite")]
#[app(flags(EmailFlag, RoleFlag))]
#[app(action = invite_user)]
struct InviteCmd;

fn invite_user(
    _ctx: State<AppContext>,
    email: FlagArg<EmailFlag>,
    role: FlagArg<RoleFlag>,
) -> KoralResult<()> {
    println!("Inviting {} to role {}...", *email, *role);
    Ok(())
}
