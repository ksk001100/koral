use crate::state::SharedState;
use koral::prelude::*;

#[derive(Default, koral::App, FromArgs)]
#[app(name = "done", action = complete_task)]
pub struct DoneCmd;

fn complete_task(ctx: Context) -> KoralResult<()> {
    if let Some(id_str) = ctx.args.first() {
        if let Ok(id) = id_str.parse::<usize>() {
            let state = ctx
                .state::<SharedState>()
                .ok_or_else(|| KoralError::Validation("State not found".to_string()))?;

            let mut guard = state
                .lock()
                .map_err(|_| KoralError::Validation("Lock poisoned".to_string()))?;

            if id > 0 && id <= guard.tasks.len() {
                let removed = guard.tasks.remove(id - 1);
                println!("Marked task '{}' as done.", removed);
            } else {
                println!("Error: Invalid task ID.");
            }
        } else {
            println!("Error: Task ID must be a number.");
        }
    } else {
        println!("Error: Task ID required.");
    }
    Ok(())
}
