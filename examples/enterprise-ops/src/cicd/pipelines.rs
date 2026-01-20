use crate::common::print_output;
use crate::context::AppContext;
use koral::prelude::*;
use serde::Serialize;

#[derive(Subcommand)]
#[subcommand(name = "pipelines", about = "Manage Pipelines")]
#[subcommand(subcommands(ListPipelinesCmd, RunPipelineCmd))]
pub enum PipelinesCmd {
    #[subcommand(name = "list")]
    List(ListPipelinesCmd),
    #[subcommand(name = "run")]
    Run(RunPipelineCmd),
}

impl Default for PipelinesCmd {
    fn default() -> Self {
        Self::List(ListPipelinesCmd::default())
    }
}

#[derive(Serialize, Debug)]
struct Pipeline {
    id: String,
    name: String,
    last_run_status: String,
}

#[derive(Default, App)]
#[app(name = "list")]
#[app(action = list_pipelines)]
pub struct ListPipelinesCmd;

fn list_pipelines(ctx: State<AppContext>) -> KoralResult<()> {
    let pipes = vec![
        Pipeline {
            id: "build-backend".into(),
            name: "Backend Build".into(),
            last_run_status: "Success".into(),
        },
        Pipeline {
            id: "deploy-prod".into(),
            name: "Production Deploy".into(),
            last_run_status: "Failed".into(),
        },
    ];
    print_output(&pipes, ctx.global_flags.output);
    Ok(())
}

#[derive(Flag, Debug)]
#[flag(name = "id", required = true)]
struct IdFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "branch", default = "main")]
struct BranchFlag(String);

#[derive(Default, App)]
#[app(name = "run")]
#[app(flags(IdFlag, BranchFlag))]
#[app(action = run_pipeline)]
pub struct RunPipelineCmd;

fn run_pipeline(
    _ctx: State<AppContext>,
    id: FlagArg<IdFlag>,
    branch: FlagArg<BranchFlag>,
) -> KoralResult<()> {
    println!("Triggering pipeline '{}' on branch '{}'...", *id, *branch);
    Ok(())
}
