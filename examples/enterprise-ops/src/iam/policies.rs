use crate::context::AppContext;
use koral::prelude::*;

#[derive(Subcommand)]
#[subcommand(name = "policies", about = "Manage IAM Policies")]
#[subcommand(subcommands(ValidatePolicyCmd))]
pub enum PoliciesCmd {
    #[subcommand(name = "validate")]
    Validate(ValidatePolicyCmd),
}

impl Default for PoliciesCmd {
    fn default() -> Self {
        Self::Validate(ValidatePolicyCmd::default())
    }
}

#[derive(Flag, Debug)]
#[flag(name = "file", required = true)]
struct FileFlag(String);

#[derive(Default, App)]
#[app(name = "validate")]
#[app(flags(FileFlag))]
#[app(action = validate_policy)]
pub struct ValidatePolicyCmd;

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
