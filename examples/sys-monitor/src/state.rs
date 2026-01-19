use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub counter: Arc<Mutex<u32>>,
    pub db_url: String,
}
