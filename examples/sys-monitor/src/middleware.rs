use koral::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// Static Middleware (Timing)
#[derive(Default)]
pub struct TimingMiddleware {
    pub start: Arc<Mutex<Option<Instant>>>,
}

impl Middleware for TimingMiddleware {
    fn before(&self, _ctx: &mut Context) -> KoralResult<()> {
        *self.start.lock().unwrap() = Some(Instant::now());
        Ok(())
    }

    fn after(&self, _ctx: &mut Context) -> KoralResult<()> {
        if let Some(start) = *self.start.lock().unwrap() {
            println!("[TimingMiddleware] Execution took: {:?}", start.elapsed());
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub role: String,
}

// Configurable Middleware (Injected)
#[derive(Clone)]
pub struct AuthMiddleware {
    pub api_key: String,
}

impl Middleware for AuthMiddleware {
    fn before(&self, ctx: &mut Context) -> KoralResult<()> {
        println!("[AuthMiddleware] Checking API Key: {}", self.api_key);
        // Simulate authentication and user resolution
        let user = User {
            name: "Alice".to_string(),
            role: "Admin".to_string(),
        };
        ctx.insert_extension(user);
        Ok(())
    }
}
