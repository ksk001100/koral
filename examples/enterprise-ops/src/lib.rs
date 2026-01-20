#![allow(dead_code, unused_variables)]
use crate::common::OutputFormat;
// use crate::context::{AppContext, Config, GlobalFlags, MockApiClient, Profile, Session};
use koral::prelude::*;

pub mod common;
pub mod context;

// Domain Modules
pub mod cicd;
pub mod db;
pub mod iam;
pub mod k8s;
pub mod monitor;
pub mod network;

// --- Global Flags ---

#[derive(Flag, Debug)]
#[flag(name = "verbose", short = 'v', help = "Enable verbose output")]
pub struct Verbose(bool);

#[derive(Flag, Debug)]
#[flag(name = "dry-run", help = "Preview changes without executing")]
pub struct DryRun(bool);

#[derive(Flag, Debug)]
#[flag(
    name = "output",
    short = 'o',
    default = "text",
    help = "Output format (text, json, yaml, table)"
)]
pub struct Output(OutputFormat);

#[derive(Flag, Debug)]
#[flag(
    name = "profile",
    default = "default",
    help = "Configuration profile to use"
)]
pub struct ProfileFlag(String);

// --- Middleware ---

#[derive(Clone, Default)]
struct AuthMiddleware;

impl Middleware for AuthMiddleware {
    fn before(&self, _ctx: &mut Context) -> KoralResult<()> {
        // "Load" the state from flags into the context
        // In a real app, we would extract flags here and set up the context
        // But koral passes Context to handler, so we need a way to initialize it.
        // Actually, koral's Context holds `Box<dyn Any>`, so we can inject our AppContext.

        // This middleware is a placeholder for logic that runs before *every* command.
        // For example, verifying that a user is logged in if the command requires it.
        Ok(())
    }
}

#[derive(App, Default)]
#[app(
    name = "ops",
    version = "1.0.0",
    author = "Platform Team",
    about = "Enterprise Platform CLI"
)]
#[app(flags(Verbose, DryRun, Output, ProfileFlag))]
#[app(subcommands(
    k8s::K8sCmd,
    db::DbCmd,
    cicd::CicdCmd,
    monitor::MonitorCmd,
    iam::IamCmd,
    network::NetworkCmd
))]
#[app(middleware(AuthMiddleware))]
pub struct OpsApp;
