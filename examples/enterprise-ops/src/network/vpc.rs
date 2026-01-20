use crate::common::print_output;
use crate::context::AppContext;
use koral::prelude::*;
use serde::Serialize;

#[derive(Subcommand)]
#[subcommand(name = "vpc", about = "Manage Virtual Private Clouds")]
#[subcommand(subcommands(ListVpcCmd, CreateVpcCmd, PeeringCmd))]
pub enum VpcCmd {
    #[subcommand(name = "list")]
    List(ListVpcCmd),
    #[subcommand(name = "create")]
    Create(CreateVpcCmd),
    #[subcommand(name = "peering")]
    Peering(PeeringCmd),
}

impl Default for VpcCmd {
    fn default() -> Self {
        Self::List(ListVpcCmd::default())
    }
}

#[derive(Serialize, Debug)]
struct Vpc {
    id: String,
    cidr: String,
    region: String,
}

#[derive(Default, App)]
#[app(name = "list")]
#[app(action = list_vpcs)]
pub struct ListVpcCmd;

fn list_vpcs(ctx: State<AppContext>) -> KoralResult<()> {
    let vpcs = vec![
        Vpc {
            id: "vpc-main".into(),
            cidr: "10.0.0.0/16".into(),
            region: "us-east-1".into(),
        },
        Vpc {
            id: "vpc-staging".into(),
            cidr: "10.1.0.0/16".into(),
            region: "us-east-1".into(),
        },
    ];
    print_output(&vpcs, ctx.global_flags.output);
    Ok(())
}

#[derive(Flag, Debug)]
#[flag(name = "cidr", required = true)]
struct CidrFlag(String);

#[derive(Default, App)]
#[app(name = "create")]
#[app(flags(CidrFlag))]
#[app(action = create_vpc)]
pub struct CreateVpcCmd;

fn create_vpc(_ctx: State<AppContext>, cidr: FlagArg<CidrFlag>) -> KoralResult<()> {
    println!("Creating VPC with CIDR {}...", *cidr);
    Ok(())
}

// --- Nested Peering ---

#[derive(Subcommand)]
#[subcommand(name = "peering", about = "VPC Peering Connections")]
#[subcommand(subcommands(CreatePeerCmd))]
pub enum PeeringCmd {
    #[subcommand(name = "create")]
    Create(CreatePeerCmd),
}

impl Default for PeeringCmd {
    fn default() -> Self {
        Self::Create(CreatePeerCmd::default())
    }
}

#[derive(Flag, Debug)]
#[flag(name = "vpc-id", required = true)]
struct VpcIdFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "peer-vpc-id", required = true)]
struct PeerVpcIdFlag(String);

#[derive(Default, App)]
#[app(name = "create")]
#[app(flags(VpcIdFlag, PeerVpcIdFlag))]
#[app(action = create_peer)]
pub struct CreatePeerCmd;

fn create_peer(
    _ctx: State<AppContext>,
    vpc1: FlagArg<VpcIdFlag>,
    vpc2: FlagArg<PeerVpcIdFlag>,
) -> KoralResult<()> {
    println!(
        "Creating peering connection between {} and {}...",
        *vpc1, *vpc2
    );
    Ok(())
}
