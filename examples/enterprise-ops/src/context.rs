use crate::common::OutputFormat;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct AppContext {
    pub config: Config,
    pub session: Session,
    pub client: MockApiClient,
    pub global_flags: GlobalFlags,
}

#[derive(Clone, Default, Debug)]
pub struct GlobalFlags {
    pub verbose: bool,
    pub dry_run: bool,
    pub output: OutputFormat,
    pub profile: String,
}

#[derive(Clone, Default, Debug)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
}

#[derive(Clone, Default, Debug)]
pub struct Profile {
    pub region: String,
    pub account_id: String,
}

#[derive(Clone, Default, Debug)]
pub struct Session {
    pub user_id: Option<String>,
    pub token: Option<String>,
}

#[derive(Clone, Default)]
pub struct MockApiClient {
    // In a real app, this would be a reqwest::Client or similar
    // Here we just simulate state
    pub call_log: Arc<Mutex<Vec<String>>>,
}

impl MockApiClient {
    pub fn log_request(&self, msg: &str) {
        let mut log = self.call_log.lock().unwrap();
        log.push(msg.to_string());
    }

    pub fn dump_log(&self) -> Vec<String> {
        self.call_log.lock().unwrap().clone()
    }
}
