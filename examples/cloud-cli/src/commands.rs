use crate::domain::Instance;
use crate::flags::{FormatFlag, InstanceTypeFlag, OutputFormat, RegionFlag, UserFlag};
// use crate::middleware::UserContext;
use crate::state::CloudState;
use koral::prelude::*;
use uuid::Uuid;

// --- Subcommands Definition ---

#[derive(Subcommand)]
pub enum Commands {
    #[subcommand(name = "login", help = "Authenticate with the cloud provider")]
    Login(LoginCmd),

    #[subcommand(name = "instance", help = "Manage compute instances")]
    Instance(InstanceCmd),

    #[subcommand(name = "s3", help = "Manage S3 buckets")]
    S3(S3Cmd),
}

impl Default for Commands {
    fn default() -> Self {
        Self::Login(Default::default())
    }
}

// --- Login Command ---

#[derive(App, FromArgs, Default)]
#[app(name = "login", action = login_handler)]
#[app(flags(UserFlag))]
pub struct LoginCmd;

fn login_handler(state: State<CloudState>, user: FlagArg<UserFlag>) -> KoralResult<()> {
    let token = Uuid::new_v4().to_string();
    state
        .auth_tokens
        .lock()
        .unwrap()
        .insert(token.clone(), user.0.clone());
    println!("Successfully logged in as '{}'.", user.0);
    println!("Your session token is: {}", token);
    println!("Set it to CLOUD_CLI_TOKEN env var or use --token flag.");
    Ok(())
}

// --- Instance Commands ---

#[derive(Subcommand)]
pub enum InstanceCmd {
    #[subcommand(name = "launch", help = "Launch a new instance")]
    Launch(LaunchInstanceCmd),

    #[subcommand(name = "list", help = "List instances")]
    List(ListInstancesCmd),

    #[subcommand(name = "terminate", help = "Terminate an instance")]
    Terminate(TerminateInstanceCmd),
}

impl Default for InstanceCmd {
    fn default() -> Self {
        Self::Launch(Default::default())
    }
}

#[derive(App, FromArgs, Default)]
#[app(name = "launch", action = launch_instance_handler)]
#[app(flags(InstanceTypeFlag, RegionFlag))]
pub struct LaunchInstanceCmd;

fn launch_instance_handler(
    state: State<CloudState>,
    // Removed injection: user_ctx: Extension<UserContext>,
    instance_type: FlagArg<InstanceTypeFlag>,
    region: FlagArg<RegionFlag>,
) -> KoralResult<()> {
    // Retrieve user from state (populated by middleware)
    let username = state
        .current_user
        .lock()
        .unwrap()
        .clone()
        .ok_or(KoralError::Validation("User not authenticated".into()))?;

    println!("Launching instance for user: {}", username);

    let i_type = instance_type.0.clone();
    let i_region = region.0.clone();

    let instance = Instance::new("my-server".to_string(), i_type.into(), i_region.into());

    let id = instance.id.clone();
    state.add_instance(instance);

    println!("Instance launched successfully: {}", id);
    Ok(())
}

#[derive(App, FromArgs, Default)]
#[app(name = "list", action = list_instances_handler)]
#[app(flags(RegionFlag, FormatFlag))]
pub struct ListInstancesCmd;

fn list_instances_handler(
    state: State<CloudState>,
    // Removed injection: _user_ctx: Extension<UserContext>,
    region: FlagArg<RegionFlag>,
    format: FlagArg<FormatFlag>,
) -> KoralResult<()> {
    // Ensure we are authenticated (though Middleware should have enforced it)
    if state.current_user.lock().unwrap().is_none() {
        return Err(KoralError::Validation("User not authenticated".into()));
    }

    let instances = state.list_instances();

    // Use region flag to avoid unused warning
    let _r = region.0.clone();

    match format.0 {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&instances).unwrap());
        }
        _ => {
            println!("Instances ({})", instances.len());
            for inst in instances {
                println!(
                    "- [{}] {} ({}) in {}",
                    inst.id, inst.name, inst.instance_type, inst.region
                );
            }
        }
    }
    Ok(())
}

#[derive(App, FromArgs, Default)]
#[app(name = "terminate", action = terminate_instance_handler)]
pub struct TerminateInstanceCmd {
    // Positional arg: Instance ID
    // Koral handles positional args via `Args`.
}

fn terminate_instance_handler(
    state: State<CloudState>,
    // Removed injection: _user_ctx: Extension<UserContext>,
    args: Args,
) -> KoralResult<()> {
    if state.current_user.lock().unwrap().is_none() {
        return Err(KoralError::Validation("User not authenticated".into()));
    }

    if args.is_empty() {
        return Err(KoralError::Validation("Missing instance ID".into()));
    }
    let id = &args[0];
    if state.terminate_instance(id).is_some() {
        println!("Instance {} terminated.", id);
    } else {
        println!("Instance {} not found.", id);
    }
    Ok(())
}

// --- S3 Commands ---

#[derive(Subcommand)]
pub enum S3Cmd {
    #[subcommand(name = "ls", help = "List buckets")]
    ListBuckets(ListBucketsCmd),

    #[subcommand(name = "mb", help = "Make bucket")]
    MakeBucket(MakeBucketCmd),
}

impl Default for S3Cmd {
    fn default() -> Self {
        Self::ListBuckets(Default::default())
    }
}

#[derive(App, FromArgs, Default)]
#[app(name = "ls", action = list_buckets_handler)]
pub struct ListBucketsCmd;

fn list_buckets_handler(
    state: State<CloudState>,
    // Removed injection: _user_ctx: Extension<UserContext>,
) -> KoralResult<()> {
    if state.current_user.lock().unwrap().is_none() {
        return Err(KoralError::Validation("User not authenticated".into()));
    }

    let buckets = state.list_buckets();
    println!("Buckets:");
    if buckets.is_empty() {
        println!("  (none)");
    }
    for b in buckets {
        println!("  s3://{}", b.name);
    }
    Ok(())
}

#[derive(App, FromArgs, Default)]
#[app(name = "mb", action = make_bucket_handler)]
#[app(flags(RegionFlag))]
pub struct MakeBucketCmd;

fn make_bucket_handler(
    state: State<CloudState>,
    args: Args,
    region: FlagArg<RegionFlag>,
) -> KoralResult<()> {
    if state.current_user.lock().unwrap().is_none() {
        return Err(KoralError::Validation("User not authenticated".into()));
    }

    if args.is_empty() {
        return Err(KoralError::Validation("Missing bucket name".into()));
    }
    let name = &args[0];

    // Check if exists?
    // For simplicity just overwrite or add
    let bucket = crate::domain::Bucket::new(name.clone(), region.0.into());
    state.add_bucket(bucket);
    println!("make_bucket: s3://{}", name);
    Ok(())
}
