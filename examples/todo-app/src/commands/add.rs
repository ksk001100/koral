use crate::state::SharedState;
use koral::prelude::*;

#[derive(Default, koral::App)]
#[app(name = "add", action = add_task)]
pub struct AddCmd;

fn add_task(ctx: Context) -> KoralResult<()> {
    if ctx.args.is_empty() {
        println!("Error: Task description required.");
        return Ok(());
    }
    let task = ctx.args.join(" ");

    let state = ctx.state::<SharedState>().ok_or_else(|| {
        koral::clap::Error::raw(
            koral::clap::error::ErrorKind::InvalidValue,
            "State not found",
        )
    })?;

    let mut guard = state.lock().map_err(|_| {
        koral::clap::Error::raw(koral::clap::error::ErrorKind::InvalidValue, "Lock poisoned")
    })?;
    guard.tasks.push(task.clone());

    println!("Added task: '{}'", task);
    Ok(())
}
