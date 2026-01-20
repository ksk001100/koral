use crate::state::SharedState;
use koral::prelude::*;

#[derive(Default, koral::App, FromArgs)]
#[app(name = "add", action = add_task)]
pub struct AddCmd;

fn add_task(ctx: Context) -> KoralResult<()> {
    if ctx.args.is_empty() {
        println!("Error: Task description required.");
        return Ok(());
    }
    let task = ctx.args.join(" ");

    let state = ctx
        .state::<SharedState>()
        .ok_or_else(|| KoralError::Validation("State not found".to_string()))?;

    let mut guard = state
        .lock()
        .map_err(|_| KoralError::Validation("Lock poisoned".to_string()))?;
    guard.tasks.push(task.clone());

    println!("Added task: '{}'", task);
    Ok(())
}
