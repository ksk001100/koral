use crate::common::print_output;
use crate::context::AppContext;
use koral::prelude::*;
use serde::Serialize;

#[derive(Default, App)]
#[app(name = "vpc", about = "Manage Virtual Private Clouds")]
#[app(subcommands(ListVpcCmd, CreateVpcCmd, PeeringCmd))]
pub struct VpcCmd;

#[derive(Serialize, Debug)]
struct Vpc {
    id: String,
    cidr: String,
    region: String,
}

#[derive(Default, App)]
#[app(name = "list")]
#[app(action = list_vpcs)]
struct ListVpcCmd;

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
struct CreateVpcCmd;

fn create_vpc(_ctx: State<AppContext>, cidr: FlagArg<CidrFlag>) -> KoralResult<()> {
    println!("Creating VPC with CIDR {}...", *cidr);
    Ok(())
}

// --- Nested Peering ---

#[derive(Default, App)]
#[app(name = "peering", about = "VPC Peering Connections")]
#[app(subcommands(CreatePeerCmd))]
struct PeeringCmd;

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
struct CreatePeerCmd;

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
