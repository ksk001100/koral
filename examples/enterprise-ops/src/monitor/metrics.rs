use crate::context::AppContext;
use koral::prelude::*;

#[derive(Default, App)]
#[app(name = "metrics", about = "Query metrics")]
#[app(subcommands(QueryCmd, DashboardCmd))]
pub struct MetricsCmd;

#[derive(Flag, Debug)]
#[flag(name = "query", required = true, help = "PromQL Query")]
struct QueryFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "start", help = "Start time (RFC3339)")]
struct StartTimeFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "end", help = "End time (RFC3339)")]
struct EndTimeFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "step", default = "1m", help = "Query resolution")]
struct StepFlag(String);

#[derive(Default, App)]
#[app(name = "query", about = "Run a PromQL query")]
#[app(flags(QueryFlag, StartTimeFlag, EndTimeFlag, StepFlag))]
#[app(action = query_metrics)]
struct QueryCmd;

fn query_metrics(
    ctx: State<AppContext>,
    query: FlagArg<QueryFlag>,
    start: FlagArg<StartTimeFlag>,
    end: FlagArg<EndTimeFlag>,
    step: FlagArg<StepFlag>,
) -> KoralResult<()> {
    ctx.client
        .log_request(&format!("Query metrics: {}", *query));
    println!("Executing PromQL: '{}'", *query);
    if !start.is_empty() {
        println!(
            "  Time Range: {} -> {}",
            *start,
            if end.is_empty() { "now" } else { &*end }
        );
    }
    println!("  Step: {}", *step);

    // Fake results
    println!("\nMetric                               Value");
    println!("-------------------------------------------");
    println!("up{{job=\"kubelet\"}}                   1");
    println!("node_cpu_seconds_total{{mode=\"idle\"}}   4230.1");
    Ok(())
}

// --- Dashboard ---

#[derive(Flag, Debug)]
#[flag(name = "id", required = true)]
struct DashboardIdFlag(String);

#[derive(Default, App)]
#[app(name = "dashboard", about = "Open a dashboard URL")]
#[app(flags(DashboardIdFlag))]
#[app(action = open_dashboard)]
struct DashboardCmd;

fn open_dashboard(_ctx: State<AppContext>, id: FlagArg<DashboardIdFlag>) -> KoralResult<()> {
    println!("Opening dashboard: https://grafana.internal/d/{}", *id);
    Ok(())
}
