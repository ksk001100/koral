use crate::store::Store;
use koral::prelude::*;

#[derive(App, Default, Clone, Debug, PartialEq)]
#[app(name = "set", action = set_handler, help = "Set a key-value pair")]
pub struct SetCmd;

fn set_handler(ctx: Context) -> KoralResult<()> {
    if ctx.args.len() < 2 {
        println!("Error: <key> <value> required");
        return Ok(());
    }
    let key = ctx.args[0].clone();
    let value = ctx.args[1].clone();

    let store = ctx.state::<Store>().unwrap();
    store.set(key.clone(), value).unwrap();
    println!("Set '{}'", key);
    Ok(())
}

#[derive(App, Default, Clone, Debug, PartialEq)]
#[app(name = "get", action = get_handler, help = "Get a value by key")]
pub struct GetCmd;

fn get_handler(ctx: Context) -> KoralResult<()> {
    if ctx.args.is_empty() {
        println!("Error: <key> required");
        return Ok(());
    }
    let key = &ctx.args[0];
    let store = ctx.state::<Store>().unwrap();
    match store.get(key) {
        Some(val) => println!("{}", val),
        None => println!("(not found)"),
    }
    Ok(())
}

#[derive(App, Default, Clone, Debug, PartialEq)]
#[app(name = "del", action = del_handler, help = "Delete a key")]
pub struct DelCmd;

fn del_handler(ctx: Context) -> KoralResult<()> {
    if ctx.args.is_empty() {
        println!("Error: <key> required");
        return Ok(());
    }
    let key = &ctx.args[0];
    let store = ctx.state::<Store>().unwrap();
    if store.delete(key).unwrap() {
        println!("Deleted '{}'", key);
    } else {
        println!("Key '{}' not found", key);
    }
    Ok(())
}

#[derive(App, Default, Clone, Debug, PartialEq)]
#[app(name = "list", action = list_handler, help = "List all keys")]
pub struct ListCmd;

fn list_handler(ctx: Context) -> KoralResult<()> {
    let store = ctx.state::<Store>().unwrap();
    let items = store.list();
    if items.is_empty() {
        println!("(empty)");
    } else {
        for (k, v) in items {
            println!("{}: {}", k, v);
        }
    }
    Ok(())
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum Commands {
    #[subcommand(name = "set")]
    Set(SetCmd),
    #[subcommand(name = "get")]
    Get(GetCmd),
    #[subcommand(name = "del")]
    Del(DelCmd),
    #[subcommand(name = "list", aliases = "ls")]
    List(ListCmd),
}

impl Default for Commands {
    fn default() -> Self {
        Self::List(ListCmd::default())
    }
}
