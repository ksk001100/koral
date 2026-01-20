use crate::common::print_output;
use crate::context::AppContext;
use koral::prelude::*;
use serde::Serialize;

#[derive(Subcommand)]
#[subcommand(name = "nodepools", about = "Manage Node Pools within a cluster")]
#[subcommand(subcommands(ListPoolsCmd, CreatePoolCmd, ScalePoolCmd))]
pub enum NodePoolsCmd {
    #[subcommand(name = "list")]
    List(ListPoolsCmd),
    #[subcommand(name = "create")]
    Create(CreatePoolCmd),
    #[subcommand(name = "scale")]
    Scale(ScalePoolCmd),
}

impl Default for NodePoolsCmd {
    fn default() -> Self {
        Self::List(ListPoolsCmd::default())
    }
}

// Shared Flags
#[derive(Flag, Debug)]
#[flag(name = "cluster", required = true, help = "Target cluster name")]
struct ClusterFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "pool", required = true, help = "Node pool name")]
struct PoolFlag(String);

// --- List ---

#[derive(Default, App)]
#[app(name = "list", about = "List node pools")]
#[app(flags(ClusterFlag))]
#[app(action = list_pools)]
pub struct ListPoolsCmd;

#[derive(Serialize, Debug)]
struct PoolInfo {
    name: String,
    instance_type: String,
    size: u32,
    autoscaling: bool,
}

fn list_pools(ctx: State<AppContext>, cluster: FlagArg<ClusterFlag>) -> KoralResult<()> {
    ctx.client
        .log_request(&format!("List pools for cluster {}", *cluster));
    let pools = vec![
        PoolInfo {
            name: "default-pool".into(),
            instance_type: "t3.medium".into(),
            size: 3,
            autoscaling: true,
        },
        PoolInfo {
            name: "gpu-pool".into(),
            instance_type: "p3.2xlarge".into(),
            size: 1,
            autoscaling: false,
        },
    ];
    print_output(&pools, ctx.global_flags.output);
    Ok(())
}

// --- Create ---

#[derive(Flag, Debug)]
#[flag(name = "type", default = "t3.medium", help = "EC2 Instance type")]
struct InstanceTypeFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "min", default = 1, help = "Min nodes")]
struct MinNodesFlag(u32);

#[derive(Flag, Debug)]
#[flag(name = "max", default = 5, help = "Max nodes")]
struct MaxNodesFlag(u32);

#[derive(Flag, Debug)]
#[flag(name = "labels", help = "Node labels (comma separated)")]
struct LabelsFlag(String);

#[derive(Default, App)]
#[app(name = "create", about = "Create a node pool")]
#[app(flags(
    ClusterFlag,
    PoolFlag,
    InstanceTypeFlag,
    MinNodesFlag,
    MaxNodesFlag,
    LabelsFlag
))]
#[app(action = create_pool)]
pub struct CreatePoolCmd;

fn create_pool(
    ctx: State<AppContext>,
    cluster: FlagArg<ClusterFlag>,
    pool: FlagArg<PoolFlag>,
    inst_type: FlagArg<InstanceTypeFlag>,
) -> KoralResult<()> {
    ctx.client
        .log_request(&format!("Create pool {} in {}", *pool, *cluster));
    println!(
        "Creating node pool '{}' in cluster '{}' with type '{}'",
        *pool, *cluster, *inst_type
    );
    Ok(())
}

// --- Scale ---

#[derive(Flag, Debug)]
#[flag(name = "replicas", required = true, help = "Target replica count")]
struct ReplicasFlag(u32);

#[derive(Default, App)]
#[app(name = "scale", about = "Resize a node pool")]
#[app(flags(ClusterFlag, PoolFlag, ReplicasFlag))]
#[app(action = scale_pool)]
pub struct ScalePoolCmd;

fn scale_pool(
    ctx: State<AppContext>,
    cluster: FlagArg<ClusterFlag>,
    pool: FlagArg<PoolFlag>,
    replicas: FlagArg<ReplicasFlag>,
) -> KoralResult<()> {
    ctx.client.log_request(&format!(
        "Scale pool {} in {} to {}",
        *pool, *cluster, *replicas
    ));
    println!("Scaling pool '{}' to {} replicas...", *pool, *replicas);
    Ok(())
}
