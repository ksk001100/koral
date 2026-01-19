use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct TodoState {
    pub tasks: Vec<String>,
}

pub type SharedState = Arc<Mutex<TodoState>>;
