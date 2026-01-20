use koral::prelude::*;

pub mod clusters;
pub mod nodepools;
pub mod workloads;

#[derive(Subcommand)]
#[subcommand(name = "k8s", about = "Manage Kubernetes clusters and resources")]
#[subcommand(subcommands(
    clusters::ClustersCmd,
    nodepools::NodePoolsCmd,
    workloads::WorkloadsCmd
))]
pub enum K8sCmd {
    #[subcommand(name = "clusters")]
    Clusters(clusters::ClustersCmd),
    #[subcommand(name = "nodepools")]
    NodePools(nodepools::NodePoolsCmd),
    #[subcommand(name = "workloads")]
    Workloads(workloads::WorkloadsCmd),
}

impl Default for K8sCmd {
    fn default() -> Self {
        Self::Clusters(Default::default())
    }
}
