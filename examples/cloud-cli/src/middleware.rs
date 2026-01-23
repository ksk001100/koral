use crate::flags::{ProfileFlag, VerboseFlag};
use crate::state::CloudState;
use koral::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// --- User Context Extension ---
// (No longer used for injection, but keeping struct if needed or we can remove)
/*
#[derive(Debug, Clone)]
pub struct UserContext {
    #[allow(dead_code)]
    pub username: String,
    #[allow(dead_code)]
    pub token: String,
}
*/

// --- Auth Middleware ---
#[derive(Clone, Default)]
pub struct AuthMiddleware {
    pub state: CloudState,
}

impl Middleware for AuthMiddleware {
    fn before(&self, ctx: &mut Context) -> KoralResult<()> {
        // Skip auth for "login" and "help" commands
        if let Some(cmd) = ctx.args.get(0) {
            if cmd == "login" || cmd == "help" || cmd == "--help" || cmd == "-h" {
                return Ok(());
            }
        } else {
            return Ok(());
        }

        // rely on context which should be populated by Parser's EnvProvider
        let token = ctx.value_of("token").map(|s| s.to_string());

        if let Some(token_val) = token {
            // Validate using state. If valid, 'login' method updates current_user in state.
            if self.state.login(&token_val) {
                return Ok(());
            }
            // If we are here, token was provided but invalid?
            // `login` returns false if invalid.
            println!("Debug: Invalid token provided: {}", token_val);
        } else {
            // println!("Debug: No token found in env or args or flags");
        }

        Err(koral::clap::Error::raw(
            koral::clap::error::ErrorKind::InvalidValue,
            "Authentication failed: Please provide a valid --token or set CLOUD_CLI_TOKEN. Try 'login' first.",
        ))
    }
}

// --- Audit Middleware ---
#[derive(Default)]
pub struct AuditMiddleware {
    start_time: Arc<Mutex<Option<Instant>>>,
}

impl Middleware for AuditMiddleware {
    fn before(&self, ctx: &mut Context) -> KoralResult<()> {
        let verbose = ctx.get::<VerboseFlag>().unwrap_or(false);
        if verbose {
            println!("[Audit] Command started: {:?}", ctx.args);
        }

        if let Some(profile) = ctx.get::<ProfileFlag>() {
            if verbose {
                println!("[Audit] Using profile: {}", profile);
            }
        }

        *self.start_time.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn after(&self, _ctx: &mut Context) -> KoralResult<()> {
        if let Some(start) = *self.start_time.lock().unwrap() {
            println!("[Audit] Command finished in {:?}", start.elapsed());
        }
        Ok(())
    }
}
