use koral::prelude::*;

pub mod logs;
pub mod metrics;

#[derive(Subcommand)]
#[subcommand(name = "monitor", about = "Platform Observability")]
#[subcommand(subcommands(metrics::MetricsCmd, logs::LogsCmd))]
pub enum MonitorCmd {
    Metrics(metrics::MetricsCmd),
    Logs(logs::LogsCmd),
}
