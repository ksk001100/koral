use koral::prelude::*;

pub mod clusters;
pub mod nodepools;
pub mod workloads;

#[derive(Default, App)]
#[app(name = "k8s", about = "Manage Kubernetes clusters and resources")]
#[app(subcommands(
    clusters::ClustersCmd,
    nodepools::NodePoolsCmd,
    workloads::WorkloadsCmd
))]
pub struct K8sCmd;
