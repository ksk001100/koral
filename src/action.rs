use crate::context::Context;

pub type Action = fn(Context) -> Result<(), Box<dyn std::error::Error>>;
