use crate::common::print_output;
use crate::context::AppContext;
use koral::prelude::*;
use serde::Serialize;

#[derive(Default, App)]
#[app(name = "postgres", about = "Manage Postgres instances")]
#[app(subcommands(ListInstancesCmd, CreateInstanceCmd, BackupsCmd))]
pub struct PostgresCmd;

// --- List ---

#[derive(Default, App)]
#[app(name = "list", about = "List instances")]
#[app(action = list_db_instances)]
struct ListInstancesCmd;

#[derive(Serialize, Debug)]
struct DbInstance {
    id: String,
    engine: String,
    status: String,
    storage_gb: u32,
}

fn list_db_instances(ctx: State<AppContext>) -> KoralResult<()> {
    ctx.client.log_request("List DB instances");
    let dbs = vec![
        DbInstance {
            id: "pg-primary".into(),
            engine: "postgres-16".into(),
            status: "Available".into(),
            storage_gb: 100,
        },
        DbInstance {
            id: "pg-analytics".into(),
            engine: "postgres-15".into(),
            status: "Maintenance".into(),
            storage_gb: 500,
        },
    ];
    print_output(&dbs, ctx.global_flags.output);
    Ok(())
}

// --- Create ---

#[derive(Flag, Debug)]
#[flag(name = "name", required = true, help = "DB identifier")]
struct NameFlag(String);

#[derive(Flag, Debug)]
#[flag(name = "storage", default = 20, help = "Storage size in GB")]
struct StorageFlag(u32);

#[derive(Default, App)]
#[app(name = "create", about = "Provision new postgres instance")]
#[app(flags(NameFlag, StorageFlag))]
#[app(action = create_db)]
struct CreateInstanceCmd;

fn create_db(
    _ctx: State<AppContext>,
    name: FlagArg<NameFlag>,
    size: FlagArg<StorageFlag>,
) -> KoralResult<()> {
    println!(
        "Provisioning Postgres instance '{}' with {} GB storage...",
        *name, *size
    );
    Ok(())
}

// --- Backups (Nested Subcommand) ---

#[derive(Default, App)]
#[app(name = "backups", about = "Manage database backups")]
#[app(subcommands(ListBackups, CreateBackup))]
struct BackupsCmd;

#[derive(Default, App)]
#[app(name = "list")]
#[app(flags(NameFlag))]
#[app(action = list_backups)]
struct ListBackups;

fn list_backups(_ctx: State<AppContext>, db_name: FlagArg<NameFlag>) -> KoralResult<()> {
    println!("Backups for {}:", *db_name);
    println!("- backup-2023-10-01 (Full)");
    println!("- backup-2023-10-02 (Incremental)");
    Ok(())
}

#[derive(Default, App)]
#[app(name = "create")]
#[app(flags(NameFlag))]
#[app(action = create_backup)]
struct CreateBackup;

fn create_backup(_ctx: State<AppContext>, db_name: FlagArg<NameFlag>) -> KoralResult<()> {
    println!("Starting backup for {}...", *db_name);
    Ok(())
}
