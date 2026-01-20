use crate::common::print_output;
use crate::context::AppContext;
use koral::prelude::*;
use serde::Serialize;

#[derive(Default, App)]
#[app(name = "clusters", about = "Manage Kubernetes Clusters")]
#[app(subcommands(ListCmd, CreateCmd, DeleteCmd, GetCmd))]
pub struct ClustersCmd;

// --- List ---

#[derive(Flag, Debug)]
#[flag(name = "region", short = 'r', help = "Filter by region")]
struct RegionFlag(String);

#[derive(Default, App)]
#[app(name = "list", about = "List all clusters")]
#[app(flags(RegionFlag))]
#[app(action = list_clusters)]
struct ListCmd;

#[derive(Serialize, Debug)]
struct ClusterInfo {
    name: String,
    region: String,
    status: String,
    nodes: u32,
    version: String,
}

fn list_clusters(ctx: State<AppContext>, region: FlagArg<RegionFlag>) -> KoralResult<()> {
    // Simulate API call
    ctx.client
        .log_request(&format!("Listing clusters (region={:?})", *region));

    let clusters = vec![
        ClusterInfo {
            name: "prod-us-east-1".into(),
            region: "us-east-1".into(),
            status: "ACTIVE".into(),
            nodes: 50,
            version: "1.29".into(),
        },
        ClusterInfo {
            name: "staging-eu-west-1".into(),
            region: "eu-west-1".into(),
            status: "ACTIVE".into(),
            nodes: 10,
            version: "1.28".into(),
        },
    ];

    // Filter if region provided
    let filtered: Vec<_> = if !region.is_empty() {
        let r_str = &*region;
        clusters
            .into_iter()
            .filter(|c| &c.region == r_str)
            .collect()
    } else {
        clusters
    };

    print_output(&filtered, ctx.global_flags.output);
    Ok(())
}

// --- Create ---

#[derive(Clone, Debug, Copy, PartialEq)]
enum K8sVersion {
    V1_27,
    V1_28,
    V1_29,
}

impl std::str::FromStr for K8sVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.27" => Ok(K8sVersion::V1_27),
            "1.28" => Ok(K8sVersion::V1_28),
            "1.29" => Ok(K8sVersion::V1_29),
            _ => Err(format!("Unsupported version: {}", s)),
        }
    }
}
impl std::fmt::Display for K8sVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            K8sVersion::V1_27 => write!(f, "1.27"),
            K8sVersion::V1_28 => write!(f, "1.28"),
            K8sVersion::V1_29 => write!(f, "1.29"),
        }
    }
}

#[derive(Flag, Debug)]
#[flag(name = "name", required = true, help = "Cluster name")]
struct NameFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "version", default = "1.29", help = "Kubernetes version")]
struct VersionFlag(K8sVersion);

#[derive(Flag, Debug)]
#[flag(name = "node-count", default = 3, help = "Initial node count")]
struct NodeCountFlag(u32);

#[derive(Flag, Debug)]
#[flag(name = "tags", help = "Comma-separated tags (e.g. key=value)")]
struct TagsFlag(String);

#[derive(Default, App)]
#[app(name = "create", about = "Create a new cluster")]
#[app(flags(NameFlag, VersionFlag, NodeCountFlag, TagsFlag))]
#[app(action = create_cluster)]
struct CreateCmd;

fn create_cluster(
    ctx: State<AppContext>,
    name: FlagArg<NameFlag>,
    version: FlagArg<VersionFlag>,
    nodes: FlagArg<NodeCountFlag>,
    tags: FlagArg<TagsFlag>,
) -> KoralResult<()> {
    ctx.client.log_request(&format!(
        "Creating cluster {} (v{}, {} nodes)",
        *name, *version, *nodes
    ));

    if ctx.global_flags.dry_run {
        println!("(Dry Run) Would create cluster '{}'", *name);
        return Ok(());
    }

    println!("Creating cluster '{}'...", *name);
    println!("Creating cluster '{}'...", *name);
    if !tags.is_empty() {
        println!("  Tags: {}", *tags);
    }

    // Simulate long operation
    println!("  Provisioning control plane...");
    println!("  Cluster '{}' created successfully.", *name);
    Ok(())
}

// --- Delete ---

#[derive(Default, App)]
#[app(name = "delete", about = "Delete a cluster")]
#[app(flags(NameFlag))]
#[app(action = delete_cluster)]
struct DeleteCmd;

fn delete_cluster(ctx: State<AppContext>, name: FlagArg<NameFlag>) -> KoralResult<()> {
    ctx.client
        .log_request(&format!("Deleting cluster {}", *name));
    if ctx.global_flags.dry_run {
        println!("(Dry Run) Would delete cluster '{}'", *name);
        return Ok(());
    }
    println!("Deleted cluster '{}'", *name);
    Ok(())
}

// --- Get ---
#[derive(Default, App)]
#[app(name = "get", about = "Get cluster details")]
#[app(flags(NameFlag))]
#[app(action = get_cluster)]
struct GetCmd;

fn get_cluster(ctx: State<AppContext>, name: FlagArg<NameFlag>) -> KoralResult<()> {
    ctx.client.log_request(&format!("Get cluster {}", *name));
    let info = ClusterInfo {
        name: name.to_string(),
        region: "us-east-1".into(),
        status: "ACTIVE".into(),
        nodes: 50,
        version: "1.29".into(),
    };
    print_output(&info, ctx.global_flags.output);
    Ok(())
}
