use crate::context::AppContext;
use koral::prelude::*;

#[derive(Default, App)]
#[app(name = "policies", about = "Manage IAM Policies")]
#[app(subcommands(ValidatePolicyCmd))]
pub struct PoliciesCmd;

#[derive(Flag, Debug)]
#[flag(name = "file", required = true)]
struct FileFlag(String);

#[derive(Default, App)]
#[app(name = "validate")]
#[app(flags(FileFlag))]
#[app(action = validate_policy)]
struct ValidatePolicyCmd;

fn validate_policy(_ctx: State<AppContext>, file: FlagArg<FileFlag>) -> KoralResult<()> {
    println!("Validating policy file '{}'...", *file);
    // Simulate validation
    if file.ends_with("invalid.json") {
        return Err(koral::KoralError::Validation(
            "Policy contains wildcard permission '*:*'".to_string(),
        ));
    }
    println!("Policy is valid.");
    Ok(())
}
