use enterprise_ops::{context::AppContext, OpsApp};
use koral::prelude::*;

fn main() -> KoralResult<()> {
    // Initialize our shared state
    let mut state = AppContext::default();

    // Seed some mock data
    state.config.profiles.insert(
        "default".to_string(),
        enterprise_ops::context::Profile {
            region: "us-east-1".to_string(),
            account_id: "123456789012".to_string(),
        },
    );

    let mut app = OpsApp::default();
    let args: Vec<String> = std::env::args().collect();
    if let Err(e) = app.run_with_state(&mut state, args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}
