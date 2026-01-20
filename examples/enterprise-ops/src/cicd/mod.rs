use koral::prelude::*;

pub mod pipelines;
pub mod runners;

#[derive(Subcommand)]
#[subcommand(name = "cicd", about = "CI/CD Orchestration")]
#[subcommand(subcommands(pipelines::PipelinesCmd, runners::RunnersCmd))]
pub enum CicdCmd {
    #[subcommand(name = "pipelines")]
    Pipelines(pipelines::PipelinesCmd),
    #[subcommand(name = "runners")]
    Runners(runners::RunnersCmd),
}

impl Default for CicdCmd {
    fn default() -> Self {
        Self::Pipelines(pipelines::PipelinesCmd::default())
    }
}
