use crate::common::print_output;
use crate::context::AppContext;
use koral::prelude::*;
use serde::Serialize;
use std::{thread, time::Duration};

#[derive(Default, App)]
#[app(name = "workloads", about = "Inspect workloads (pods, deployments)")]
#[app(subcommands(ListWorkloadsCmd, LogsCmd, ExecCmd))]
pub struct WorkloadsCmd;

#[derive(Flag, Debug)]
#[flag(
    name = "namespace",
    short = 'n',
    default = "default",
    help = "Kubernetes namespace"
)]
struct NamespaceFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "pod", required = true, help = "Pod name")]
struct PodFlag(String);

// --- List ---

#[derive(Default, App)]
#[app(name = "list", about = "List workloads in a namespace")]
#[app(flags(NamespaceFlag))]
#[app(action = list_workloads)]
struct ListWorkloadsCmd;

#[derive(Serialize, Debug)]
struct PodInfo {
    name: String,
    status: String,
    restarts: u32,
    age: String,
}

fn list_workloads(ctx: State<AppContext>, ns: FlagArg<NamespaceFlag>) -> KoralResult<()> {
    let pods = vec![
        PodInfo {
            name: "api-server-xyz".into(),
            status: "Running".into(),
            restarts: 0,
            age: "2d".into(),
        },
        PodInfo {
            name: "worker-abc".into(),
            status: "CrashLoopBackOff".into(),
            restarts: 5,
            age: "10m".into(),
        },
    ];

    println!("Resources in namespace '{}':", *ns);
    print_output(&pods, ctx.global_flags.output);
    Ok(())
}

// --- Logs ---

#[derive(Flag, Debug)]
#[flag(name = "follow", short = 'f', help = "Stream logs")]
struct FollowFlag(bool);

#[derive(Flag, Debug)]
#[flag(name = "tail", default = 10, help = "Number of lines to show")]
struct TailFlag(u32);

#[derive(Default, App)]
#[app(name = "logs", about = "Get pod logs")]
#[app(flags(NamespaceFlag, PodFlag, FollowFlag, TailFlag))]
#[app(action = get_logs)]
struct LogsCmd;

fn get_logs(
    _ctx: State<AppContext>,
    ns: FlagArg<NamespaceFlag>,
    pod: FlagArg<PodFlag>,
    follow: FlagArg<FollowFlag>,
) -> KoralResult<()> {
    println!("Fetching logs for {}/{}...", *ns, *pod);

    println!("[INFO] Starting application...");
    println!("[INFO] Connected to database.");
    println!("[WARN] High latency detected.");

    if *follow {
        println!("(streaming - press Ctrl+C to stop)");
        for i in 0..5 {
            thread::sleep(Duration::from_millis(500));
            println!("[INFO] Processed request #{}", i);
        }
    }

    Ok(())
}

// --- Exec ---

#[derive(Flag, Debug)]
#[flag(name = "container", short = 'c', help = "Container name")]
struct ContainerFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "command", required = true, help = "Command to execute")]
struct CommandFlag(String);

#[derive(Default, App)]
#[app(name = "exec", about = "Execute a command in a pod")]
#[app(flags(NamespaceFlag, PodFlag, ContainerFlag, CommandFlag))]
#[app(strict = false)] // Allow args after -- to be passed
#[app(action = exec_pod)]
struct ExecCmd;

fn exec_pod(
    ctx: State<AppContext>,
    pod: FlagArg<PodFlag>,
    cmd: FlagArg<CommandFlag>,
) -> KoralResult<()> {
    ctx.client
        .log_request(&format!("Exec in {}: {:?}", *pod, *cmd));
    println!("Executing command in pod '{}'...", *pod);
    println!("> {:?}", *cmd);
    println!("(Mock output) /bin/sh: command executed successfully");
    Ok(())
}
