use koral::prelude::*;

pub mod pipelines;
pub mod runners;

#[derive(Default, App)]
#[app(name = "cicd", about = "CI/CD Orchestration")]
#[app(subcommands(pipelines::PipelinesCmd, runners::RunnersCmd))]
pub struct CicdCmd;
